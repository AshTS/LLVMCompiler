#[derive(Debug, Clone)]
pub struct Options
{
    short_flags: Vec<String>,
    long_flags: Vec<String>,
    raw_vals: Vec<String>
}

impl Options
{
    pub fn new(opts: Vec<String>) -> Self
    {
        let mut mut_opts = opts.clone();
        let mut short_flags: Vec<String> = vec![];
        let mut long_flags: Vec<String> = vec![];
        let mut raw_vals: Vec<String> = vec![];

        mut_opts.remove(0);

        for opt in mut_opts
        {
            let mut mut_opt = opt.clone();

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
                raw_vals.push(mut_opt);
            }
        }

        Options
        {
            short_flags,
            long_flags,
            raw_vals
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