//! ✍️ Text writer: compact and pretty serialization of values, strings and numbers. See [`super`].

use super::*;

/// 📏️ Counts compact JSON bytes without allocating, stopping when the limit or nesting bound is exceeded.
pub fn bounded_json_len(value: &Value, maximum: usize) -> Option<usize> {
    #[derive(Clone, Copy)]
    struct Frame<'a> { value: &'a Value, next: usize }
    fn add(total: &mut usize, amount: usize, maximum: usize) -> Option<()> {
        *total = total.checked_add(amount).filter(|size| *size <= maximum)?;
        Some(())
    }
    fn string(total: &mut usize, text: &str, maximum: usize) -> Option<()> {
        add(total, text.len().checked_add(2)?, maximum)?;
        for byte in text.bytes() {
            let extra = match byte { b'"' | b'\\' | 8 | 12 | 10 | 13 | 9 => 1, 0..=31 => 5, _ => 0 };
            add(total, extra, maximum)?;
        }
        Some(())
    }
    let mut total = 0;
    let mut frames: [Option<Frame<'_>>; MAX_DEPTH as usize] = [None; MAX_DEPTH as usize];
    let mut depth = 0;
    let mut next = Some(value);
    loop {
        if let Some(value) = next.take() {
            match value {
                Value::Null => add(&mut total, 4, maximum)?,
                Value::Bool(value) => add(&mut total, if *value { 4 } else { 5 }, maximum)?,
                Value::Number(number) => {
                    let node = match number { Number::UInt(value) => ArtifactCanonicalJsonNode::U64(*value), Number::Int(value) => ArtifactCanonicalJsonNode::I64(*value), Number::Float(value) => ArtifactCanonicalJsonNode::F64(*value) };
                    add(&mut total, ArtifactCanonicalJsonScalarBytes::from_node(node).ok()?.length, maximum)?;
                }
                Value::String(text) => string(&mut total, text, maximum)?,
                Value::Array(items) => {
                    if depth >= MAX_DEPTH as usize { return None; }
                    add(&mut total, 2usize.checked_add(items.len().saturating_sub(1))?, maximum)?;
                    if !items.is_empty() { *frames.get_mut(depth)? = Some(Frame { value, next: 0 }); depth += 1; }
                }
                Value::Object(object) => {
                    if depth >= MAX_DEPTH as usize { return None; }
                    add(&mut total, 2usize.checked_add(object.len().saturating_sub(1))?, maximum)?;
                    if !object.is_empty() { *frames.get_mut(depth)? = Some(Frame { value, next: 0 }); depth += 1; }
                }
            }
        }
        loop {
            if depth == 0 { return Some(total); }
            let frame = frames[depth - 1].as_mut()?;
            match frame.value {
                Value::Array(items) if frame.next < items.len() => { next = Some(&items[frame.next]); frame.next += 1; break; }
                Value::Object(object) if frame.next < object.len() => {
                    let (key, value) = &object.0[frame.next];
                    string(&mut total, key, maximum)?;
                    add(&mut total, 1, maximum)?;
                    next = Some(value); frame.next += 1; break;
                }
                _ => { depth -= 1; frames[depth] = None; }
            }
        }
    }
}

pub fn to_string(value: &Value) -> String {
    let mut out = String::new();
    write_value(value, &mut out);
    out
}

pub fn to_string_pretty(value: &Value) -> String {
    let mut out = String::new();
    write_value_pretty(value, &mut out, 0);
    out
}

fn write_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn write_value_pretty(value: &Value, out: &mut String, depth: usize) {
    match value {
        Value::Array(items) if !items.is_empty() => {
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                write_indent(out, depth + 1);
                write_value_pretty(item, out, depth + 1);
            }
            out.push('\n');
            write_indent(out, depth);
            out.push(']');
        }
        Value::Object(object) if !object.is_empty() => {
            out.push_str("{\n");
            for (index, (key, value)) in object.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                write_indent(out, depth + 1);
                write_string(key, out);
                out.push_str(": ");
                write_value_pretty(value, out, depth + 1);
            }
            out.push('\n');
            write_indent(out, depth);
            out.push('}');
        }
        other => write_value(other, out),
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&to_string(self))
    }
}

fn write_value(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => write_number(*number, out),
        Value::String(text) => write_string(text, out),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_value(item, out);
            }
            out.push(']');
        }
        Value::Object(object) => {
            out.push('{');
            for (index, (key, value)) in object.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_string(key, out);
                out.push(':');
                write_value(value, out);
            }
            out.push('}');
        }
    }
}

fn write_string(text: &str, out: &mut String) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if (control as u32) < 0x20 => {
                use fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", control as u32);
            }
            other => out.push(other),
        }
    }
    out.push('"');
}

fn write_number(number: Number, out: &mut String) {
    match number {
        Number::UInt(value) => {
            use fmt::Write as _;
            let _ = write!(out, "{value}");
        }
        Number::Int(value) => {
            use fmt::Write as _;
            let _ = write!(out, "{value}");
        }
        Number::Float(value) => write_float(value, out),
    }
}

pub(super) fn write_float(value: f64, out: &mut String) {
    write_float_to(value, out).expect("String accepts formatted JSON floats");
}
