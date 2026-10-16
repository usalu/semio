//! 🖨️ Canonical printers for grammar and protocol files, and the idempotent `canonicalize`.

use crate::{CommentDialect, StringEscape, TextError};
use super::{Alternative, Block, Cond, CondOp, Count, DispatchOrder, Field, Framing, GrammarFile, MacroArg, Prim, ProtocolFile, RepeatArm, SemioDialect, Symbol, is_protocol_source, parse_grammar, parse_protocol, prim_fixed_width};

//#region 🔖️Writer
fn print_symbol(symbol: &Symbol, out: &mut String) {
    match symbol {
        Symbol::Literal(text) => {
            out.push('"');
            out.push_str(text);
            out.push('"');
        }
        Symbol::Terminal(name) | Symbol::Ref(name) => out.push_str(name),
        Symbol::Macro(name, args) => {
            out.push_str(name);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                match arg {
                    MacroArg::Literal(text) => {
                        out.push('"');
                        out.push_str(text);
                        out.push('"');
                    }
                    MacroArg::Ident(name) => out.push_str(name),
                }
            }
            out.push(')');
        }
        Symbol::Group(alts) => {
            out.push('{');
            // 🔁️ Box::pin: print_symbol <-> print_alternatives is a mutual-recursion cycle, and
            // an fn's own opaque Future type cannot embed a cycle-partner's opaque type at
            // an unboxed, unbounded size (R10 residue shape 3).
            print_alternatives(alts, out);
            out.push('}');
        }
        Symbol::Optional(inner) => {
            print_symbol(inner, out);
            out.push('?');
        }
        Symbol::Star(inner) => {
            print_symbol(inner, out);
            out.push('*');
        }
        Symbol::Plus(inner) => {
            print_symbol(inner, out);
            out.push('+');
        }
    }
}

fn print_alternatives(alts: &[Alternative], out: &mut String) {
    for (i, alt) in alts.iter().enumerate() {
        if i > 0 {
            out.push_str(" | ");
        }
        for (j, symbol) in alt.symbols.iter().enumerate() {
            if j > 0 {
                out.push(' ');
            }
            print_symbol(symbol, out);
        }
    }
}

/// 🖨️ Canonical printer — `parse_grammar(print_grammar(g)) == g` is this crate's own
/// round-trip law, checked by the `self_hosting` test below over this crate's own grammar file.
pub fn print_grammar(grammar: &GrammarFile) -> String {
    let mut out = String::new();
    out.push_str("dialect ");
    out.push_str(match grammar.dialect {
        SemioDialect::Grammar => "grammar",
        SemioDialect::Protocol => "protocol",
    });
    out.push('\n');
    out.push_str(match grammar.dialect {
        SemioDialect::Grammar => "grammar ",
        SemioDialect::Protocol => "protocol ",
    });
    out.push_str(&grammar.id);
    out.push('\n');
    if let Some(extension) = &grammar.extension {
        out.push_str("extension ");
        out.push_str(extension);
        out.push('\n');
    }
    // P2-M1 items 1 & 4: only emitted when they differ from `LexOptions::default()`, so a grammar
    // that never declared `comment`/`string` directives prints byte-identically to before P2-M1.
    let default_comment = CommentDialect::default();
    if grammar.lex.comment.line != default_comment.line {
        match &grammar.lex.comment.line {
            Some(marker) => {
                out.push_str("comment line \"");
                out.push_str(marker);
                out.push_str("\"\n");
            }
            None => out.push_str("comment line none\n"),
        }
    }
    if let Some((open, close)) = &grammar.lex.comment.block {
        out.push_str("comment block \"");
        out.push_str(open);
        out.push_str("\" \"");
        out.push_str(close);
        out.push_str("\"\n");
    }
    for mode in &grammar.lex.strings {
        let which = match mode.quote {
            '"' => "double",
            '\'' => "single",
            _ => continue,
        };
        let escape = match mode.escape {
            StringEscape::Raw => "raw",
            StringEscape::Backslash => "backslash",
            StringEscape::Doubled => "doubled",
        };
        out.push_str("string ");
        out.push_str(which);
        out.push(' ');
        out.push_str(escape);
        out.push('\n');
    }
    for fragment in &grammar.uses {
        out.push_str("use ");
        out.push_str(fragment);
        out.push('\n');
    }
    out.push_str("start ");
    out.push_str(&grammar.start);
    out.push('\n');
    for production in &grammar.productions {
        out.push_str(&production.name);
        out.push_str(" = ");
        print_alternatives(&production.alternatives, &mut out);
        out.push('\n');
    }
    out
}

fn print_count(count: &Count, out: &mut String) {
    match count {
        Count::Fixed(n) => {
            out.push_str("Fixed(");
            out.push_str(&n.to_string());
            out.push(')');
        }
        Count::Varint => out.push_str("Varint"),
        Count::Field(name) => {
            out.push_str("Field(");
            out.push_str(name);
            out.push(')');
        }
    }
}

fn print_prim(prim: &Prim, out: &mut String) {
    match prim {
        Prim::U8 => out.push_str("u8"),
        Prim::U16 => out.push_str("u16"),
        Prim::U32 => out.push_str("u32"),
        Prim::U64 => out.push_str("u64"),
        Prim::I32 => out.push_str("i32"),
        Prim::I64 => out.push_str("i64"),
        Prim::F32 => out.push_str("f32"),
        Prim::F64 => out.push_str("f64"),
        Prim::Varint => out.push_str("varint"),
        Prim::Zigzag => out.push_str("zigzag"),
        Prim::Bytes => out.push_str("bytes"),
        Prim::Utf8 => out.push_str("utf8"),
        Prim::Tag => out.push_str("tag"),
        Prim::Fixed(n) => {
            out.push_str("fixed ");
            out.push_str(&n.to_string());
        }
        Prim::Array(inner, count) => {
            if matches!(inner.as_ref(), Prim::U8) && matches!(count, Count::Varint) {
                out.push_str("varint bytes");
            } else if matches!(count, Count::Varint) {
                out.push_str("array ");
                // 🔁️ Box::pin: self-recursive fn (R10 residue shape 3).
                print_prim(inner, out);
            } else {
                out.push_str("Array(");
                // 🔁️ Box::pin: self-recursive fn (R10 residue shape 3).
                print_prim(inner, out);
                out.push_str(", ");
                print_count(count, out);
                out.push(')');
            }
        }
        Prim::Ref(name) => out.push_str(name),
        Prim::U16Be => out.push_str("u16be"),
        Prim::U32Be => out.push_str("u32be"),
        Prim::U64Be => out.push_str("u64be"),
        Prim::I32Be => out.push_str("i32be"),
        Prim::I64Be => out.push_str("i64be"),
        Prim::F32Be => out.push_str("f32be"),
        Prim::F64Be => out.push_str("f64be"),
        Prim::MarkerScan(prefix) => out.push_str(&format!("marker(0x{prefix:02X})")),
        Prim::Endian(arms) => {
            out.push_str("endian {");
            for (i, (key, be)) in arms.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                out.push('"');
                out.push_str(key);
                out.push_str("\"=");
                out.push_str(if *be { "be" } else { "le" });
            }
            out.push('}');
        }
    }
}

fn print_cond(cond: &Cond, out: &mut String) {
    out.push_str(" if ");
    out.push_str(&cond.field);
    out.push(' ');
    out.push_str(match cond.op {
        CondOp::Eq => "eq",
        CondOp::Ne => "ne",
        CondOp::Lt => "lt",
        CondOp::Le => "le",
        CondOp::Gt => "gt",
        CondOp::Ge => "ge",
        CondOp::BitsSet => "bits",
    });
    out.push(' ');
    out.push_str(&cond.value.to_string());
}

/// 🏷️ Prints raw discriminator/magic bytes back to source: printable multi-byte ASCII
/// round-trips as a `TEXT` literal (PNG/GLB's 4-char tags stay readable), anything else as a hex
/// integer literal — matches [`parse_tag_value`]'s two accepted input forms exactly.
fn print_tag_bytes(tag: &[u8], out: &mut String) {
    let printable = tag.len() > 1 && tag.iter().all(|b| b.is_ascii_graphic() || *b == b' ');
    if printable {
        out.push('"');
        out.push_str(&String::from_utf8_lossy(tag));
        out.push('"');
    } else {
        out.push_str("0x");
        for b in tag {
            out.push_str(&format!("{b:02X}"));
        }
    }
}

fn print_repeat_arm(arm: &RepeatArm, out: &mut String) {
    out.push_str("arm ");
    print_tag_bytes(&arm.tag, out);
    out.push_str(" {");
    for (i, field) in arm.fields.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        print_field(field, out);
    }
    if let Some(nested) = &arm.nested {
        if !arm.fields.is_empty() {
            out.push(' ');
        }
        out.push_str("nested ");
        out.push_str(&nested.name);
        out.push(' ');
        print_prim(&nested.discriminator, out);
        out.push_str(" {");
        for (i, narm) in nested.arms.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            // 🔁️ Box::pin: self-recursive fn (R10 residue shape 3).
            print_repeat_arm(narm, out);
        }
        out.push('}');
    }
    out.push_str("}\n");
}

fn print_field(field: &Field, out: &mut String) {
    out.push_str(&field.name);
    out.push(' ');
    print_prim(&field.ty, out);
    if let Some(cond) = &field.cond {
        print_cond(cond, out);
    }
}

fn header_fixed_size(fields: &[Field]) -> usize {
    let mut total = 0;
    for f in fields {
        total += prim_fixed_width(&f.ty).unwrap_or(0);
    }
    total
}

/// 🖨️ Lossless protocol printer — `parse_protocol(print_protocol(p)) == p`.
pub fn print_protocol(protocol: &ProtocolFile) -> String {
    let mut out = String::new();
    out.push_str("dialect protocol\nprotocol ");
    out.push_str(&protocol.id);
    out.push('\n');
    out.push_str("version ");
    out.push_str(&protocol.version.to_string());
    out.push('\n');
    out.push_str("schema ");
    out.push_str(&protocol.schema);
    out.push('\n');
    out.push_str("start ");
    out.push_str(&protocol.start);
    out.push('\n');
    for fragment in &protocol.uses {
        out.push_str("use ");
        out.push_str(fragment);
        out.push('\n');
    }
    match &protocol.framing {
        Framing::Magic(bytes) => {
            let value = u64::from_be_bytes(*bytes);
            out.push_str(&format!("framing magic 0x{value:016X}\n"));
        }
        Framing::Record => out.push_str("framing record\n"),
        Framing::Chunked => out.push_str("framing chunked\n"),
        Framing::Literal => out.push_str("framing literal\n"),
    }
    for block in &protocol.blocks {
        match block {
            Block::Header(fields) => {
                let size = header_fixed_size(fields);
                out.push_str("header fixed ");
                out.push_str(&size.to_string());
                out.push('\n');
                for field in fields {
                    out.push_str("field ");
                    print_field(field, &mut out);
                    out.push('\n');
                }
            }
            Block::Segment { name, kind, fields, cond } => {
                if name.is_empty() && kind.is_none() && cond.is_none() {
                    for field in fields {
                        out.push_str("segment ");
                        print_field(field, &mut out);
                        out.push('\n');
                    }
                } else {
                    out.push_str("segment ");
                    out.push_str(name);
                    if let Some(k) = kind {
                        out.push_str(" kind=");
                        out.push_str(&k.to_string());
                    }
                    if let Some(c) = cond {
                        print_cond(c, &mut out);
                    }
                    out.push_str(" {");
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push(' ');
                        }
                        print_field(field, &mut out);
                    }
                    out.push_str("}\n");
                }
            }
            Block::Record { name, tag, fields } => {
                if name.is_empty() && tag.is_none() {
                    for field in fields {
                        out.push_str("field ");
                        print_field(field, &mut out);
                        out.push('\n');
                    }
                } else {
                    out.push_str("record ");
                    out.push_str(name);
                    if let Some(t) = tag {
                        out.push_str(" tag=");
                        out.push_str(&t.to_string());
                    }
                    out.push(' ');
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push(' ');
                        }
                        print_field(field, &mut out);
                    }
                    out.push('\n');
                }
            }
            Block::Struct { name, fields } => {
                out.push_str("struct ");
                out.push_str(name);
                out.push_str(" {");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    print_field(field, &mut out);
                }
                out.push_str("}\n");
            }
            Block::Enum { name, variants } => {
                out.push_str("enum ");
                out.push_str(name);
                out.push_str(" {");
                for (i, (vname, value)) in variants.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    out.push_str(vname);
                    out.push('=');
                    out.push_str(&value.to_string());
                }
                out.push_str("}\n");
            }
            Block::Footer(size) => {
                out.push_str("footer fixed ");
                out.push_str(&size.to_string());
                out.push('\n');
            }
            Block::Chain(prim) => {
                out.push_str("chain ");
                print_prim(prim, &mut out);
                out.push('\n');
            }
            Block::Repeat { name, dispatch } => {
                out.push_str("repeat ");
                out.push_str(name);
                out.push_str(" {\n");
                out.push_str("tag ");
                print_prim(&dispatch.discriminator, &mut out);
                out.push('\n');
                if let Some(length) = &dispatch.length {
                    out.push_str("length ");
                    print_prim(length, &mut out);
                    out.push('\n');
                }
                if matches!(dispatch.order, DispatchOrder::LengthFirst) {
                    out.push_str("order length-first\n");
                }
                if let Some(trailer) = &dispatch.trailer {
                    out.push_str("trailer ");
                    print_prim(trailer, &mut out);
                    out.push('\n');
                }
                if let Some(until) = &dispatch.until {
                    out.push_str("until ");
                    print_tag_bytes(until, &mut out);
                    out.push('\n');
                }
                for arm in &dispatch.arms {
                    print_repeat_arm(arm, &mut out);
                }
                out.push_str("}\n");
            }
            Block::BackwardScan { name, magic, fields } => {
                out.push_str("backward ");
                out.push_str(name);
                out.push_str(" magic ");
                print_tag_bytes(magic, &mut out);
                out.push_str(" {");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    print_field(field, &mut out);
                }
                out.push_str("}\n");
            }
            Block::JumpTo { name, offset_field, fields } => {
                out.push_str("jump ");
                out.push_str(name);
                out.push_str(" from ");
                out.push_str(offset_field);
                out.push_str(" {");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    print_field(field, &mut out);
                }
                out.push_str("}\n");
            }
        }
    }
    out
}

/// ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)` — the idempotence law every
/// technology's canonical form must satisfy.
pub fn canonicalize(text: &str) -> Result<String, TextError> {
    if is_protocol_source(text) {
        Ok(print_protocol(&parse_protocol(text)?))
    } else {
        Ok(print_grammar(&parse_grammar(text)?))
    }
}
//#endregion 🔖️Writer

//#region 🔖️FromRecordSpec
// 🗑️ `from_record_spec` hatch deleted — handcrafted `.grammar.semio` / `.protocol.semio` are normative.
//#endregion 🔖️FromRecordSpec
