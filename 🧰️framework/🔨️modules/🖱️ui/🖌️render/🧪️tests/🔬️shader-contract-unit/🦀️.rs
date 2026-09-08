
use super::*;

//#region 🔖️OwnedWgslValidation
fn wgsl_tokens(source: &str) -> Result<Vec<String>, String> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index].is_ascii_whitespace() {
            index += 1;
        } else if bytes[index..].starts_with(b"//") {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
        } else if bytes[index..].starts_with(b"/*") {
            index += 2;
            let mut depth = 1usize;
            while index < bytes.len() && depth > 0 {
                if bytes[index..].starts_with(b"/*") {
                    depth += 1;
                    index += 2;
                } else if bytes[index..].starts_with(b"*/") {
                    depth -= 1;
                    index += 2;
                } else {
                    index += 1;
                }
            }
            if depth != 0 {
                return Err("unterminated block comment".into());
            }
        } else if bytes[index].is_ascii_alphabetic() || bytes[index] == b'_' {
            let start = index;
            index += 1;
            while index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') {
                index += 1;
            }
            tokens.push(source[start..index].to_string());
        } else {
            tokens.push((bytes[index] as char).to_string());
            index += 1;
        }
    }
    Ok(tokens)
}

fn validate_wgsl_structure(source: &str) -> Result<Vec<String>, String> {
    let tokens = wgsl_tokens(source)?;
    let mut delimiters = Vec::new();
    let mut entry_points = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        match token.as_str() {
            "(" | "[" | "{" => delimiters.push(token.as_str()),
            ")" | "]" | "}" => {
                let expected = match token.as_str() {
                    ")" => "(",
                    "]" => "[",
                    _ => "{",
                };
                if delimiters.pop() != Some(expected) {
                    return Err(format!("unbalanced delimiter {token} at token {index}"));
                }
            }
            "async" if tokens.get(index + 1).is_some_and(|next| next == "fn") => return Err("WGSL does not permit async functions".into()),
            "fn" => {
                let name = tokens.get(index + 1).filter(|name| name.bytes().next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')).ok_or_else(|| format!("function at token {index} has no identifier"))?;
                let attribute_start = tokens[..index].iter().rposition(|candidate| matches!(candidate.as_str(), ";" | "{" | "}" | "fn")).map_or(0, |position| position + 1);
                for pair in tokens[attribute_start..index].windows(2) {
                    if pair[0] == "@" && matches!(pair[1].as_str(), "vertex" | "fragment" | "compute") {
                        entry_points.push(name.clone());
                    }
                }
            }
            _ => {}
        }
    }
    if let Some(delimiter) = delimiters.last() {
        return Err(format!("unclosed delimiter {delimiter}"));
    }
    if !tokens.iter().any(|token| token == "fn") {
        return Err("shader declares no function".into());
    }
    entry_points.sort();
    entry_points.dedup();
    Ok(entry_points)
}

fn assert_wgsl_valid(label: &str, source: &str) -> Vec<String> {
    validate_wgsl_structure(source).unwrap_or_else(|error| panic!("{label}: owned WGSL structural validation failed — {error}"))
}
//#endregion 🔖️OwnedWgslValidation

#[test]
fn all_canonical_shaders_pass_owned_structural_validation() {
    for family in ALL_SHADERS {
        for variant in family.variants {
            assert_wgsl_valid(variant.name, variant.wgsl);
        }
    }
}

#[test]
fn pipeline_entry_points_exist_in_shader() {
    for family in ALL_SHADERS {
        for variant in family.variants {
            let entry_names = assert_wgsl_valid(variant.name, variant.wgsl);
            for pipeline in variant.pipelines {
                assert!(
                    entry_names.iter().any(|entry| entry == pipeline.vertex_entry),
                    "{}/{}: PipelineSpec '{}' names vertex entry {:?} but the WGSL only declares {:?}",
                    family.name,
                    variant.name,
                    pipeline.label,
                    pipeline.vertex_entry,
                    entry_names
                );
                assert!(
                    entry_names.iter().any(|entry| entry == pipeline.fragment_entry),
                    "{}/{}: PipelineSpec '{}' names fragment entry {:?} but the WGSL only declares {:?}",
                    family.name,
                    variant.name,
                    pipeline.label,
                    pipeline.fragment_entry,
                    entry_names
                );
            }
        }
    }
}

#[test]
fn owned_validator_rejects_async_and_unbalanced_wgsl() {
    assert!(validate_wgsl_structure("@vertex async fn vs_main() {}").is_err());
    assert!(validate_wgsl_structure("@fragment fn fs_main() -> @location(0) vec4<f32> {").is_err());
}

#[test]
fn vertex_attribute_offsets_fit_declared_stride() {
    for family in ALL_SHADERS {
        for variant in family.variants {
            for pipeline in variant.pipelines {
                for buffer in pipeline.vertex_buffers {
                    for attribute in buffer.attributes {
                        let end = attribute.offset + attribute.format.byte_size();
                        assert!(end <= buffer.stride, "{}/{}: PipelineSpec '{}' attribute at location {} ends at byte {} but the buffer stride is only {}", family.name, variant.name, pipeline.label, attribute.shader_location, end, buffer.stride);
                    }
                }
            }
        }
    }
}
