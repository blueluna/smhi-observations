use clap::Parser;
use smhi_observations::model;

/// Get measurements
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Parameter to fetch
    #[arg(short, long)]
    parameter: model::Parameter,

    /// Station to fetch values from
    #[arg(short, long)]
    station: u32,

    /// Period to fetch data for
    #[arg(short = 't', long)]
    period: model::Period,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

    let station = smhi_observations::StationComponent::Station(args.station);

    match smhi_observations::request_data(args.parameter, station, args.period, true).await {
        Ok(data) => {
            println!("{}", data.parameter);
            println!("{}", data.station);
            println!("{}", data.period);

            for position in data.position {
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

            for sample in data.value {
                println!(
                    "{} {} {:?}",
                    sample
                        .date
                        .with_timezone(&chrono::Local)
                        .format("%Y-%m-%d %H:%M:%S"),
                    sample.value,
                    sample.quality
                );
            }
        }
        Err(error) => {
            eprintln!("Request failed {}", error.to_string());
        }
    }
}
