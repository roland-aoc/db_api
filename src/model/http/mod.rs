use serde_json;
use serde;

#[derive(serde::Deserialize, Debug)]
pub struct Payload {
    pub q6_email6: String,
    pub rumpfmaterial: Rumpfmaterial,
    pub rigg_typ: String
}

#[derive(serde::Deserialize, Debug)]
pub enum Rumpfmaterial {
    STAHL,
    GFK,
    ALUMINIUM,
    SONSTIGE(String)
}