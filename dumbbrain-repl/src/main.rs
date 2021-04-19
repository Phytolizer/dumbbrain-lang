use std::process;

use dialoguer::console::style;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use dumbbrain_parser::ast::traits::SyntaxNode;
use dumbbrain_parser::Parser;
use ptree::TreeBuilder;
use slog::crit;
use slog::o;
use slog::Drain;
use slog::Logger;
use slog_term::FullFormat;
use slog_term::TermDecorator;

fn main() {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain)
        .thread_name("logging thread".into())
        .build()
        .fuse();
    let log = Logger::root(drain, o!());
    let theme = ColorfulTheme {
        prompt_prefix: style("> ".into()),
        ..Default::default()
    };
    let mut input = Input::<String>::with_theme(&theme);
    input.allow_empty(true);
    loop {
        let input = input.interact_on(&Term::stdout()).unwrap_or_else(|e| {
            crit!(log, "Could not get user input: {}", e);
            process::exit(1);
        });

        let mut parser = Parser::new(input.as_str());
        let expression = parser.parse();
        let mut tree_builder = TreeBuilder::new("Expression".into());
        pretty_print(&mut tree_builder, &expression);
        let tree = tree_builder.build();
        ptree::print_tree_with(
            &tree,
            &ptree::PrintConfig {
                indent: 4,
                characters: ptree::print_config::ASCII_CHARS_PLUS.into(),
                ..Default::default()
            },
        )
        .unwrap();
    }
}

fn format_node(node: &dyn SyntaxNode) -> String {
    let mut s = format!("{:?}", node.kind());
    if let Some(value) = node.value() {
        s += &format!(" {}", value);
    }
    s
}

fn pretty_print(tree: &mut TreeBuilder, node: &dyn SyntaxNode) {
    let children = node.children();

    if children.is_empty() {
        tree.add_empty_child(format_node(node));
    } else {
        tree.begin_child(format_node(node));
        for child in children {
            pretty_print(tree, child);
        }
        tree.end_child();
    }
}
