//! ✍️ Writer of the Part-21 codec: the canonical text writer with its preamble hook and the string escaping. See [`super`].

use super::*;

//#region 🔖️Write
/// 🧭️ Deterministic physical layout selected by a standard-specific serializer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Part21WriteOptions {
    pub line_ending: &'static str,
    pub blank_after_header: bool,
    pub blank_before_data: bool,
    pub blank_before_terminator: bool,
    pub space_after_instance_equals: bool,
}

impl Default for Part21WriteOptions {
    fn default() -> Self {
        Self { line_ending: "\n", blank_after_header: false, blank_before_data: false, blank_before_terminator: false, space_after_instance_equals: false }
    }
}

/// 🏭️ Logical producer metadata capable of deterministic Part-21 header materialization.
pub trait Part21Preamble {
    fn write_preamble(&self, out: &mut String, line_ending: &str);
}

/// 🈳️ Zero-sized stand-in preamble type (O1 — R11(a): `write_part21_with`'s preamble is a
/// borrowed-reference parameter, trivially generic; `write_part21` still needs SOME concrete type to
/// instantiate that generic with when it passes `None`, and picking a real implementor — e.g. ifc's
/// `Ifc2x3EdmPreamble` — would make this generic `step` module depend downward on a specific format
/// built on top of it). Never constructed; `write_preamble` is unreachable by construction.
struct NoPreamble;

impl Part21Preamble for NoPreamble {
    fn write_preamble(&self, _out: &mut String, _line_ending: &str) {}
}

/// 📤️ Regenerates valid Part-21 text from the generic graph — round-trip losslessness is
/// the writer's job; it never re-derives STEP/IFC semantics.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_part21(doc: &Part21Document) -> String {
    write_part21_with::<NoPreamble>(doc, Part21WriteOptions::default(), None)
}

/// 📤️ Regenerates Part-21 with a standard-selected deterministic layout and typed preamble. Generic
/// over the preamble implementor (O1 — R11(a): borrowed-reference parameter, trivially generic).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_part21_with<P: Part21Preamble>(doc: &Part21Document, options: Part21WriteOptions, preamble: Option<&P>) -> String {
    let eol = options.line_ending;
    let mut out = format!("ISO-10303-21;{eol}HEADER;{eol}");
    if options.blank_after_header {
        out.push_str(eol);
    }
    if let Some(preamble) = preamble {
        preamble.write_preamble(&mut out, eol);
    }
    write_header_record(&mut out, "FILE_DESCRIPTION", FILE_DESCRIPTION_ATTRIBUTES, &doc.header.file_description, eol);
    write_header_record(&mut out, "FILE_NAME", FILE_NAME_ATTRIBUTES, &doc.header.file_name, eol);
    write_header_record(&mut out, "FILE_SCHEMA", FILE_SCHEMA_ATTRIBUTES, &doc.header.file_schema, eol);
    out.push_str("ENDSEC;");
    out.push_str(eol);
    if options.blank_before_data {
        out.push_str(eol);
    }
    out.push_str("DATA;");
    out.push_str(eol);
    for inst in &doc.instances {
        write_instance(&mut out, inst, eol, options.space_after_instance_equals);
    }
    out.push_str("ENDSEC;");
    out.push_str(eol);
    if options.blank_before_terminator {
        out.push_str(eol);
    }
    out.push_str("END-ISO-10303-21;");
    out.push_str(eol);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_record(out: &mut String, name: &str, args: &[Part21Value], line_ending: &str) {
    out.push_str(name);
    out.push('(');
    write_value_list(out, args);
    out.push_str(");");
    out.push_str(line_ending);
}

/// 📜️ One mandatory `HEADER` record, written against ISO 10303-21 §8.2's fixed attribute list so
/// the emitted exchange structure is always one a conformant reader accepts.
///
/// Two spec obligations the generic [`write_record`] cannot know about, both of which this
/// codebase was breaching until wave 15's differential run caught it (`ruststep` refusing
/// `FILE_DESCRIPTION((),'')` with "expected ')', found ("):
///
/// * every attribute the standard declares is present, so a header carrying fewer values than its
///   record's arity is padded with that position's unpopulated spelling rather than emitted short;
/// * a `LIST[1:?] OF STRING` is never emitted empty — `()` violates the lower bound, and the
///   conformant spelling of an empty description/author/organization/schema list is `('')`.
///
/// Values the caller DID populate are written verbatim; this only ever fills in what is missing,
/// so nothing a real document carried is normalized away.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_record(out: &mut String, name: &str, attributes: &[HeaderAttribute], args: &[Part21Value], line_ending: &str) {
    let conformant: Vec<Part21Value> = attributes
        .iter()
        .enumerate()
        .map(|(position, attribute)| match args.get(position) {
            Some(Part21Value::List(items)) if items.is_empty() && *attribute == HeaderAttribute::NonEmptyTextList => attribute.unpopulated(),
            Some(value) => value.clone(),
            None => attribute.unpopulated(),
        })
        .chain(args.iter().skip(attributes.len()).cloned())
        .collect();
    write_record(out, name, &conformant, line_ending);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_instance(out: &mut String, inst: &Part21Instance, line_ending: &str, space_after_equals: bool) {
    let _ = write!(out, "#{}=", inst.id);
    if space_after_equals {
        out.push(' ');
    }
    if inst.entities.len() == 1 {
        let (name, args) = &inst.entities[0];
        out.push_str(name);
        out.push('(');
        write_value_list(out, args);
        out.push(')');
    } else {
        out.push('(');
        for (name, args) in &inst.entities {
            out.push_str(name);
            out.push('(');
            write_value_list(out, args);
            out.push(')');
        }
        out.push(')');
    }
    out.push(';');
    out.push_str(line_ending);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_value_list(out: &mut String, items: &[Part21Value]) {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write_value(out, item);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_value(out: &mut String, v: &Part21Value) {
    match v {
        Part21Value::Ref(id) => {
            let _ = write!(out, "#{id}");
        }
        Part21Value::Str(s) => {
            out.push('\'');
            out.push_str(&escape_part21_string(s));
            out.push('\'');
        }
        Part21Value::Enum(name) => {
            out.push('.');
            out.push_str(name);
            out.push('.');
        }
        Part21Value::Int(i) => {
            let _ = write!(out, "{i}");
        }
        Part21Value::Real(r) => write!(out, "{r}").expect("String write"),
        Part21Value::List(items) => {
            out.push('(');
            write_value_list(out, items);
            out.push(')');
        }
        Part21Value::Typed { name, items } => {
            out.push_str(name);
            out.push('(');
            write_value_list(out, items);
            out.push(')');
        }
        Part21Value::Unset => out.push('$'),
        Part21Value::Derived => out.push('*'),
    }
}

/// 🔡️ Inverse of the lexer's `read_escape`: `'` doubles, backslash is escaped (to stay
/// unambiguous with `\X..` on reparse), any other non-printable-ASCII goes through `\X2\..\X0\`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn escape_part21_string(s: &str) -> String {
    let mut out = String::new();
    let mut run: Vec<char> = Vec::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn flush(run: &mut Vec<char>, out: &mut String) {
        if run.is_empty() {
            return;
        }
        out.push_str("\\X2\\");
        for c in run.drain(..) {
            let _ = write!(out, "{:04X}", c as u32);
        }
        out.push_str("\\X0\\");
    }
    for c in s.chars() {
        if c == '\'' {
            flush(&mut run, &mut out);
            out.push_str("''");
        } else if c == '\\' {
            flush(&mut run, &mut out);
            run.push(c);
            flush(&mut run, &mut out);
        } else if (0x20..=0x7E).contains(&(c as u32)) {
            flush(&mut run, &mut out);
            out.push(c);
        } else {
            run.push(c);
        }
    }
    flush(&mut run, &mut out);
    out
}
//#endregion 🔖️Write
