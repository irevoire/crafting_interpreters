use crate::expr::*;

impl Expr {
    pub fn print(&self) -> String {
        let mut res = String::new();

        match self {
            Self::Binary {
                left,
                right,
                operator,
            } => {
                res.push_str(&format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.print(),
                    right.print()
                ));
            }
            Self::Grouping { expression } => {
                res.push_str(&format!("(group {})", expression.print()));
            }
            Self::Literal { value } => res.push_str(value),
            Self::Unary { right, operator } => {
                res.push_str(&format!("({} {})", operator.lexeme, right.print()));
            }
        }

        res
    }
}
