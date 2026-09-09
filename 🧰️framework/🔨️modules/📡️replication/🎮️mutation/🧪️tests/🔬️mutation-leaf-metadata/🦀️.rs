
use super::*;

const LEAF_DESCRIPTOR: MutationLeafDescriptor = MutationLeafDescriptor {
    schema_version: 1,
    owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page",
    semantic_kind: "insert-page",
    display_name: "Insert Page",
    emoji: "➕️",
    aggregate_variant: "InsertPage",
    payload_schema: "🦀️.rs#InsertPage",
    text_opcode: None,
    binary_tag: None,
    invertibility: MutationInvertibility::ExplicitMutation,
    diff_participation: MutationDiffParticipation::ApplyOnly,
    outcome_classes: &[MutationOutcomeClass::Applied],
    composition: MutationComposition::Atomic,
    required_language_surfaces: &[MutationLanguageSurface::Rust],
};
const LEAF_PROVENANCE: MutationSourceProvenance = MutationSourceProvenance {
    workspace_token: [0x2a; 32],
    mutation_root: "✏️s/🔌️plugins/🧪️probe/🧬️mutations",
    owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page",
    source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🦀️.rs",
    descriptor_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🔣️.json",
    taxonomy_path: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
};
const LEAF_SOURCE_SCOPE: MutationLeafSourceScope = MutationLeafSourceScope {
    workspace_token: [0x2a; 32],
    mutation_root: "✏️s/🔌️plugins/🧪️probe/🧬️mutations",
    owner_layout: MutationOwnerLayout::Flat,
    taxonomy_path: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
    mutation_payload_facet: "🦠️mutation",
    source_filename: "🦀️.rs",
    descriptor_filename: "🔣️.json",
};
const LEAF_PROVENANCE_TOKEN_MISMATCH: MutationSourceProvenance = MutationSourceProvenance { workspace_token: [0x2b; 32], ..LEAF_PROVENANCE };
const LEAF_SOURCE_VALID: Result<(), MutationLeafSourceValidationError> = validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE, &LEAF_SOURCE_SCOPE);
const LEAF_SOURCE_TOKEN_REJECTED: Result<(), MutationLeafSourceValidationError> = validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE_TOKEN_MISMATCH, &LEAF_SOURCE_SCOPE);
const _: () = match LEAF_SOURCE_VALID {
    Ok(()) => (),
    Err(_) => panic!("canonical leaf source must validate"),
};
const _: () = match LEAF_SOURCE_TOKEN_REJECTED {
    Err(_) => (),
    Ok(()) => panic!("workspace-token mismatch must reject"),
};
const DOMAIN_OWNER: &str = "✏️s/🔌️plugins/🧪️probe/🧬️mutations/🎥️camera/🔀️reorder";
const DOMAIN_DESCRIPTOR: MutationLeafDescriptor = MutationLeafDescriptor { owner: DOMAIN_OWNER, semantic_kind: "reorder-cameras", ..LEAF_DESCRIPTOR };
const DOMAIN_SCOPE: MutationLeafSourceScope = MutationLeafSourceScope { owner_layout: MutationOwnerLayout::DomainOperations(&[MutationDomainOperation { owner: DOMAIN_OWNER, semantic_kind: "reorder-cameras" }]), ..LEAF_SOURCE_SCOPE };
const DOMAIN_PROVENANCE: MutationSourceProvenance =
    MutationSourceProvenance {
        owner: DOMAIN_OWNER, source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/🎥️camera/🔀️reorder/🦀️.rs", descriptor_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/🎥️camera/🔀️reorder/🔣️.json", ..LEAF_PROVENANCE
    };
const _: () = match validate_mutation_leaf_source(&DOMAIN_DESCRIPTOR, &DOMAIN_PROVENANCE, &DOMAIN_SCOPE) {
    Ok(()) => (),
    Err(_) => panic!("exact domain source must validate in const evaluation"),
};
struct BorrowedLeaf<'a, T>(&'a T);
impl<'a, T> MutationLeaf for BorrowedLeaf<'a, T> {
    const DESCRIPTOR: MutationLeafDescriptor = LEAF_DESCRIPTOR;
    const PROVENANCE: MutationSourceProvenance = LEAF_PROVENANCE;
}
fn metadata_from<T: MutationLeaf>(_: &T) -> (MutationLeafDescriptor, MutationSourceProvenance) {
    (T::DESCRIPTOR, T::PROVENANCE)
}

#[test]
fn borrowed_generic_leaf_infers_static_metadata() {
    let local = 42_u32;
    let borrowed = &local;
    let leaf = BorrowedLeaf(&borrowed);
    let (descriptor, provenance) = metadata_from(&leaf);
    assert_eq!(**leaf.0, 42);
    assert_eq!(descriptor, LEAF_DESCRIPTOR);
    assert_eq!(provenance, LEAF_PROVENANCE);
    assert_eq!(provenance.owner, descriptor.owner);
}

#[test]
fn compiler_contract_vectors_have_complete_expected_outcomes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🤝️mutation-leaf-contract/🔣️.json")).expect("valid lower mutation leaf contract fixture");
    let cases = fixture["cases"].as_array().expect("compiler cases");
    assert_eq!(cases.len(), 3);
    assert!(cases.iter().any(|case| case["borrowedGeneric"] == true && case["expectedCompile"] == true));
    for case in cases.iter().filter(|case| case["expectedCompile"] == false) {
        assert_eq!(case["errorCode"], "E0046", "{}", case["name"]);
    }
}

#[test]
fn source_contract_rejects_every_workspace_token_byte_and_path_decoy() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧭️mutation-leaf-source-contract/🔣️.json")).expect("valid lower mutation leaf source fixture");
    let bytes = fixture["workspaceTokenMismatchBytes"].as_array().expect("workspace token byte vectors");
    assert_eq!(bytes.len(), 32);
    for byte in bytes {
        let index = byte.as_u64().expect("workspace token index") as usize;
        let mut provenance = LEAF_PROVENANCE;
        provenance.workspace_token[index] ^= 0xff;
        assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &provenance, &LEAF_SOURCE_SCOPE), Err(MutationLeafSourceValidationError { field: "workspaceToken", requirement: "must equal the aggregate workspace token" }));
    }
    let nested = MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🧪️tests", ..LEAF_DESCRIPTOR };
    assert_eq!(validate_mutation_leaf_source(&nested, &LEAF_PROVENANCE, &LEAF_SOURCE_SCOPE).unwrap_err().field, "owner");
    let historical = MutationSourceProvenance { source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/component.rs", ..LEAF_PROVENANCE };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &historical, &LEAF_SOURCE_SCOPE).unwrap_err().field, "sourcePath");
    let split = MutationSourceProvenance { source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🦠️mutation/🦀️.rs", ..LEAF_PROVENANCE };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &split, &LEAF_SOURCE_SCOPE), Ok(()));
    let wrong_facet = MutationSourceProvenance { source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/payload/🦀️.rs", ..LEAF_PROVENANCE };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &wrong_facet, &LEAF_SOURCE_SCOPE).unwrap_err().field, "sourcePath");
    let nested_facet = MutationSourceProvenance { source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🦠️mutation/nested/🦀️.rs", ..LEAF_PROVENANCE };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &nested_facet, &LEAF_SOURCE_SCOPE).unwrap_err().field, "sourcePath");
    let split_descriptor = MutationSourceProvenance { descriptor_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🦠️mutation/🔣️.json", ..LEAF_PROVENANCE };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &split_descriptor, &LEAF_SOURCE_SCOPE).unwrap_err().field, "descriptorPath");
    let alternate_scope = MutationLeafSourceScope { source_filename: "operation.rs", descriptor_filename: "metadata.json", ..LEAF_SOURCE_SCOPE };
    let alternate_provenance = MutationSourceProvenance {
        source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/operation.rs", descriptor_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/metadata.json", ..LEAF_PROVENANCE
    };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &alternate_provenance, &alternate_scope), Ok(()));
    let unsafe_taxonomy = MutationLeafSourceScope { taxonomy_path: "🧰️framework/../🔣️taxonomy.json", ..LEAF_SOURCE_SCOPE };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE, &unsafe_taxonomy).unwrap_err().field, "taxonomyPath");
    let compose_filename = MutationLeafSourceScope { source_filename: "Compose", ..LEAF_SOURCE_SCOPE };
    assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE, &compose_filename).unwrap_err().field, "sourceFilename");
    assert_eq!(fixture["cases"].as_array().expect("source cases").len(), 26);
}

#[test]
fn exact_domain_layout_preserves_full_identity_and_rejects_unregistered_pairs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json")).expect("shared exact-owner fixture");
    let root: &'static str = Box::leak(fixture["mutationRoot"].as_str().unwrap().to_string().into_boxed_str());
    let owners: &'static [MutationDomainOperation] = Box::leak(
        fixture["domains"]
            .as_object()
            .unwrap()
            .iter()
            .flat_map(|(domain, operations)| {
                operations
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(move |(operation, identity)| MutationDomainOperation { owner: Box::leak(format!("{root}/{domain}/{operation}").into_boxed_str()), semantic_kind: Box::leak(identity.as_str().unwrap().to_string().into_boxed_str()) })
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let layout = MutationOwnerLayout::DomainOperations(owners);
    let descriptors: &'static [MutationLeafDescriptor] = Box::leak(owners.iter().map(|entry| MutationLeafDescriptor { owner: entry.owner, semantic_kind: entry.semantic_kind, ..LEAF_DESCRIPTOR }).collect::<Vec<_>>().into_boxed_slice());
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, descriptors, layout), Ok(()));
    assert!(validate_mutation_leaf_descriptor_roster(root, &descriptors[..descriptors.len() - 1], layout).is_err());
    assert!(validate_mutation_leaf_descriptor_roster(root, descriptors, MutationOwnerLayout::Flat).is_err());
    for vector in fixture["cases"].as_array().unwrap().iter().filter(|vector| vector["fault"].is_null()) {
        let owner: &'static str = Box::leak(format!("{root}/{}", vector["owner"].as_str().unwrap()).into_boxed_str());
        let descriptor = MutationLeafDescriptor { owner, semantic_kind: Box::leak(vector["semanticKind"].as_str().unwrap().to_string().into_boxed_str()), ..LEAF_DESCRIPTOR };
        let provenance = MutationSourceProvenance {
            mutation_root: root,
            owner,
            source_path: Box::leak(format!("{owner}/{}", vector["source"].as_str().unwrap()).into_boxed_str()),
            descriptor_path: Box::leak(format!("{owner}/🔣️.json").into_boxed_str()),
            ..LEAF_PROVENANCE
        };
        let scope = MutationLeafSourceScope { mutation_root: root, owner_layout: layout, ..LEAF_SOURCE_SCOPE };
        assert_eq!(validate_mutation_leaf_source(&descriptor, &provenance, &scope).is_ok(), vector["accepted"].as_bool().unwrap(), "{}", vector["name"]);
        assert_eq!(scope.validate().and_then(|checked| checked.validate_leaf(&descriptor, &provenance)).is_ok(), vector["accepted"].as_bool().unwrap(), "{}", vector["name"]);
    }
    let duplicate = MutationOwnerLayout::DomainOperations(&[
        MutationDomainOperation { owner: DOMAIN_OWNER, semantic_kind: "reorder-cameras" },
        MutationDomainOperation { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/🎥️camera/🌱️create", semantic_kind: "reorder-cameras" },
    ]);
    assert!(validate_mutation_leaf_source(&DOMAIN_DESCRIPTOR, &DOMAIN_PROVENANCE, &MutationLeafSourceScope { owner_layout: duplicate, ..DOMAIN_SCOPE }).is_err());
    assert!(validate_mutation_leaf_source(&DOMAIN_DESCRIPTOR, &DOMAIN_PROVENANCE, &MutationLeafSourceScope { owner_layout: MutationOwnerLayout::DomainOperations(&[]), ..DOMAIN_SCOPE }).is_err());
}
