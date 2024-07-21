use tree_sitter::Node;
use colored::*;

pub fn print_tree(node: Node, source_code: &str, indent: usize) {
    let indent_str = " ".repeat(indent);
    let formatted_node = format!(
        "{} [grammar_name='{}', grammar_id='{}', named='{}', extra='{}'] {} - {}",
        node.kind().replace('\n', "\\n").yellow(),
        node.grammar_name().replace('\n', "\\n").blue(),
        node.grammar_id().to_string().green(),
        node.is_named().to_string().cyan(),
        node.is_extra().to_string().magenta(),
        node.start_position().to_string().red(),
        node.end_position().to_string().red()
    );

    println!("{}{}", indent_str, formatted_node);

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_tree(child, source_code, indent + 2);
    }
}
