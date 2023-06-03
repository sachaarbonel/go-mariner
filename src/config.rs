use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuleConfig {
    pub name: String,
    pub enabled: bool,
}
