use chrono::NaiveDate;
use migration_macro::Migrate;
use std::fmt::Display;

#[derive(serde::Deserialize, Debug)]
pub enum Rumpfmaterial {
    STAHL,
    GFK,
    ALUMINIUM,
    SONSTIGE(String),
}

impl Display for Rumpfmaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rumpfmaterial::STAHL => write!(f, "Stahl"),
            Rumpfmaterial::GFK => write!(f, "GFK"),
            Rumpfmaterial::ALUMINIUM => write!(f, "Aluminium"),
            Rumpfmaterial::SONSTIGE(other) => write!(f, "{}", other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Picture {
    pub url: String,
    pub description: String,
}

// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Migrate)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataSet {
    pub submit_source: String,
    #[serde(with = "date_format")]
    pub submit_date: NaiveDate,
    #[serde(with = "date_format")]
    pub build_date: NaiveDate,
    pub upload_server_url: String,
    pub event_observer: String,

    pub author_email_address: String,
    pub hull_material: String,
    pub rig_type: String,
    pub length: f32,
    pub width: f32,
    pub draft: f32,
    pub additional_notes: String,
    pub keel_material: String,
    pub displacement: f32,
    pub total_weight: f32,
    pub number_of_engines: u32,
    pub engine_power_per_unit: f32,
    pub engine_manufacturer: String,
    pub engine_models: String,
    pub engine_serial_numbers: String,
    pub engine_hours: u32,
    pub boat_name: String,
    pub hin_number: String,
    pub trailer_or_cradle: Vec<String>,
    pub ce_certificate: Vec<String>,
    pub vat_proof: Vec<String>,
    pub ownership_proof: Vec<String>,
    pub registration_papers: String,
    pub previous_operating_area: String,
    pub number_of_previous_owners: u32,
    pub lay_up_period: String,
    pub antifouling_system: String,
    pub sail_configuration: String,
    pub standing_rigging_age: u32,
    pub file_upload: Vec<String>,

    pub pictures: Vec<Picture>,

    pub time_to_submit: String,
    pub preview: String,
    pub validated_new_required_field_ids: String,
    pub path: String,
}

mod date_format {
    use chrono::NaiveDate;
    use serde::Deserialize;

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
