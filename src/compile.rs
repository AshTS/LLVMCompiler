use std::io::Write;

use super::io::InputFile;
use super::cli::{Error, ErrorRecorder, Options};
use super::tokenizer::tokenize;
use super::irgen;
use super::codegen::{CodeGenerator, CodegenMode};

use super::parser::ParseTreeNode;

pub fn compile(input: InputFile, options: &Options) -> Result<(), Error>
{
    let mut recorder: ErrorRecorder = ErrorRecorder::new();
    let data = input.data;
    let filename = input.filename;

    let tokens = tokenize(data, filename);

    let node = recorder.wrap_return(super::parser::parse(tokens))?;

    if node.is_some()
    {
        super::parser::display_parse_tree(node.clone().unwrap(), String::new(), false);
    }
    else
    {
        Err(Error::fatal_error("No Parse Tree Returned"))?
    }

    let mut functions = vec![];

    match node.unwrap()
    {
        ParseTreeNode::Library(children) =>
        {
            for child in children
            {
                let mut function = irgen::Function::from_parse_tree_node(child)?;
                function = irgen::optimize_function(function);

                functions.push(function);
            }
        },
        _ => {}
    }

    let mut codegen_mode = CodegenMode::IntermediateRepresentation;

    if let Some(name) = options.map.get("-g")
    {
        codegen_mode = CodegenMode::from_mode(&name[0]);
    }

    let output = CodeGenerator::new(codegen_mode, functions).render()?;

    //let target = llvm::TargetTriple::new(llvm::Architecture::X86_64, llvm::Vendor::Unknown, llvm::OperatingSystem::Linux);
    //let mut module = llvm::Module::new(target);

    //module.load_from_parse_tree(node.clone().unwrap())?;

    if options.has_long_flag("stdout")
    {
        println!("Output:\n{}", output);
    }
    else
    {
        let mut output_filename = "out.ll";

        if let Some(name) = options.map.get("-o")
        {
            output_filename = &name[0];
        }


        // Write to the output file
        let file = std::fs::File::create(output_filename);

        if file.is_err()
        {
            Err(Error::fatal_error(&format!("Could not create output file '{}'", output_filename)))?;
        }

        if let Err(_error) = write!(file.unwrap(), "{}", output)
        {
            Err(Error::fatal_error(&format!("Could not write to output file '{}'", output_filename)))?;
        }
    }
    
    Ok(())
}