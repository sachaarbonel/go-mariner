use gosyn::Parser;

use crate::{
    config::Config,
    issue::Issue,
    rules::{Rule, UnusedVariableRule},
};

pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new(config: Config) -> Self {
        Linter {
            rules: config
                .rules
                .into_iter()
                .filter_map(|rule_config| {
                    if rule_config.enabled {
                        match rule_config.name.as_str() {
                            "unused_variable" => {
                                Some(Box::new(UnusedVariableRule) as Box<dyn Rule>)
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    pub fn lint(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        let mut parser = Parser::from(code);
        let file = parser.parse_file().unwrap();
        for rule in &self.rules {
            rule.apply(&file, &mut issues);
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{Config, RuleConfig},
        issue::Issue,
    };

    use super::Linter;

    fn get_linter_with_rule() -> Linter {
        let config = Config {
            rules: vec![RuleConfig {
                name: "unused_variable".to_string(),
                enabled: true,
            }],
        };

        Linter::new(config)
    }

    fn apply_unused_variable_rule(code: &str) -> Vec<Issue> {
        let linter = get_linter_with_rule();

        linter.lint(code)
    }

    #[test]
    fn test_unused_variable_found() {
        let code = r#"
package main

func main() {
    a := 5
    b := 6
    fmt.Println(a)
}
"#;

        let issues = apply_unused_variable_rule(code);
        assert_eq!(1, issues.len());
        assert!(issues[0].description.contains("Unused variable: b"));
    }

    #[test]
    fn test_no_unused_variables() {
        let code = r#"
package main

func main() {
    a := 5
    fmt.Println(a)
}
"#;

        let issues = apply_unused_variable_rule(code);
        assert_eq!(0, issues.len());
    }
}
