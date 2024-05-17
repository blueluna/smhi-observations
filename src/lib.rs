mod error;
pub mod model;

pub use error::Error;
use model::Parameter;

const BASE_URL: &str = "https://opendata-download-metobs.smhi.se/api/version/1.0";

fn stringify<T: serde::ser::Serialize>(value: T) -> String {
    if let Ok(string) = serde_json::ser::to_string(&value) {
        let string = string.trim_matches('"').to_string();
        string
    } else {
        String::new()
    }
}

fn http_headers(media_type: model::MediaType) -> reqwest::header::HeaderMap {
    let media_type_string = reqwest::header::HeaderValue::from_str(&stringify(media_type)).unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", media_type_string.clone());
    headers.insert("Accept", media_type_string.clone());
    return headers;
}

type StationIdentifier = u32;
type StationSetIdentifier = String;

pub enum StationComponent {
    Station(StationIdentifier),
    StationSet(StationSetIdentifier),
}

impl StationComponent {
    pub fn url_part(&self) -> String {
        match self {
            Self::Station(station) => {
                format!("/station/{}", station)
            }
            Self::StationSet(station) => {
                format!("/station-set/{}", station)
            }
        }
    }

    pub fn filename_part(&self) -> String {
        match self {
            Self::Station(station) => {
                format!("{}", station)
            }
            Self::StationSet(station) => {
                format!("{}", station)
            }
        }
    }
}

pub enum ApiComponent {
    Parameter {
        parameter: Parameter,
    },
    Station {
        parameter: Parameter,
        station: StationComponent,
    },
    Period {
        parameter: Parameter,
        station: StationComponent,
        period: model::Period,
    },
    Data {
        parameter: Parameter,
        station: StationComponent,
        period: model::Period,
    },
}

impl ApiComponent {
    pub fn build_json_url(&self) -> String {
        let mut url = String::from(BASE_URL);
        match self {
            Self::Parameter { parameter } => {
                url.push_str(&format!("/parameter/{}", u8::from(parameter)));
            }
            Self::Station { parameter, station } => {
                url.push_str(&format!(
                    "/parameter/{}{}",
                    u8::from(parameter),
                    station.url_part()
                ));
            }
            Self::Period {
                parameter,
                station,
                period,
            } => {
                url.push_str(&format!(
                    "/parameter/{}{}/period/{}",
                    u8::from(parameter),
                    station.url_part(),
                    period.to_url()
                ));
            }
            Self::Data {
                parameter,
                station,
                period,
            } => {
                url.push_str(&format!(
                    "/parameter/{}{}/period/{}/data",
                    u8::from(parameter),
                    station.url_part(),
                    period.to_url()
                ));
            }
        }
        url.push_str(".json");
        url
    }

    pub fn build_filename(&self) -> String {
        match self {
            Self::Parameter { parameter } => {
                format!("stations-{}", u8::from(parameter))
            }
            Self::Station { parameter, station } => {
                format!(
                    "station-{}-{}",
                    u8::from(parameter),
                    station.filename_part()
                )
            }
            Self::Period {
                parameter,
                station,
                period,
            } => {
                format!(
                    "period-{}-{}-{}",
                    u8::from(parameter),
                    station.filename_part(),
                    period.to_url()
                )
            }
            Self::Data {
                parameter,
                station,
                period,
            } => {
                format!(
                    "data-{}-{}-{}",
                    u8::from(parameter),
                    station.filename_part(),
                    period.to_url()
                )
            }
        }
    }
}

pub async fn request_json<T>(url: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + model::Updated,
{
    let headers = http_headers(model::MediaType::Json);

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .https_only(true)
        .gzip(true)
        .timeout(std::time::Duration::new(60, 0))
        .build()?;

    let response = client.get(url).send().await?;

    match response.error_for_status_ref() {
        Ok(_) => {
            let text = response.text().await?;

            let body = serde_json::from_str(&text)?;

            Ok(body)
        }
        Err(error) => {
            println!("Error {:?}", &error);
            Err(error.into())
        }
    }
}

pub async fn request_json_component<T>(component: &ApiComponent, dump: bool) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + model::Updated,
{
    let url = component.build_json_url();
    let headers = http_headers(model::MediaType::Json);

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .https_only(true)
        .gzip(true)
        .timeout(std::time::Duration::new(60, 0))
        .build()?;

    let response = client.get(url).send().await?;

    match response.error_for_status_ref() {
        Ok(_) => {
            let text = response.text().await?;

            let body: T = serde_json::from_str(&text)?;

            if dump {
                let update = body.updated();
                let datetime = update.format("%Y%m%d-%H%M%S");
                let mut filepath = component.build_filename();
                filepath += &format!("-{}.json", datetime);
                println!("Dump: {}", filepath);
                let _ = std::fs::write(filepath, &text);
            }

            Ok(body)
        }
        Err(error) => {
            println!("Error {:?}", &error);
            Err(error.into())
        }
    }
}

pub async fn request_stations(
    parameter: model::Parameter,
    dump: bool,
) -> Result<model::StationsResponse, Error> {
    request_json_component(&ApiComponent::Parameter { parameter }, dump).await
}

pub async fn request_data(
    parameter: Parameter,
    station: StationComponent,
    period: model::Period,
    dump: bool,
) -> Result<model::Data, Error> {
    request_json_component(
        &ApiComponent::Data {
            parameter,
            station,
            period,
        },
        dump,
    )
    .await
}
