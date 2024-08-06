#[cfg(feature = "condition")]
use crate::condition::Condition;
#[cfg(feature = "bincode")]
use bincode::Encode;
#[cfg(feature = "des_opt_timestamp")]
use chrono::NaiveDate;
#[cfg(feature = "merge")]
use merge::Merge;

use chrono::NaiveDateTime;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use crate::deserializer::{
    deserialize_bool, from_optional_naivedatetime, from_str, from_str_optional, from_str_tof32,
};

#[cfg(feature = "des_opt_timestamp")]
use crate::deserializer::from_optional_timestamp;

#[cfg(feature = "des_opt_bool")]
use crate::deserializer::deserialize_optional_bool;
#[cfg(feature = "condition")]
use crate::deserializer::deserialize_state;
#[cfg(feature = "ser_opt_bool_to_int")]
use crate::deserializer::serialize_optional_bool_to_integer;
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Derivative)]
#[cfg_attr(feature = "bincode", derive(Encode))]
#[cfg_attr(feature = "merge", derive(Merge))]
#[derivative(Default)]
pub struct Product {
    #[cfg_attr(feature = "merge", merge(skip))]
    #[serde(deserialize_with = "from_str", rename = "id")]
    pub rowid: u32,
    #[cfg_attr(feature = "merge", merge(skip))]
    #[serde(rename = "ref")]
    pub reference: String,
    #[cfg_attr(feature = "merge", merge(skip))]
    pub label: String,
    #[serde(
        deserialize_with = "from_optional_naivedatetime",
        rename = "date_creation"
    )]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub date_creation_dolibarr: Option<NaiveDateTime>,
    #[serde(
        deserialize_with = "from_optional_naivedatetime",
        rename = "date_modification"
    )]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub date_modification_dolibarr: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub note_public: Option<String>,
    pub note_private: Option<String>,
    #[serde(deserialize_with = "from_str_optional", rename = "weight")]
    pub poids: Option<f32>,
    #[serde(deserialize_with = "from_str_optional", rename = "weight_units")]
    #[derivative(Default(value = "Some(-3)"))]
    pub poids_units: Option<i8>,
    #[serde(deserialize_with = "from_str_optional", rename = "length")]
    pub longueur: Option<f32>,
    #[serde(deserialize_with = "from_str_optional", rename = "length_units")]
    #[derivative(Default(value = "Some(-3)"))]
    pub length_units: Option<i8>,
    #[serde(deserialize_with = "from_str_optional", rename = "width")]
    pub largeur: Option<f32>,
    #[serde(deserialize_with = "from_str_optional", rename = "width_units")]
    #[derivative(Default(value = "Some(-3)"))]
    pub largeur_units: Option<i8>,
    #[serde(deserialize_with = "from_str_optional", rename = "height")]
    pub epaisseur: Option<f32>,
    #[serde(deserialize_with = "from_str_optional", rename = "height_units")]
    #[derivative(Default(value = "Some(-3)"))]
    pub epaisseur_units: Option<i8>,
    #[cfg_attr(feature = "merge", merge(skip))]
    #[serde(deserialize_with = "from_str_tof32")]
    pub price: f32,
    #[serde(deserialize_with = "from_str_optional")]
    pub price_min: Option<f32>, // Dolibarr API strange behavior change this field to None when product is updated.
    #[derivative(Default(value = "Some(\"HT\".to_owned())"))]
    // change by TTC if societe is subject to TVA
    pub price_base_type: Option<String>,
    #[serde(deserialize_with = "from_str_optional")]
    pub cost_price: Option<f32>,
    // change by TTC if societe is subject to TVA
    //  tva_tx: Option<f32>,
    #[serde(
        deserialize_with = "from_str_optional",
        rename = "stock_reel",
        skip_serializing
    )]
    pub stock: Option<i32>,
    pub barcode: Option<String>,
    #[cfg_attr(feature = "merge", merge(skip))]
    #[serde(deserialize_with = "deserialize_bool", rename = "status_buy")]
    pub tobuy: bool,
    #[cfg_attr(feature = "merge", merge(skip))]
    #[serde(deserialize_with = "deserialize_bool", rename = "status")]
    #[derivative(Default(value = "true"))]
    pub tosell: bool,
    #[serde(rename = "array_options")]
    pub extra_fields: ProductExtraFields,
}
#[cfg_attr(feature = "bincode", derive(Encode))]
#[serde_as]
#[skip_serializing_none]
#[cfg_attr(feature = "merge", derive(Merge))]
#[derive(Deserialize, Serialize, Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct ProductExtraFields {
    #[cfg(feature = "ef_libelle")]
    #[serde(rename = "options_libelle_caisse")]
    pub libelle_caisse: Option<String>,
    #[cfg(feature = "ef_auteur")]
    #[serde(rename = "options_auteur")]
    pub auteur: Option<String>,
    #[cfg(feature = "ef_collection")]
    #[serde(rename = "options_collection_et_etendu")]
    pub collection_et_etendu: Option<String>,
    #[cfg(feature = "ef_isbnediteur")]
    #[serde(rename = "options_isbnediteur")]
    pub isbnediteur: Option<String>,
    #[cfg(feature = "ef_theme")]
    #[serde(rename = "options_theme")]
    pub theme: Option<String>,
    #[cfg(feature = "condition")]
    #[serde(deserialize_with = "deserialize_state", rename = "options_etat")]
    pub etat: Option<Condition>,
    #[cfg(feature = "ef_last-modif")]
    #[serde(
        deserialize_with = "from_optional_timestamp",
        rename = "options_last_modif"
    )]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub last_modif: Option<NaiveDate>,
    #[cfg(feature = "ef_datedeparution")]
    #[serde(
        deserialize_with = "from_optional_timestamp",
        rename = "options_datedeparution"
    )]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub datedeparution: Option<NaiveDate>,
    #[cfg(feature = "ef_fincommerce")]
    #[serde(
        deserialize_with = "from_optional_timestamp",
        rename = "options_fincommerce"
    )]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub fincommerce: Option<NaiveDate>,
    #[cfg(feature = "dilicom")]
    #[serde(rename = "options_dispo")]
    pub dispo: Option<String>,
    #[cfg(feature = "ef_title")]
    #[serde(rename = "options_title")]
    pub title: Option<String>,
    #[cfg(feature = "gse")]
    #[serde(rename = "options_emplacement_gse")]
    pub emplacement_gse: Option<String>,
    #[cfg(feature = "ef_stock_origin")]
    #[cfg_attr(feature = "merge", merge(skip))]
    #[serde(rename = "options_stock_origine")]
    pub stock_origine: String,
    #[cfg(feature = "dilicom")]
    #[serde(rename = "options_distri")]
    pub distri: Option<String>,
    #[cfg(feature = "gse")]
    #[serde(
        deserialize_with = "deserialize_optional_bool",
        rename = "options_gse_statut",
        serialize_with = "serialize_optional_bool_to_integer"
    )]
    #[derivative(Default(value = "Some(true)"))]
    #[serialize_always]
    pub gse_statut: Option<bool>,
    #[cfg(feature = "ef_public_cible")]
    #[serde(rename = "options_public_cible")]
    pub public_cible: Option<String>,
    #[cfg(feature = "dilicom")]
    #[serde(rename = "options_ref_dilicom")]
    pub ref_dilicom: Option<String>,
    #[cfg(feature = "ef_presentation_editeur")]
    #[serde(rename = "options_presentation_editeur")]
    pub presentation_editeur: Option<String>,
    #[cfg(feature = "ef_theme_code")]
    #[serde(deserialize_with = "from_str_optional", rename = "options_theme_code")]
    pub theme_code: Option<u32>,
    #[serde(
        deserialize_with = "deserialize_optional_bool",
        rename = "options_commandable_dilicom"
    )]
    #[cfg(feature = "dilicom")]
    pub commandable_dilicom: Option<bool>,
    #[cfg(feature = "bnf")]
    #[serde(rename = "options_bnf_cadre")]
    pub bnf_cadre: Option<String>,
    #[cfg(feature = "bnf")]
    #[serde(deserialize_with = "from_str_optional", rename = "options_bnf_sujet")]
    pub bnf_sujet: Option<String>,
    // advised price is TTC
    #[cfg(feature = "ef_advised_price")]
    #[serde(
        deserialize_with = "from_str_optional",
        rename = "options_price_advised"
    )]
    pub price_advised_ttc: Option<f32>,
    #[cfg(feature = "dilicom")]
    #[derivative(Default(value = "None"))]
    #[serde(
        rename = "options_dmaj",
        deserialize_with = "deserialize_optional_bool"
    )]
    #[serialize_always]
    pub dmaj: Option<bool>, // on Dolibarr, false is not recognized. (true is). It must be set to Null.
    #[cfg(feature = "ef_ecommerce")]
    #[derivative(Default(value = "Some(true)"))]
    #[serialize_always]
    #[serde(
        rename = "options_enable_ecommerce",
        deserialize_with = "deserialize_optional_bool"
    )]
    pub ecommerce: Option<bool>,
    #[cfg(feature = "rakuten")]
    #[derivative(Default(value = "Some(true)"))]
    #[serialize_always]
    #[serde(
        rename = "options_rakuten_present",
        deserialize_with = "deserialize_optional_bool"
    )]
    pub rakuten_present: Option<bool>,
    #[cfg(feature = "rakuten")]
    #[serialize_always]
    #[serde(rename = "options_rakuten_id", deserialize_with = "from_str_optional")]
    pub rakuten_id: Option<u32>,
}
impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.reference.cmp(&other.reference))
    }
}

// permet d'ordonner les produits entre eux (par référence)
impl Ord for Product {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.reference).cmp(&other.reference)
    }
}
// permet de comparer les produits entre eux (par référence)
impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.reference == other.reference
    }
}

impl Eq for Product {}
