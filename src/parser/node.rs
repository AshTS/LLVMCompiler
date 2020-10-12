use crate::tokenizer::Token;

/// Expression Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpressionType
{
    ArrayAccess,
    FunctionCall,
    PostIncrement,
    PostDecrement,
    PreIncrement,
    PreDecrement,
    UnaryPlus,
    UnaryMinus,
    LogicalNot,
    BitwiseNot,
    Dereference,
    DereferenceLeft,
    Reference,
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    Ternary,
    Assignment,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModulusAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
    Cast,
    Comma
}

/// Parse Tree Node
#[derive(Debug, Clone)]
pub enum ParseTreeNode
{
    Library(Vec<ParseTreeNode>),
    Function(Vec<ParseTreeNode>),
    Arguments(Vec<ParseTreeNode>),
    Argument(Vec<ParseTreeNode>),
    Type(Vec<ParseTreeNode>),
    Identifier(Token),
    RawType(Token),
    Statement(Vec<ParseTreeNode>),
    Statements(Vec<ParseTreeNode>),
    Assignments(Vec<ParseTreeNode>),
    Assignment(Vec<ParseTreeNode>),
    Expression(ExpressionType, Vec<ParseTreeNode>),
    RawToken(Token),
    IntegerLiteral(Token),
    AssignmentStatement(Vec<ParseTreeNode>),
    IfStatement(Vec<ParseTreeNode>),
    ReturnStatement(Vec<ParseTreeNode>),
    WhileLoop(Vec<ParseTreeNode>),
    DoWhileLoop(Vec<ParseTreeNode>),
    Loop(Vec<ParseTreeNode>),
    Empty
}

fn render_node(node: ParseTreeNode) -> (String, Vec<ParseTreeNode>)
{
    match node
    {
        ParseTreeNode::Library(nodes) => (format!("Library"), nodes),
        ParseTreeNode::Function(nodes) => (format!("Function"), nodes),
        ParseTreeNode::Arguments(nodes) => (format!("Arguments"), nodes),
        ParseTreeNode::Argument(nodes) => (format!("Argument"), nodes),
        ParseTreeNode::Type(nodes) => (format!("Type"), nodes),
        ParseTreeNode::Identifier(token) => (format!("Identifier ({})", token.data), vec![]),
        ParseTreeNode::RawType(token) => (format!("Raw Type ({})", token.data), vec![]),
        ParseTreeNode::Statement(nodes) => (format!("Statement"), nodes),
        ParseTreeNode::Statements(nodes) => (format!("Statements"), nodes),
        ParseTreeNode::Assignments(nodes) => (format!("Assignments"), nodes),
        ParseTreeNode::Assignment(nodes) => (format!("Assignment"), nodes),
        ParseTreeNode::Expression(exprtype, nodes) => (format!("Expression ({:?})", exprtype), nodes),
        ParseTreeNode::RawToken(token) => (format!("Raw Token ({})", token.data), vec![]),
        ParseTreeNode::IntegerLiteral(token) => (format!("Integer ({})", token.data), vec![]),
        ParseTreeNode::AssignmentStatement(nodes) => (format!("Assignment Statement"), nodes),
        ParseTreeNode::IfStatement(nodes) => (format!("If Statement"), nodes),
        ParseTreeNode::ReturnStatement(nodes) => (format!("Return Statement"), nodes),
        ParseTreeNode::WhileLoop(nodes) => (format!("While Loop"), nodes),
        ParseTreeNode::DoWhileLoop(nodes) => (format!("Do While Loop"), nodes),
        ParseTreeNode::Loop(nodes) => (format!("Loop"), nodes),
        ParseTreeNode::Empty => (format!("Empty"), vec![]),
    }
}

pub fn display_parse_tree(node: ParseTreeNode, prev: String, is_last: bool)
{
    let mut mprev = prev.clone();
    let (text, nodes) = render_node(node);

    if mprev.len() > 0
    {
        mprev.pop();
        mprev += if is_last {"└"} else {"├"};
    }
    
    println!("{}{}{}", mprev, if nodes.len() == 0 {"─"} else {"┬"}, text);

    if mprev.len() > 0
    {
        mprev.pop();
        mprev += if is_last {" "} else {"│"};
    }

    for (i, n) in nodes.iter().enumerate()
    {
        let new_prev = format!("{}{}", mprev, "│");
        display_parse_tree(n.clone(), new_prev, i == nodes.len() - 1);
    }
}