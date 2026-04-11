use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
    Neuter,
    Unknown,
}

impl From<String> for Gender {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for Gender {
    fn from(value: &str) -> Self {
        match value {
            "m" => Gender::Male,
            "f" => Gender::Female,
            _ => Gender::Neuter,
        }
    }
}
