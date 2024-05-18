use clap::Parser;

/// Get stations
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Parameter to fetch
    #[arg(short, long)]
    parameter: smhi_observations::model::Parameter,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let parameter = smhi_observations::model::Parameter::from(args.parameter);

    const FORMAT_DATE_TIME: &str = "%Y-%m-%d %H:%M:%S";
    const FORMAT_DATE: &str = "%Y-%m-%d";

    let response = smhi_observations::request_stations(parameter, true).await;
    match response {
        Ok(response) => {
            for station in response.station {
                if station.active {
                    println!(
                        "{:8} {:8} {:32} {:64} {:8?} {:?} {} {} {}",
                        station.id,
                        station.key,
                        station.name,
                        station.owner,
                        station.owner_category,
                        station.measuring_network,
                        station
                            .updated
                            .with_timezone(&chrono::Local)
                            .format(FORMAT_DATE_TIME),
                        station
                            .from
                            .with_timezone(&chrono::Local)
                            .format(FORMAT_DATE),
                        station.to.with_timezone(&chrono::Local).format(FORMAT_DATE)
                    );
                }
            }
        }
        Err(error) => {
            eprintln!("Request failed {}", error.to_string());
        }
    }
}
