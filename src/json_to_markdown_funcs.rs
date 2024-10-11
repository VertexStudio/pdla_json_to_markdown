use serde::{Deserialize, Serialize};

// enum Types {
//     Tittle,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "type")]
    type_: String,
    text: String,
}

pub fn parse_types(item: &Item) -> String {
    match item.type_.as_str() {
        "Title" => format!("# {}", item.text),
        "Section header" => format!("## {}", item.text),
        "Table" => "".to_string(),
        _ => format!("{}  ", item.text),
    }
}
