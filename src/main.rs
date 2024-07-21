use std::env;
use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Language};

extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_python() -> Language;
}

mod node_printer;
mod node_map;

fn guess_language(extension: &str) -> Option<Language> {
    match extension {
        "rs" => Some(unsafe { tree_sitter_rust() }),
        "py" => Some(unsafe { tree_sitter_python() }),
        _ => None,
    }
}

fn main() {
    pretty_env_logger::try_init_timed_custom_env("DFT_LOG")
        .expect("The logger has not been previously initialized");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file-path> [--highlight|--list-ids]", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let highlight = args.len() == 3 && args[2] == "--highlight";
    let list_ids = args.len() == 3 && args[2] == "--list-ids";

    let path = Path::new(file_path);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let language = match guess_language(extension) {
        Some(lang) => lang,
        None => {
            eprintln!("Unsupported file extension: {}", extension);
            std::process::exit(1);
        }
    };

    if list_ids {
        node_map::print_node_type_mapping(language);
        return;
    }

    let source_code = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    let mut parser = Parser::new();
    parser.set_language(&language).expect("Failed to set language");

    let tree = parser.parse(&source_code, None)
        .expect("Failed to parse the source code");

    let root_node = tree.root_node();

    if highlight {
        let node_types = node_map::get_node_types(language);
        node_map::print_node(root_node, &source_code, &node_types);
    } else {
        node_printer::print_tree(root_node, &source_code, 0);
    }
}
