fn main() {
    let input_string = "1234:5678:90ab:cdef:ghij:klmn:opqr:stuv"; // Example input string

    match parse_string(input_string) {
        Ok(parsed_strings) => {
            for (i, s) in parsed_strings.iter().enumerate() {
                println!("String {}: {}", i + 1, s);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn parse_string(input: &str) -> Result<Vec<String>, &'static str> {
    let parts: Vec<&str> = input.split(':').collect();

    if parts.len() != 8 {
        return Err("Input string does not have 8 parts separated by ':'.");
    }

    let mut parsed_strings = Vec::new();
    for part in parts {
        parsed_strings.push(part.to_string());
    }

    Ok(parsed_strings)
}