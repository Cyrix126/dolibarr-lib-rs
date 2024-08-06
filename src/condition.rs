use std::str::FromStr;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::FromRepr;
use strum::IntoEnumIterator;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, EnumIter, FromRepr)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode))]
#[repr(u8)]
pub enum Condition {
    New,
    LikeNew,
    VeryGood,
    Good,
    Correct,
    Bad,
}

impl FromStr for Condition {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        cfg_if! {
            if #[cfg(feature="lang-fr")] {
        match s {
            "NEUF" => return Ok(Condition::New),
            "COMME NEUF" => return Ok(Self::LikeNew),
            "TRÈS BON ÉTAT" => return Ok(Self::VeryGood),
            "BON ÉTAT" => return Ok(Self::Good),
            "ÉTAT CORRECT" => return Ok(Self::Correct),
            "MAUVAIS ÉTAT" => return Ok(Self::Bad),
            "Neuf" => return Ok(Self::New),
            "Comme neuf" => return Ok(Self::LikeNew),
            "Très bon état" => return Ok(Self::VeryGood),
            "Bon état" => return Ok(Self::Good),
            "État correct" => return Ok(Self::Correct),
            "Mauvais état" => return Ok(Self::Bad),
            _ => {}
        };
        } else {
        match s {
            "NEW" => return Ok(Self::New),
            "USED_LIKE_NEW" => return Ok(Self::LikeNew),
            "USED_VERY_GOOD" => return Ok(Self::VeryGood),
            "USED_GOOD" => return Ok(Self::Good),
            "USED_CORRECT" => return Ok(Self::Correct),
            "USED_BAD" => return Ok(Self::Bad),
            "New" => return Ok(Self::New),
            "LikeNew" => return Ok(Self::LikeNew),
            "VeryGood" => return Ok(Self::VeryGood),
            "Good" => return Ok(Self::Good),
            "Correct" => return Ok(Self::Correct),
            "Bad" => return Ok(Self::Bad),
            _ => {}
        };
        }
        }
        #[cfg(feature = "rakuten")]
        match s {
            "CN" => return Ok(Self::LikeNew),
            "TBE" => return Ok(Self::VeryGood),
            "BE" => return Ok(Self::Good),
            "EC" => return Ok(Self::Correct),
            _ => {}
        };
        Ok(Self::New)
    }
}
impl Condition {
    pub fn used_variants() -> Vec<Condition> {
        Condition::iter().filter(|x| x != &Condition::New).collect()
    }
    #[cfg(feature = "rakuten")]
    pub fn rakuten_string(&self) -> String {
        match self {
            Self::New => "USED_LIKE_NEW",
            Self::LikeNew => "USED_LIKE_NEW",
            Self::VeryGood => "USED_VERY_GOOD",
            Self::Good => "USED_GOOD",
            Self::Correct => "USED_CORRECT",
            Self::Bad => "USED_CORRECT",
        }
        .to_string()
    }
    #[cfg(feature = "rakuten")]
    pub fn rakuten_api_string(&self) -> String {
        match self {
            Self::New => "CN",
            Self::LikeNew => "CN",
            Self::VeryGood => "TBE",
            Self::Good => "BE",
            Self::Correct => "EC",
            Self::Bad => "EC",
        }
        .to_string()
    }
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        cfg_if! {
            if #[cfg(feature="lang-fr")] {
            match self {
                Condition::New => write!(f, "NEUF"),
                Condition::LikeNew => write!(f, "COMME NEUF"),
                Condition::VeryGood => write!(f, "TRÈS BON ÉTAT"),
                Condition::Good => write!(f, "BON ÉTAT"),
                Condition::Correct => write!(f, "ÉTAT CORRECT"),
                Condition::Bad => write!(f, "MAUVAIS ÉTAT"),
            }
            } else {
            match self {
                Condition::New => write!(f, "New"),
                Condition::LikeNew => write!(f, "Like New"),
                Condition::VeryGood => write!(f, "Very Good"),
                Condition::Good => write!(f, "Good"),
                Condition::Correct => write!(f, "Correct"),
                Condition::Bad => write!(f, "Bad"),
            }
            }
        }
    }
}
