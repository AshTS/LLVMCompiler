use std::collections::HashMap;

static ACCEPT_ARGUMENTS: &[&str] = &["-o", "--out"];

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

            if opt.starts_with("-")
            {
                if current_key.len() > 0
                {
                    map.insert(current_key, current_arguments);
                    current_arguments = Vec::new();
                }

                if ACCEPT_ARGUMENTS.contains(&opt.as_str())
                {
                    current_key = opt.clone();
                }
                else
                {
                    current_key = String::new();
                }
            }

            if opt.starts_with("--")
            {
                mut_opt.remove(0);
                mut_opt.remove(0);

                long_flags.push(mut_opt);
            }
            else if opt.starts_with("-")
            {
                mut_opt.remove(0);

                short_flags.push(mut_opt);
            }
            else
            {
                if current_key.len() == 0
                {
                    raw_vals.push(mut_opt);
                }
                else
                {
                    current_arguments.push(mut_opt);
                }
            }
        }

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

    pub fn has_long_flag(&self, flag: &str) -> bool
    {
        self.long_flags.contains(&String::from(flag))
    }

    pub fn has_short_flag(&self, flag: &str) -> bool
    {
        self.short_flags.contains(&String::from(flag))
    }

    pub fn get_raw_values(&self) -> Vec<String>
    {
        self.raw_vals.clone()
    }
}