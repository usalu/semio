//! 🔤️ A comment-, string- and bracket-aware Rust token stream for the module tree reader: identifiers, single punctuation characters and string literals with their value, plus the matching bracket of every opener.

/// 🔤️ One lexical token of a Rust source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustToken {
    Identifier(String),
    Punct(char),
    Str(Option<String>),
    Other,
}

impl RustToken {
    pub fn is_punct(&self, expected: char) -> bool {
        matches!(self, RustToken::Punct(found) if *found == expected)
    }

    pub fn is_identifier(&self, expected: &str) -> bool {
        matches!(self, RustToken::Identifier(found) if found == expected)
    }
}

/// 🔗️ Splits a Rust source into tokens; comments and whitespace vanish and string contents never leak as tokens.
/// @see <https://doc.rust-lang.org/reference/tokens.html>
pub fn rust_tokens(source: &str) -> Vec<RustToken> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let current = chars[index];
        if current.is_whitespace() {
            index += 1;
        } else if current == '/' && chars.get(index + 1) == Some(&'/') {
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
        } else if current == '/' && chars.get(index + 1) == Some(&'*') {
            index = skip_block_comment(&chars, index);
        } else if let Some((end, value)) = string_literal(&chars, index) {
            tokens.push(RustToken::Str(value));
            index = end;
        } else if current == '\'' {
            index = skip_quote(&chars, index);
            tokens.push(RustToken::Other);
        } else if is_word(current) {
            let start = index;
            while index < chars.len() && is_word(chars[index]) {
                index += 1;
            }
            if chars[start..index] == ['r'] && chars.get(index) == Some(&'#') && chars.get(index + 1).is_some_and(|next| is_word(*next)) {
                index += 1;
                while index < chars.len() && is_word(chars[index]) {
                    index += 1;
                }
            }
            tokens.push(RustToken::Identifier(chars[start..index].iter().collect()));
        } else {
            tokens.push(RustToken::Punct(current));
            index += 1;
        }
    }
    tokens
}

/// 🧷️ The matching closer of every `(`, `[` and `{` token, and the opener of every closer.
pub fn rust_token_pairs(tokens: &[RustToken]) -> Vec<Option<usize>> {
    let mut pairs = vec![None; tokens.len()];
    let mut open: Vec<(char, usize)> = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let RustToken::Punct(punct) = token else { continue };
        match punct {
            '(' | '[' | '{' => open.push((*punct, index)),
            ')' | ']' | '}' => {
                let expected = match punct {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                if let Some(position) = open.iter().rposition(|(kind, _)| *kind == expected) {
                    let (_, opener) = open[position];
                    open.truncate(position);
                    pairs[opener] = Some(index);
                    pairs[index] = Some(opener);
                }
            }
            _ => {}
        }
    }
    pairs
}

fn is_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn skip_block_comment(chars: &[char], start: usize) -> usize {
    let mut depth = 0usize;
    let mut index = start;
    while index < chars.len() {
        if chars[index] == '/' && chars.get(index + 1) == Some(&'*') {
            depth += 1;
            index += 2;
        } else if chars[index] == '*' && chars.get(index + 1) == Some(&'/') {
            depth -= 1;
            index += 2;
            if depth == 0 {
                return index;
            }
        } else {
            index += 1;
        }
    }
    chars.len()
}

fn skip_quote(chars: &[char], start: usize) -> usize {
    let mut index = start + 1;
    if chars.get(index) == Some(&'\\') {
        index += 2;
        while index < chars.len() && chars[index] != '\'' && chars[index] != '\n' {
            index += 1;
        }
        return (index + 1).min(chars.len());
    }
    if chars.get(index + 1) == Some(&'\'') {
        return index + 2;
    }
    while index < chars.len() && is_word(chars[index]) {
        index += 1;
    }
    index
}

fn string_literal(chars: &[char], start: usize) -> Option<(usize, Option<String>)> {
    let mut index = start;
    let mut prefix = String::new();
    while index < chars.len() && matches!(chars[index], 'b' | 'c' | 'r') && prefix.len() < 2 {
        prefix.push(chars[index]);
        index += 1;
    }
    let raw = prefix.ends_with('r');
    let mut hashes = 0;
    if raw {
        while chars.get(index) == Some(&'#') {
            hashes += 1;
            index += 1;
        }
    }
    if chars.get(index) != Some(&'"') || !matches!(prefix.as_str(), "" | "b" | "c" | "r" | "br" | "cr") {
        return None;
    }
    if start > 0 && is_word(chars[start - 1]) {
        return None;
    }
    index += 1;
    let mut value = String::new();
    while index < chars.len() {
        if raw {
            if chars[index] == '"' && (0..hashes).all(|offset| chars.get(index + 1 + offset) == Some(&'#')) {
                return Some((index + 1 + hashes, plain(&prefix).then_some(value)));
            }
            value.push(chars[index]);
            index += 1;
        } else if chars[index] == '\\' {
            index = push_escape(chars, index, &mut value);
        } else if chars[index] == '"' {
            return Some((index + 1, plain(&prefix).then_some(value)));
        } else {
            value.push(chars[index]);
            index += 1;
        }
    }
    Some((chars.len(), None))
}

fn plain(prefix: &str) -> bool {
    prefix.is_empty() || prefix == "r"
}

fn push_escape(chars: &[char], start: usize, value: &mut String) -> usize {
    let Some(next) = chars.get(start + 1) else { return chars.len() };
    match next {
        'n' => value.push('\n'),
        'r' => value.push('\r'),
        't' => value.push('\t'),
        '0' => value.push('\0'),
        '\\' | '"' | '\'' => value.push(*next),
        '\n' => {
            let mut index = start + 2;
            while chars.get(index).is_some_and(|skipped| skipped.is_whitespace()) {
                index += 1;
            }
            return index;
        }
        'u' if chars.get(start + 2) == Some(&'{') => {
            let close = chars[start + 2..].iter().position(|character| *character == '}').map(|offset| start + 2 + offset);
            if let Some(close) = close {
                let digits: String = chars[start + 3..close].iter().filter(|digit| **digit != '_').collect();
                if let Some(decoded) = u32::from_str_radix(&digits, 16).ok().and_then(char::from_u32) {
                    value.push(decoded);
                }
                return close + 1;
            }
        }
        'x' => {
            let digits: String = chars[start + 2..(start + 4).min(chars.len())].iter().collect();
            if let Some(decoded) = u32::from_str_radix(&digits, 16).ok().and_then(char::from_u32) {
                value.push(decoded);
            }
            return (start + 4).min(chars.len());
        }
        other => value.push(*other),
    }
    start + 2
}
