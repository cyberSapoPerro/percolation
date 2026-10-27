mod random_lattice;

use random_lattice::RandomLatice;

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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if !(0.0..=1.0).contains(&cli.parameter) {
        return Err("p should be in [0, 1]".into());
    }

    if cli.length < 2 {
        return Err("L should be > 2".into());
    }

    let mut lattice = RandomLatice::new(cli.length, cli.parameter)?;
    lattice.cluster_lattice();
    lattice.graph_lattice(cli.output)?;

    Ok(())
}
