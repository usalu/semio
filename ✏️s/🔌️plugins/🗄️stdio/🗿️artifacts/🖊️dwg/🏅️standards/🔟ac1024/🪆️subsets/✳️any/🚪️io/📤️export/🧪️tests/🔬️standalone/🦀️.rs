use semio_framework_test_fixtures::fixture_path;

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2010_object_inventory(sections: &[DwgRawSection]) -> Result<Vec<(u64, u16)>, String> {
    let handles = sections.iter().find(|section| section.name == "AcDb:Handles").ok_or("R2004 Handles section missing")?;
    let objects = sections.iter().find(|section| section.name == "AcDb:AcDbObjects").ok_or("R2004 AcDbObjects section missing")?;
    let handle_map = decode_r2004_handle_map(&r2004_section_data(handles)?)?;
    let object_data = r2004_section_data(objects)?;
    let mut inventory = Vec::with_capacity(handle_map.len());
    for (handle, address) in handle_map {
        let Some(bytes) = object_data.get(address..) else { continue };
        let mut frame = DwgBitReader::new(bytes);
        let Ok(payload_size) = frame.read_ms().map(|value| value as usize) else { continue };
        if frame.read_umc().is_err() {
            continue;
        }
        frame.pad_to_byte();
        let Some(payload) = bytes.get(frame.byte_pos..frame.byte_pos.saturating_add(payload_size)) else {
            continue;
        };
        let mut payload = DwgBitReader::new(payload);
        let Ok(object_type) = payload.read_bot() else { continue };
        let Ok((_, object_handle)) = payload.read_handle() else { continue };
        if object_handle == handle {
            inventory.push((handle, object_type));
        }
    }
    Ok(inventory)
}

#[cfg(test)]
/// 📂️ Reads one committed fixture by its path relative to this crate's manifest directory.
#[cfg(test)]
fn fixture(relative: &str) -> Vec<u8> {
    std::fs::read(fixture_path!(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

#[cfg(test)]
#[test]
fn schema_facets_reject_imported_byte_shadow_state() {
    let facets = [
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/🧮️geometry/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/🎛️metadata/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/🏷️xrecord/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/🗃️tables/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/📐️entities/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/🔗️associativity/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/🖌️styles/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/🧩️blocks/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📦️objects/📏️constraints/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/✏️drawing/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/🔧️header/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/📄️document/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/🟦️.ts")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/📸️snapshot/🔣️.json")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../📝️text/📸️snapshot/🔗️.graphql")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../📝️text/📸️snapshot/🛰️.proto")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/🧬️mutations/🟦️.ts")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/🧬️mutations/🔣️.json")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/🧬️mutations/🔗️.graphql")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/🧬️mutations/🛰️.proto")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../📝️text/🔺️diff/📖️.grammar.semio")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../💾️binary/🔺️diff/📡️.protocol.semio")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../📝️text/🧬️mutations/📖️.grammar.semio")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../💾️binary/🧬️mutations/📡️.protocol.semio")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/💡️inferences/🦀️.rs")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/💡️inferences/🟦️.ts")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../../🧬️schema/💡️inferences/🔣️.json")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../📝️text/💡️inferences/🔗️.graphql")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../📝️text/💡️inferences/🛰️.proto")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../📝️text/💡️inferences/📖️.grammar.semio")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../💾️binary/💡️inferences/🔠️.abnf")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../💾️binary/💡️inferences/🥋️.ksy")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../💾️binary/💡️inferences/🌶️.spicy")).expect("utf-8 facet"),
        String::from_utf8(fixture("../../../💾️binary/💡️inferences/📡️.protocol.semio")).expect("utf-8 facet"),
    ];
    for facet in &facets {
        for forbidden in
            [concat!("pub decod", "ed:"), concat!("\"decod", "ed\""), concat!("decod", "ed="), concat!("bytes_", "wire"), concat!("\"by", "tes\":"), concat!("source-", "field"), concat!("artifact-", "source"), concat!("semantic-", "blake3")]
        {
            assert!(!facet.contains(forbidden), "forbidden DWG shadow-state facet: {forbidden}");
        }
        for forbidden in ["payload = *OCTET", "size-eos", "bytes &eod", "chain body bytes"] {
            assert!(!facet.contains(forbidden), "forbidden opaque DWG facet: {forbidden}");
        }
    }
}
