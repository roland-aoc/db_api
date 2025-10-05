use serde_json;
use serde;

#[derive(serde::Deserialize)]
pub struct Form {
    pub q6_email6: String,
    pub rumpfmaterial: Rumpfmaterial,
    pub rigg_typ: String
}

#[derive(serde::Deserialize)]
pub enum Rumpfmaterial {
    STAHL,
    GFK,
    ALUMINIUM,
    SONSTIGE(String)
}