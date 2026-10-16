//! 🧩️ Pattern matcher: the backtracking matcher for the declarative token patterns.
//!
//! Holds the `🔖️Matcher` region of the crate root; the root re-exports its items.

use super::*;

// #region 🔖️Matcher

const fn is_ws_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C)
}

const fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// 🪪️ The captures a pattern match produced, in match order.
pub type Captures = Vec<(String, String)>;

enum Cont<'a> {
    Nil,
    Cons(&'a [Token], &'a Cont<'a>),
}

fn eq_at(line: &str, pos: usize, lit: &str, ci: bool) -> bool {
    let bytes = line.as_bytes();
    let lb = lit.as_bytes();
    if pos + lb.len() > bytes.len() {
        return false;
    }
    if ci {
        bytes[pos..pos + lb.len()].eq_ignore_ascii_case(lb)
    } else {
        &bytes[pos..pos + lb.len()] == lb
    }
}

fn run_len(line: &str, pos: usize, pred: impl Fn(u8) -> bool) -> usize {
    line.as_bytes()[pos..].iter().take_while(|b| pred(**b)).count()
}

pub(super) fn scoped_ends(line: &str, pos: usize, sep: &str) -> Vec<usize> {
    let first = ident_len(line, pos);
    if first == 0 {
        return Vec::new();
    }
    let mut ends = vec![pos + first];
    let mut cursor = pos + first;
    loop {
        if !eq_at(line, cursor, sep, false) {
            break;
        }
        let next = cursor + sep.len();
        let n = ident_len(line, next);
        if n == 0 {
            break;
        }
        cursor = next + n;
        ends.push(cursor);
    }
    ends.reverse();
    ends
}

pub(super) fn ident_len(line: &str, pos: usize) -> usize {
    let b = line.as_bytes();
    if pos >= b.len() || !(b[pos].is_ascii_alphabetic() || b[pos] == b'_') {
        return 0;
    }
    1 + run_len(line, pos + 1, is_word_byte)
}

fn upper_ident_len(line: &str, pos: usize, extra: &[u8]) -> usize {
    let b = line.as_bytes();
    if pos >= b.len() || !b[pos].is_ascii_uppercase() {
        return 0;
    }
    1 + run_len(line, pos + 1, |c| is_word_byte(c) || extra.contains(&c))
}

fn char_count_until(line: &str, pos: usize, stop: &str) -> usize {
    line[pos..].chars().take_while(|c| !stop.contains(*c)).count()
}

fn nth_char_offset(line: &str, pos: usize, n: usize) -> usize {
    line[pos..].char_indices().nth(n).map_or(line.len(), |(i, _)| pos + i)
}

/// 🔚️ The lazy `(.+?)\s*$` capture: the trailing-trimmed rest, or one character when all-blank.
fn rest_trim_capture(content: &str) -> Option<String> {
    if content.is_empty() {
        return None;
    }
    let trimmed = content.trim_end_matches(|c: char| is_ws_byte(c as u8) && c.is_ascii());
    if trimmed.is_empty() {
        content.chars().next().map(|c| c.to_string())
    } else {
        Some(trimmed.to_string())
    }
}

/// 🔢️ The end offsets a single token can reach from `pos`, in Go's preference order.
fn token_ends(tok: &Token, line: &str, pos: usize, ci: bool) -> Vec<usize> {
    let bytes = line.as_bytes();
    match tok {
        Token::Ws => {
            let n = run_len(line, pos, is_ws_byte);
            (0..=n).rev().map(|k| pos + k).collect()
        }
        Token::Ws1 => {
            let n = run_len(line, pos, is_ws_byte);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::WsChar => {
            if pos < bytes.len() && is_ws_byte(bytes[pos]) {
                vec![pos + 1]
            } else {
                Vec::new()
            }
        }
        Token::End => {
            if pos == bytes.len() {
                vec![pos]
            } else {
                Vec::new()
            }
        }
        Token::WordEnd => {
            if pos >= bytes.len() || !is_word_byte(bytes[pos]) {
                vec![pos]
            } else {
                Vec::new()
            }
        }
        Token::LineStart => {
            if pos == 0 {
                vec![pos]
            } else {
                Vec::new()
            }
        }
        Token::Lit { v } => {
            if eq_at(line, pos, v, ci) {
                vec![pos + v.len()]
            } else {
                Vec::new()
            }
        }
        Token::AnyOf { v } => v.iter().filter(|s| eq_at(line, pos, s, ci)).map(|s| pos + s.len()).collect(),
        Token::CharIn { v } => line[pos..].chars().next().filter(|c| v.contains(*c)).map(|c| vec![pos + c.len_utf8()]).unwrap_or_default(),
        Token::Ident => {
            let n = ident_len(line, pos);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::IdentUpper => {
            let n = upper_ident_len(line, pos, &[]);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::TypeIdentUpper => {
            let n = upper_ident_len(line, pos, b"<>");
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::Word => {
            let n = run_len(line, pos, is_word_byte);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::IdentScoped { sep } => scoped_ends(line, pos, sep),
        Token::Until { stop, min } => {
            let n = char_count_until(line, pos, stop);
            if n < *min {
                return Vec::new();
            }
            (*min..=n).rev().map(|k| nth_char_offset(line, pos, k)).collect()
        }
        Token::RestTrim => {
            if pos < bytes.len() {
                vec![bytes.len()]
            } else {
                Vec::new()
            }
        }
        Token::RestTrimNonSpace => {
            if pos < bytes.len() && !is_ws_byte(bytes[pos]) {
                vec![bytes.len()]
            } else {
                Vec::new()
            }
        }
        Token::Opt { .. } | Token::Alt { .. } | Token::Rep { .. } | Token::Cap { .. } | Token::RepChar { .. } => Vec::new(),
    }
}

fn capture_text(tok: &Token, line: &str, pos: usize, end: usize) -> String {
    match tok {
        Token::RestTrim | Token::RestTrimNonSpace => rest_trim_capture(&line[pos..]).unwrap_or_default(),
        _ => line[pos..end].to_string(),
    }
}

fn run_tokens(toks: &[Token], k: &Cont<'_>, line: &str, pos: usize, ci: bool, caps: &mut Captures) -> Option<usize> {
    let Some((head, tail)) = toks.split_first() else {
        return match k {
            Cont::Nil => Some(pos),
            Cont::Cons(next, rest) => run_tokens(next, rest, line, pos, ci, caps),
        };
    };
    let saved = caps.len();
    match head {
        Token::Opt { of } => {
            let k2 = Cont::Cons(tail, k);
            if let Some(e) = run_tokens(of, &k2, line, pos, ci, caps) {
                return Some(e);
            }
            caps.truncate(saved);
            run_tokens(tail, k, line, pos, ci, caps)
        }
        Token::Alt { of } => {
            let k2 = Cont::Cons(tail, k);
            for branch in of {
                if let Some(e) = run_tokens(branch, &k2, line, pos, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        Token::Rep { of, min } => {
            let mut stops = vec![pos];
            let mut cursor = pos;
            while let Some(next) = run_tokens(of, &Cont::Nil, line, cursor, ci, caps) {
                caps.truncate(saved);
                if next == cursor {
                    break;
                }
                cursor = next;
                stops.push(cursor);
            }
            caps.truncate(saved);
            for (count, stop) in stops.iter().enumerate().rev() {
                if count < *min {
                    break;
                }
                if let Some(e) = run_tokens(tail, k, line, *stop, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        Token::Cap { name, of } => {
            for end in token_ends(of, line, pos, ci) {
                caps.push((name.clone(), capture_text(of, line, pos, end)));
                if let Some(e) = run_tokens(tail, k, line, end, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        Token::RepChar { v, min, max, name } => {
            let c = v.chars().next()?;
            let run = line[pos..].chars().take_while(|x| *x == c).count();
            let hi = run.min(*max);
            for n in (*min..=hi).rev() {
                let end = pos + n * c.len_utf8();
                caps.push((name.clone(), line[pos..end].to_string()));
                if let Some(e) = run_tokens(tail, k, line, end, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        simple => {
            for end in token_ends(simple, line, pos, ci) {
                if let Some(e) = run_tokens(tail, k, line, end, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
    }
}

/// 🎯️ Matches a pattern anchored at the start of `line`, returning the end offset and captures.
pub fn pattern_match(p: &Pattern, line: &str) -> Option<(usize, Captures)> {
    let mut caps = Captures::new();
    run_tokens(&p.tokens, &Cont::Nil, line, 0, p.ci, &mut caps).map(|end| (end, caps))
}

/// 🔍️ Whether a pattern matches anywhere in `line`.
pub fn pattern_find(p: &Pattern, line: &str) -> bool {
    (0..=line.len()).any(|i| line.is_char_boundary(i) && pattern_match_at(p, line, i).is_some())
}

fn pattern_match_at(p: &Pattern, line: &str, pos: usize) -> Option<usize> {
    let mut caps = Captures::new();
    run_tokens(&p.tokens, &Cont::Nil, line, pos, p.ci, &mut caps)
}

pub(super) fn capture(caps: &Captures, name: &str) -> Option<String> {
    caps.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone())
}

// #endregion 🔖️Matcher
