/// 🧪️ Laws of owned test guests: served codec rows, guest tables and the per-package load progress.

/// 🎲️ What [`owned_test_guest`]'s `pack-schema-hash` does: answer a hash, or loop until its interpretation is ended.
#[derive(Clone, Copy)]
enum OwnedTestHash {
    Answers([u8; 32]),
    Spins,
}

/// 🧩️ A real owned-ABI guest, one memory page and every owned export: `channel-version` answers the host's app channel
/// (the owned codec origin admits it first), `pack-schema-hash` does what `hash` says, `genesis` answers the pair
/// `([1, 2, 3], [4, 5])`, every other export answers nothing. Built byte by byte (the
/// interpreter's own component framing), so a law runs the production interpreter without a plugin build.
fn owned_test_guest(hash: OwnedTestHash) -> Vec<u8> {
    fn uleb(mut value: u64, output: &mut Vec<u8>) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            output.push(if value == 0 { byte } else { byte | 0x80 });
            if value == 0 {
                return;
            }
        }
    }
    fn sleb(mut value: i64, output: &mut Vec<u8>) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            let done = (value == 0 && byte & 0x40 == 0) || (value == -1 && byte & 0x40 != 0);
            output.push(if done { byte } else { byte | 0x80 });
            if done {
                return;
            }
        }
    }
    fn section(id: u8, payload: Vec<u8>, output: &mut Vec<u8>) {
        output.push(id);
        uleb(payload.len() as u64, output);
        output.extend(payload);
    }
    let answer = match hash {
        OwnedTestHash::Answers(answer) => answer,
        OwnedTestHash::Spins => [0; 32],
    };
    let hash_output = format!("{{\"Ok\":{}}}", serde_json::to_string(&answer.to_vec()).unwrap()).into_bytes();
    let genesis_output = br#"{"Ok":{"pack":[1,2,3],"spr":[4,5]}}"#.to_vec();
    let channel_output = directory::os_spr::CHANNEL_VERSION.to_string().into_bytes();
    let returning = |offset: i64, output: &[u8]| {
        let mut body = vec![0x00, 0x42];
        sleb(((output.len() as i64) << 32) | offset, &mut body);
        body.push(0x0b);
        body
    };
    let exports = semio_framework_plugin_interpreter::OwnedSemioExport::ALL;
    let (types, bodies): (Vec<u8>, Vec<Vec<u8>>) = exports
        .iter()
        .map(|export| match export {
            semio_framework_plugin_interpreter::OwnedSemioExport::Allocate => (0u8, vec![0x00, 0x41, 0x00, 0x0b]),
            semio_framework_plugin_interpreter::OwnedSemioExport::Deallocate => (1, vec![0x00, 0x0b]),
            semio_framework_plugin_interpreter::OwnedSemioExport::Checkpoint | semio_framework_plugin_interpreter::OwnedSemioExport::Describe => (2, vec![0x00, 0x42, 0x00, 0x0b]),
            semio_framework_plugin_interpreter::OwnedSemioExport::ChannelVersion => (2, returning(3072, &channel_output)),
            semio_framework_plugin_interpreter::OwnedSemioExport::PackSchemaHash => match hash {
                OwnedTestHash::Answers(_) => (3, returning(1024, &hash_output)),
                OwnedTestHash::Spins => (3, vec![0x00, 0x03, 0x40, 0x0c, 0x00, 0x0b, 0x42, 0x00, 0x0b]),
            },
            semio_framework_plugin_interpreter::OwnedSemioExport::Genesis => (3, returning(2048, &genesis_output)),
            _ => (3, vec![0x00, 0x42, 0x00, 0x0b]),
        })
        .unzip();
    let mut core = b"\0asm\x01\0\0\0".to_vec();
    section(1, vec![4, 0x60, 1, 0x7f, 1, 0x7f, 0x60, 2, 0x7f, 0x7f, 0, 0x60, 0, 1, 0x7e, 0x60, 2, 0x7f, 0x7f, 1, 0x7e], &mut core);
    let mut functions = Vec::new();
    uleb(types.len() as u64, &mut functions);
    functions.extend(&types);
    section(3, functions, &mut core);
    section(5, vec![1, 0x00, 1], &mut core);
    let mut export_section = Vec::new();
    uleb(exports.len() as u64 + 1, &mut export_section);
    export_section.push(6);
    export_section.extend(b"memory");
    export_section.extend([2, 0]);
    for (index, export) in exports.iter().enumerate() {
        uleb(export.core_name().len() as u64, &mut export_section);
        export_section.extend(export.core_name().as_bytes());
        export_section.push(0);
        uleb(index as u64, &mut export_section);
    }
    section(7, export_section, &mut core);
    let mut code = Vec::new();
    uleb(bodies.len() as u64, &mut code);
    for body in &bodies {
        uleb(body.len() as u64, &mut code);
        code.extend(body);
    }
    section(10, code, &mut core);
    let mut data = vec![3];
    for (offset, output) in [(1024i64, &hash_output), (2048, &genesis_output), (3072, &channel_output)] {
        data.extend([0x00, 0x41]);
        sleb(offset, &mut data);
        data.push(0x0b);
        uleb(output.len() as u64, &mut data);
        data.extend(output.iter());
    }
    section(11, data, &mut core);
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    section(1, core, &mut component);
    component
}

/// 🧩️ Replaces the fixture editor package's component with [`owned_test_guest`] hashing as `hash` says, leaving its
/// one codec row (`0x11 × 32`) to be pinned against the guest: no provider binds it.
fn install_owned_guest(fixture: &mut FixtureDirectory, hash: OwnedTestHash) {
    let bytes = owned_test_guest(hash);
    std::fs::write(fixture.component_path(0), &bytes).expect("owned guest component");
    let sha256 = hex_lower(&Sha256::digest(&bytes));
    let mut blake3 = Hasher::new();
    blake3.update(&bytes);
    let component = &mut fixture.bundle["packages"][0]["component"];
    component["byteLength"] = bytes.len().into();
    component["sha256"] = sha256.clone().into();
    component["blake3"] = hex_lower(blake3.finalize().as_bytes()).into();
    fixture.bundle["packages"][0]["browserActor"]["sourceComponentSha256"] = sha256.into();
    let schema = fixture.schema.clone();
    fixture.rewrite_descriptor(0, Some(&schema), Some(("fixture-base", "1.0.0")));
}

/// 🔐️ LAW (coordinator, session 13): no codec call is ever served from a row its component has not answered as the trust
/// record says — not before the background pass reached it (the first call verifies it itself), not after the guest was
/// released and compiled again (zero residency budget: every operation compiles, the row is verified once), and not from
/// component bytes that changed after the verification. A guest that answers differently refuses every call of its
/// package. Runs the production interpreter on a real owned-ABI guest ([`owned_test_guest`]).
#[tokio::test]
async fn no_codec_call_is_served_from_a_row_its_component_has_not_answered() {
    let control = TestControl::new();
    let answered = |fixture: &FixtureDirectory, catalog: &VerifiedTrustedCatalog| catalog.codecs.iter().position(|codec| codec.identity.artifact_schema == fixture.schema).expect("the fixture row's codec");
    let mut fixture = prepared_fixture();
    install_owned_guest(&mut fixture, OwnedTestHash::Answers([0x11; 32]));
    let catalog = load_fixture(&fixture, &[], &control.context()).await.expect("a catalog whose row waits for its guest");
    catalog.configure_guest_residency(TrustedCatalogGuestResidencyV1 { resident_component_bytes: 0, ..TRUSTED_CATALOG_GUEST_RESIDENCY });
    let owner = &catalog.codecs[answered(&fixture, &catalog)];
    let position = owner.guest.component.position;
    assert_eq!(catalog.load_progress().packages[position].phase, TrustedCatalogPackagePhaseV1::Staged);
    let first = owner.guest.genesis("document-1", &control.context()).await.expect("the row answers as recorded, then the call is served");
    assert_eq!((first.pack.clone(), first.spr.clone()), (vec![1, 2, 3], vec![4, 5]));
    let progress = catalog.load_progress();
    assert_eq!((progress.packages[position].phase, progress.rows_verified), (TrustedCatalogPackagePhaseV1::Ready, 1));
    assert_eq!(catalog.guest_residency().resident_guests, 0, "a zero budget released the guest with its operation");
    let again = owner.guest.genesis("document-1", &control.context()).await.expect("a released guest compiles again and is served");
    assert_eq!((again.pack, again.spr), (first.pack, first.spr), "the reloaded guest answers the same");
    let residency = catalog.guest_residency();
    assert_eq!((residency.compiles, residency.admitted), (2, 0), "the row was verified once; the second operation compiled again");
    assert_eq!(catalog.load_progress().rows_verified, 1);
    std::fs::write(fixture.component_path(0), owned_test_guest(OwnedTestHash::Answers([0x11; 32])).iter().rev().copied().collect::<Vec<u8>>()).expect("tamper the verified component");
    assert!(owner.guest.genesis("document-1", &control.context()).await.is_err(), "bytes that changed after the verification are never compiled, so never served");

    let mut lying = prepared_fixture();
    install_owned_guest(&mut lying, OwnedTestHash::Answers([0x22; 32]));
    let refused = load_fixture(&lying, &[], &control.context()).await.expect("a catalog whose guest will answer differently");
    let liar = &refused.codecs[answered(&lying, &refused)];
    let error = liar.guest.genesis("document-1", &control.context()).await.expect_err("a row answered differently is never served");
    assert!(error.to_string().contains("differs from its trust record"), "{error}");
    refused.verify_pending_guests(&control.context()).await.expect("the background pass records the refusal and moves on");
    assert_eq!(refused.load_progress().packages_refused, 1);
    assert!(liar.guest.genesis("document-2", &control.context()).await.is_err(), "the refusal is permanent for the catalog");

    let mut background = prepared_fixture();
    install_owned_guest(&mut background, OwnedTestHash::Answers([0x11; 32]));
    let verified = load_fixture(&background, &[], &control.context()).await.expect("a catalog verified in the background");
    verified.verify_pending_guests(&control.context()).await.expect("background pass");
    let progress = verified.load_progress();
    assert_eq!((progress.packages_ready, progress.packages_total, progress.rows_verified), (progress.packages_total, 2, 1));
    let served = &verified.codecs[answered(&background, &verified)];
    assert!(served.guest.genesis("document-1", &control.context()).await.is_ok());
}

/// 🧬️ LAW (ticket 26/09/23, H12): a guest `codec` call answers through its owner's codec table and constructs no app,
/// and those answers equal the independently linked native codecs of the plugins this hub links an assembly of — gis
/// and vcs (puzzle's own law covers a third): the same pack-schema hash, the NATIVE codec decodes the table's genesis
/// pair, and both print the same op log of it (the `dsl` renderings differ by design: document text vs the kind's DSL). A schema the bundle declares without owning is
/// refused as unowned. Every plugin contributes at least one owned comparison.
#[tokio::test]
async fn guest_codec_tables_answer_like_the_linked_native_codecs() {
    macro_rules! compare_plugin {
        ($plugin:expr, $codecs:expr) => {{
            let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).expect("explicit test mounted owner policy");
            semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, $plugin.expect("plugin assembles"));
            let document_id = format!("artifact-{}", "2".repeat(32));
            let (mut owned, mut unowned) = (Vec::new(), 0usize);
            for codec in $codecs {
                match semio_framework_plugin::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, &codec.schema).await {
                    Err(fault) => {
                        assert!(format!("{fault:?}").contains("owned by no app"), "{}: {fault:?}", codec.schema);
                        unowned += 1;
                    }
                    Ok(hash) => {
                        assert_eq!(hash, codec.pack_schema_hash, "{}: pack-schema hash", codec.schema);
                        let pair = semio_framework_plugin::plugin_runtime::plugin_artifact_genesis(&runtime, &codec.schema, &document_id).await.unwrap_or_else(|fault| panic!("{}: genesis {fault:?}", codec.schema));
                        let guest = semio_framework_plugin::plugin_runtime::plugin_artifact_print_mirror(&runtime, &codec.schema, &pair.pack, &pair.spr).await.unwrap_or_else(|fault| panic!("{}: guest mirror {fault:?}", codec.schema));
                        let native = (codec.print_mirror)(&pair.pack, &pair.spr).await.unwrap_or_else(|error| panic!("{}: native mirror {error:?}", codec.schema));
                        assert_eq!(guest.ops, native.ops, "{}: op-log mirror", codec.schema);
                        assert!(!guest.dsl.is_empty() && !native.dsl.is_empty(), "{}: both render the document", codec.schema);
                        owned.push(codec.schema.clone());
                    }
                }
            }
            (owned, unowned)
        }};
    }
    let gis = compare_plugin!(semio_hub_gis::plugin(), semio_hub_gis::native_codecs::native_codec_factory_receipts().expect("gis receipts").into_iter().map(|receipt| receipt.into_codec().expect("gis codec")));
    let vcs = compare_plugin!(semio_hub_vcs::plugin(), semio_hub_vcs::native_codecs::native_codec_factory_receipts().expect("vcs receipts").into_iter().map(|receipt| receipt.into_codec().expect("vcs codec")));
    for (plugin, (owned, unowned)) in [("gis", &gis), ("vcs", &vcs)] {
        assert!(!owned.is_empty(), "{plugin}: at least one owned kind compared ({unowned} unowned)");
    }
}

/// 📈️ A catalog load reports every selected package into the host's progress cell — phase, component bytes and
/// codec rows — and the counts add up: a load whose rows a linked codec pins ends with every package ready,
/// every component byte read and no row interpreted; the body is exactly `TrustedCatalogLoadProgressV1`.
#[tokio::test]
async fn a_catalog_load_reports_every_package_phase_bytes_and_rows_into_the_hosts_cell() {
    struct ObservedControl(Arc<TrustedCatalogLoadProgressCellV1>);
    impl AuthorityOperationControl for ObservedControl {
        fn now_ms(&self) -> u64 {
            0
        }

        fn is_cancelled(&self) -> bool {
            false
        }

        fn report(&self, _progress: AuthorityProgress) {}
    }
    let fixture = prepared_fixture();
    let control = ObservedControl(Arc::default());
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
    let catalog = TrustedCatalogLoader::load_fixture_observed(&fixture.bundle_path, "fixture", &FixtureProviderSource::new(vec![fixture.binding()]), &control.0, &context).await.expect("fixture catalog");
    let observed = control.0.snapshot();
    assert_eq!(observed, catalog.load_progress(), "the catalog keeps reporting into the host's cell");
    assert!(observed.packages_total >= 1 && observed.packages.len() as u64 == observed.packages_total);
    assert_eq!(observed.packages_ready, observed.packages_total, "{observed:?}");
    assert_eq!((observed.packages_refused, observed.rows_verified), (0, 0));
    assert_eq!(observed.component_bytes_read, observed.component_bytes_total);
    assert_eq!(observed.rows_pinned, observed.rows_total);
    assert_eq!(observed.rows_total, observed.packages.iter().map(|package| package.rows).sum::<u64>());
    catalog.verify_pending_guests(&context).await.expect("nothing is pending");
    let module: serde_json::Value = serde_json::from_str(semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_SCHEMA_JSON).unwrap();
    let fields = |def: &str| module["$defs"][def]["required"].as_array().unwrap().iter().map(|field| field.as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    let body = serde_json::to_value(&observed).unwrap();
    assert_eq!(body.as_object().unwrap().keys().cloned().collect::<BTreeSet<_>>(), fields("TrustedCatalogLoadProgressV1"));
    assert_eq!(body["packages"][0].as_object().unwrap().keys().cloned().collect::<BTreeSet<_>>(), fields("TrustedCatalogPackageProgressV1"));
    let phases: Vec<String> = module["$defs"]["TrustedCatalogPackagePhaseV1"]["enum"].as_array().unwrap().iter().map(|phase| phase.as_str().unwrap().to_string()).collect();
    let spelled: Vec<String> = [TrustedCatalogPackagePhaseV1::Pending, TrustedCatalogPackagePhaseV1::Reading, TrustedCatalogPackagePhaseV1::Staged, TrustedCatalogPackagePhaseV1::Verifying, TrustedCatalogPackagePhaseV1::Ready, TrustedCatalogPackagePhaseV1::Refused]
        .iter()
        .map(|phase| serde_json::to_value(phase).unwrap().as_str().unwrap().to_string())
        .collect();
    assert_eq!(spelled, phases);
}

#[tokio::test]
async fn descriptor_owned_surface_is_required_before_any_catalog_or_codec_publication() {
    for (field, value) in [
        ("artifactKind", serde_json::json!("fixture.document")),
        ("surfaceId", serde_json::json!("s.fixture.document@1/*#foreign")),
        ("appId", serde_json::json!("s.fixture.document@1/*#foreign")),
        ("windowKindId", serde_json::json!("foreign.window")),
        ("role", serde_json::json!("viewer")),
        ("rendererTarget", serde_json::json!("wgpu")),
    ] {
        let mut fixture = prepared_fixture();
        fixture.bundle["packages"][0]["openTargets"][0][field] = value.clone();
        fixture.bundle["profiles"][0]["openTargets"][0]["target"][field] = value;
        fixture.persist_bundle();
        let error = expect_load_error(&fixture, &[fixture.binding()], &TestControl::new()).await;
        assert!(error.to_string().contains("document-open target"), "{field}: {error}");
        assert!(document_codec(&fixture.schema).await.expect("codec registry").is_none(), "{field} published a codec");
    }
}
