use super::{Options, Error, ErrorRecorder};

/// Execute the compiler
pub fn execute(opts: &Options) -> Result<(), Error>
{
    let mut recorder: ErrorRecorder = ErrorRecorder::new();
    let mut input_files: Vec<crate::io::InputFile> = Vec::new();

    // Open input files
    for filename in opts.get_raw_values()
    {
        let v = recorder.wrap_return(crate::io::InputFile::new(filename))?;
        if !v.is_none()
        {
            input_files.push(v.unwrap());
        }
        
    }

    // If no files are found, error
    if input_files.len() == 0
    {
        recorder.report_error(Error::fatal_error("No input files"))?;
    }

    // Loop over input files and compile them
    for input_file in input_files
    {
        recorder.wrap_return(crate::compile::compile(input_file, opts))?;
    }

    Ok(())
}