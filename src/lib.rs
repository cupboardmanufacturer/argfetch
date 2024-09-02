// checks if a flag exists, and returns the argument
// if it does not exist it returns an empty string
pub fn fetch(flags: String, args: &Vec<String>) -> String {
    let flags_vec: Vec<String> = flags.split_whitespace().map(str::to_string).collect();
    
    let mut i = 0;
    let mut result = String::new();

    for flag in &flags_vec {
        result = parse(&flags_vec[i], &args);

        if !result.is_empty() {
            return result;
        }
        i += 1;
    }
    return result;
}

fn parse(flag: &String, args: &Vec<String>) -> String {
    let mut argument = String::new();
    let mut i = 0;
    
    for arg in args {
        if arg == flag {
            loop {
                if i >= args.len() {
                    return argument.trim().to_string();
                }
                if args[i].starts_with("-") && &args[i] != flag {
                    return argument.trim().to_string();
                }
                if &args[i] != flag {
                    argument.push_str(&args[i]);
                    argument.push_str(" ");
                }
                i += 1;
            }
        }
        i += 1;
    }

    return argument.trim().to_string();
}
