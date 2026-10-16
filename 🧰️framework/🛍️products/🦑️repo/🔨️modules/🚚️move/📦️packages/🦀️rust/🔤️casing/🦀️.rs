//! 🔤️ Casing: the casing variants of a renamed token.

//#region 🔤️Casing

/// 🅰️ `value` with its first byte upper-cased and the rest lower-cased, as the Go twin's byte
/// slicing does. Case mapping is the simple one-to-one mapping Go's `strings` package uses, so a
/// rune whose full mapping grows — `ß` — is left alone rather than expanded.
pub fn title_case_token(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    let bytes = value.as_bytes();
    let head = bytes[0].to_ascii_uppercase();
    let tail = match std::str::from_utf8(&bytes[1..]) {
        Ok(rest) => simple_lower(rest),
        Err(_) => String::from_utf8_lossy(&bytes[1..]).into_owned(),
    };
    let mut out = String::with_capacity(value.len());
    out.push(head as char);
    out.push_str(&tail);
    out
}

/// 🔠️ Go's `strings.ToUpper`: the simple, length-preserving case mapping.
pub fn simple_upper(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            let mut mapped = c.to_uppercase();
            match (mapped.next(), mapped.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🔡️ Go's `strings.ToLower`: the simple, length-preserving case mapping.
pub fn simple_lower(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            let mut mapped = c.to_lowercase();
            match (mapped.next(), mapped.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🔤️ `content` with the UPPER, Title and lower variants of `old_token` rewritten to the matching
/// variant of `new_token`. A token spelled in kebab, camel, pascal, snake or screaming style is
/// carried through unchanged apart from those three case foldings, exactly as the Go twin does.
pub fn apply_rename_casings(content: &str, old_token: &str, new_token: &str) -> String {
    let old_upper = simple_upper(old_token);
    let new_upper = simple_upper(new_token);
    let old_lower = simple_lower(old_token);
    let new_lower = simple_lower(new_token);
    let old_title = title_case_token(&old_lower);
    let new_title = title_case_token(&new_lower);
    let mut out = content.to_string();
    if old_upper != old_lower {
        out = out.replace(&old_upper, &new_upper);
    }
    if old_title != old_upper && old_title != old_lower {
        out = out.replace(&old_title, &new_title);
    }
    out.replace(&old_lower, &new_lower)
}

//#endregion 🔤️Casing
