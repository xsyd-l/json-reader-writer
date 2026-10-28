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

    fn json_array_split(s: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let s = s.trim();
        if let Some(inner) = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            return Self::json_split_by(s, ',');
        } else {
            return Vec::new();
        }
    }

    fn json_object_split(s: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let s = s.trim();
        if let Some(inner) = s.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
            return Self::json_split_by(s, ',');
        } else {
            return Vec::new();
        }
    }

    fn pair_split(s: &str) -> (&str, &str) {
        let pair = Self::json_split_by(s, ':');
        return (pair[0], pair[1]);
    }

    // !!! cannot asign c as [\]\{\}\"
    fn json_split_by(s: &str, c: char) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let inner = s.trim();
        let mut stack = Vec::<u8>::new();
        let mut index = 0;
        let mut prev = 0;
        let mut is_escaped = false;
        for b in inner.bytes() {
            if is_escaped {
                is_escaped = false;
                index += 1;
                continue;
            }
           
            match b {
                b'\\' => {
                    is_escaped = true;
                },
                b'[' | b'{' | b'\"' => {
                    if let Some(top) = stack.last() {
                        if top != &b'\"' {
                            stack.push(b);
                        } else if b == b'\"' {
                            stack.pop();
                        }
                    } else {
                        stack.push(b);
                    }
                },
                b']' => {
                    if let Some(top) = stack.last() {
                        if top == &b'[' {
                            stack.pop();
                        }
                    }
                },
                b'}' => {
                    if let Some(top) = stack.last() {
                        if top == &b'{' {
                            stack.pop();
                        }
                    }
                },
                _ => {
                    if b == c as u8 && stack.is_empty(){
                        result.push(&inner[prev..index].trim());
                        prev = index + 1;
                    }
                },
            }
            index += 1;
        }
        if !&inner[prev..index].trim().is_empty() {
            result.push(&inner[prev..index].trim());
        }
        result
    }
}