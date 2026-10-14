//! 🖨️ Original canonical Record text emission and retained physical writer.
pub use semio_framework_dsl_record_model::*;
use semio_framework_dsl::format_f64;
use semio_framework_dsl_notation as dsl_notation;
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind};
#[path="../🛫️encoding/🦀️.rs"]
mod controlled_encoding;
pub use controlled_encoding::{print_controlled,print_expr_controlled,RetainedRecordWriter};
#[path="../🫳️borrowed/🦀️.rs"]
mod borrowed_schema;
pub use borrowed_schema::measure_print_borrowed;

/// 🎨️ Canonical `Shape::Expr` printer. Parenthesizes the minimum necessary to guarantee
/// `parse_expr(print_expr(e)) == e` for EVERY tree shape (not just canonically-left-nested ones):
/// a `Binary` right operand is parenthesized whenever its own precedence isn't STRICTLY higher
/// than the parent's (so even a commutative `a+(b+c)` keeps its parens — losing them would
/// reparse as the structurally different `(a+b)+c`), and a left operand only when strictly lower
/// (left-associativity already makes equal precedence safe there).
// 🚫️async: E1 pure AST pretty-printer — the `Call` arm's `Iterator::map(...).join(...)` recurses
// through `print_expr_prec` inside a sync closure, and both are inlined directly into `format!`
// args elsewhere in this fn, which requires `Display`, not `Future` — see R9
pub fn print_expr(expr: &ExprValue) -> String {
    print_expr_prec(expr, 0)
}

fn print_expr_prec(expr: &ExprValue, min_prec: u8) -> String {
    let (body, own_prec) = match expr {
        ExprValue::Num(v) => (format_f64(*v), 255),
        ExprValue::Var(name) => (expression_name(name), 255),
        ExprValue::Call(name, args) => {
            let joined = args.iter().map(|a| print_expr_prec(a, 0)).collect::<Vec<_>>().join(", ");
            (format!("{}({joined})",expression_name(name)), 255)
        }
        // min_prec=4 is higher than every binary op (max 2) and Neg's own rank (3), so a nested
        // Binary OR another Neg always gets parenthesized — the latter specifically avoids ever
        // printing adjacent `--`, which would relex as `DashArrow`, not two `Minus` tokens.
        ExprValue::Neg(inner) => (format!("-{}", print_expr_prec(inner, 4)), 3),
        ExprValue::Binary(op, l, r) => {
            let prec = op.precedence();
            let l_text = print_expr_prec(l, prec);
            let r_text = print_expr_prec(r, prec + 1);
            (format!("{l_text} {} {r_text}", op.symbol()), prec)
        }
    };
    if own_prec < min_prec {
        format!("({body})")
    } else {
        body
    }
}

fn expression_name(name:&str)->String{if semio_framework_dsl::is_bare_ident(name){name.to_string()}else{format!("\"{}\"",semio_framework_dsl::escape_text(name))}}

/// 📛️ `Shape::Text`'s own body, factored out so `Shape::Ref` (identical grammar, distinct
/// type only) can share it without a redundant match arm duplicating both branches.

fn print_table_list(spec_fn: RecordSpecProducer, items: &[FieldValue], writer: &mut Writer) {
    writer.atom("[");
    for item in items {
        let FieldValue::Record(record) = item else { continue };
        writer.atom("{");
        writer.glue();
        print_record(record, &(spec_fn.ordinary)(), writer);
        writer.glue();
        writer.atom("}");
    }
    writer.atom("]");
}

fn base64_encode(bytes:&[u8])->String{semio_framework_value::bytes::encode_base64(bytes)}

//#region 🔖️Writer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinMode {
    Document,
    Inline,
}

/// ✍️ A chunk tree that renders in either join mode — the structural half of the newline
/// law. `atom` asserts its argument contains no raw `\n` (Document mode still separates atoms with
/// synthesized whitespace, never embeds one inside an atom), so `render(Inline)` joining every
/// chunk with a single space can never produce an embedded newline.
///
/// 📏️ Canonical spacing rules (both join modes, structurally guaranteed — never hand-tuned
/// per callsite): never a space adjacent to `=` (`key=[ a b ]`, not `key= [ a b ]` — the printer
/// achieves this by pushing a bare `key=` atom, calling [`Writer::glue`], then printing the
/// value); exactly one space between sibling atoms; exactly one space just inside `[ ]`/`{ }` when
/// rendered inline (`[ a b ]`, not `[a b]`) — EXCEPT a `Table` header's `[ ]`, which is glued
/// tight on both sides (`[id:TEXT x:NUM]`) since it's a fixed one-shot header, not a
/// space-joined element list; a space appears before a keyword-led block's `{` (`children {
/// ... }`) but never before a glued composite's `{` (`data={ ... }`).
#[derive(Default)]
pub struct Writer {
    chunks: Vec<Chunk>,
    indent: usize,
}

enum Chunk {
    Atom(String),
    OpenBlock,
    CloseBlock,
    NewRecord,
    /// 🧲️ One-shot marker: the very next `Atom`/`OpenBlock` chunk renders with NO
    /// preceding separator (space in Inline mode, space-or-newline-continuation in Document mode)
    /// — consumed by that one chunk, then normal spacing resumes. See [`Writer::glue`].
    Glue,
    /// 📜️ `Shape::Embed`'s payload — the one chunk kind whose Document and Inline renders
    /// genuinely differ in FORM (fenced block vs. escaped quoted string), not just spacing.
    Verbatim {
        lang: String,
        content: String,
    },
}

impl Writer {
    pub fn new() -> Self {
        Self { chunks: Vec::new(), indent: 0 }
    }

    pub fn atom(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
        debug_assert!(!s.contains('\n'), "Writer::atom must not contain a raw newline: {s:?}");
        self.chunks.push(Chunk::Atom(s.to_string()));
    }

    pub fn key_value(&mut self, key: &str, value: impl AsRef<str>) {
        self.atom(format!("{key}={}", value.as_ref()));
    }

    pub fn open_block(&mut self) {
        self.chunks.push(Chunk::OpenBlock);
        self.indent += 1;
    }

    pub fn close_block(&mut self) {
        self.indent = self.indent.saturating_sub(1);
        self.chunks.push(Chunk::CloseBlock);
    }

    pub fn new_record(&mut self) {
        self.chunks.push(Chunk::NewRecord);
    }

    /// 🧲️ Fuses the next pushed chunk onto whatever precedes it, with no separator, in
    /// BOTH join modes — the mechanism behind every `key=value`/`key=[...]`/`key={...}` fusion in
    /// this printer. Replaces the old approach of mutating an already-pushed atom's string in
    /// place (which only worked for single-atom scalar values): `glue()` composes with arbitrarily
    /// structured values (nested blocks, lists, whole sub-records) since it's a rendering-time
    /// join, not a string-splice.
    pub fn glue(&mut self) {
        self.chunks.push(Chunk::Glue);
    }

    /// 📜️ Pushes a `Shape::Embed` payload — content MAY contain raw newlines (unlike
    /// [`Self::atom`], which forbids them), since Document mode renders it as a fence.
    pub fn verbatim(&mut self, lang: &str, content: &str) {
        self.chunks.push(Chunk::Verbatim { lang: lang.to_string(), content: content.to_string() });
    }

    pub fn render(&self, mode: JoinMode) -> String {
        match mode {
            JoinMode::Inline => {
                let mut parts: Vec<String> = Vec::new();
                let mut glued = false;
                let mut push = |piece: String, glued: &mut bool| {
                    if *glued {
                        if let Some(last) = parts.last_mut() {
                            last.push_str(&piece);
                        } else {
                            parts.push(piece);
                        }
                    } else {
                        parts.push(piece);
                    }
                    *glued = false;
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => push(s.clone(), &mut glued),
                        Chunk::OpenBlock => push("{".to_string(), &mut glued),
                        Chunk::CloseBlock => push("}".to_string(), &mut glued),
                        Chunk::NewRecord => {}
                        Chunk::Verbatim { content, .. } => push(format!("\"{}\"", semio_framework_dsl::escape_text(content)), &mut glued),
                    }
                }
                parts.join(" ")
            }
            JoinMode::Document => {
                let mut out = String::new();
                let mut indent = 0usize;
                let mut line_open = false;
                let mut glued = false;
                let push_indent = |out: &mut String, indent: usize| {
                    for _ in 0..indent {
                        out.push_str("  ");
                    }
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => {
                            if !line_open {
                                push_indent(&mut out, indent);
                                line_open = true;
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str(s);
                            glued = false;
                        }
                        Chunk::OpenBlock => {
                            if glued {
                                out.push('{');
                            } else {
                                out.push_str(" {");
                            }
                            out.push('\n');
                            line_open = false;
                            indent += 1;
                            glued = false;
                        }
                        Chunk::CloseBlock => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                            indent = indent.saturating_sub(1);
                            push_indent(&mut out, indent);
                            out.push('}');
                            out.push('\n');
                        }
                        Chunk::NewRecord => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                        }
                        Chunk::Verbatim { lang, content } => {
                            if !line_open {
                                push_indent(&mut out, indent);
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str("```");
                            out.push_str(lang);
                            out.push('\n');
                            out.push_str(content);
                            if !content.is_empty() {
                                out.push('\n');
                            }
                            out.push_str("```");
                            line_open = true;
                            glued = false;
                        }
                    }
                }
                if line_open {
                    out.push('\n');
                }
                out
            }
        }
    }
}

/// 🥇️ Field print order within one record — NOT declaration order: keyword, then
/// positionals (unchanged), then keyed fields grouped scalar-before-composite-before-table-
/// before-statements, ties broken by original declaration order (a stable sort over an
/// already-declaration-order slice achieves this for free). Metadata/scalars land before large
/// nested/tabular blocks, which is friendlier to lazy loading/streaming readers — parsing stays
/// completely order-independent, so this is a print-only change.
// 🚫️async: E1 pure, consumed by `Iterator::sort_by_key`'s sync closure (its `u8` result must be
// `Ord`, which `impl Future<Output = u8>` is not) — see R9
fn keyed_field_rank(shape: &Shape) -> u8 {
    match shape {
        Shape::Bool
        | Shape::Int
        | Shape::UInt
        | Shape::Float
        | Shape::Text
        | Shape::Bytes64
        | Shape::Enum(_)
        | Shape::Tuple(_, _)
        | Shape::Quantity(_)
        | Shape::Angle(_)
        | Shape::Ref(_)
        | Shape::Coord(_)
        | Shape::Dir
        | Shape::Dim(_)
        | Shape::Range
        | Shape::Count
        | Shape::Expr => 0,
        Shape::List(_) | Shape::Map(_) | Shape::Record(_) | Shape::Block(_) | Shape::Value | Shape::Wire => 1,
        Shape::Table(_) => 2,
        Shape::Statements(_) => 3,
        // Ranks LAST of all: a multi-line fence dwarfs everything else in a record, so it should
        // print after every scalar/composite/table field, not interleaved among them.
        Shape::Embed(_) | Shape::EmbedFrom(_) => 4,
    }
}

pub fn print_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    if spec.layout == RecordLayout::Call {
        print_call_record(value, spec, writer);
        return;
    }
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    print_record_fields(value, spec, writer);
}

/// 📛️ Prints a `RecordLayout::Call` record: `<name> = <keyword>(args)`. The argument list
/// is built by [`print_record_fields`] — the exact same field-printing logic every other layout
/// uses — rendered to its own `JoinMode::Inline` string and glued onto the keyword inside parens,
/// so a positional/keyed field prints identically here as it would under `Inline` layout.
fn print_call_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    let Some(name_field) = spec.fields.iter().find(|f| f.is_call_name) else {
        debug_assert!(false, "RecordLayout::Call requires exactly one field marked call_name()");
        return;
    };
    let name_text = match value.get(name_field.id) {
        Some(fv @ FieldValue::Text(_)) => scalar_to_text(fv),
        _ => String::new(),
    };
    writer.atom(name_text);
    writer.atom("=");
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    let mut args_writer = Writer::new();
    print_record_fields(value, spec, &mut args_writer);
    let args_text = args_writer.render(JoinMode::Inline);
    writer.glue();
    writer.atom(format!("({args_text})"));
}

/// 🖨️ Prints a record's fields: positional bare in declaration order, then order-
/// independent `key=value` attributes. Excludes any field marked `call_name()` — see
/// [`parse_record_fields`]'s matching doc comment for why.
fn print_record_fields(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    let mut positional: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some() && !f.is_call_name).collect();
    positional.sort_by_key(|f| f.position.unwrap());
    for (index, field) in positional.iter().enumerate() {
        match value.get(field.id) {
            Some(fv) if !matches!(fv, FieldValue::Absent) => print_shape(fv, &field.shape, writer),
            _ => {
                // An absent OPTIONAL positional prints as `_` only if some LATER positional in
                // this same record is actually present — that's what keeps slots aligned for the
                // reader (and reparse). A run of trailing absents needs no placeholder at all.
                let later_present = positional[index + 1..].iter().any(|f| matches!(value.get(f.id), Some(fv) if !matches!(fv, FieldValue::Absent)));
                if later_present {
                    writer.atom("_");
                }
            }
        }
    }

    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !f.is_call_name).collect();
    keyed.sort_by_key(|f| keyed_field_rank(&f.shape));
    for field in keyed {
        match value.get(field.id) {
            Some(FieldValue::Absent) | None => continue,
            Some(fv) => match &field.shape {
                Shape::EmbedFrom(lang_key) => {
                    writer.new_record();
                    writer.atom(format!("{}=", field.key));
                    writer.glue();
                    let lang = spec
                        .fields
                        .iter()
                        .find(|f| f.key == *lang_key)
                        .and_then(|f| value.get(f.id))
                        .and_then(|v| match v {
                            FieldValue::Text(t) => Some(t.as_str()),
                            _ => None,
                        })
                        .unwrap_or("plaintext");
                    if let FieldValue::Text(content) = fv {
                        writer.verbatim(lang, content);
                    }
                }
                // `Statements` items each carry their own leading keyword — no field-level key at
                // all is ever printed for this shape.
                Shape::Statements(_) => print_shape(fv, &field.shape, writer),
                // `Block`'s own key is a bare leading keyword, not a `key=value` attribute
                // (`children { ... }`, never `children={...}`).
                Shape::Block(_) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    print_shape(fv, &field.shape, writer);
                }
                // `Table`'s own key is likewise a bare leading keyword, but — unlike `Block` —
                // it must always go through the dedicated SoA writer (`print_table`), never the
                // generic `print_shape` dispatch: that dispatch renders `Table` as the bracketed
                // AoS list (see its `Shape::Table` arm below) so a `Table` value reached any OTHER
                // way (nested inside a table row, a list, ...) stays self-delimiting. Only here,
                // directly after a record's own leading keyword, is the bare `[col:TYPE ...]
                // {rows}` form reachable on the parse side (`parse_record_body`'s dedicated
                // bare-SoA lookahead) — printing it via `print_shape` here would silently regress
                // to the AoS form for every top-level table field.
                Shape::Table(spec_fn) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    if let FieldValue::List(items) = fv {
                        print_table(*spec_fn, items, writer);
                    }
                }
                _ => {
                    writer.atom(format!("{}=", field.key));
                    print_key_value(field, fv, writer);
                }
            },
        }
    }
}

/// 🧲️ `key=` was just pushed by the caller — glue the value onto it with no separator,
/// then print it normally (composed, not string-spliced, so this handles arbitrarily structured
/// values exactly like a bare `print_shape` call would).
fn print_key_value(field: &FieldSpec, value: &FieldValue, writer: &mut Writer) {
    writer.glue();
    match (&field.shape, value) {
        (Shape::Enum(variants), FieldValue::Enum(ordinal)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        _ => print_shape(value, &field.shape, writer),
    }
}

fn scalar_to_text(value: &FieldValue) -> String {
    match value {
        FieldValue::Bool(b) => b.to_string(),
        FieldValue::Int(i) => i.to_string(),
        FieldValue::UInt(u) => u.to_string(),
        FieldValue::Float(f) => format_f64(*f),
        // Bare (unquoted) whenever the text lexes back as exactly this one ident — the printer's
        // half of the "strings bare-preferred" law; `is_bare_ident` also excludes reserved literal
        // idents (`_`/`true`/`false`/`null`/`nan`/`inf`) and number-shaped text, which always fall
        // through to the quoted+escaped form instead.
        FieldValue::Text(s) => {
            if semio_framework_dsl::is_bare_ident(s) {
                s.clone()
            } else {
                format!("\"{}\"", semio_framework_dsl::escape_text(s))
            }
        }
        FieldValue::Bytes64(bytes) => format!("\"{}\"", base64_encode(bytes)),
        FieldValue::Enum(_) => String::new(), // resolved by caller via variants table when needed
        _ => String::new(),
    }
}

/// 🔢️ Renders one `FieldValue::Tuple` element as bare text for the `Coord`/`Dir`/`Dim`/
/// `Range` printers above — every element of those tuples is always `FieldValue::Float` by
/// construction (their parsers only ever push `FieldValue::Float`), so this panics rather than
/// falling back on a malformed value, matching the rest of this module's "trust the parser built
/// this" convention for shapes whose `FieldValue` invariant is enforced entirely at parse time.
// 🚫️async: E1 pure, passed as a bare fn item into `Iterator::map` sync closures at every call site — see R9
fn number_tuple_component(value: &FieldValue) -> String {
    match value {
        FieldValue::Float(v) => format_f64(*v),
        other => panic!("Coord/Dir/Dim/Range tuple element must be Float, found {other:?}"),
    }
}

pub fn print_shape(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    match (value, shape) {
        // Must precede the generic scalar arm below: that arm's shape pattern is `_` and would
        // otherwise swallow every `FieldValue::Float` regardless of shape, printing a bare number
        // with no unit suffix even for a `Quantity`/`Angle` field.
        (FieldValue::Float(v), Shape::Quantity(unit) | Shape::Angle(unit)) => {
            writer.atom(format!("{}{}", format_f64(*v), unit.symbol));
        }
        (FieldValue::UInt(v), Shape::Count) => {
            writer.atom(format!("x{v}"));
        }
        (FieldValue::Tuple(items), Shape::Coord(_)) => {
            writer.atom(format!("@{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dir) => {
            writer.atom(format!("^{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dim(_)) => {
            writer.atom(items.iter().map(number_tuple_component).collect::<Vec<_>>().join("x"));
        }
        (FieldValue::Tuple(items), Shape::Range) => {
            let parts: Vec<String> = items.iter().map(number_tuple_component).collect();
            let body = match parts.as_slice() {
                [lo, hi] => format!("{lo}..{hi}"),
                [lo, hi, step] => format!("{lo}..{hi},{step}"),
                _ => parts.join(","),
            };
            writer.atom(format!("({body})"));
        }
        (FieldValue::Expr(expr), Shape::Expr) => {
            writer.atom(format!("({})", print_expr(expr)));
        }
        (FieldValue::Text(content), Shape::Embed(lang)) => {
            writer.verbatim(lang, content);
        }
        (FieldValue::Text(content), Shape::EmbedFrom(lang_key)) => {
            // Fallback when print_shape is called without sibling resolution — prefer plaintext fence.
            let _ = lang_key;
            writer.verbatim("plaintext", content);
        }
        (FieldValue::Bool(_) | FieldValue::Int(_) | FieldValue::UInt(_) | FieldValue::Float(_) | FieldValue::Text(_) | FieldValue::Bytes64(_), _) => {
            writer.atom(scalar_to_text(value));
        }
        (FieldValue::Enum(ordinal), Shape::Enum(variants)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        (FieldValue::Tuple(items), Shape::Tuple(elem, _)) => {
            let mut rendered: Vec<String> = Vec::with_capacity(items.len());
            for item in items {
                let mut sub = Writer::new();
                print_shape(item, elem, &mut sub);
                rendered.push(sub.render(JoinMode::Inline));
            }
            writer.atom(rendered.join(","));
        }
        // A `Table` reached here (NOT via `print_record`'s own keyed-field dispatch, which calls
        // `print_table` directly) is nested inside another shape — a table row's own column, a
        // list element, ... — where the bare `key [col:TYPE ...] {rows}` form has no bracket of
        // its own to mark where it ends. Render the braced-row AoS list instead (see
        // `print_table_list`), matching what `parse_shape`'s own `Shape::Table` arm parses in
        // every one of these same contexts.
        (FieldValue::List(items), Shape::Table(spec_fn)) => print_table_list(*spec_fn, items, writer),
        (FieldValue::List(items), Shape::List(elem)) => {
            writer.atom("[");
            for item in items {
                if matches!(elem.as_ref(), Shape::Record(_)) {
                    writer.atom("{");
                    print_shape(item, elem, writer);
                    writer.atom("}");
                } else {
                    print_shape(item, elem, writer);
                }
            }
            writer.atom("]");
        }
        (FieldValue::Record(record), Shape::Record(spec_fn)) => {
            print_record(record, &(spec_fn.ordinary)(), writer);
        }
        (FieldValue::Block(inner_value), Shape::Block(inner_shape)) => {
            writer.open_block();
            print_shape(inner_value, inner_shape, writer);
            writer.close_block();
        }
        (FieldValue::Statements(items), Shape::Statements(variants)) => {
            for (keyword, record) in items {
                writer.new_record();
                if let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == keyword) {
                    print_record(record, &(spec_fn.ordinary)(), writer);
                }
            }
        }
        (FieldValue::Map(entries), Shape::Map(inner)) => {
            writer.open_block();
            let mut sorted = entries.iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            for (key, value) in &sorted {
                writer.atom(dynamic_key_text(key));
                writer.glue();
                if matches!(inner.as_ref(), Shape::Record(_)) {
                    writer.atom("{");
                    print_shape(value, inner, writer);
                    writer.atom("}");
                } else { print_shape(value, inner, writer); }
            }
            writer.close_block();
        }
        (FieldValue::Value(dsl_value), Shape::Value) => print_dsl_value(dsl_value, writer),
        (FieldValue::Wire(wire), Shape::Wire) => print_wire(wire, writer),
        _ => {}
    }
}

/// 📊️ Always prints the compact SoA form — this (not the parser, which still accepts the
/// verbose AoS form too) is what makes `canonicalize` migrate old AoS documents to SoA
/// automatically. Header `[ ]` is glued tight on both sides (`[id:TEXT x:NUM]`); rows have no
/// separator, one row per line in Document mode purely for readability (`new_record` is a no-op
/// in Inline mode).
fn print_table(spec_fn: RecordSpecProducer, items: &[FieldValue], writer: &mut Writer) {
    let element_spec = (spec_fn.ordinary)();
    writer.atom("[");
    writer.glue();
    for field in &element_spec.fields {
        writer.atom(format!("{}:{}", field.key, shape_type_name(&field.shape)));
    }
    writer.glue();
    writer.atom("]");
    writer.open_block();
    for item in items {
        writer.new_record();
        let FieldValue::Record(record) = item else { continue };
        for field in &element_spec.fields {
            match record.get(field.id) {
                Some(fv) if !matches!(fv, FieldValue::Absent) => print_table_cell(fv, &field.shape, writer),
                _ => writer.atom("_"),
            }
        }
    }
    writer.close_block();
}

/// 🧱️ Prints one table cell's value. See `parse_table_cell` for why a bare `Shape::Record`
/// column is brace-wrapped here — `{ }` glued tight on both sides, the same technique the header's
/// own `[ ]` uses, so bracing never disturbs the "no space just inside" canonical spacing rule for
/// a one-shot wrapper — and every other shape is left to the ordinary `print_shape`, already
/// self-delimiting.
fn print_table_cell(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    if let (FieldValue::Record(record), Shape::Record(spec_fn)) = (value, shape) {
        writer.atom("{");
        writer.glue();
        print_record(record, &(spec_fn.ordinary)(), writer);
        writer.glue();
        writer.atom("}");
        return;
    }
    print_shape(value, shape, writer);
}

fn dynamic_key_text(key: &str) -> String {
    if semio_framework_dsl::is_bare_ident(key) { format!("{key}=") } else { format!("\"{}\"=", semio_framework_dsl::escape_text(key)) }
}

fn print_dsl_value(value: &DslValue, writer: &mut Writer) {
    match value {
        DslValue::Null => writer.atom("null"),
        DslValue::Bool(b) => writer.atom(b.to_string()),
        DslValue::Number(Number::UInt(n)) => writer.atom(n.to_string()),
        DslValue::Number(Number::Int(n)) if *n >= 0 => writer.atom(format!("int64({n})")),
        DslValue::Number(Number::Int(n)) => writer.atom(n.to_string()),
        DslValue::Number(Number::Float(n)) => {
            let mut value = format_f64(*n);
            if n.is_finite() && !value.contains(['.', 'e', 'E']) {
                value.push_str(".0");
            }
            writer.atom(value);
        }
        DslValue::String(s) => writer.atom(format!("\"{}\"", semio_framework_dsl::escape_text(s))),
        DslValue::Bytes(bytes)=>writer.atom(format!("bytes64(\"{}\")",base64_encode(bytes))),
        DslValue::Array(items) => {
            writer.atom("[");
            for item in items {
                print_dsl_value(item, writer);
            }
            writer.atom("]");
        }
        DslValue::Object(entries) => {
            writer.open_block();
            for (key, value) in entries {
                writer.atom(dynamic_key_text(key));
                writer.glue();
                print_dsl_value(value, writer);
            }
            writer.close_block();
        }
    }
}

fn print_wire(wire: &WireValue, writer: &mut Writer) {
    let map_node = |node: &WireNode| dsl_notation::EdgeNode { id: node.id.clone(), kind: node.kind.clone(), port: node.port.clone() };
    let edge = dsl_notation::EdgeValue {
        from: map_node(&wire.from),
        link: wire.edge.as_ref().map(|(directed, to)| dsl_notation::EdgeLink { directed: *directed, label: dsl_notation::EdgeLabel { id: wire.edge_label.id.clone(), kind: wire.edge_label.kind.clone() }, to: map_node(to) }),
    };
    writer.atom(dsl_notation::print_edge(&edge));
    if !matches!(&wire.properties, DslValue::Object(entries) if entries.is_empty()) {
        print_dsl_value(&wire.properties, writer);
    }
}

/// 🔁️ Prints `value` against `spec` in the given join mode — the top-level entry point
/// `dsl_derive`-generated code calls from `ArtifactDsl::print_dsl`/`OpText::print_op`.
pub fn print(value: &RecordValue, spec: &RecordSpec, mode: JoinMode) -> String {
    let mut writer = Writer::new();
    print_record(value, spec, &mut writer);
    writer.render(mode)
}
//#endregion 🔖️Writer


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
