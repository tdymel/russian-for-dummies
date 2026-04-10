use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Csfr {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    C2Plus,
}

impl From<String> for Csfr {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for Csfr {
    fn from(value: &str) -> Self {
        match value {
            "A1" => Csfr::A1,
            "A2" => Csfr::A2,
            "B1" => Csfr::B1,
            "B2" => Csfr::B2,
            "C1" => Csfr::C1,
            "C2" => Csfr::C2,
            _ => Csfr::C2Plus,
        }
    }
}
