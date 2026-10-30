use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use bond_percolation::Lattice;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    length: usize,
    #[arg(short)]
    parameter: f64,
    #[arg(short, long)]
    iter: i32,
}

fn save_cluster_data(data: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("histo.dat")?;
    let mut writer = BufWriter::new(file);
    for size in &data {
        writeln!(writer, "{size}")?;
    }
    drop(writer);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut data: Vec<usize> = Vec::new();
    for _ in 0..cli.iter {
        let clusters_data = Lattice::new(cli.length, cli.parameter)?
            .get_clusters_size();
        data.extend(&clusters_data);
    }

    save_cluster_data(data)?;

    Ok(())
}
