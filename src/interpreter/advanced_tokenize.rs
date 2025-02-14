use super::super::util::error;
use super::tokens::{AdvancedTokens, SimpleTokens};

pub fn advance_tokens(tokens: Vec<SimpleTokens>) -> Vec<AdvancedTokens> {
    let mut token_index = 0;
    let mut advanced_tokens = vec![];
    while token_index < tokens.len() {
        match &tokens[token_index] {
            SimpleTokens::Operator(operator) => {
                let chars = operator.chars().collect::<Vec<char>>();
                let mut char_index = 0;
                while char_index < chars.len() {
                    match chars[char_index] {
                        '>' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::GreaterThanOrEqual);
                                    char_index += 1;
                                }
                            } else {
                                advanced_tokens.push(AdvancedTokens::GreaterThan);
                            }
                        }
                        '<' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::LessThanOrEqual);
                                    char_index += 1;
                                }
                            } else {
                                advanced_tokens.push(AdvancedTokens::LessThan);
                            }
                        }
                        '=' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::IsEqual);
                                    char_index += 1;
                                }
                            } else {
                                advanced_tokens.push(AdvancedTokens::SetEqualTo);
                            }
                        }
                        '!' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::IsNotEqual);
                                    char_index += 1;
                                }
                            } else {
                                todo!();
                            }
                        }
                        ':' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == ':' {
                                    advanced_tokens.push(AdvancedTokens::MemberOf);
                                    char_index += 1;
                                } else {
                                    todo!();
                                }
                            }
                        }
                        '+' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::AdditionEquals);
                                    char_index += 1;
                                } else {
                                    advanced_tokens.push(AdvancedTokens::Addition);
                                }
                            }
                        }
                        '-' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::SubtractionEquals);
                                    char_index += 1;
                                } else if chars[char_index + 1] == '>' {
                                    advanced_tokens.push(AdvancedTokens::Returns);
                                } else {
                                    advanced_tokens.push(AdvancedTokens::Subtraction);
                                }
                            }
                        }
                        '*' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::MultiplicationEquals);
                                    char_index += 1;
                                } else {
                                    advanced_tokens.push(AdvancedTokens::Multiplication);
                                }
                            }
                        }
                        '/' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::DivisionEquals);
                                    char_index += 1;
                                } else {
                                    advanced_tokens.push(AdvancedTokens::Division);
                                }
                            }
                        }
                        '%' => {
                            if chars.len() - char_index > 1 {
                                if chars[char_index + 1] == '=' {
                                    advanced_tokens.push(AdvancedTokens::ModuloEquals);
                                    char_index += 1;
                                } else {
                                    advanced_tokens.push(AdvancedTokens::Modulo);
                                }
                            }
                        }
                        ';' => {
                            advanced_tokens.push(AdvancedTokens::SemiColon);
                        }
                        ',' => {
                            advanced_tokens.push(AdvancedTokens::Comma);
                        }
                        '(' => {
                            advanced_tokens.push(AdvancedTokens::OpeningParentheses);
                        }
                        ')' => {
                            advanced_tokens.push(AdvancedTokens::ClosingParentheses);
                        }
                        '{' => {
                            advanced_tokens.push(AdvancedTokens::OpeningBrace);
                        }
                        '}' => {
                            advanced_tokens.push(AdvancedTokens::ClosingBrace);
                        }
                        '[' => {
                            advanced_tokens.push(AdvancedTokens::OpeningParentheses);
                        }
                        ']' => {
                            advanced_tokens.push(AdvancedTokens::ClosingParentheses);
                        }
                        '"' => {
                            advanced_tokens.push(AdvancedTokens::StringOpenClose);
                        }
                        invalid_character => {
                            error(format!("Invalid character: \"{invalid_character}\""));
                        }
                    }
                    char_index += 1;
                }
            }
            SimpleTokens::Number(number) => {
                let mut number = number.to_string();
                number.retain(|c| c != '_');
                let point_count = number.chars().filter(|&c| c == '.').count();
                if point_count > 1 {
                    error("Invalid integer!".to_string());
                } else if point_count == 0 {
                    advanced_tokens.push(AdvancedTokens::Integer(number));
                } else if point_count == 1 {
                    advanced_tokens.push(AdvancedTokens::Float(number));
                }
            }
            SimpleTokens::Identifier(identifier) => match identifier.as_str() {
                "if" => {
                    advanced_tokens.push(AdvancedTokens::If);
                }
                "while" => {
                    advanced_tokens.push(AdvancedTokens::While);
                }
                "else" => {
                    advanced_tokens.push(AdvancedTokens::Else);
                }
                "for" => {
                    advanced_tokens.push(AdvancedTokens::For);
                }
                "in" => {
                    advanced_tokens.push(AdvancedTokens::In);
                }
                "pub" => {
                    advanced_tokens.push(AdvancedTokens::Pub);
                }
                "struct" => {
                    advanced_tokens.push(AdvancedTokens::Struct);
                }
                "enum" => {
                    advanced_tokens.push(AdvancedTokens::Enum);
                }
                "use" => {
                    advanced_tokens.push(AdvancedTokens::Use);
                }
                "const" => {
                    advanced_tokens.push(AdvancedTokens::Const);
                }
                "let" => {
                    advanced_tokens.push(AdvancedTokens::Let);
                }
                "fn" => {
                    advanced_tokens.push(AdvancedTokens::Fn);
                }
                other_identifier => {
                    advanced_tokens.push(AdvancedTokens::Identifier(other_identifier.to_string()));
                }
            },
        }
        token_index += 1;
    }

    advanced_tokens
}
