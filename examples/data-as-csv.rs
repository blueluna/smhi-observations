use clap::Parser;
use smhi_observations::model;
use smhi_observations::model::Updated;
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

    for position in data.position.iter() {
        println!(
            "{} {} {} {} - {}",
            position.latitude,
            position.longitude,
            position.height,
            position
                .from
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d"),
            position.to.with_timezone(&chrono::Local).format("%Y-%m-%d")
        );
    }

    let parameter_label = data.parameter.key.to_string();

    let update = data.updated();
    let filename = format!("{}-{}-{}.csv", parameter_label, data.station.key, update.with_timezone(&chrono::Local).format("%Y%m%d-%H%M%S"));

    let file = File::create_new(filename).unwrap();
    let mut csv = csv::Writer::from_writer(file);
    let _ = csv.write_record(["timestamp", &parameter_label]);

    for sample in data.value {
        let record = vec![
            sample.date.with_timezone(&chrono::Local).to_rfc3339(),
            sample.value,
        ];
        let _ = csv.write_record(&record);
    }
}
