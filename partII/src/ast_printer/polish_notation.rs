use crate::expr::*;

impl Expr {
    pub fn polish_notation(&self) -> String {
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
                    left.polish_notation(),
                    right.polish_notation()
                ));
            }
            Self::Grouping { expression } => {
                res.push_str(&format!("(group {})", expression.polish_notation()));
            }
            Self::Literal { value } => res.push_str(value),
            Self::Unary { right, operator } => {
                res.push_str(&format!(
                    "({} {})",
                    operator.lexeme,
                    right.polish_notation()
                ));
            }
        }

        res
    }
}
