#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {

    pub fn as_str (&self) -> String {
        
        match self {

            Command::Power (boolean, integer) => {

                if *boolean == false {

                    let str = format!("Power decreased by {}%", *integer);
                    return str.as_str().to_string();

                } else{

                    let str = format!("Power increased by {}%", *integer);
                    return str.as_str().to_string();

                }

            },

            Command::Missiles (boolean, integer) => {

                if *boolean == false {

                    let str = format!("Missiles decreased by {}", *integer);
                    return str.as_str().to_string();

                } else {

                    let str = format!("Missiles increased by {}", *integer);
                    return str.as_str().to_string();

                }

            },

            Command::Shield(boolean) => {

                if *boolean == false {

                    return "Shield turned off".to_string();

                } else {

                    return "Shield turned on".to_string();

                }

            },

            Command::Try => {

                return "Call attempt failed".to_string();

            },

            _ => return "Not a command".to_string(),

        }

    }

}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {

    let next_command = s.chars().next();

    match next_command {

        Some(command) => 
        
        if command == 'p' {

            let mut slice = &s[6..];
    
            if slice.starts_with("inc") || slice.starts_with("dec") {
    
                let choice = slice.starts_with("inc");
                slice = &slice[4..];
                let str_num = slice.parse :: <i32>();
    
                match str_num{
    
                    Ok(num) => {
    
                        let slice_len = num.to_string().len();
                        slice = &slice[slice_len..];
    
                        match slice.trim() {
    
                            "" => return Command :: Power(choice, num),
    
                            _ => return Command :: Invalid,
    
                        }
    
                    },
    
                    Err(x) => {
    
                        return Command :: Invalid;
    
                    },
    
                }
    
            } else {
    
                return Command::Invalid;
    
            }
    
        } else if command == 'a' || command == 'f' {
    
            let decision = s.starts_with("add");
            let mut len = if decision {4} else {5};
            let mut slice = &s[len..];
            let mut split_slice = slice.split_whitespace();
            let num = split_slice.next().unwrap().parse::<i32>();
    
            match num {
    
                Ok(n) => {
    
                    match split_slice.next() {
    
                        None => {
    
                            return Command::Invalid;
    
                        }
    
                        Some(string) =>{
    
                            if string.starts_with("missiles") {
    
                                return Command::Missiles(decision, n);
    
                            } else {
    
                                return Command::Invalid;
    
                            }
    
                        }
    
                    }
                },
    
                Err(x) => {
    
                    return Command :: Invalid;
    
                },
    
            }
    
        } else if command == 's' {
    
            let mut slice = &s[7..];
            let choice = slice.starts_with("on");
            let mut len = if choice {2} else {3};
    
            slice = &slice[len..];
    
            match slice.trim() {
    
                "" => return Command :: Shield(choice),
    
                _ => return Command :: Invalid,
            }
    
        } else if command == 't' {
    
            return Command :: Try;
    
        } else {

            return Command :: Invalid;

        }

        None => return Command :: Invalid,

    }

}