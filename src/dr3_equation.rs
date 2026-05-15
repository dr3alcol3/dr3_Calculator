// Realy basic
pub mod equation {
    #[derive(Debug, Clone, Copy)]
    enum Dr3Token {
        Number(f64),
        Symbol(Dr3Symbol),
        Eoe,
    }

    #[derive(Debug, Clone, Copy)]
    enum Dr3Symbol {
        OpenBracket,
        CloseBracket,
        Power,
        Root,
        Division,
        Multiplication,
        Addition,
        Subtraction,
    }

    #[derive(Debug, Clone)]
    enum Dr3Expression {
        Atom(f64),
        Operation {
            operation: Dr3Symbol,
            lhs: Box<Dr3Expression>,
            rhs: Box<Dr3Expression>,
        },
    }

    fn tokenize_equation(expression: &str) -> Vec<Dr3Token> {
        let mut result: Vec<Dr3Token> = Vec::new();
        let mut number_buffer: String = String::from("");

        for element in expression.chars() {
            if element.is_numeric() || element == '.' {
                number_buffer += &element.to_string();
            } else {
                if number_buffer.len() > 0 {
                    result.push(Dr3Token::Number(
                        number_buffer.parse::<f64>().unwrap_or(0.0),
                    ));

                    number_buffer.clear();
                }

                let dr3_symbol = char_to_symbol(&element);

                match dr3_symbol {
                    Some(s) => {
                        result.push(Dr3Token::Symbol(s));
                    }
                    None => {}
                }
            }
        }

        if number_buffer.len() > 0 {
            result.push(Dr3Token::Number(
                number_buffer.parse::<f64>().unwrap_or(0.0),
            ));
        }

        result.reverse();
        result
    }

    fn parse_equation_to_tree(
        token_vec: &mut Vec<Dr3Token>,
        min_bp: f32,
    ) -> Result<Dr3Expression, String> {
        let mut lhs = match next(token_vec) {
            Dr3Token::Number(n) => Dr3Expression::Atom(n),
            Dr3Token::Symbol(Dr3Symbol::OpenBracket) => {
                let lhs = parse_equation_to_tree(token_vec, 0.0)?;
                match next(token_vec) {
                    Dr3Token::Symbol(Dr3Symbol::CloseBracket) => lhs,
                    _ => return Err(String::from("Close all bracket blocks")),
                }
            }
            Dr3Token::Symbol(Dr3Symbol::Root) => {
                let op = Dr3Symbol::Root;
                let (_, r_bp) = infix_binding_power(&op);
                let rhs = parse_equation_to_tree(token_vec, r_bp)?;
                Dr3Expression::Operation {
                    operation: op,
                    lhs: Box::new(Dr3Expression::Atom(2.0)),
                    rhs: Box::new(rhs),
                }
            }
            Dr3Token::Eoe => Dr3Expression::Atom(0.0),
            _ => {
                return Err(String::from(
                    "Start with number or open bracket not a symbol",
                ));
            }
        };

        loop {
            let op = match peek(token_vec) {
                Dr3Token::Eoe => break,
                Dr3Token::Symbol(Dr3Symbol::OpenBracket) => {
                    return Err(String::from("Don't open brackets after a number"));
                }
                Dr3Token::Symbol(Dr3Symbol::CloseBracket) => break,
                Dr3Token::Symbol(Dr3Symbol::Root) => {
                    return Err(String::from(
                        "There needs to be an operation(+/*-^) before the root",
                    ));
                }
                Dr3Token::Symbol(s) => s,
                Dr3Token::Number(_n) => {
                    return Err(String::from(
                        "Change occurrences like (num)(num) to (num)+(num) or (num)-(num)...",
                    ));
                }
            };
            let (l_bp, r_bp) = infix_binding_power(&op);
            if l_bp < min_bp {
                break;
            }
            next(token_vec);
            let rhs = parse_equation_to_tree(token_vec, r_bp)?;
            lhs = Dr3Expression::Operation {
                operation: op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn resolve_equation_tree(node: &Dr3Expression) -> f64 {
        match node {
            Dr3Expression::Atom(n) => *n,
            Dr3Expression::Operation {
                operation,
                lhs,
                rhs,
            } => {
                let l = resolve_equation_tree(lhs);
                let r = resolve_equation_tree(rhs);
                match operation {
                    Dr3Symbol::Power => l.powf(r),
                    Dr3Symbol::Root => r.powf(1.0 / l),
                    Dr3Symbol::Division => l / r,
                    Dr3Symbol::Multiplication => l * r,
                    Dr3Symbol::Addition => l + r,
                    Dr3Symbol::Subtraction => l - r,
                    _ => panic!("NOT AN OPERATION"),
                }
            }
        }
    }

    pub fn evaluate(expression: &str) -> Result<f64, String> {
        let mut token_vec: Vec<Dr3Token> = tokenize_equation(expression);
        let parsed_equation_tree = parse_equation_to_tree(&mut token_vec, 0.0)?;
        Ok(resolve_equation_tree(&parsed_equation_tree))
    }

    pub fn resolve_evaluation_result_to_string(result: Result<f64, String>) -> String {
        match result {
            Ok(r) => r.to_string(),
            Err(e) => e,
        }
    }

    fn next(token_vec: &mut Vec<Dr3Token>) -> Dr3Token {
        token_vec.pop().unwrap_or(Dr3Token::Eoe)
    }

    fn peek(token_vec: &Vec<Dr3Token>) -> Dr3Token {
        token_vec.last().copied().unwrap_or(Dr3Token::Eoe)
    }

    fn char_to_symbol(c: &char) -> Option<Dr3Symbol> {
        match c {
            '(' => Some(Dr3Symbol::OpenBracket),
            ')' => Some(Dr3Symbol::CloseBracket),
            '^' => Some(Dr3Symbol::Power),
            '√' => Some(Dr3Symbol::Root),
            '/' => Some(Dr3Symbol::Division),
            '*' => Some(Dr3Symbol::Multiplication),
            '+' => Some(Dr3Symbol::Addition),
            '-' => Some(Dr3Symbol::Subtraction),
            _ => None,
        }
    }

    fn infix_binding_power(op: &Dr3Symbol) -> (f32, f32) {
        match op {
            Dr3Symbol::Addition | Dr3Symbol::Subtraction => (1.0, 1.1),
            Dr3Symbol::Division | Dr3Symbol::Multiplication => (2.0, 2.1),
            Dr3Symbol::Power | Dr3Symbol::Root => (3.1, 3.0),
            _ => panic!("NOT AN OPERATION"),
        }
    }
}
