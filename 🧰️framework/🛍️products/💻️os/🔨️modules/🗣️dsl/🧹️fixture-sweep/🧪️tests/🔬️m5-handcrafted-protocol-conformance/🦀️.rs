
use super::m5_auto_discovery::{self, ConformanceFacet, ProtocolFacetKind};
use super::m5_soft_skip::{soft_skip_empty_bytes, soft_skip_missing};
use super::pilot_resolve;
use crate::os_dsl::{parse_protocol, verify_protocol_source, walk_protocol};
use crate::os_store::semio_format::unwrap_binary;

async fn inner_payload_from_semio_example(bytes: &[u8], label: &str) -> Option<Vec<u8>> {
    match unwrap_binary(bytes) {
        Ok((_, inner)) => Some(inner.to_vec()),
        Err(error) => {
            eprintln!("[DEBUG] soft-skip {label}: unwrap failed: {error}");
            None
        }
    }
}

/// @emoji ✅️ Real check, no panics — lets the caller choose hard-assert vs. soft-log per facet.
async fn check_protocol_conformance(protocol_semio: &str, bytes: &[u8]) -> Result<(), String> {
    verify_protocol_source(protocol_semio, bytes)?;
    let spec = parse_protocol(protocol_semio).map_err(|error| format!("parse_protocol: {error:?}"))?;
    walk_protocol(&spec, bytes).map(|_| ()).map_err(|error| format!("walk_protocol @{}: {}", error.offset, error.message))
}

#[semio_framework_async_macros::async_test]
async fn all_discovered_snapshot_protocols_walk_their_shipped_fixtures() {
    let facets = m5_auto_discovery::discover_protocol_facets().await;
    assert!(!facets.is_empty(), "auto-discovery found zero 🧬️schema/{{📸️snapshot,🧬️mutations}}/💾️binary/📡️.protocol.semio files under ✏️s/🔌️plugins — discovery walk is broken");

    let mut hard_failures: Vec<String> = Vec::new();
    let mut soft_failures: Vec<String> = Vec::new();
    let mut checked = 0usize;
    let mut soft_skipped = 0usize;

    for facet in &facets {
        let protocol_text = std::fs::read_to_string(&facet.file_path).unwrap_or_else(|error| panic!("{}: read {}: {error}", facet.label, facet.file_path.display()));
        if soft_skip_missing(&format!("{}.protocol", facet.label), &protocol_text).await {
            soft_skipped += 1;
            continue;
        }
        let kind_suffix = match facet.kind {
            ProtocolFacetKind::Pack => ".pack.semio",
            ProtocolFacetKind::Spr => ".spr.semio",
        };
        let Some(example_bytes) = pilot_resolve::read_example_bytes(&facet.artifact_rel, facet.standard.as_deref(), kind_suffix).await else {
            eprintln!("[DEBUG] soft-skip {}: no {kind_suffix} under 📚️examples (🖼️assets-first walk)", facet.label);
            soft_skipped += 1;
            continue;
        };
        let Some(bytes) = inner_payload_from_semio_example(&example_bytes, &facet.label).await else {
            soft_skipped += 1;
            continue;
        };
        if soft_skip_empty_bytes(&facet.label, &bytes).await {
            soft_skipped += 1;
            continue;
        }
        checked += 1;
        let conformance_facet = match facet.kind {
            ProtocolFacetKind::Pack => ConformanceFacet::ProtocolPack,
            ProtocolFacetKind::Spr => ConformanceFacet::ProtocolSpr,
        };
        if let Err(detail) = check_protocol_conformance(&protocol_text, &bytes).await {
            let stdio_exempt = facet.is_stdio && m5_auto_discovery::stdio_is_exempt(conformance_facet, &facet.artifact, facet.standard.as_deref()).await;
            let known_gap = !facet.is_stdio && m5_auto_discovery::non_stdio_is_known_gap(conformance_facet, &facet.plugin, &facet.artifact, facet.standard.as_deref()).await;
            if stdio_exempt || known_gap {
                eprintln!("[DEBUG] soft (stdio-exempt or known pre-existing gap) protocol conformance failure for {}: {detail}", facet.label);
                soft_failures.push(facet.label.clone());
            } else {
                hard_failures.push(format!("{}: {detail}", facet.label));
            }
        }
    }

    eprintln!(
        "[dsl-fixture-sweep] m5 protocol auto-discovery: {} facet(s) found, {} checked, {} soft-skipped, {} stdio-exempt-or-known-gap soft failure(s), {} hard failure(s)",
        facets.len(),
        checked,
        soft_skipped,
        soft_failures.len(),
        hard_failures.len()
    );
    assert!(hard_failures.is_empty(), "m5 protocol conformance failed for {} artifact(s):\n\n{}", hard_failures.len(), hard_failures.join("\n\n"));
}
