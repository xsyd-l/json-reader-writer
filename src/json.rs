use std::io;
use std::collections::HashMap;

enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    Float(f32),
    Int(i32),
    String(String),
    Null,
}

impl Json {
    fn is_object(s: &str) -> bool {
        s.starts_with('{')
    }

    fn is_array(s: &str) -> bool {
        s.starts_with('[')
    }

    fn is_float(s: &str) -> bool {
        s.parse::<f32>().is_ok() && s.contains('.')
    }

    fn is_int(s: &str) -> bool {
        s.parse::<i32>().is_ok()
    }

    fn is_string(s: &str) -> bool {
        s.starts_with('\"')
    }

    fn is_null(s: &str) -> bool {
        s == "null"
    }
    
    fn from(s: &str) -> Json {
        if Self::is_object(s) {

        } else if Self::is_array(s) {

        } else if Self::is_float(s) {
            return Json::Float(s.parse::<f32>().unwrap());
        } else if Self::is_int(s) {
            return Json::Int(s.parse::<i32>().unwrap());
        } else if Self::is_string(s) {
            if s.len() == 2 {
                return Json::String(String::from(""));
            }
            if let Some(inner) = s.strip_prefix('\"').and_then(|s| s.strip_suffix('\"')) {
                return Json::String(String::from(inner));
            }
        } else if Self::is_null(s) {
            return Json::Null;
        }
        Json::Null
    }

    fn json_array_split(s: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if let Some(inner) = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            let mut stack = Vec::<char>::new();
            let mut string = String::new();
            let mut is_escaped = false;
            for c in inner.chars() {
                if is_escaped {
                    string.push(c);
                    is_escaped = false;
                    continue;
                }
               
                match c {
                    '\\' => {
                        string.push(c);
                        is_escaped = true;
                    },
                    '[' | '{' | '\"' => {
                        if let Some(top) = stack.last() {
                            if top != &'\"' {
                                stack.push(c);
                            } else if c == '\"' {
                                stack.pop();
                            }
                            string.push(c);
                        } else {
                            stack.push(c);
                            string.push(c);
                        }
                    },
                    ']' => {
                        if let Some(top) = stack.last() {
                            if top == &'[' {
                                stack.pop();
                            }
                            string.push(c);
                        }
                    },
                    '}' => {
                        if let Some(top) = stack.last() {
                            if top == &'{' {
                                stack.pop();
                            }
                            string.push(c);
                        }
                    },
                    ',' => {
                        if stack.is_empty() {
                            result.push(std::mem::take(&mut string));
                        } else {
                            string.push(',');
                        }
                    }
                    _ => {
                        string.push(c);
                    },
                }
            }
            if !string.is_empty() {
                string.trim();
                result.push(std::mem::take(&mut string));
            }
        }
        result
    }

    fn json_object_split(s: &str) -> Vec<&str> {
        if let Some(inner) = s.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {

        }
        Vec::new()
    }
}