//! 🧮️ Bounded CSS length expressions shared with the adjacent TypeScript target and neutral contract.

use serde_json::Value;
use std::sync::OnceLock;

pub(crate) fn contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| serde_json::from_str(include_str!("🧬️contract/🔣️.json")).expect("theme geometry contract"))
}

pub(crate) fn bindings() -> &'static Value {
    static BINDINGS: OnceLock<Value> = OnceLock::new();
    BINDINGS.get_or_init(|| serde_json::from_str(include_str!("🔣️.json")).expect("theme geometry bindings"))
}

fn whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0c')
}

fn numeric_end(raw: &[u8]) -> Option<usize> {
    let mut index = 0;
    while raw.get(index).is_some_and(u8::is_ascii_digit) { index += 1; }
    let whole = index;
    if raw.get(index) == Some(&b'.') && raw.get(index + 1).is_some_and(u8::is_ascii_digit) {
        index += 1;
        while raw.get(index).is_some_and(u8::is_ascii_digit) { index += 1; }
    }
    if index == 0 || (whole == 0 && index == 1) { return None; }
    if matches!(raw.get(index), Some(b'e' | b'E')) {
        let mut exponent = index + 1;
        if matches!(raw.get(exponent), Some(b'+' | b'-')) { exponent += 1; }
        let start = exponent;
        while raw.get(exponent).is_some_and(u8::is_ascii_digit) { exponent += 1; }
        if exponent > start { index = exponent; }
    }
    Some(index)
}

#[derive(Clone)]
struct Token { text: String, start: usize, end: usize }

#[derive(Clone, Copy)]
struct Quantity { value: f64, length: bool }

impl Quantity {
    fn new(value: f64, length: bool) -> Option<Self> {
        value.is_finite().then_some(Self { value, length })
    }
}

struct Parser<'a> { source: &'a [u8], tokens: Vec<Token>, cursor: usize, root_rem_px: f64 }

impl Parser<'_> {
    fn peek(&self) -> Option<&str> { self.tokens.get(self.cursor).map(|token| token.text.as_str()) }

    fn take(&mut self, text: &str) -> Option<()> {
        if self.peek()? != text { return None; }
        self.cursor += 1;
        Some(())
    }

    fn expression(&mut self, depth: usize) -> Option<Quantity> {
        let mut left = self.product(depth)?;
        while matches!(self.peek(), Some("+" | "-")) {
            let operator = self.tokens[self.cursor].clone();
            self.cursor += 1;
            if !operator.start.checked_sub(1).and_then(|index| self.source.get(index)).is_some_and(|byte| whitespace(*byte))
                || !self.source.get(operator.end).is_some_and(|byte| whitespace(*byte)) { return None; }
            let right = self.product(depth)?;
            if left.length != right.length { return None; }
            left = Quantity::new(left.value + if operator.text == "+" { right.value } else { -right.value }, left.length)?;
        }
        Some(left)
    }

    fn product(&mut self, depth: usize) -> Option<Quantity> {
        let mut left = self.atom(depth)?;
        while matches!(self.peek(), Some("*" | "/")) {
            let multiply = self.peek() == Some("*");
            self.cursor += 1;
            let right = self.atom(depth)?;
            left = if multiply {
                if left.length && right.length { return None; }
                Quantity::new(left.value * right.value, left.length || right.length)?
            } else {
                if right.value == 0.0 || (!left.length && right.length) { return None; }
                Quantity::new(left.value / right.value, left.length && !right.length)?
            };
        }
        Some(left)
    }

    fn atom(&mut self, depth: usize) -> Option<Quantity> {
        let rules = &contract()["x-semio-resolution"];
        if depth > rules["maxDepth"].as_u64()? as usize { return None; }
        let token = self.tokens.get(self.cursor)?.clone();
        self.cursor += 1;
        if token.text == "+" || token.text == "-" {
            let next = self.tokens.get(self.cursor)?;
            if token.end != next.start || numeric_end(next.text.as_bytes()).is_none() { return None; }
            let operand = self.atom(depth + 1)?;
            return Quantity::new(if token.text == "-" { -operand.value } else { operand.value }, operand.length);
        }
        if token.text == "(" {
            let nested = self.expression(depth + 1)?;
            self.take(")")?;
            return Some(nested);
        }
        if let Some(end) = numeric_end(token.text.as_bytes()) {
            let unit = &token.text[end..];
            let multiplier = if unit.is_empty() { 1.0 } else if unit == "rem" { self.root_rem_px } else { rules["absoluteUnits"].get(unit)?.as_f64()? };
            return Quantity::new(token.text[..end].parse::<f64>().ok()? * multiplier, !unit.is_empty());
        }
        if !rules["functions"].as_array()?.iter().any(|function| function.as_str() == Some(token.text.as_str())) { return None; }
        if rules["functionNameAdjacency"].as_bool()? && token.end != self.tokens.get(self.cursor)?.start { return None; }
        self.take("(")?;
        let mut arguments = vec![self.expression(depth + 1)?];
        while self.peek() == Some(",") {
            self.cursor += 1;
            arguments.push(self.expression(depth + 1)?);
            if arguments.len() > rules["maxArguments"].as_u64()? as usize { return None; }
        }
        self.take(")")?;
        if arguments.iter().any(|argument| argument.length != arguments[0].length) { return None; }
        let value = match token.text.as_str() {
            "calc" if arguments.len() == 1 => arguments[0].value,
            "min" => arguments.iter().map(|argument| argument.value).fold(f64::INFINITY, f64::min),
            "max" => arguments.iter().map(|argument| argument.value).fold(f64::NEG_INFINITY, f64::max),
            "clamp" if arguments.len() == 3 => arguments[0].value.max(arguments[1].value.min(arguments[2].value)),
            _ => return None,
        };
        Quantity::new(value, arguments[0].length)
    }
}

/// 📏️ Resolves an authored CSS length using the shared limits, unit factors and explicit root reference.
pub(crate) fn resolve_spacing_px(compact: &str, root_rem_px: f64) -> Option<f64> {
    let rules = &contract()["x-semio-resolution"];
    if compact.len() > contract()["properties"]["compact"]["maxLength"].as_u64()? as usize || !root_rem_px.is_finite() || root_rem_px <= 0.0 { return None; }
    let raw = compact.as_bytes();
    let mut offset = 0;
    let mut tokens = Vec::new();
    while offset < raw.len() {
        if whitespace(raw[offset]) { offset += 1; continue; }
        let start = offset;
        if let Some(end) = numeric_end(&raw[offset..]) {
            offset += end;
            while raw.get(offset).is_some_and(u8::is_ascii_alphabetic) { offset += 1; }
        } else if raw[offset].is_ascii_alphabetic() {
            while raw.get(offset).is_some_and(u8::is_ascii_alphabetic) { offset += 1; }
        } else if b"()+-*/,".contains(&raw[offset]) { offset += 1; }
        else { return None; }
        tokens.push(Token { text: compact[start..offset].to_ascii_lowercase(), start, end: offset });
        if tokens.len() > rules["maxTokens"].as_u64()? as usize { return None; }
    }
    let mut parser = Parser { source: raw, tokens, cursor: 0, root_rem_px };
    if parser.peek() == Some("(") { return None; }
    let result = parser.atom(0)?;
    (parser.cursor == parser.tokens.len() && result.value >= 0.0 && result.value <= rules["maxMagnitudePx"].as_f64()? && (result.length || result.value == 0.0)).then_some(result.value)
}
