use crate::ast::{Type, Node};

fn compile_type(t: &Type) -> String {
    let mut out = String::new();

    match t {
        Type::Uint8 => {
            out += "unsigned char";
        }
        Type::Uint16 => {
            out += "unsigned short";
        }
        Type::Uint32 => {
            out += "unsigned int";
        }
        Type::Uint64 => {
            out += "unsigned long long";
        }
        Type::Int8 => {
            out += "signed char";
        }
        Type::Int16 => {
            out += "signed short";
        }
        Type::Int32 => {
            out += "signed int";
        }
        Type::Int64 => {
            out += "signed long long";
        }
        Type::Ptr(x) => {
            out += &format!("{} *", compile_type(x));
        }
    };

    out
}

fn compile_node(node: &Node) -> String {
    let mut out = String::new();

    match node {
        Node::Int(x) => {
            out += &x;
        }

        Node::Name(x) => {
            out += &x;
        }

        Node::Op(op, a, b) => {
            out += &format!("{} {} {}", compile_node(a), op, compile_node(b));
        }

        Node::Type(x) => {
            out += &compile_type(x);
        }

        Node::Assign(name, t, val) => {
            out += &format!("{} {} = {};\n", compile_node(t), name, compile_node(val));
        }

        Node::Re(name, val) => {
            out += &format!("{} = {};\n", name, compile_node(val));
        }

        Node::Return(e) => {
            out += &format!("return {};\n", compile_node(e));
        }

        Node::Block(all) => {
            for i in all.iter() {
                out += &compile_node(i);
            }
        }

        Node::Func(name, args, ret, body) => {
            out += 
                &format!(
                    "{} {} (",
                    compile_node(ret),
                    name
                );
            for (name, i) in args.iter() {
                out += &format!("{} {},", compile_node(i), name);
            }

            out += &format!(") {{\n{}\n}}\n", &compile_node(body));
        }

        Node::If(e, body, other) => {
            out += &format!("if ({}) {{\n{}\n}}\n", compile_node(e), compile_node(body));
            match other {
                Some(x) => {
                    out += &format!("else {{\n{}\n}}\n", compile_node(x));
                }
                None => { }
            }
        }

        Node::While(e, body) => {
            out += &format!("while ({}) {{\n{}\n}}\n", compile_node(e), compile_node(body));
        }
    }

    out
}

pub fn compile(ast: &Vec<Node>) -> String {
    let mut out = String::new();

    for item in ast.iter() {
        out += &compile_node(item);
    }

    out
}
