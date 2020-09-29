use super::io::InputFile;
use super::cli::{Error, ErrorRecorder};
use super::tokenizer::tokenize;

pub fn compile(input: InputFile) -> Result<(), Error>
{
    let mut recorder: ErrorRecorder = ErrorRecorder::new();
    let data = input.data;
    let filename = input.filename;

    let tokens = tokenize(data, filename);

    let node = recorder.wrap_return(super::parser::parse(tokens))?;

    if node.is_some()
    {
        super::parser::display_parse_tree(node.unwrap(), String::new(), false);
    }
    else
    {
        println!("Error: No Parse Tree Returned");
    }

    Ok(())
}