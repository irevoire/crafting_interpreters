pub trait Expr {
}

pub trait Expr<R> {
	fn visit_binary_expr(&self, expr: Binary) -> R;
	fn visit_grouping_expr(&self, expr: Grouping) -> R;
	fn visit_literal_expr(&self, expr: Literal) -> R;
	fn visit_unary_expr(&self, expr: Unary) -> R;
}

#[derive(Debug, Clone)]
pub struct Binary {
	pub left: Expr,
	pub operator: Token,
	pub right: Expr,
}

impl Binary {
	pub fn accept<R>(&self, visitor: impl Visitor<R>) -> R {
		visitor.visit_binary_expr(self)
	}
}

#[derive(Debug, Clone)]
pub struct Grouping {
	pub expression: Expr,
}

impl Grouping {
	pub fn accept<R>(&self, visitor: impl Visitor<R>) -> R {
		visitor.visit_grouping_expr(self)
	}
}

#[derive(Debug, Clone)]
pub struct Literal {
	pub value: Object,
}

impl Literal {
	pub fn accept<R>(&self, visitor: impl Visitor<R>) -> R {
		visitor.visit_literal_expr(self)
	}
}

#[derive(Debug, Clone)]
pub struct Unary {
	pub operator: Token,
	pub right: Expr,
}

impl Unary {
	pub fn accept<R>(&self, visitor: impl Visitor<R>) -> R {
		visitor.visit_unary_expr(self)
	}
}

