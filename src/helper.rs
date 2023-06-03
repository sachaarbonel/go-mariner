use std::collections::HashMap;

use gosyn::{
    ast::{Declaration, Expression, File, Statement},
    token::Operator,
};
use std::collections::HashSet;

pub fn find_unused_variables(
    used_variables: &HashSet<String>,
    declared_variables: &HashMap<String, usize>,
) -> HashSet<String> {
    let mut unused_variables = HashSet::new();

    for (var_name, _) in declared_variables {
        if !used_variables.contains(var_name) {
            unused_variables.insert(var_name.clone());
        }
    }

    unused_variables
}

pub fn collect_used_variables(file: &File) -> (HashSet<String>, HashMap<String, usize>) {
    let mut used_variables = HashSet::new();
    let mut declared_variables = HashMap::new();

    for decl in &file.decl {
        match decl {
            Declaration::Function(func_decl) => {
                if let Some(block_stmt) = &func_decl.body {
                    for stmt in &block_stmt.list {
                        visit_statement(stmt, &mut used_variables, &mut declared_variables);
                    }
                }
            }
            _ => (),
        }
    }

    (used_variables, declared_variables)
}

fn visit_statement(
    stmt: &Statement,
    used_variables: &mut HashSet<String>,
    declared_variables: &mut HashMap<String, usize>,
) {
    match stmt {
        Statement::Expr(expr_stmt) => visit_expression(&expr_stmt.expr, used_variables),
        Statement::Block(block_stmt) => {
            for stmt in &block_stmt.list {
                visit_statement(stmt, used_variables, declared_variables);
            }
        }
        Statement::If(if_stmt) => {
            visit_expression(&if_stmt.cond, used_variables);
            for stmt in &if_stmt.init {
                visit_statement(stmt, used_variables, declared_variables);
            }
        }
        Statement::For(for_stmt) => {
            for stmt in &for_stmt.init {
                visit_statement(stmt, used_variables, declared_variables);
            }
        }
        Statement::Assign(assign_stmt) => {
            for (idx, expr) in assign_stmt.left.iter().enumerate() {
                if let Expression::Ident(ident) = expr {
                    if assign_stmt.op == Operator::Define {
                        declared_variables.insert(ident.name.clone(), assign_stmt.pos);
                    }
                }
                visit_expression(&assign_stmt.right[idx], used_variables);
            }
        }
        _ => (),
    }
}

fn visit_expression(expr: &Expression, used_variables: &mut HashSet<String>) {
    match expr {
        Expression::Ident(ident) => {
            used_variables.insert(ident.name.clone());
        }
        Expression::Call(call) => {
            visit_expression(&*call.func, used_variables);
            for arg in &call.args {
                visit_expression(arg, used_variables);
            }
        }
        _ => (), // TODO: Add more cases as needed
    }
}

#[cfg(test)]
mod tests {
    use gosyn::Parser;

    use super::*;

    #[test]
    fn test_find_a_in_println() {
        let code = r#"
package main

func main() {
    a := 5
    fmt.Println(a)
}
"#;

        let mut parser = Parser::from(code);
        let file = parser.parse_file().unwrap();

        let (used_variables, declared_variables) = collect_used_variables(&file);

        assert!(used_variables.contains("a"));

        assert!(declared_variables.contains_key("a"));

        let unused_variables = find_unused_variables(&used_variables, &declared_variables);

        assert!(unused_variables.is_empty());
    }

    //     #[test]
    //     fn test_collect_used_variables() {
    //         let code = r#"
    // package main

    // func main() {
    //     a := 5
    //     b := 3
    //     fmt.Println(a)
    //     foo(b)
    // }

    // func foo(x int) {
    //     y := x + 1
    //     fmt.Println(y)
    // }
    // "#;

    //         let mut parser = Parser::from(code);
    //         let file = parser.parse_file().unwrap();

    // let (used_variables, declared_variables)= collect_used_variables(&file);

    //         assert!(used_variables.contains("a"));
    //         assert!(used_variables.contains("b"));
    //         assert!(used_variables.contains("x"));
    //         assert!(used_variables.contains("y"));
    //     }
}
