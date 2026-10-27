mod random_lattice;
mod plot;

use random_lattice::RandomLatice;
use plot::graph_lattice;
use plot::gnu_plot_hist;

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

    let mut data: Vec<usize> = Vec::new();
    for _ in 0..1000 {
        let mut lattice = RandomLatice::new(cli.length, cli.parameter)?;
        lattice.label_clusters();
        let clusters_data = lattice.get_clusters_data();
        data.extend(&clusters_data);
    }
    gnu_plot_hist(data)?;

    Ok(())
}
