use clap::Parser;
use smhi_observations::model::{Parameter, StationResponse};

/// Get stations
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Parameter to fetch
    #[arg(short, long)]
    parameter: Parameter,

    /// Station to fetch values from
    #[arg(short, long)]
    station: u32,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let station = smhi_observations::StationComponent::Station(args.station);
    let component = smhi_observations::ApiComponent::Station {
        parameter: args.parameter,
        station,
    };

    const FORMAT_DATE_TIME: &str = "%Y-%m-%d %H:%M:%S";

    let response =
        smhi_observations::request_json_component::<StationResponse>(&component, true).await;

    match response {
        Ok(response) => {
            println!(
                "{:8} {} {}",
                response.key,
                response
                    .updated
                    .with_timezone(&chrono::Local)
                    .format(FORMAT_DATE_TIME),
                response.title
            );
        }
        Err(error) => {
            eprintln!("Request failed {}", error.to_string());
        }
    }
}
