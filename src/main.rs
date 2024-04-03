use std::env;
use md5;

fn main() -> Result<(), String> {
    let chars: Vec<u8> = (48..=57)  // from 0 to 9
        .chain(65..=90)             // from A to Z
        .chain(97..=122)            // from a to z
        .chain(vec![b'@', b'$'])    // additional chars
        .collect();

    let args: Vec<String> = env::args().skip(1).collect();

    let (target, secret, prefix) = if args.is_empty() {
        let target = ask_user("Enter target name (for what app/service)")?;
        let secret = ask_user("Enter secret (easy to remember)")?;
        let prefix = ask_user("Enter prefix (or leave blank)")?;
        print!("Result: ");
        (target, secret, prefix)
    } else {
        let target = parse_arg(&args, 0);
        let secret = parse_arg(&args, 1);
        let prefix = parse_arg(&args, 2);
        (target, secret, prefix)
    };

    if target.is_empty() || secret.is_empty() {
        return Err("Target and secret required".to_string());
    }

    let result: String = md5::compute(format!("{secret}{target}"))
        .iter()
        .map(|&x| get_char(&chars, x as usize))
        .collect();
    
    println!("{prefix}{}", result);
    return Ok(());
}

fn ask_user(question: &str) -> Result<String, String> {
    println!("{}: ", question);
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(_) => Ok(buf.trim_end().to_owned()),
        Err(v) => Err(v.to_string()),
    }
}

fn get_char(dict: &Vec<u8>, idx: usize) -> char {
    let idx = idx % dict.len();
    return dict[idx] as char;
}

fn parse_arg(args: &Vec<String>, idx: usize) -> String {
    return args.get(idx).cloned().unwrap_or_default();
}
