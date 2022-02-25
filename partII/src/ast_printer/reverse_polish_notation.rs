use crate::expr::*;

impl Expr {
    pub fn reverse_polish_notation(&self) -> String {
        let mut res = String::new();

        match self {
            Self::Binary {
                left,
                right,
                operator,
            } => {
                res.push_str(&format!(
                    "{} {} {}",
                    left.reverse_polish_notation(),
                    right.reverse_polish_notation(),
                    operator.lexeme,
                ));
            }
            Self::Grouping { expression } => {
                res.push_str(&format!("{}", expression.reverse_polish_notation()));
            }
            Self::Literal { value } => res.push_str(&value.to_string()),
            Self::Unary { right, operator } => {
                res.push_str(&format!(
                    "{} {}",
                    right.reverse_polish_notation(),
                    operator.lexeme,
                ));
            }
            _ => unimplemented!(),
        }

        res
    }
}
