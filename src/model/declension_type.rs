use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DeclensionType {
    F1,
    F2,
    F3,
    M1,
    M2,
    M3,
    M4,
    N,
    Irregular,
}
