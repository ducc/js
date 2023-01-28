use nom::{IResult, bytes::complete::{tag, take_until, take}, number::complete::float, combinator::opt};

fn main() {
    let input = r#"
    
        println("Hello, world!", "meme", 36.99, "mooo")

        something("dabbing", true, false,   true)

        if (5 == 5) {
            println("it is 5")

            println("i am joe")
        }

    "#;

    let (_, statements) = parse_statements(input.trim()).unwrap();
    
    let ast = Tree {
        statements,
    };

    dbg!(ast);
}

#[derive(Debug)]
pub struct Tree {
    pub statements: Vec<JsStatement>,
}

#[derive(Debug, PartialEq)]
pub enum JsStatement {
    Invocation(JsInvocation),
    If(JsIf),
}

#[derive(Debug, PartialEq)]
pub struct JsInvocation {
    pub name: String,
    pub arguments: Vec<JsPrimitive>,
}

#[derive(Debug, PartialEq)]
pub struct JsIf {
    pub expression: JsIfExpression,
    pub statements: Vec<JsStatement>,
}

#[derive(Debug, PartialEq)]
pub struct JsIfExpression {
    pub left: JsPrimitive,
    pub comparitor: JsComparitor,
    pub right: JsPrimitive,
}

#[derive(Debug, PartialEq)]
pub enum JsComparitor {
    Equals,
    NotEquals,
}

#[derive(Debug, PartialEq)]
pub enum JsPrimitive {
    String(JsString),
    Number(JsNumber),
    Bool(JsBool),
}

#[derive(Debug, PartialEq)]
pub struct JsString {
    pub inner: String,
}

#[derive(Debug, PartialEq)]
pub struct JsNumber {
    pub inner: f32,
}

#[derive(Debug, PartialEq)]
pub struct JsBool {
    pub inner: bool,
}

fn parse_statements(mut input: &str) -> IResult<&str, Vec<JsStatement>> {
    let mut statements = vec![];
    
    loop {
        if input.is_empty() {
            break;
        }
        
        let (new_input, statement) = parse_statement(input.trim()).unwrap();

        input = new_input;
        statements.push(statement);
    }
    
    Ok((input, statements))
}

fn parse_statement(input: &str) -> IResult<&str, JsStatement> {
    if let (input, Some(_)) = opt(tag("if"))(input)? {
        let (input, jsif) = parse_if(input)?;
        return Ok((input, JsStatement::If(jsif)));
    }

    parse_invocation(input).map(|(input, invocation)| (input, JsStatement::Invocation(invocation)))
}

fn parse_if(input: &str) -> IResult<&str, JsIf> {
    let (input, _) = take_until("(")(input)?;

    let (input, _) = take(1usize)(input)?;

    let (input, expression) = take_until(")")(input)?;
    
    let (_, expression) = parse_if_expression(expression)?;

    let (input, _) = take_until("{")(input)?;
    println!("inp '{}'", input);

    let (input, _) = take(1usize)(input)?;

    let input = input.trim();

    let (input, statements) = take_until("}")(input)?;

    println!("inp '{}' stmts '{}'", input, statements);

    let (input, statements) = parse_statements(statements)?;

    println!("inp '{}' stmts '{:?}", input, statements);

    Ok((input, JsIf {
        expression,
        statements,
    }))    
}

fn parse_if_expression(input: &str) -> IResult<&str, JsIfExpression> {
    let (input, left) = parse_primitive(input)?;

    let input = input.trim();

    let (input, comparitor) = parse_comparitor(input)?;
    
    let input = input.trim();

    let (input, right) = parse_primitive(input)?;

    Ok((input, JsIfExpression {
        left: left.unwrap(),
        comparitor,
        right: right.unwrap(),
    }))
}

fn parse_comparitor(input: &str) -> IResult<&str, JsComparitor> {
    if let (input, Some(equals)) = opt(tag("=="))(input)? {
        return Ok((input, JsComparitor::Equals));
    }

    if let (input, Some(not_equals)) = opt(tag("!="))(input)? {
        return Ok((input, JsComparitor::NotEquals));
    }

    panic!("unknown comparitor")
}

fn parse_invocation(input: &str) -> IResult<&str, JsInvocation> {
    // Consume function name until we find (
    let (input, function_name) = take_until("(")(input)?;

    // Remove opening (
    let (input, _) = take(1usize)(input)?;

    // Find all arguments
    let (input, args) = take_until(")")(input)?;

    // Parse arguments
    let (_, function_args) = parse_arguments(args, vec![])?;
    
    // Remove closing )
    let (input, _) =  take(1usize)(input)?;

    Ok((input, JsInvocation {
        name: function_name.into(),
        arguments: function_args,
    }))
}

fn parse_arguments(input: &str, mut args: Vec<JsPrimitive>) -> IResult<&str, Vec<JsPrimitive>> {
    let first_char = match input.chars().next() {
        Some(c) => c,
        None => return Ok((input, args)),
    };

    if let (input, Some(primitive)) = parse_primitive(input)? {
        args.push(primitive);
        return parse_arguments(input, args);
    }
    
    match first_char {
        // Drop any separation characters
        ',' | ' ' => {
            let (input, _) = take(1usize)(input)?;
            parse_arguments(input, args)
        }
        c => panic!("wtf is {}", c),
    }
}

fn parse_primitive(input: &str) -> IResult<&str, Option<JsPrimitive>> {
    if let (input, Some(parsed_string)) = opt(parse_string)(input)? {
        return Ok((input, Some(JsPrimitive::String(parsed_string))));
    }

    if let (input, Some(parsed_number)) = opt(parse_number)(input)? {
        return Ok((input, Some(JsPrimitive::Number(parsed_number))));
    }

    if let (input, Some(parsed_bool)) = opt(parse_true)(input)? {
        return Ok((input, Some(JsPrimitive::Bool(parsed_bool))));
    }

    if let (input, Some(parsed_bool)) = opt(parse_false)(input)? {
        return Ok((input, Some(JsPrimitive::Bool(parsed_bool))));
    }

    Ok((input, None))
}

fn parse_string(input: &str) -> IResult<&str, JsString> {
    // Find opening quote
    let (input, _) = tag("\"")(input)?;

    // Read until ending quote
    let (input, inner) = take_until("\"")(input)?;

    // Remove ending quote
    let (input, _) = take(1usize)(input)?;

    Ok((input, JsString { inner: inner.into() }))
}

fn parse_number(input: &str) -> IResult<&str, JsNumber> {
    let (input, num) = float(input)?;
    Ok((input, JsNumber { inner: num }))
}

fn parse_true(input: &str) -> IResult<&str, JsBool> {
    let (input, _) = tag("true")(input)?;
    Ok((input, JsBool { inner: true }))
}

fn parse_false(input: &str) -> IResult<&str, JsBool> {
    let (input, _) = tag("false")(input)?;
    Ok((input, JsBool { inner: false }))
}

