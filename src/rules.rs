pub trait Rule {
    fn apply(&self, node: &File, issues: &mut Vec<Issue>);
}

use gosyn::ast::File;

use crate::{
    helper::{collect_used_variables, find_unused_variables},
    issue::Issue,
};
pub struct UnusedVariableRule;
impl Rule for UnusedVariableRule {
    fn apply(&self, file: &File, issues: &mut Vec<Issue>) {
        let (used_variables, declared_variables) = collect_used_variables(file);
        let unused_variables = find_unused_variables(&used_variables, &declared_variables);

        for unused_variable in unused_variables {
            issues.push(Issue {
                description: format!("Unused variable: {}", unused_variable),
            });
        }
    }
}
