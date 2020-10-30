use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Exactly 2 arguments are expected");
    }
    let list_type = args[1].as_str();
    let filename = &args[2];
    match list_type {
        "ul" => generate_ul(filename),
        "ol" => generate_ol(filename),
        "dl" => generate_dl(filename),
        _ => panic!("Invalid list type"),
    }
}

fn generate_ul(filename: &String) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut output = String::from("<ul>\n");
    for line in lines.iter() {
        output.push_str(format!("    <li>{}</li>\n",line).as_str());
    }
    output.push_str("</ul>");
    fs::write(format!("{}.html",filename), output).expect("Write failed");
}

fn generate_ol(filename: &String) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut output = String::from("<ol>\n");
    for line in lines.iter() {
        output.push_str(format!("    <li>{}</li>\n",line).as_str());
    }
    output.push_str("</ol>");
    fs::write(format!("{}.html",filename), output).expect("Write failed");
}

fn generate_dl(filename: &String) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut output = String::from("<dl>\n");
    for entry in lines.chunks_exact(2) {
        output.push_str(format!("    <dt>{}</dt>\n    <dd>{}</dd>\n",entry[0],entry[1]).as_str());
    }
    output.push_str("</dl>");
    fs::write(format!("{}.html",filename), output).expect("Write failed");
}
