use std::error::Error;
use plotters::prelude::*;
use rand::{Rng, RngExt};

const L: usize = 30;
const P: f64 = 0.5;
const CELL: usize = 20;
const MARGIN: usize = 30;

struct RandomLatice {
    horizontal_edges: Vec<Vec<bool>>,
    vertical_edges: Vec<Vec<bool>>,
}

fn generate_random_latice() -> RandomLatice {
    let mut rng = rand::rng();

    let mut horizontal_edges = vec![vec![false; L - 1]; L];
    let mut vertical_edges = vec![vec![false; L]; L - 1];

    for row in &mut horizontal_edges {
        for edge in row.iter_mut() {
            *edge = rng.random_bool(P);
        }
    }

    for row in &mut vertical_edges {
        for edge in row.iter_mut() {
            *edge = rng.random_bool(P);
        }
    }

    RandomLatice {
        horizontal_edges,
        vertical_edges,
    }

}

fn generate_graph(latice: RandomLatice) -> Result<(), Box<dyn Error>>{

    let size = 2 * MARGIN + (L - 1)* CELL;
    let size = size as u32;
    let root = SVGBackend::new("img.svg", (size, size))
        .into_drawing_area();

    root.fill(&WHITE)?;

    // Horizontal edges
    for y in 0..L {
        for x in 0..L - 1 {

            if !latice.horizontal_edges[y][x] {
                continue;
            }

            let x1 = (MARGIN + x * CELL) as i32;
            let y1 = (MARGIN + y * CELL) as i32;
            let x2 = x1 + CELL as i32;

            let color = BLACK.stroke_width(2);
            // } else {
            //     RGBColor(220, 220, 220).stroke_width(1)
            // };
            //
            root.draw(&PathElement::new(
                    vec![(x1, y1), (x2, y1)],
                    color
            ))?;
        }
    }

    // Vertical edges
    for y in 0..L - 1 {
        for x in 0..L {
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

fn main() -> Result<(), Box<dyn Error>> {
    let latice = generate_random_latice();
    generate_graph(latice)?;

    Ok(())
}
