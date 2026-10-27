use plotters::prelude::*;
use rand::RngExt;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    length: usize,
    #[arg(short)]
    parameter: f64,
    #[arg(short, long)]
    output: String,
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size:vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
    }

    fn cluster_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

struct RandomLatice {
    length: usize,
    horizontal_edges: Vec<Vec<i32>>,
    vertical_edges: Vec<Vec<i32>>,
    union_find: UnionFind,
}

impl RandomLatice {
    fn new(length: usize, parameter: f64) -> Self {
        let mut rng = rand::rng();
        let mut horizontal_edges = vec![vec![0; length - 1]; length];
        let mut vertical_edges = vec![vec![0; length]; length - 1];
        for row in &mut horizontal_edges {
            for edge in row.iter_mut() {
                *edge = if rng.random_bool(parameter) {1} else {-1}
            }
        }
        for row in &mut vertical_edges {
            for edge in row.iter_mut() {
                *edge = if rng.random_bool(parameter) {1} else {-1}
            }
        }
        Self {
            length,
            horizontal_edges: horizontal_edges,
            vertical_edges: vertical_edges,
            union_find: UnionFind::new(length*length),
        }
    }

    fn vertex_id(&self, row: usize, col: usize) -> usize {
        row * self.length + col
    }

    fn label_clusters(&mut self) {
        for (row, edges) in self.horizontal_edges.iter().enumerate() {
            for (col, &is_open) in edges.iter().enumerate() {
                if is_open == 1 {
                    let u = self.vertex_id(row, col);
                    let v = self.vertex_id(row, col + 1);
                    self.union_find.union(u, v);
                }
            }
        }
        for (row, edges) in self.vertical_edges.iter().enumerate() {
            for (col, &is_open) in edges.iter().enumerate() {
                if is_open == 1 {
                    let u = self.vertex_id(row, col);
                    let v = self.vertex_id(row + 1, col);
                    self.union_find.union(u, v);
                }
            }
        }
    }

    fn cluster_lattice(&mut self) {
        self.label_clusters();
        for row in 0..self.length {
            for col in 0..self.length - 1 {
                if self.horizontal_edges[row][col] == 1 {
                    let u = self.vertex_id(row, col);
                    self.horizontal_edges[row][col] = self.union_find.find(u) as i32;
                }
            }
        }
        for row in 0..self.length - 1 {
            for col in 0..self.length {
                if self.vertical_edges[row][col] == 1 {
                    let u = self.vertex_id(row, col);
                    self.vertical_edges[row][col] = self.union_find.find(u) as i32;
                }
            }
        }

    }

    fn graph_lattice(&self, output_path: String) -> Result<(), Box<dyn std::error::Error>>  {
        let length = self.length;
        let cell: usize = 20;
        let margin: usize = 30;
        let size = 2 * margin + (length - 1)* cell;
        let size = size as u32;
        let root = SVGBackend::new(&output_path, (size, size))
            .into_drawing_area();
        root.fill(&WHITE)?;

        // Horizontal edges
        for y in 0..length {
            for x in 0..length - 1 {
                if self.horizontal_edges[y][x] == -1 {
                    continue;
                }
                let x1 = (margin + x * cell) as i32;
                let y1 = (margin + y * cell) as i32;
                let x2 = x1 + cell as i32;
                let color = Palette99::pick(self.horizontal_edges[y][x] as usize).stroke_width(2);
                root.draw(&PathElement::new(
                        vec![(x1, y1), (x2, y1)],
                        color
                ))?;
            }
        }

        // Vertical edges
        for y in 0..length - 1 {
            for x in 0..length {
                if self.vertical_edges[y][x] == -1 {
                    continue;
                }
                let x1 = (margin + x * cell) as i32;
                let y1 = (margin + y * cell) as i32;
                let y2 = y1 + cell as i32;
                let color = Palette99::pick(self.vertical_edges[y][x] as usize).stroke_width(2);
                root.draw(&PathElement::new(
                        vec![(x1, y1), (x1, y2)],
                        color
                ))?;
            }
        }

        root.present()?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if !(0.0..=1.0).contains(&cli.parameter) {
        return Err("p should be in [0, 1]".into());
    }

    if cli.length < 2 {
        return Err("L should be > 2".into());
    }

    let mut lattice = RandomLatice::new(cli.length, cli.parameter);
    lattice.cluster_lattice();
    lattice.graph_lattice(cli.output)?;

    Ok(())
}
