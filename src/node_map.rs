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

    println!("{}: {}", node_type, node_text.bright_green());

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_node(child, source_code, node_types);
    }
}

pub fn print_node_type_mapping(language: Language) {
    let node_type_count = language.node_kind_count();
    for id in 0..node_type_count {
        let node_kind = language.node_kind_for_id(id as u16).unwrap();
        let is_named = language.node_kind_is_named(id as u16);
        let is_visible = language.node_kind_is_visible(id as u16);
        let is_hidden = node_kind.starts_with("_");
        println!("{}: kind={}, named={}, hidden={}, visible={}", id, node_kind.bright_green(), is_named, is_hidden, is_visible);
    }
}
