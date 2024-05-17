use clap::Parser;
use smhi_observations::model;
use std::fs::File;
use std::io::BufReader;

/// Parse measurements
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename
    #[arg(short, long)]
    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.file).unwrap();
    let reader = BufReader::new(file);

    let stations: model::StationsResponse = serde_json::from_reader(reader).unwrap();

    const FORMAT_DATE_TIME: &str = "%Y-%m-%d %H:%M:%S";
    const FORMAT_DATE: &str = "%Y-%m-%d";

    for station in stations.station {
        if station.active {
            println!(
                "{:8} {:8} {:32} {:64} {:8?} {:?} {} {} {}",
                station.id,
                station.key,
                station.name,
                station.owner,
                station.owner_category,
                station.measuring_network,
                station.updated.format(FORMAT_DATE_TIME),
                station.from.format(FORMAT_DATE),
                station.to.format(FORMAT_DATE)
            );
        }
    }
}
