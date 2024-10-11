mod pdla_json_to_markdown_funcs;

use std::fmt::Error;

use pdla_json_to_markdown_funcs::*;

pub fn convert_pdla_json_to_markdown(value: &str) -> Result<String, Error> {
    let items: Vec<Item> = serde_json::from_str(value).expect("Error parsing JSON");

    let mut markdown = String::new();

    for item in items {
        markdown.push_str(&parse_types(&item));
        markdown.push_str("\n");
    }

    Ok(markdown)
}
