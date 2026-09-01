//! 🎨️ Third-party SVG fixture generator for `s.stdio.semio@v1/✳️drawing`.
//!
//! Every fixture is a `(before.svg, after.svg)` PAIR for exactly one mutation kind. The pair is the
//! whole expectation: `quick-xml-drawing-svg-reader` parses BOTH halves and the difference it reports
//! is what the mutation is required to produce. Nothing here applies one of our mutations — the
//! `after` scene is authored directly, which is what keeps this a reader oracle rather than a
//! predicting one.
//!
//! @see ../../🧪️oracle/🔣️.json — the manifest whose 17 kinds this corpus binds to, one fixture each.

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone)]
enum Node {
    Path { id: &'static str, d: String, fill: String, stroke: String, width: String, transform: Option<String> },
    Text { id: &'static str, x: String, y: String, body: String },
    Group { id: &'static str, transform: String, children: Vec<Node> },
}

#[derive(Clone)]
struct Layer { id: String, nodes: Vec<Node> }

#[derive(Clone)]
struct Scene { layers: Vec<Layer> }

fn path(id: &'static str, d: &str, fill: &str, stroke: &str, width: &str) -> Node {
    Node::Path { id, d: d.into(), fill: fill.into(), stroke: stroke.into(), width: width.into(), transform: None }
}

fn base() -> Scene {
    Scene {
        layers: vec![Layer {
            id: "layer-0".into(),
            nodes: vec![
                Node::Group {
                    id: "group-0",
                    transform: "translate(0 0)".into(),
                    children: vec![
                        path("path-a", "M 0 0 L 10 0 L 10 10 Z", "#ff0000", "#000000", "1"),
                        path("path-b", "M 20 0 L 30 0 L 30 10 Z", "#00ff00", "#000000", "1"),
                    ],
                },
                Node::Text { id: "text-0", x: "0".into(), y: "40".into(), body: "semio".into() },
            ],
        }],
    }
}

fn group0<'a>(scene: &'a mut Scene) -> &'a mut Node { &mut scene.layers[0].nodes[0] }

fn children<'a>(scene: &'a mut Scene) -> &'a mut Vec<Node> {
    match group0(scene) { Node::Group { children, .. } => children, _ => unreachable!() }
}

fn set_group_transform(scene: &mut Scene, value: &str) {
    if let Node::Group { transform, .. } = group0(scene) { *transform = value.into(); }
}

/// 🧪️ `(kind, before, after)` — one authored pair per manifested mutation kind.
fn corpus() -> Vec<(&'static str, Scene, Scene)> {
    let mut out: Vec<(&'static str, Scene, Scene)> = Vec::new();

    // ── layer vocabulary ────────────────────────────────────────────────────────────────────────
    let mut after = base();
    after.layers.push(Layer { id: "layer-1".into(), nodes: Vec::new() });
    out.push(("create-layer", base(), after));

    let mut before = base();
    before.layers.push(Layer { id: "layer-1".into(), nodes: Vec::new() });
    out.push(("delete-layer", before, base()));

    // ── node vocabulary ─────────────────────────────────────────────────────────────────────────
    let mut after = base();
    children(&mut after).push(path("path-c", "M 40 0 L 50 0 L 50 10 Z", "#0000ff", "#000000", "1"));
    out.push(("create-node", base(), after));

    let mut before = base();
    children(&mut before).push(path("path-c", "M 40 0 L 50 0 L 50 10 Z", "#0000ff", "#000000", "1"));
    out.push(("delete-node", before, base()));

    let mut after = base();
    if let Node::Path { transform, .. } = &mut children(&mut after)[0] { *transform = Some("translate(5 7)".into()); }
    out.push(("move-node", base(), after));

    let mut after = base();
    for node in children(&mut after).iter_mut() {
        if let Node::Path { transform, .. } = node { *transform = Some("translate(3 3)".into()); }
    }
    out.push(("drag-nodes", base(), after));

    let mut after = base();
    children(&mut after).swap(0, 1);
    out.push(("reorder-nodes", base(), after));

    // ── spatial transforms — SVG's own `transform` attribute is the witness ─────────────────────
    let mut after = base();
    set_group_transform(&mut after, "rotate(90)");
    out.push(("rotate-node", base(), after));

    let mut after = base();
    set_group_transform(&mut after, "scale(2 2)");
    out.push(("scale-node", base(), after));

    // ── structural nesting — SVG's own `<g>` nesting is the witness ─────────────────────────────
    let grouped = |inner: Vec<Node>| Node::Group { id: "group-1", transform: "translate(0 0)".into(), children: inner };
    let mut after = base();
    {
        let kids = children(&mut after);
        let moved: Vec<Node> = kids.drain(..).collect();
        kids.push(grouped(moved));
    }
    out.push(("group-nodes", base(), after.clone()));
    out.push(("ungroup-node", after.clone(), base()));

    // flatten: the nested group's transform is folded away and its children are hoisted.
    let mut nested = base();
    {
        let kids = children(&mut nested);
        let moved: Vec<Node> = kids.drain(..).collect();
        kids.push(Node::Group { id: "group-1", transform: "translate(4 4)".into(), children: moved });
    }
    let mut flattened = base();
    for node in children(&mut flattened).iter_mut() {
        if let Node::Path { transform, .. } = node { *transform = Some("translate(4 4)".into()); }
    }
    out.push(("flatten-node", nested.clone(), flattened.clone()));
    out.push(("unflatten-node", flattened, nested));

    // ── style + geometry vocabulary ─────────────────────────────────────────────────────────────
    let mut after = base();
    if let Node::Path { d, .. } = &mut children(&mut after)[0] { *d = "M 0 0 L 12 0 L 12 12 Z".into(); }
    out.push(("replace-path", base(), after));

    let mut after = base();
    if let Node::Path { fill, .. } = &mut children(&mut after)[0] { *fill = "#123456".into(); }
    out.push(("replace-fill", base(), after));

    let mut after = base();
    if let Node::Path { stroke, .. } = &mut children(&mut after)[0] { *stroke = "#abcdef".into(); }
    out.push(("change-stroke-color", base(), after));

    let mut after = base();
    if let Node::Path { width, .. } = &mut children(&mut after)[0] { *width = "4".into(); }
    out.push(("change-stroke-width", base(), after));

    out
}

fn write_node(writer: &mut Writer<Cursor<Vec<u8>>>, node: &Node) -> Result<(), quick_xml::Error> {
    match node {
        Node::Path { id, d, fill, stroke, width, transform } => {
            let mut element = BytesStart::new("path");
            element.push_attribute(("id", *id));
            element.push_attribute(("d", d.as_str()));
            element.push_attribute(("fill", fill.as_str()));
            element.push_attribute(("stroke", stroke.as_str()));
            element.push_attribute(("stroke-width", width.as_str()));
            if let Some(value) = transform { element.push_attribute(("transform", value.as_str())); }
            writer.write_event(Event::Empty(element))?;
        }
        Node::Text { id, x, y, body } => {
            let mut element = BytesStart::new("text");
            element.push_attribute(("id", *id));
            element.push_attribute(("x", x.as_str()));
            element.push_attribute(("y", y.as_str()));
            writer.write_event(Event::Start(element))?;
            writer.write_event(Event::Text(BytesText::new(body)))?;
            writer.write_event(Event::End(BytesEnd::new("text")))?;
        }
        Node::Group { id, transform, children } => {
            let mut element = BytesStart::new("g");
            element.push_attribute(("id", *id));
            element.push_attribute(("transform", transform.as_str()));
            writer.write_event(Event::Start(element))?;
            for child in children { write_node(writer, child)?; }
            writer.write_event(Event::End(BytesEnd::new("g")))?;
        }
    }
    Ok(())
}

fn render(scene: &Scene) -> Result<Vec<u8>, quick_xml::Error> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    let mut svg = BytesStart::new("svg");
    svg.push_attribute(("xmlns", "http://www.w3.org/2000/svg"));
    svg.push_attribute(("version", "1.1"));
    svg.push_attribute(("viewBox", "0 0 100 100"));
    writer.write_event(Event::Start(svg))?;
    for layer in &scene.layers {
        let mut element = BytesStart::new("g");
        element.push_attribute(("id", layer.id.as_str()));
        element.push_attribute(("class", "semio-layer"));
        writer.write_event(Event::Start(element))?;
        for node in &layer.nodes { write_node(&mut writer, node)?; }
        writer.write_event(Event::End(BytesEnd::new("g")))?;
    }
    writer.write_event(Event::End(BytesEnd::new("svg")))?;
    let mut bytes = writer.into_inner().into_inner();
    bytes.push(b'\n');
    Ok(bytes)
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| { eprintln!("usage: generate <out-dir>"); std::process::exit(2) });
    let mut written = 0usize;
    for (kind, before, after) in corpus() {
        let dir = std::path::Path::new(&out).join(kind);
        std::fs::create_dir_all(&dir).expect("fixture directory");
        for (name, scene) in [("before.svg", &before), ("after.svg", &after)] {
            std::fs::write(dir.join(name), render(scene).expect("render")).expect("write");
            written += 1;
        }
        println!("{kind}");
    }
    eprintln!("{written} file(s)");
}
