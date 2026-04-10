use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
    Neuter,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => f.write_str("Maskulin"),
            Gender::Female => f.write_str("Feminin"),
            Gender::Neuter => f.write_str("Neutrum"),
        }
    }
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
