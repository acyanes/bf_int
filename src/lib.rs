use pyo3::prelude::*;

#[pyfunction]
fn evaluate(bf: String) -> PyResult<String> {
    let new_bf = Lexer::new(bf);
    println!("{:?}", new_bf.tokens);
    let mut arr: [u8; 30] = [0; 30]; // u8 0-255
    let mut pointer = 0;

    for token in new_bf.tokens {
        match token {
            TokenType::Plus => {
                // if pointer is 255, set to 0
                if arr[pointer] == 255 {
                    arr[pointer] = 0
                } else {
                    arr[pointer] += 1
                }
            }
            TokenType::Minus => {
                // if pointer is 0, minus goes to 255
                if arr[pointer] == 0 {
                    arr[pointer] = 255
                } else {
                    arr[pointer] -= 1
                }
            }
            TokenType::GreaterThan => pointer += 1,
            TokenType::LessThan => pointer -= 1,
            TokenType::OpenBracket => {
                // loop until block does not equal 0
            }
            TokenType::CloseBracket => {}
            //TokenType::Comma => arr[pointer],
            //TokenType::Period => arr[pointer]++,
            _ => {}
        }
    }
    let mut result = String::new();
    for n in arr {
        result.push(n as char);
    }
    println!("array {:?}", arr);

    Ok(result)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn bf_transpiler(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    Ok(())
}

pub struct Lexer {
    bf_string: String,
    tokens: Vec<TokenType>,
}

impl Lexer {
    pub fn new(bf_string: String) -> Lexer {
        let tokens = Self::tokenize(bf_string.clone()).unwrap();
        Lexer { bf_string, tokens }
    }

    fn tokenize(bf_string: String) -> Result<Vec<TokenType>, String> {
        if bf_string.chars().count() == 0 {
            return Err(String::from("No Tokens"));
        }

        let mut tokens: Vec<TokenType> = Vec::new();
        for ch in bf_string.chars() {
            match ch {
                '>' => tokens.push(TokenType::GreaterThan),
                '<' => tokens.push(TokenType::LessThan),
                '-' => tokens.push(TokenType::Minus),
                '[' => tokens.push(TokenType::OpenBracket),
                ']' => tokens.push(TokenType::CloseBracket),
                ',' => tokens.push(TokenType::Comma),
                '.' => tokens.push(TokenType::Period),
                '+' => tokens.push(TokenType::Plus),
                _ => {} // skip over other characters
            }
        }
        Ok(tokens)
    }
}

#[derive(Debug)]
enum TokenType {
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    OpenBracket,
    CloseBracket,
    Comma,
    Period,
}
