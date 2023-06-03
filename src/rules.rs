pub trait Rule {
    fn apply(&self, node: &File, issues: &mut Vec<Issue>);
}

use gosyn::ast::{Declaration, File};
use std::collections::HashSet;

use crate::{
    helper::{find_variable_declarations},
    issue::Issue,
};
pub struct UnusedVariableRule;
impl Rule for UnusedVariableRule {
    fn apply(&self, file: &File, issues: &mut Vec<Issue>) {
        let mut declared_vars = Vec::new();
        let mut used_vars = Vec::new();

        crate::helper::find_variable_declarations(file, &mut declared_vars);
        crate::helper::find_variable_usages(file, &mut used_vars);

        let declared_vars: HashSet<String> = declared_vars.into_iter().collect();
        let used_vars: HashSet<String> = used_vars.into_iter().collect();

        println!("declared_vars: {:?}", declared_vars);
        println!("used_vars: {:?}", used_vars);

        // Iterate through declared variables and check if they were used.
        for var in declared_vars {
            if !used_vars.contains(&var) {
                issues.push(Issue {
                    description: format!("Unused variable: {}", var),
                    // TODO: Add information about the location (line, column) 
                });
            }
        }
    }
}
