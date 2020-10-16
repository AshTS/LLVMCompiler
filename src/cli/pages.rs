/// Display the help page
pub fn display_help()
{
    println!("Usage: compiler [options] file...");
    println!("Options:");
    println!("     --help               Display this page");
    println!(" -g            [MODE]     Set the code gen mode to use");
    println!("     --nocomp             Do not collapse register usage");
    println!(" -o            [FILE]     Redirect the output to the given file");
    println!(" -O            [VAL]      Set the optimization level (defaults to 2)");
    println!("     --stdout             Display the output on stdout");
    println!(" -T  --tree               Display the parse tree");
    println!("\nAllowable Codegen Modes:");
    println!("   ir");
    println!("   avrasm");
}

// Display the version page
pub fn display_version()
{
    println!("compiler v0.0.0");
    println!("    (c) 2020 Carter Plasek");
}