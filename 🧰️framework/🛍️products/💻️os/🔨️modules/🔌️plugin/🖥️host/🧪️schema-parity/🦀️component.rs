//! 🧪️ Owned source-level parity checks for the single plugin WIT world.

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs;
    use std::path::PathBuf;

    //#region 🧬️OwnedWitInspection

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Field {
        name: String,
        ty: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Function {
        name: String,
        is_async: bool,
        params: Vec<Field>,
        result: Option<String>,
    }

    fn schema_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../🧬️schema/📜️component.wit")
    }

    fn source() -> String {
        let source = fs::read_to_string(schema_path()).expect("plugin WIT schema must be readable");
        source.lines().map(|line| line.split_once("//").map_or(line, |(code, _)| code)).collect::<Vec<_>>().join("\n")
    }

    fn normalize(value: &str) -> String {
        value.chars().filter(|character| !character.is_whitespace()).collect()
    }

    fn matching_close(source: &str, open: usize, opening: u8, closing: u8) -> usize {
        let mut depth = 0usize;
        for (offset, byte) in source.as_bytes()[open..].iter().enumerate() {
            if *byte == opening {
                depth += 1;
            } else if *byte == closing {
                depth -= 1;
                if depth == 0 {
                    return open + offset;
                }
            }
        }
        panic!("unclosed WIT delimiter at byte {open}")
    }

    fn named_block<'a>(source: &'a str, kind: &str, name: &str) -> &'a str {
        let marker = format!("{kind} {name}");
        let start = source.find(&marker).unwrap_or_else(|| panic!("missing WIT {kind} `{name}`"));
        let open = source[start + marker.len()..].find('{').map(|offset| start + marker.len() + offset).unwrap_or_else(|| panic!("WIT {kind} `{name}` has no body"));
        let close = matching_close(source, open, b'{', b'}');
        &source[open + 1..close]
    }

    fn split_top_level(source: &str, delimiter: char) -> Vec<&str> {
        let mut result = Vec::new();
        let mut start = 0usize;
        let mut depth = 0usize;
        for (index, character) in source.char_indices() {
            match character {
                '<' | '(' | '{' | '[' => depth += 1,
                '>' | ')' | '}' | ']' => depth = depth.saturating_sub(1),
                _ => {}
            }
            if character == delimiter && depth == 0 {
                result.push(source[start..index].trim());
                start = index + character.len_utf8();
            }
        }
        let tail = source[start..].trim();
        if !tail.is_empty() {
            result.push(tail);
        }
        result
    }

    fn fields(source: &str) -> Vec<Field> {
        split_top_level(source, ',')
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .map(|entry| {
                let (name, ty) = entry.split_once(':').unwrap_or_else(|| panic!("invalid WIT field `{entry}`"));
                Field { name: name.trim().to_string(), ty: normalize(ty) }
            })
            .collect()
    }

    fn record_fields(interface: &str, name: &str) -> Vec<Field> {
        fields(named_block(interface, "record", name))
    }

    fn variant_cases(interface: &str, name: &str) -> Vec<(String, Option<String>)> {
        split_top_level(named_block(interface, "variant", name), ',')
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .map(|entry| {
                if let Some(open) = entry.find('(') {
                    let close = matching_close(entry, open, b'(', b')');
                    (entry[..open].trim().to_string(), Some(normalize(&entry[open + 1..close])))
                } else {
                    (entry.trim().to_string(), None)
                }
            })
            .collect()
    }

    fn parse_function(line: &str) -> Function {
        let (name, signature) = line.split_once(':').unwrap_or_else(|| panic!("invalid WIT function `{line}`"));
        let signature = signature.trim();
        let is_async = signature.starts_with("async func(");
        let marker = if is_async { "async func(" } else { "func(" };
        let open = signature.find('(').unwrap_or_else(|| panic!("function `{name}` has no parameter list"));
        assert!(signature.starts_with(marker), "unsupported WIT function shape `{line}`");
        let close = matching_close(signature, open, b'(', b')');
        let remainder = signature[close + 1..].trim().trim_end_matches(';').trim();
        Function { name: name.trim().to_string(), is_async, params: fields(&signature[open + 1..close]), result: remainder.strip_prefix("->").map(normalize) }
    }

    fn functions(interface: &str) -> BTreeMap<String, Function> {
        interface.lines().map(str::trim).filter(|line| line.contains(": func(") || line.contains(": async func(")).map(parse_function).map(|function| (function.name.clone(), function)).collect()
    }

    fn declarations(source: &str, keyword: &str) -> BTreeSet<String> {
        source.lines().filter_map(|line| line.trim().strip_prefix(keyword)).filter_map(|tail| tail.split_whitespace().next()).map(|name| name.trim_end_matches('{').to_string()).collect()
    }

    fn world_members(world: &str, keyword: &str) -> BTreeSet<String> {
        world.lines().filter_map(|line| line.trim().strip_prefix(keyword)).map(|name| name.trim().trim_end_matches(';').to_string()).collect()
    }

    //#endregion 🧬️OwnedWitInspection

    //#region 🧪️ContractParity

    #[test]
    fn every_req_bearing_effect_has_a_matching_host_async_import() {
        let source = source();
        let effects = named_block(&source, "interface", "effects");
        let host = functions(named_block(&source, "interface", "host-async"));
        let mut checked = BTreeSet::new();
        for (case, payload) in variant_cases(effects, "effect") {
            let Some(payload) = payload else { continue };
            let payload_fields = record_fields(effects, &payload);
            if !payload_fields.iter().any(|field| field.name == "req") {
                continue;
            }
            if case == "respond" {
                assert!(!host.contains_key("respond"));
                continue;
            }
            let async_name = if case == "http-request" { "http-fetch" } else { &case };
            let function = host.get(async_name).unwrap_or_else(|| panic!("missing host-async import `{async_name}` for effect `{case}`"));
            assert!(function.is_async, "host-async.{async_name} must be async");
            let expected = payload_fields.into_iter().filter(|field| field.name != "req").collect::<Vec<_>>();
            assert_eq!(function.params, expected, "host-async.{async_name} must reuse `{case}` payload fields");
            checked.insert(case);
        }
        let expected = [
            "storage-read",
            "storage-write",
            "storage-delete",
            "blob-load",
            "blob-write",
            "http-request",
            "document-read",
            "document-write",
            "link-resolve",
            "registry-query",
            "io-compose",
            "io-run",
            "cache-derive",
            "cache-read",
            "invoke-extension",
            "open-window",
            "open-dialog",
            "dispatch-action",
            "spawn-plugin-instance",
            "request-file-open",
            "request-media-frames",
            "request-capability",
        ]
        .into_iter()
        .map(String::from)
        .collect::<BTreeSet<_>>();
        assert_eq!(checked, expected);
    }

    #[test]
    fn spawn_job_reuses_the_effect_payload() {
        let source = source();
        let effects = named_block(&source, "interface", "effects");
        let payload = variant_cases(effects, "effect").into_iter().find_map(|(case, payload)| (case == "spawn-job").then_some(payload).flatten()).expect("spawn-job effect must carry a payload");
        let expected = record_fields(effects, &payload);
        assert!(!expected.iter().any(|field| field.name == "req"));
        let host = functions(named_block(&source, "interface", "host-async"));
        let function = host.get("spawn-job").expect("host-async.spawn-job must exist");
        assert!(function.is_async);
        assert_eq!(function.params, expected);
    }

    #[test]
    fn emit_carries_the_effect_variant() {
        let source = source();
        let host = functions(named_block(&source, "interface", "host-async"));
        let emit = host.get("emit").expect("host-async.emit must exist");
        assert!(!emit.is_async);
        assert_eq!(emit.params, vec![Field { name: "value".into(), ty: "effect".into() }]);
        assert!(emit.result.is_none());
        assert!(host.contains_key("emit-patch"));
    }

    #[test]
    fn exactly_one_world_exists() {
        let source = source();
        assert_eq!(declarations(&source, "world "), BTreeSet::from(["actor".to_string()]));
        assert!(!declarations(&source, "interface ").contains("runner"));
    }

    #[test]
    fn actor_world_has_the_exact_explicit_boundary() {
        let source = source();
        let actor = named_block(&source, "world", "actor");
        assert_eq!(world_members(actor, "import "), BTreeSet::from(["host-async".to_string(), "pure".to_string()]));
        assert_eq!(world_members(actor, "export "), ["checkpoint", "describe", "jobs", "reactor"].into_iter().map(String::from).collect());
    }

    #[test]
    fn every_actor_export_is_async() {
        let source = source();
        let mut seen = BTreeSet::new();
        for interface_name in ["reactor", "jobs", "checkpoint", "describe"] {
            for function in functions(named_block(&source, "interface", interface_name)).into_values() {
                assert!(function.is_async, "{interface_name}.{} must be async", function.name);
                seen.insert(format!("{interface_name}.{}", function.name));
            }
        }
        let expected = ["reactor.poll", "jobs.start-job", "jobs.step-job", "jobs.cancel-job", "jobs.take-segmented-download-chunk", "checkpoint.checkpoint", "checkpoint.restore", "describe.describe"].into_iter().map(String::from).collect();
        assert_eq!(seen, expected);
        let pure = functions(named_block(&source, "interface", "pure"));
        for name in ["log", "now-ms", "trace-span"] {
            assert!(!pure.get(name).unwrap_or_else(|| panic!("pure.{name} must exist")).is_async);
        }
    }

    #[test]
    fn every_fallible_host_import_returns_result() {
        let source = source();
        let host = functions(named_block(&source, "interface", "host-async"));
        let mut checked = 0usize;
        for function in host.values() {
            if matches!(function.name.as_str(), "emit" | "emit-patch") {
                assert!(!function.is_async);
                assert!(function.result.is_none());
                continue;
            }
            assert!(function.is_async, "host-async.{} must be async", function.name);
            assert!(function.result.as_deref().is_some_and(|result| result.starts_with("result<")));
            checked += 1;
        }
        assert_eq!(checked, 24);
    }

    //#endregion 🧪️ContractParity
}
