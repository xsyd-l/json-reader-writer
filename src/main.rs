mod json;

use json::Json;

fn main() {
    let s = std::fs::read_to_string("D:\\MyProject\\json\\test.json").unwrap();
    let json = Json::from(&s);
    print!("{}", json.to_string());
}
