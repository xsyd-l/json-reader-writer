use std::collections::HashMap;
use std::fmt::Write;

pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
    Null,
}

impl Json {
    pub fn from(s: &str) -> Json {
        if Self::is_object(s) {
            let parts = Self::json_object_split(s);
            let mut map = HashMap::<String, Json>::new();
            for part in parts {
                let (left, right) = Self::pair_split(part);
                if let Some(left_inner) = left.strip_prefix('\"').and_then(|s| s.strip_suffix('\"'))
                {
                    map.insert(String::from(left_inner), Self::from(right));
                }
            }
            return Json::Object(map);
        } else if Self::is_array(s) {
            let parts = Self::json_array_split(s);
            let mut vec = Vec::<Json>::new();
            for part in parts {
                vec.push(Self::from(part));
            }
            return Json::Array(vec);
        } else if Self::is_float(s) {
            return Json::Float(s.parse::<f32>().unwrap());
        } else if Self::is_int(s) {
            return Json::Int(s.parse::<i32>().unwrap());
        } else if Self::is_string(s) {
            if s.len() == 2 {
                return Json::String(String::from(""));
            }
            if let Some(inner) = s.strip_prefix('\"').and_then(|s| s.strip_suffix('\"')) {
                let mut restr = String::new();
                let mut is_escaped = false;
                for c in inner.chars() {
                    if is_escaped {
                        is_escaped = false;
                    }
                    if c == '\\' {
                        is_escaped = true;
                        continue;
                    }
                    restr.push(c);
                }
                return Json::String(restr);
            }
        } else if Self::is_bool(s) {
            return Json::Bool(s == "true");
        } else if Self::is_null(s) {
            return Json::Null;
        }
        Json::Null
    }

    pub fn to_string(&self) -> String {
        Self::js_to_string(self, 0)
    }

    pub fn read(path: &str) -> Json {
        if let Ok(content) = std::fs::read_to_string(path) {
            Json::from(&content)
        } else {
            Json::Null
        }
    }

    pub fn write(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, self.to_string())
    }

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

    fn is_bool(s: &str) -> bool {
        s == "true" || s == "false"
    }

    fn is_null(s: &str) -> bool {
        s == "null"
    }

    fn js_to_string(js: &Json, deep: u8) -> String {
        match js {
            Json::Object(map) => {
                let mut result = String::from("{\n");
                let mut number = 0;
                for (k, v) in map.iter() {
                    let mut key: String = k.to_string().escape_default().collect();
                    key.push('\"');
                    key.insert(0, '\"');
                    writeln!(
                        result,
                        "{}{}:{},",
                        &(" ".repeat((deep * 4 + 4) as usize)),
                        key,
                        &Self::js_to_string(v, deep + 1)
                    )
                    .unwrap();
                    number += 1;
                }
                if number != 0 {
                    result.pop();
                    result.pop();
                    write!(result, "\n{}}}", &(" ".repeat((deep * 4) as usize))).unwrap();
                    result
                } else {
                    String::from("{}")
                }
            }
            Json::Array(array) => {
                let mut number = 0;
                let mut oora = 0;
                for v in array.iter() {
                    match v {
                        Json::Array(_) | Json::Object(_) => {
                            oora += 1;
                        }
                        _ => {}
                    }
                }
                let mut result = if oora == 0 {
                    String::from("[")
                } else {
                    String::from("[\n")
                };
                let end = if oora == 0 { "," } else { ",\n" };
                let space = if oora == 0 {
                    String::new()
                } else {
                    " ".repeat((deep * 4 + 4) as usize) 
                };
                for v in array.iter() {
                    write!(
                        result,
                        "{}{}{}",
                        &space,
                        &Self::js_to_string(v, deep + 1),
                        end
                    )
                    .unwrap();
                    number += 1;
                }
                if number == 0 {
                    return String::from("[]");
                }
                if oora != 0 {
                    result.pop();
                    result.pop();
                    write!(result, "\n{}]", &(" ".repeat((deep * 4) as usize))).unwrap();
                    result
                } else {
                    result.pop();
                    result + "]"
                }
            }
            Json::Float(value) => {
                value.to_string()
            }
            Json::Int(value) => {
                value.to_string()
            }
            Json::String(value) => {
                let mut result: String = value.to_string().escape_default().collect();
                result.push('\"');
                result.insert(0, '\"');
                result
            }
            Json::Bool(value) => {
                value.to_string()
            }
            Json::Null => {
                String::from("null")
            }
        }
    }

    fn json_array_split(s: &str) -> Vec<&str> {
        let _result: Vec<&str> = Vec::new();
        let s = s.trim();
        if let Some(inner) = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            Self::json_split_by(inner, ',')
        } else {
            Vec::new()
        }
    }

    fn json_object_split(s: &str) -> Vec<&str> {
        let _result: Vec<&str> = Vec::new();
        let s = s.trim();
        if let Some(inner) = s.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
            Self::json_split_by(inner, ',')
        } else {
            Vec::new()
        }
    }

    fn pair_split(s: &str) -> (&str, &str) {
        let pair = Self::json_split_by(s, ':');
        (pair[0], pair[1])
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
                }
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
                }
                b']' => {
                    if let Some(top) = stack.last()
                        && top == &b'[' {
                            stack.pop();
                        }
                }
                b'}' => {
                    if let Some(top) = stack.last()
                        && top == &b'{' {
                            stack.pop();
                        }
                }
                _ => {
                    if b == c as u8 && stack.is_empty() {
                        result.push(inner[prev..index].trim());
                        prev = index + 1;
                    }
                }
            }
            index += 1;
        }
        if !&inner[prev..index].trim().is_empty() {
            result.push(inner[prev..index].trim());
        }
        result
    }
}
