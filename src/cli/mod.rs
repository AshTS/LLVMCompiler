mod pages;
mod options;
mod errors;
mod execute;

pub use options::*;
pub use errors::*;

/// Run the application with the given options
pub fn run(opts: &Options) -> Result<(), Error>
{
    // Display the help documentation if the help flag is included
    if opts.has_long_flag("help")
    {
        pages::display_help();
        Ok(())
    }
    // Display the version if the version flag is included
    else if opts.has_long_flag("version")
    {
        pages::display_version();
        Ok(())
    }
    else
    {
        execute::execute(opts)
    }
}