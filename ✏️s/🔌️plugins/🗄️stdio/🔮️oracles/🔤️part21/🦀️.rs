//! 🔤️ Owned Part21 reference string grammar shared by independent oracle providers.

/// 🔡️ Decodes the exact ISO 10303-21 string lexeme independently of subject codecs.
pub fn decode_string_literal(lexeme: &str) -> Result<String, String> {
    let chars: Vec<char> = lexeme.chars().collect();
    let mut out = String::with_capacity(lexeme.len());
    let mut index = 0;
    let mut alphabet = 'A';
    let hex_group = |chars: &[char], from: usize, width: usize| -> Result<char, String> {
        let slice: String = chars.get(from..from + width).ok_or_else(|| format!("truncated hex group in {lexeme:?}"))?.iter().collect();
        if !slice.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("bad hex group {slice:?} in {lexeme:?}"));
        }
        let code = u32::from_str_radix(&slice, 16).map_err(|error| format!("bad hex group {slice:?}: {error}"))?;
        char::from_u32(code).ok_or_else(|| format!("hex group {slice:?} is not a codepoint"))
    };
    while index < chars.len() {
        if chars[index] != '\\' {
            out.push(chars[index]);
            index += 1;
            continue;
        }
        match chars.get(index + 1) {
            Some('\\') => {
                out.push('\\');
                index += 2;
            }
            Some('P') => {
                let page = *chars.get(index + 2).ok_or_else(|| format!("truncated \\P directive in {lexeme:?}"))?;
                if !('A'..='I').contains(&page) || chars.get(index + 3) != Some(&'\\') {
                    return Err(format!("malformed \\P directive in {lexeme:?}"));
                }
                alphabet = page;
                index += 4;
            }
            Some('S') => {
                if chars.get(index + 2) != Some(&'\\') {
                    return Err(format!("malformed \\S directive in {lexeme:?}"));
                }
                if alphabet != 'A' {
                    return Err(format!("\\S\\ on ISO 8859 page {alphabet} needs a mapping table this projection does not carry"));
                }
                let shifted = *chars.get(index + 3).ok_or_else(|| format!("truncated \\S directive in {lexeme:?}"))?;
                out.push(char::from_u32(shifted as u32 + 128).ok_or_else(|| format!("bad \\S\\ character in {lexeme:?}"))?);
                index += 4;
            }
            Some('X') => match chars.get(index + 2) {
                Some(width @ ('2' | '4')) => {
                    let group = if *width == '2' { 4 } else { 8 };
                    if chars.get(index + 3) != Some(&'\\') {
                        return Err(format!("malformed \\X{width} directive in {lexeme:?}"));
                    }
                    index += 4;
                    loop {
                        out.push(hex_group(&chars, index, group)?);
                        index += group;
                        if chars.get(index) == Some(&'\\') && chars.get(index + 1) == Some(&'X') && chars.get(index + 2) == Some(&'0') && chars.get(index + 3) == Some(&'\\') {
                            index += 4;
                            break;
                        }
                    }
                }
                Some('\\') => {
                    out.push(hex_group(&chars, index + 3, 2)?);
                    index += 5;
                }
                other => return Err(format!("malformed \\X directive {other:?} in {lexeme:?}")),
            },
            other => return Err(format!("unsupported control directive {other:?} in {lexeme:?}")),
        }
    }
    Ok(out)
}
