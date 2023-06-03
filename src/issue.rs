#[derive(Debug)]
pub struct Issue {
    pub description: String,
    // TODO: Add fields for other information, such as location (line, column), severity etc.
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
