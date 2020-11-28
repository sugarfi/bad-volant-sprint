#[derive(Clone, Debug)]
pub enum Type {
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Ptr(Box<Type>)
}

#[derive(Clone, Debug)]
pub enum Node {
    Int(String),
    Name(String),
    Op(String, Box<Node>, Box<Node>),
    Type(Type),
    Assign(String, Box<Node>, Box<Node>),
    Re(String, Box<Node>),
    Return(Box<Node>),
    Block(Vec<Node>),
    Func(String, Vec<(String, Node)>, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Option<Box<Node>>),
    While(Box<Node>, Box<Node>)
}
