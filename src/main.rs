use std::error::Error;
use plotters::prelude::*;
use rand::{Rng, RngExt};

const L: usize = 30;
const P: f64 = 0.5;
const CELL: usize = 20;
const MARGIN: usize = 30;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let size = 2 * MARGIN + (L - 1)* CELL;
    let size = size as u32;
    let root = SVGBackend::new("img.svg", (size, size))
        .into_drawing_area();

    root.fill(&WHITE)?;

    // Horizontal
    for y in 0..L {
        for x in 0..L - 1 {
            let x1 = (MARGIN + x * CELL) as i32;
            let y1 = (MARGIN + y * CELL) as i32;
            let x2 = x1 + CELL as i32;

            let color = if rng.random_bool(P) {
                BLACK.stroke_width(2)
            } else {
                RGBColor(220, 220, 220).stroke_width(1)
            };

            root.draw(&PathElement::new(
                    vec![(x1, y1), (x2, y1)],
                    color
            ))?;
        }
    }

    // Vertical
    for y in 0..L - 1 {
        for x in 0..L {
            let x1 = (MARGIN + x * CELL) as i32;
            let y1 = (MARGIN + y * CELL) as i32;
            let y2 = y1 + CELL as i32;

            let color = if rng.random_bool(P) {
                BLACK.stroke_width(2)
            } else {
                RGBColor(220, 220, 220).stroke_width(1)
            };

            root.draw(&PathElement::new(
                    vec![(x1, y1), (x1, y2)],
                    color
            ))?;
        }
    }

    root.present()?;

    Ok(())
}
