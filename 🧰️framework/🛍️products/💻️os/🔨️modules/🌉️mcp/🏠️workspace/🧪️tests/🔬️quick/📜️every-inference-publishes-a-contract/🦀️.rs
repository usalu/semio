/// 📜️ Every inference a committed descriptor declares publishes a payload contract with parseable schemas: a service nobody wrote a contract for cannot be run by an agent, so it fails here by name.
#[test]
fn every_declared_inference_publishes_a_payload_contract() {
    let Ok(repo_root) = find_repo_root() else {
        eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");
        return;
    };
    let Ok(registry) = load_plugin_registry(&repo_root) else {
        eprintln!("skipped: plugin registry not generated");
        return;
    };
    let (mut checked, mut missing) = (0usize, Vec::new());
    for entry in &registry {
        let Ok(descriptor) = load_package_descriptor(&entry.owner_root) else { continue };
        let rows = descriptor.contributions.inference_services.iter().chain(descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()));
        for row in rows {
            checked += 1;
            match row.payload.as_ref() {
                None => missing.push(format!("{}/{}", row.artifact_kind, row.inference_schema)),
                Some(contract) => {
                    assert!(serde_json::from_str::<serde_json::Value>(&contract.input_schema).is_ok(), "{} publishes an unparseable input schema", row.inference_schema);
                    assert!(serde_json::from_str::<serde_json::Value>(&contract.output_schema).is_ok(), "{} publishes an unparseable output schema", row.inference_schema);
                }
            }
        }
    }
    assert!(missing.is_empty(), "{} of {checked} declared inferences publish no payload contract: {missing:?}", missing.len());
}

/// 🧱️ A pack-bytes binding writes the named artifact's pack as the whole request body, and refuses to run without one.
#[test]
fn a_pack_bytes_binding_writes_the_artifact_pack_as_the_whole_body() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BYTES);
    let command = bound_inference_command(Some(ArtifactDocumentBinding { pack: b"PACK-BYTES".to_vec(), spr: b"SPR".to_vec() }), b"{}");
    assert_eq!(bind_inference_document(&declared, &command).expect("a named artifact binds"), b"PACK-BYTES".to_vec());
    let fault = bind_inference_document(&declared, &bound_inference_command(None, b"{}")).expect_err("no artifact named");
    assert!(fault.message.contains("artifactId"), "{}", fault.message);
}
