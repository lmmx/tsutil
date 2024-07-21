use tree_sitter::{Language, Node};
use colored::Colorize;

pub fn get_node_types(language: Language) -> Vec<String> {
    let mut node_types = Vec::new();
    let node_type_count = language.node_kind_count();

    for id in 0..node_type_count {
        let node_type = language.node_kind_for_id(id as u16).unwrap();
        node_types.push(node_type.to_string());
    }

    node_types
}

pub fn print_node(node: Node, source_code: &str, node_types: &[String]) {
    let node_type_id = node.kind_id() as usize;
    let node_type = &node_types[node_type_id];
    let node_text = &source_code[node.byte_range()];

    let colored_text = match node_type.as_str() {
        "function_item" => node_text.bright_blue(),
        "identifier" => node_text.bright_green(),
        "number_literal" => node_text.bright_yellow(),
        "let" => node_text.bright_magenta(),
        _ => node_text.normal(),
    };

    println!("{}: {}", node_type, colored_text);

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_node(child, source_code, node_types);
    }
}

pub fn print_node_type_mapping(language: Language) {
    let node_types = get_node_types(language);
    for (id, node_type) in node_types.iter().enumerate() {
        println!("{}: {}", id, node_type);
    }
}
