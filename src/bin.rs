use passt::Passt;
use std::env;
use std::process::exit;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// checks if an CLI parameter is set
fn extract_arg_present(param: &str, args: &Vec<String>) -> bool {
    let pos = match args.iter().position(|x| x.as_str() == param) {
        Some(x) => x + 1,
        None => 0,
    };

    return pos > 0;
}

// return the argument specified for a parameter
fn extract_arg(param: &str, args: &Vec<String>) -> String {
    let idx = args
        .iter()
        .position(|x| x.as_str() == param)
        .unwrap_or(123456789);

    // hacky way of checking if this param was set or not. If the index is 123456789 we return an empty string.
    // This means this will break if we have a 123456789th cli param :D
    if idx == 123456789 {
        return String::from("");
    }
    if args.len() == idx + 1 {
        usage(Some(format!(
            "Error: Parameter {} requires value.\n",
            param
        )));
    }
    args[idx + 1].to_owned()
}

fn usage(msg: Option<String>) {
    let usage = r#"USAGE: passt -l <int> [-s] [-chars "<str>"] [-n <int>]

-l      length of the generated password
-n      number of passwords to create (default: 1)
-s      use special characters
-chars  possible characters as a string, e.g. "abc012"
"#;

    println!(
        "passt v{}\n\n{}{}",
        VERSION,
        msg.unwrap_or(String::from("")),
        usage
    );
    exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        usage(None);
    }

    let len = match extract_arg("-l", &args).as_str().parse::<i32>() {
        Ok(n) => n,
        Err(err) => {
            usage(Some(format!("Error: {}\n", err)));
            0
        }
    };
    let mut iterations = match extract_arg("-n", &args).as_str().parse::<i32>() {
        Ok(n) => n,
        Err(_) => 1,
    };
    let chars = extract_arg("-chars", &args);
    let special = extract_arg_present("-s", &args);

    while iterations > 0 {
        // Create password with custom set if set, or defaults
        if chars.len() > 0 {
            println!(
                "{}",
                Passt::random_password_with_custom_set(len, chars.as_str())
            );
        } else {
            let passwd = Passt::random_password(len, Some(special));
            println!("{}", passwd);
        }
        iterations = iterations - 1;
    }
}

mod tests {
    #[test]
    fn test_extract_arg() {
        use super::extract_arg;
        use std::collections::HashMap;

        let mut cases: HashMap<Vec<String>, Vec<&str>> = HashMap::new();
        cases.insert(
            vec![String::from("-l"), String::from("16"), String::from("c")],
            vec!["-l", "16"],
        );
        cases.insert(
            vec![
                String::from("-s"),
                String::from("hello-world"),
                String::from("c"),
            ],
            vec!["-l", ""],
        );
        cases.insert(
            vec![
                String::from("-s"),
                String::from("hello-world"),
                String::from("c"),
            ],
            vec!["-s", "hello-world"],
        );
        for case in cases.iter() {
            let input = case.0;
            let expected = (case.1)[1];
            let param = (case.1)[0];
            assert_eq!(extract_arg(param, input), expected);
        }
    }

    #[test]
    fn test_extract_arg_present() {
        use super::extract_arg_present;
        use std::collections::HashMap;
        // cases has the following format
        // HashMap<vec!["cli", "inputs", "-l", "16"], vec!["search-for-this", "expected-this-result"]>
        let mut cases: HashMap<Vec<String>, Vec<&str>> = HashMap::new();
        cases.insert(
            vec![String::from("-l"), String::from("16"), String::from("c")],
            vec!["-l", "true"],
        );
        cases.insert(
            vec![
                String::from("-s"),
                String::from("hello-world"),
                String::from("c"),
            ],
            vec!["-l", "false"],
        );
        cases.insert(
            vec![
                String::from("-s"),
                String::from("hello-world"),
                String::from("c"),
            ],
            vec!["-s", "true"],
        );
        for case in cases.iter() {
            let input = case.0;
            let expected = (case.1)[1];
            let param = (case.1)[0];

            let res = match extract_arg_present(param, input) {
                true => "true",
                false => "false",
            };
            assert_eq!(res, expected);
        }
    }
}
