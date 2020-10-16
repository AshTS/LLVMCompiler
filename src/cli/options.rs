use std::collections::HashMap;

/// Flags which accept arguments
static ACCEPT_ARGUMENTS: &[&str] = &["-o", "--out", "-g", "-O"];

/// Struct containing information regarding the command line arguments passed
/// to the application
#[derive(Debug, Clone)]
pub struct Options
{
    short_flags: Vec<String>,
    long_flags: Vec<String>,
    raw_vals: Vec<String>,
    pub map: HashMap<String, Vec<String>>
}

impl Options
{
    /// Generate a new options object from a vector of strings passed to the application
    pub fn new(opts: Vec<String>) -> Self
    {
        let mut mut_opts = opts.clone();
        let mut short_flags: Vec<String> = vec![];
        let mut long_flags: Vec<String> = vec![];
        let mut raw_vals: Vec<String> = vec![];
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        mut_opts.remove(0);

        let mut current_arguments: Vec<String> = vec![];
        let mut current_key: String = String::new();

        for opt in mut_opts
        {
            let mut mut_opt = opt.clone();

            // If the argument starts with a '-' it must be a flag
            if opt.starts_with("-")
            {
                // Add the last key and arguments to the saved hashmap
                if current_key.len() > 0
                {
                    map.insert(current_key, current_arguments);
                    current_arguments = Vec::new();
                }

                // If the current argument accepts arguments then the current key will be used
                if ACCEPT_ARGUMENTS.contains(&opt.as_str())
                {
                    current_key = opt.clone();
                }
                else
                {
                    current_key = String::new();
                }
            }

            // Interpret a long flag
            if opt.starts_with("--")
            {

                // Remove the "--"
                mut_opt.remove(0);
                mut_opt.remove(0);

                // Record the flag
                long_flags.push(mut_opt);
            }
            // Interpret a short flag
            else if opt.starts_with("-")
            {
                // Remove the "-"
                mut_opt.remove(0);

                // Record the flag
                short_flags.push(mut_opt);
            }
            // Otherwise it must be an argument
            else
            {
                // Either pass it to the raw values vector
                if current_key.len() == 0
                {
                    raw_vals.push(mut_opt);
                }
                // Or use it as an argument to the current key
                else
                {
                    current_arguments.push(mut_opt);
                }
            }
        }

        // Make sure the last argument isn't dropped
        if current_key.len() > 0
        {
            map.insert(current_key, current_arguments);
        }

        Options
        {
            short_flags,
            long_flags,
            raw_vals,
            map
        }
    }

    /// Checks if a given long flag has been passed to the application
    pub fn has_long_flag(&self, flag: &str) -> bool
    {
        self.long_flags.contains(&String::from(flag))
    }

    /// Checks if a given short flag has been passed to the application
    pub fn has_short_flag(&self, flag: &str) -> bool
    {
        self.short_flags.contains(&String::from(flag))
    }

    /// Get a vector of the raw values passed to the application 
    pub fn get_raw_values(&self) -> Vec<String>
    {
        self.raw_vals.clone()
    }
}