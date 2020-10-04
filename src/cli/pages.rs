/// Display the help page
pub fn display_help()
{
    println!("Usage: compiler [options] file...");
    println!("Options:");
    println!("     --help               Display this page");
    println!(" -o            [FILE]     Redirect the output to the given file");
    println!("     --stdout             Display the output on stdout");
}

// Display the version page
pub fn display_version()
{
    println!("compiler v0.0.0");
    println!("    (c) 2020 Carter Plasek");
}