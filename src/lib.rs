// checks if a flag exists, and returns the argument
// if it does not exist it returns an empty string
pub fn parse(flag: String, args: &Vec<String>) -> String {
    let mut argument = String::new();
    let mut i = 0;
    
    for arg in args {
        if arg == &flag {
            loop {
                if i >= args.len() {
                    return argument.trim().to_string();
                }
                if args[i].starts_with("-") && args[i] != flag {
                    return argument.trim().to_string();
                }
                if args[i] != flag {
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
