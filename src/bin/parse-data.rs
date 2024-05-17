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

    let data: model::Data = serde_json::from_reader(reader).unwrap();

    println!("{}", data.parameter);
    println!("{}", data.station);
    println!("{}", data.period);

    for position in data.position {
        println!(
            "{} {} {} {} - {}",
            position.latitude,
            position.longitude,
            position.height,
            position.from.format("%Y-%m-%d"),
            position.to.format("%Y-%m-%d")
        );
    }

    for sample in data.value {
        println!("{} {} {:?}", sample.date, sample.value, sample.quality);
    }
}
