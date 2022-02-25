use crate::expr::*;

impl Expr {
    pub fn graph(&self) -> String {
        let mut res = String::new();
        res.push_str("digraph G {\n\t");
        res.push_str(&self._graph(&mut 0));
        res.push_str("}");

        res
    }

    fn _graph(&self, count: &mut usize) -> String {
        let mut res = String::new();

        match self {
            Self::Binary {
                left,
                right,
                operator,
            } => {
                let id = format!("binary_{count}");
                *count += 1;
                res.push_str(&format!("{id}\n"));
                res.push_str(&format!("\t{id} [label=\"{}\"]\n", operator.lexeme));
                res.push_str(&format!("\t{id} -> {}", left._graph(count)));
                res.push_str(&format!("\t{id} -> {}", right._graph(count)));
            }
            Self::Grouping { expression } => {
                let id = format!("group_{count}");
                *count += 1;
                res.push_str(&format!("{id}\n"));
                res.push_str(&format!("\t{id} [label=\"group\"]\n"));
                res.push_str(&format!("\t{id} -> {}", expression._graph(count)));
            }
            Self::Literal { value } => {
                let id = format!("literal_{count}");
                *count += 1;
                res.push_str(&format!("{id}\n"));
                res.push_str(&format!("\t{id} [label=\"{}\"]\n", value));
            }
            Self::Unary { right, operator } => {
                let id = format!("unary_{count}");
                *count += 1;
                res.push_str(&format!("{id}\n"));
                res.push_str(&format!("\t{id} [label=\"{}\"]\n", operator.lexeme));
                res.push_str(&format!("\t{id} -> {}", right._graph(count)));
            }
            _ => unimplemented!(),
        }
        res
    }
}
