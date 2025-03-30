use std::fs::File;
use std::io::{self, Write};

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &[
            "Binary   = left: Box<Expr>, operator: Token, right: Box<Expr>",
            "Grouping = expression: Box<Expr>",
            "Literal  = value: LiteralType",
            "Unary    = operator: Token, right: Box<Expr>",
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(&path)?;

    writeln!(file, "use crate::token::{{LiteralType, Token}};")?;
    writeln!(file, "")?;

    for type_str in types {
        let parts: Vec<&str> = type_str.split('=').collect();
        let class_name = parts[0].trim();
        let fields = parts[1].trim();
        define_type(&mut file, class_name, fields)?;
    }

    writeln!(file, "#[derive(Debug)]")?;
    writeln!(file, "pub enum {} {{", base_name)?;
    for type_str in types {
        let parts: Vec<&str> = type_str.split('=').collect();
        let class_name = parts[0].trim();

        writeln!(file, "    {}({}),", class_name, class_name)?;
    }

    writeln!(file, "}}\n")?;

    writeln!(file, "pub trait Visitor<T> {{")?;
    writeln!(file, "    fn visit(expr: {}) -> T;", base_name)?;
    writeln!(file, "}}")?;

    Ok(())
}

fn define_type(file: &mut File, class_name: &str, field_list: &str) -> io::Result<()> {
    writeln!(file, "#[derive(Debug)]")?;
    writeln!(file, "pub struct {} {{", class_name)?;

    field_list.split(", ").for_each(|field| {
        let parts: Vec<&str> = field.split(':').collect();
        let field_name = parts[0].trim();
        let field_type = parts[1].trim();
        writeln!(file, "    pub {}: {},", field_name, field_type).unwrap();
    });

    writeln!(file, "}}\n")?;

    Ok(())
}
