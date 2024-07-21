use std::env;
use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Language, Node};

extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_python() -> Language;
}

fn guess_language(extension: &str) -> Option<Language> {
    match extension {
        "rs" => Some(unsafe { tree_sitter_rust() }),
        "py" => Some(unsafe { tree_sitter_python() }),
        _ => None,
    }
}

fn print_tree(node: Node, source_code: &str, indent: usize) {
    let indent_str = " ".repeat(indent);
    let formatted_node = format!(
        "{} [grammar={}, named={}, extra={}] {} - {}",
        node.kind().replace('\n', "\\n"),
        node.grammar_name().replace('\n', "\\n"),
        node.is_named().to_string(),
        node.is_extra().to_string(),
        node.start_position(),
        node.end_position()
    );

    println!("{}{}", indent_str, formatted_node);

    for child in node.children(&mut node.walk()) {
        print_tree(child, source_code, indent + 2);
    }
}

fn main() {
    pretty_env_logger::try_init_timed_custom_env("DFT_LOG")
        .expect("The logger has not been previously initialized");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file-path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let path = Path::new(file_path);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let language = match guess_language(extension) {
        Some(lang) => lang,
        None => {
            eprintln!("Unsupported file extension: {}", extension);
            std::process::exit(1);
        }
    };

    let source_code = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    let mut parser = Parser::new();
    parser.set_language(&language).expect("Failed to set language");

    let tree = parser.parse(&source_code, None)
        .expect("Failed to parse the source code");

    let root_node = tree.root_node();
    print_tree(root_node, &source_code, 0);
}
