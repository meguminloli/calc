use calc::*;
use rust_decimal::Decimal;
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

const RESTRICTED_CHARS: &[char] = &[
    ' ', '\t', '\n', '+', '-', '/', '*', '^', '%', '!', '~', '&', '|', '<', '>', '=',
];

fn calculate(s: &str, variables: Option<&HashMap<String, Decimal>>) -> Result<Decimal, Error> {
    let tokens = parse_str(s, variables);
    let tokens = match tokens {
        Ok(tokens) => tokens,
        Err(err) => {
            return Err(err);
        }
    };
    let rpn = shunting_yard(tokens);
    let rpn = match rpn {
        Ok(rpn) => rpn,
        Err(err) => {
            return Err(err);
        }
    };
    let result = evaluate_rpn(rpn);
    let result = match result {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };
    Ok(result)
}

fn main() {
    let stdout = stdout();
    let stdin = stdin();
    let stdout_locked = stdout.lock();
    let stdin_locked = stdin.lock();
    let mut stdout = BufWriter::new(stdout_locked);
    let mut stdin = BufReader::new(stdin_locked);
    let mut variables: HashMap<String, Decimal> = HashMap::new();
    loop {
        write!(&mut stdout, ">").ok();
        stdout.flush().ok();
        let mut input = String::new();
        stdin.read_line(&mut input).ok();
        let input = input.trim_start().trim_end().to_string();
        if input == "exit" || input == "quit" {
            break;
        } else if input == "list" {
            for (key, value) in variables.iter() {
                writeln!(&mut stdout, "{} = {}", key, value).ok();
            }
        } else if input.as_str().contains('=') {
            let mut iter = input.split('=');
            let variable_name = iter.next().unwrap().to_string();
            let variable_name = variable_name.trim_start().trim_end();
            for ch in RESTRICTED_CHARS {
                if variable_name.contains(*ch) {
                    writeln!(&mut stdout, "Invalid variable name.").ok();
                    continue;
                }
            }
            let variable_value = iter.next().unwrap().to_string();
            if iter.next().is_some() {
                writeln!(&mut stdout, "Invalid assignment.").ok();
                continue;
            }
            let variable_value = match calculate(&variable_value, Some(&variables)) {
                Ok(variable_value) => variable_value,
                Err(err) => {
                    writeln!(&mut stdout, "{}", err).ok();
                    continue;
                }
            };
            variables.insert(variable_name.to_string(), variable_value);
        } else {
            let result = calculate(&input, Some(&variables));
            match result {
                Ok(result) => {
                    writeln!(&mut stdout, "{}", result).ok();
                }
                Err(err) => {
                    writeln!(&mut stdout, "{}", err).ok();
                }
            }
        }
    }
}
