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

    let response = smhi_observations::request_stations(parameter, true).await;
    match response {
        Ok(response) => {
            for station in response.station {
                if station.active {
                    println!(
                        "{:8} {:32} {:?}",
                        station.id, station.name, station.measuring_network
                    );
                }
            }
        }
        Err(error) => {
            eprintln!("Request failed {}", error.to_string());
        }
    }
}
