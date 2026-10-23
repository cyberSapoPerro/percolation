use plotters::prelude::*;
use rand::RngExt;
use clap::Parser;

const CELL: usize = 20;
const MARGIN: usize = 30;

struct RandomLatice {
    horizontal_edges: Vec<Vec<bool>>,
    vertical_edges: Vec<Vec<bool>>,
}

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    length: usize,
    #[arg(short)]
    parameter: f64,
}

fn generate_random_latice(length: usize, parameter: f64) -> RandomLatice {
    let mut rng = rand::rng();

    let mut horizontal_edges = vec![vec![false; length - 1]; length];
    let mut vertical_edges = vec![vec![false; length]; length - 1];

    for row in &mut horizontal_edges {
        for edge in row.iter_mut() {
            *edge = rng.random_bool(parameter);
        }
    }

    for row in &mut vertical_edges {
        for edge in row.iter_mut() {
            *edge = rng.random_bool(parameter);
        }
    }

    RandomLatice {
        horizontal_edges,
        vertical_edges,
    }

}

fn generate_graph(latice: RandomLatice, length: usize) 
    -> Result<(), Box<dyn std::error::Error>> {

    let size = 2 * MARGIN + (length - 1)* CELL;
    let size = size as u32;
    let root = SVGBackend::new("img.svg", (size, size))
        .into_drawing_area();

    root.fill(&WHITE)?;

    // Horizontal edges
    for y in 0..length {
        for x in 0..length - 1 {
            if !latice.horizontal_edges[y][x] {
                continue;
            }
            let x1 = (MARGIN + x * CELL) as i32;
            let y1 = (MARGIN + y * CELL) as i32;
            let x2 = x1 + CELL as i32;
            let color = BLACK.stroke_width(2);
            root.draw(&PathElement::new(
                    vec![(x1, y1), (x2, y1)],
                    color
            ))?;
        }
    }

    // Vertical edges
    for y in 0..length - 1 {
        for x in 0..length {
            if !latice.vertical_edges[y][x] {
                continue;
            }
            let x1 = (MARGIN + x * CELL) as i32;
            let y1 = (MARGIN + y * CELL) as i32;
            let y2 = y1 + CELL as i32;
            let color = BLACK.stroke_width(2);
            root.draw(&PathElement::new(
                    vec![(x1, y1), (x1, y2)],
                    color
            ))?;
        }
    }

    root.present()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if !(0.0..=1.0).contains(&cli.parameter) {
        return Err("p should be in [0, 1]".into());
    }

    if cli.length < 2 {
        return Err("L should be > 2".into());
    }

    let latice = generate_random_latice(cli.length, cli.parameter);
    generate_graph(latice, cli.length)?;

    Ok(())
}
