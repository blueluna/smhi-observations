use chrono;
use serde::{Deserialize, Deserializer, Serialize};

/*
21	Byvind	max, 1 gång/tim	meter per sekund
39	Daggpunktstemperatur	momentanvärde, 1 gång/tim	celsius
11	Global Irradians (svenska stationer)	medelvärde 1 timme, 1 gång/tim	watt per kvadratmeter
22	Lufttemperatur	medel, 1 gång per månad	celsius
26	Lufttemperatur	min, 2 gånger per dygn, kl 06 och 18	celsius
27	Lufttemperatur	max, 2 gånger per dygn, kl 06 och 18	celsius
19	Lufttemperatur	min, 1 gång per dygn	celsius
1	Lufttemperatur	momentanvärde, 1 gång/tim	celsius
2	Lufttemperatur	medelvärde 1 dygn, 1 gång/dygn, kl 00	celsius
20	Lufttemperatur	max, 1 gång per dygn	celsius
9	Lufttryck reducerat havsytans nivå	vid havsytans nivå, momentanvärde, 1 gång/tim	hektopascal
24	Långvågs-Irradians	Långvågsstrålning, medel 1 timme, varje timme	watt per kvadratmeter
40	Markens tillstånd	momentanvärde, 1 gång/dygn, kl 06	kod
25	Max av MedelVindhastighet	maximum av medelvärde 10 min, under 3 timmar, 1 gång/tim	meter per sekund
28	Molnbas	lägsta molnlager, momentanvärde, 1 gång/tim	meter
30	Molnbas	andra molnlager, momentanvärde, 1 gång/tim	meter
32	Molnbas	tredje molnlager, momentanvärde, 1 gång/tim	meter
34	Molnbas	fjärde molnlager, momentanvärde, 1 gång/tim	meter
36	Molnbas	lägsta molnbas, momentanvärde, 1 gång/tim	meter
37	Molnbas	lägsta molnbas, min under 15 min, 1 gång/tim	meter
29	Molnmängd	lägsta molnlager, momentanvärde, 1 gång/tim	kod
31	Molnmängd	andra molnlager, momentanvärde, 1 gång/tim	kod
33	Molnmängd	tredje molnlager, momentanvärde, 1 gång/tim	kod
35	Molnmängd	fjärde molnlager, momentanvärde, 1 gång/tim	kod
17	Nederbörd	2 gånger/dygn, kl 06 och 18	kod
18	Nederbörd	1 gång/dygn, kl 18	kod
15	Nederbördsintensitet	max under 15 min, 4 gånger/tim	millimeter per sekund
38	Nederbördsintensitet	max av medel under 15 min, 4 gånger/tim	millimeter per sekund
23	Nederbördsmängd	summa, 1 gång per månad	millimeter
14	Nederbördsmängd	summa 15 min, 4 gånger/tim	millimeter
5	Nederbördsmängd	summa 1 dygn, 1 gång/dygn, kl 06	millimeter
7	Nederbördsmängd	summa 1 timme, 1 gång/tim	millimeter
6	Relativ Luftfuktighet	momentanvärde, 1 gång/tim	procent
13	Rådande väder	momentanvärde, 1 gång/tim resp 8 gånger/dygn	kod
12	Sikt	momentanvärde, 1 gång/tim	meter
8	Snödjup	momentanvärde, 1 gång/dygn, kl 06	meter
10	Solskenstid	summa 1 timme, 1 gång/tim	sekund
16	Total molnmängd	momentanvärde, 1 gång/tim	procent
4	Vindhastighet	medelvärde 10 min, 1 gång/tim	meter per sekund
3	Vindriktning	medelvärde 10 min, 1 gång/tim	grader
*/

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Parameter {
    AirTemperatureCurrent = 1,
    AirTemperatureAverageDay = 2,
    WindDirectionAverage10Minutes = 3,
    WindSpeedAverage10Minutes = 4,
    PrecipitationAggregateDay = 5,
    RelativeHumidity = 6,
    PrecipitationAggregateHour = 7,
    SnowDepth = 8,
    AtmosphericPressure = 9,
    SunTime = 10,
    GlobalIrradiance = 11,
    Visibility = 12,
    WeatherObservation = 13,
    PrecipitationAggregate15minutes = 14,
    PrecipitationIntensity15minutes = 15,
    CloudCoverageTotal = 16,
    PrecipitationCode12Hours = 17,
    PrecipitationCode24Hours = 18,
    AirTemperatureMinimum24Hours = 19,
    AirTemperatureMaximum24Hours = 20,
    WindSpeedGustHour = 21,
    AirTemperatureAverageMonth = 22,
    PrecipitationAggregateMonth = 23,
    LowFrequencyIrradiance = 24,
    WindSpeedAverageMaximum = 25,
    AirTemperatureMinimum12Hours = 26,
    AirTemperatureMaximum12Hours = 27,
    CloudBaseLower = 28,
    CloudCoverageLower = 29,
    CloudBaseSecond = 30,
    CloudCoverageSecond = 31,
    CloudBaseThird = 32,
    CloudCoverageThird = 33,
    CloudBaseFourth = 34,
    CloudCoverageFourth = 35,
    CloudBaseLowest = 36,
    CloudBaseLowestMinimum = 37,
    PrecipitationIntensityMaximum15minutes = 38,
    DewPoint = 39,
    GroundState = 40,
    Unknown = 0,
}

impl std::convert::From<Parameter> for u8 {
    fn from(value: Parameter) -> u8 {
        return value as u8;
    }
}

impl std::convert::From<&Parameter> for u8 {
    fn from(value: &Parameter) -> u8 {
        return *value as u8;
    }
}

impl std::convert::From<u8> for Parameter {
    fn from(value: u8) -> Parameter {
        match value {
            1 => Self::AirTemperatureCurrent,
            2 => Self::AirTemperatureAverageDay,
            3 => Self::WindDirectionAverage10Minutes,
            4 => Self::WindSpeedAverage10Minutes,
            5 => Self::PrecipitationAggregateDay,
            6 => Self::RelativeHumidity,
            7 => Self::PrecipitationAggregateHour,
            8 => Self::SnowDepth,
            9 => Self::AtmosphericPressure,
            10 => Self::SunTime,
            11 => Self::GlobalIrradiance,
            12 => Self::Visibility,
            13 => Self::WeatherObservation,
            14 => Self::PrecipitationAggregate15minutes,
            15 => Self::PrecipitationIntensity15minutes,
            16 => Self::CloudCoverageTotal,
            17 => Self::PrecipitationCode12Hours,
            18 => Self::PrecipitationCode24Hours,
            19 => Self::AirTemperatureMinimum24Hours,
            20 => Self::AirTemperatureMaximum24Hours,
            21 => Self::WindSpeedGustHour,
            22 => Self::AirTemperatureAverageMonth,
            23 => Self::PrecipitationAggregateMonth,
            24 => Self::LowFrequencyIrradiance,
            25 => Self::WindSpeedAverageMaximum,
            26 => Self::AirTemperatureMinimum12Hours,
            27 => Self::AirTemperatureMaximum12Hours,
            28 => Self::CloudBaseLower,
            29 => Self::CloudCoverageLower,
            30 => Self::CloudBaseSecond,
            31 => Self::CloudCoverageSecond,
            32 => Self::CloudBaseThird,
            33 => Self::CloudCoverageThird,
            34 => Self::CloudBaseFourth,
            35 => Self::CloudCoverageFourth,
            36 => Self::CloudBaseLowest,
            37 => Self::CloudBaseLowestMinimum,
            38 => Self::PrecipitationIntensityMaximum15minutes,
            39 => Self::DewPoint,
            40 => Self::GroundState,
            _ => Self::Unknown,
        }
    }
}

const PARAMETER_TEMPERATURE: &str = "temperature";
const PARAMETER_WIND_DIRECTION: &str = "wind direction";
const PARAMETER_WIND_SPEED: &str = "wind speed";
const PARAMETER_HUMIDITY: &str = "humidity";
const PARAMETER_PRESSURE: &str = "pressure";

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match *self {
            Self::AirTemperatureCurrent => PARAMETER_TEMPERATURE,
            Self::WindDirectionAverage10Minutes => PARAMETER_WIND_DIRECTION,
            Self::WindSpeedAverage10Minutes => PARAMETER_WIND_SPEED,
            Self::RelativeHumidity => PARAMETER_HUMIDITY,
            Self::AtmosphericPressure => PARAMETER_PRESSURE,
            _ => "Other",
        };
        write!(f, "{}", label)
    }
}

impl std::str::FromStr for Parameter {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PARAMETER_TEMPERATURE => Ok(Self::AirTemperatureCurrent),
            PARAMETER_WIND_DIRECTION => Ok(Self::WindDirectionAverage10Minutes),
            PARAMETER_WIND_SPEED => Ok(Self::WindSpeedAverage10Minutes),
            PARAMETER_HUMIDITY => Ok(Self::RelativeHumidity),
            PARAMETER_PRESSURE => Ok(Self::AtmosphericPressure),
            _ => Err(crate::Error::InvalidValue),
        }
    }
}

/// Parse unix timestamp with millisecond resolution
fn parse_datetime<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: i64 = Deserialize::deserialize(deserializer)?;
    chrono::DateTime::from_timestamp_millis(timestamp)
        .ok_or(serde::de::Error::custom("Invalid timestamp"))
}

/// Parse key, a string containing a numerical identifier
fn parse_station_key<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let key: String = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(&key, 10).or(Err(serde::de::Error::custom("Invalid key")))
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum MediaType {
    #[serde(rename = "application/json")]
    Json,
    #[serde(rename = "application/xml")]
    Xml,
    #[serde(rename = "application/atom+xml")]
    Atom,
    #[serde(rename = "text/plain")]
    Csv,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    #[serde(rename = "rel")]
    pub relation: String,

    #[serde(rename = "type")]
    pub media_type: MediaType,

    pub href: String,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MeasuringNetwork {
    /// CORE stations are monitored, inspected and maintained by SMHI. Values reported from CORE stations are quality checked and corrected.
    Core,
    /// ADDITIONAL stations are not quality checked and the stations are not inspected by SMHI.
    Additional,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OwnerCategory {
    Climate,
    National,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum Period {
    #[serde(rename = "latest-hour")]
    Hour,
    #[serde(rename = "latest-day")]
    Day,
    #[serde(rename = "latest-months")]
    Month,
    #[serde(rename = "corrected-archive")]
    CorrectedArchive,
}

const API_PERIOD_HOUR: &str = "latest-hour";
const API_PERIOD_DAY: &str = "latest-day";
const API_PERIOD_MONTH: &str = "latest-month";
const API_PERIOD_ARCHIVE: &str = "corrected-archive";

impl Period {
    pub fn to_url(&self) -> String {
        match *self {
            Self::Hour => API_PERIOD_HOUR,
            Self::Day => API_PERIOD_DAY,
            Self::Month => API_PERIOD_MONTH,
            Self::CorrectedArchive => API_PERIOD_ARCHIVE,
        }
        .to_string()
    }
}

const PERIOD_HOUR: &str = "hour";
const PERIOD_DAY: &str = "day";
const PERIOD_MONTH: &str = "month";
const PERIOD_ARCHIVE: &str = "archive";

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match *self {
            Self::Hour => PERIOD_HOUR,
            Self::Day => PERIOD_DAY,
            Self::Month => PERIOD_MONTH,
            Self::CorrectedArchive => PERIOD_ARCHIVE,
        };
        write!(f, "{}", label)
    }
}

impl std::str::FromStr for Period {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PERIOD_HOUR => Ok(Self::Hour),
            PERIOD_DAY => Ok(Self::Day),
            PERIOD_MONTH => Ok(Self::Month),
            PERIOD_ARCHIVE => Ok(Self::CorrectedArchive),
            _ => Err(crate::Error::InvalidValue),
        }
    }
}

pub enum Key {
    Parameter(Parameter),
    Station(u32),
    StationSet(String),
    Period(Period),
}

pub trait Updated {
    fn updated(&self) -> chrono::DateTime<chrono::Utc>;
}

pub trait Keyed {
    fn key(&self) -> Key;
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct StationDescription {
    /// Station identifier
    pub id: u32,
    /// Station identifier
    #[serde(deserialize_with = "parse_station_key")]
    pub key: u32,
    /// Station name
    pub name: String,

    pub title: String,

    pub owner: String,

    #[serde(rename = "ownerCategory")]
    pub owner_category: OwnerCategory,

    #[serde(rename = "measuringStations")]
    pub measuring_network: MeasuringNetwork,

    pub height: f64,

    pub latitude: f64,

    pub longitude: f64,

    pub active: bool,

    #[serde(deserialize_with = "parse_datetime")]
    pub updated: chrono::DateTime<chrono::Utc>,

    #[serde(deserialize_with = "parse_datetime")]
    pub from: chrono::DateTime<chrono::Utc>,

    #[serde(deserialize_with = "parse_datetime")]
    pub to: chrono::DateTime<chrono::Utc>,

    pub summary: String,

    pub link: Vec<Link>,
}

impl Updated for StationDescription {
    fn updated(&self) -> chrono::DateTime<chrono::Utc> {
        return self.updated;
    }
}

impl Keyed for StationDescription {
    fn key(&self) -> Key {
        return Key::Station(self.key);
    }
}

impl std::cmp::PartialEq<StationInfo> for StationDescription {
    fn eq(&self, other: &StationInfo) -> bool {
        self.key == other.key
    }
}

impl StationDescription {
    pub fn as_brief(&self) -> StationInfo {
        StationInfo {
            key: self.key,
            name: self.name.clone(),
            owner: self.owner.clone(),
            owner_category: self.owner_category,
            measuring_network: self.measuring_network,
            height: self.height,
        }
    }
}

/// Parse key, a string containing a numerical identifier
fn parse_parameter_key<'de, D>(deserializer: D) -> Result<Parameter, D::Error>
where
    D: Deserializer<'de>,
{
    let key: String = Deserialize::deserialize(deserializer)?;
    let number = u8::from_str_radix(&key, 10).or(Err(serde::de::Error::custom("Invalid key")))?;
    Ok(Parameter::from(number))
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ParameterInfo {
    /// Station identifier
    #[serde(deserialize_with = "parse_parameter_key")]
    pub key: Parameter,
    /// Parameter name
    pub name: String,
    /// Parameter summary
    pub summary: String,
    /// Parameter unit
    pub unit: String,
}

impl Keyed for ParameterInfo {
    fn key(&self) -> Key {
        return Key::Parameter(self.key);
    }
}

impl std::fmt::Display for ParameterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parameter {:?} {}", self.key, self.name)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct StationInfo {
    /// Station identifier
    #[serde(deserialize_with = "parse_station_key")]
    pub key: u32,
    /// Station name
    pub name: String,
    /// Station owner
    pub owner: String,
    /// Station owner category
    #[serde(rename = "ownerCategory")]
    pub owner_category: OwnerCategory,
    /// Measuring network
    #[serde(rename = "measuringStations")]
    pub measuring_network: MeasuringNetwork,
    /// Height in meters
    pub height: f64,
}

impl Keyed for StationInfo {
    fn key(&self) -> Key {
        return Key::Station(self.key);
    }
}

impl std::fmt::Display for StationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Station {} {}", self.key, self.name)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PeriodInfo {
    /// Period identifier
    pub key: Period,
    /// Period from datetime
    #[serde(deserialize_with = "parse_datetime")]
    pub from: chrono::DateTime<chrono::Utc>,
    /// Period to datetime
    #[serde(deserialize_with = "parse_datetime")]
    pub to: chrono::DateTime<chrono::Utc>,
    /// Period summary
    pub summary: String,
    /// Period sampling method
    pub sampling: String,
}

impl Keyed for PeriodInfo {
    fn key(&self) -> Key {
        return Key::Period(self.key);
    }
}

impl std::fmt::Display for PeriodInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Period {} {}", self.key, self.summary)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Position {
    /// Valid from
    #[serde(deserialize_with = "parse_datetime")]
    pub from: chrono::DateTime<chrono::Utc>,
    /// Valid to
    #[serde(deserialize_with = "parse_datetime")]
    pub to: chrono::DateTime<chrono::Utc>,
    /// Height above ground
    pub height: f64,
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
}

#[derive(Deserialize)]
pub struct StationsResponse {
    #[serde(deserialize_with = "parse_datetime")]
    pub updated: chrono::DateTime<chrono::Utc>,
    /// Station identifier
    #[serde(deserialize_with = "parse_station_key")]
    pub key: u32,

    pub station: Vec<StationDescription>,
}

impl Updated for StationsResponse {
    fn updated(&self) -> chrono::DateTime<chrono::Utc> {
        return self.updated;
    }
}

impl Keyed for StationsResponse {
    fn key(&self) -> Key {
        return Key::Station(self.key);
    }
}

#[derive(Deserialize)]
pub struct StationResponse {
    /// Station identifier
    #[serde(deserialize_with = "parse_station_key")]
    pub key: u32,

    #[serde(deserialize_with = "parse_datetime")]
    pub updated: chrono::DateTime<chrono::Utc>,

    pub title: String,

    pub owner: String,

    #[serde(rename = "ownerCategory")]
    pub owner_category: OwnerCategory,

    #[serde(rename = "measuringStations")]
    pub measuring_network: MeasuringNetwork,

    pub active: bool,

    pub summary: String,
    /// Period from datetime
    #[serde(deserialize_with = "parse_datetime")]
    pub from: chrono::DateTime<chrono::Utc>,
    /// Period to datetime
    #[serde(deserialize_with = "parse_datetime")]
    pub to: chrono::DateTime<chrono::Utc>,
    /// Measuring position(s)
    pub position: Vec<Position>,

    pub link: Vec<Link>,
}

impl Updated for StationResponse {
    fn updated(&self) -> chrono::DateTime<chrono::Utc> {
        return self.updated;
    }
}

impl Keyed for StationResponse {
    fn key(&self) -> Key {
        return Key::Station(self.key);
    }
}

#[derive(Deserialize)]
pub struct PeriodResponse {
    /// Station identifier
    pub key: Period,

    #[serde(deserialize_with = "parse_datetime")]
    pub updated: chrono::DateTime<chrono::Utc>,

    pub title: String,

    pub summary: String,
    /// Period from datetime
    #[serde(deserialize_with = "parse_datetime")]
    pub from: chrono::DateTime<chrono::Utc>,
    /// Period to datetime
    #[serde(deserialize_with = "parse_datetime")]
    pub to: chrono::DateTime<chrono::Utc>,

    pub link: Vec<Link>,
}

impl Updated for PeriodResponse {
    fn updated(&self) -> chrono::DateTime<chrono::Utc> {
        return self.updated;
    }
}

impl Keyed for PeriodResponse {
    fn key(&self) -> Key {
        return Key::Period(self.key);
    }
}

/// Sample quality
#[derive(Deserialize, Debug, Clone)]
pub enum SampleQuality {
    /// Sample quality **G**reen, intepreted as good.
    #[serde(rename = "G")]
    Good,
    /// Sample quality **Y**ellow, intepreted as fair.
    #[serde(rename = "Y")]
    Fair,
    /// Sample quality **R**ed, intepreted as bad.
    #[serde(rename = "R")]
    Bad,
}

/// Measurement sample
#[derive(Deserialize)]
pub struct Sample {
    /// Timestamp for measurement
    #[serde(deserialize_with = "parse_datetime")]
    pub date: chrono::DateTime<chrono::Utc>,
    /// Measurement value
    pub value: String,
    /// Measurement quality
    pub quality: SampleQuality,
}

#[derive(Deserialize)]
pub struct Data {
    /// Timestamp for last update
    #[serde(deserialize_with = "parse_datetime")]
    pub updated: chrono::DateTime<chrono::Utc>,
    /// Measuring parameter
    pub parameter: ParameterInfo,
    /// Measuring station
    pub station: StationInfo,
    /// Measuring period
    pub period: PeriodInfo,
    /// Measuring position(s)
    pub position: Vec<Position>,
    /// Sample values
    pub value: Vec<Sample>,
}

impl Updated for Data {
    fn updated(&self) -> chrono::DateTime<chrono::Utc> {
        return self.updated;
    }
}
