use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <output directory>");
        return;
    }
    let output_dir = &args[1];

    define_ast(
        output_dir,
        "Expr",
        [
            "Binary:     Box<dyn Expr> left, Token operator, Box<dyn Expr> right",
            "Grouping:   Box<dyn Expr> expression",
            "Literal:    String value",
            "Unary:      Token operator, Box<dyn Expr> right",
        ]
        .as_slice(),
    );
}

fn define_ast(output_dir: &str, basename: &str, types: &[&str]) {
    let path = format!("{output_dir}/{}.rs", basename.to_lowercase());
    let mut file = File::create(path).unwrap();

    writeln!(file, "use crate::token::Token;").unwrap();
    writeln!(file).unwrap();

    writeln!(file, "pub trait {basename}: std::fmt::Debug {{").unwrap();
    writeln!(
        file,
        "\tfn accept<R>(&self, visitor: &impl Visitor<R>) -> R"
    )
    .unwrap();
    writeln!(file, "\twhere").unwrap();
    writeln!(file, "\t\tSelf: Sized;").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    define_visitor(&mut file, basename, types);

    for ty in types {
        let mut split = ty.split(':');
        let type_name = split.next().unwrap().trim();
        let field = split.next().unwrap().trim();
        define_type(&mut file, basename, type_name, field);
    }
}

fn define_visitor(file: &mut File, basename: &str, types: &[&str]) {
    writeln!(file, "pub trait Visitor<R> {{").unwrap();

    for ty in types {
        let mut split = ty.split(':');
        let type_name = split.next().unwrap().trim();
        // let field = split.next().unwrap().trim();

        let function_name = format!(
            "visit_{}_{}",
            type_name.to_lowercase(),
            basename.to_lowercase()
        );
        writeln!(
            file,
            "\tfn {function_name}(&self, {}: &{type_name}) -> R;",
            basename.to_lowercase()
        )
        .unwrap();
    }

    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();
}

fn define_type(file: &mut File, basename: &str, type_name: &str, fields: &str) {
    writeln!(file, "#[derive(Debug)]").unwrap();
    writeln!(file, "pub struct {type_name} {{").unwrap();
    fields.split(',').for_each(|field| {
        let (ty, name) = field.trim().rsplit_once(' ').unwrap();
        writeln!(file, "\tpub {}: {},", name.trim(), ty.trim()).unwrap();
    });
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    let visitor_function_name = format!(
        "visit_{}_{}",
        type_name.to_lowercase(),
        basename.to_lowercase()
    );

    writeln!(file, "impl {basename} for {type_name} {{").unwrap();
    writeln!(
        file,
        "\tfn accept<R>(&self, visitor: &impl Visitor<R>) -> R {{"
    )
    .unwrap();
    writeln!(file, "\t\tvisitor.{visitor_function_name}(self)").unwrap();
    writeln!(file, "\t}}").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();
}
