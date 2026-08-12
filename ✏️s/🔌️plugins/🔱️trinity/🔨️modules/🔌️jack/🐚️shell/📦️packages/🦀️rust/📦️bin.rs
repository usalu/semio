//! 🐚️ Jack query shell for trinity graphs.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use store::ArtifactDsl;
use trinity::artifacts::jack::{Graph, JackSnapshot, PropertyValue};
use trinity::executor::run; use trinity::ast::QueryResult;

//#region ⚠️ Errors
/// ⚠️ Trinity jack shell errors.
#[derive(Debug, thiserror::Error)]
enum TrinityJackShellError {
    #[error("read {path}: {source}")]
    ReadFixture { path: String, source: io::Error },
    #[error("parse {path}: {source}")]
    Dsl { path: String, source: store::TextError },
    #[error(transparent)]
    Graph(#[from] trinity::artifacts::jack::TrinityRamError),
    #[error("{0}")]
    Query(String),
    #[error(transparent)]
    Io(#[from] io::Error),
}
//#endregion ⚠️ Errors

fn main() {
    if let Err(err) = run_main() {
        eprintln!("semio_s_plugin_trinity_jack_shell: {err}");
        std::process::exit(1);
    }
}

fn run_main() -> Result<(), TrinityJackShellError> {
    let args: Vec<String> = env::args().collect();
    let fixture_path = args.get(1).map_or("trinity/example/🔱️nakagin-capsule-tower.trinity", String::as_str);
    let text = fs::read_to_string(fixture_path).map_err(|source| TrinityJackShellError::ReadFixture { path: fixture_path.to_string(), source })?;
    let fixture = JackSnapshot::parse_dsl(&text).map_err(|source| TrinityJackShellError::Dsl { path: fixture_path.to_string(), source })?;
    let mut graph = Graph::from_fixture(fixture)?;
    println!("[DEBUG] trinity jack shell loaded {} nodes, {} edges from {fixture_path}", graph.nodes.len(), graph.edges.len());
    if args.len() > 2 {
        let query = args[2..].join(" ");
        print_result(&run(&mut graph, &query).map_err(TrinityJackShellError::Query)?);
        return Ok(());
    }
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    writeln!(stdout, "jack> trinity graph loaded ({})", graph.name)?;
    for line in stdin.lock().lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }
        match run(&mut graph, trimmed) {
            Ok(result) => print_result(&result),
            Err(err) => writeln!(stdout, "error: {err}")?,
        }
    }
    Ok(())
}

fn print_result(result: &QueryResult) {
    if result.columns.is_empty() {
        println!("ok");
        return;
    }
    println!("{}", result.columns.join("\t"));
    for row in &result.rows {
        let cells: Vec<String> = row.iter().map(format_cell).collect();
        println!("{}", cells.join("\t"));
    }
}

fn format_cell(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Null => "null".into(),
        PropertyValue::Bool(b) => b.to_string(),
        PropertyValue::Number(n) => n.to_string(),
        PropertyValue::String(s) => s.clone(),
        PropertyValue::Array(a) => format!("[{}]", a.len()),
        PropertyValue::Object(o) => {
            let pairs: Vec<String> = o.iter().map(|(k, v)| format!("{k}={}", format_cell(v))).collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trinity::artifacts::jack::{Camera, JackSnapshot, Manifest, Node, Port, PortDirection, PropertyBag};

    fn mini_json() -> String {
        let fixture = JackSnapshot {
            schema: JackSnapshot::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
            root_node_id: Some("root".into()),
            nodes: vec![Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            }],
            edges: vec![],
        };
        fixture.to_json().unwrap()
    }

    #[test]
    fn shell_loads_fixture() {
        let mut graph = Graph::load_json(&mini_json()).unwrap();
        let result = run(&mut graph, "MATCH (a:Piece) RETURN a.name").unwrap();
        assert_eq!(result.rows.len(), 1);
    }
}
