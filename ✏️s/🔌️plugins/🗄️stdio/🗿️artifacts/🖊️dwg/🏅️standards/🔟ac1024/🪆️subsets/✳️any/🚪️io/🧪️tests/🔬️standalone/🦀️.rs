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
#[test]
fn schema_facets_reject_imported_byte_shadow_state() {
    let facets = [
        include_str!("../../../🧬️schema/📸️snapshot/🦀️.rs"),
        include_str!("../../../🧬️schema/📸️snapshot/🟦️.ts"),
        include_str!("../../../🧬️schema/📸️snapshot/🔣️.json"),
        include_str!("../../../🧬️schema/📸️snapshot/📝️text/🔗️.graphql"),
        include_str!("../../../🧬️schema/📸️snapshot/📝️text/🛰️.proto"),
        include_str!("../../../🧬️schema/🧬️mutations/🟦️.ts"),
        include_str!("../../../🧬️schema/🧬️mutations/🔣️.json"),
        include_str!("../../../🧬️schema/🧬️mutations/🔗️.graphql"),
        include_str!("../../../🧬️schema/🧬️mutations/🛰️.proto"),
        include_str!("../../../🧬️schema/🔺️diff/📝️text/📖️.grammar.semio"),
        include_str!("../../../🧬️schema/🔺️diff/💾️binary/📡️.protocol.semio"),
        include_str!("../../../🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio"),
        include_str!("../../../🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio"),
        include_str!("../../../🧬️schema/💡️inferences/🦀️.rs"),
        include_str!("../../../🧬️schema/💡️inferences/🟦️.ts"),
        include_str!("../../../🧬️schema/💡️inferences/🔣️.json"),
        include_str!("../../../🧬️schema/💡️inferences/📝️text/🔗️.graphql"),
        include_str!("../../../🧬️schema/💡️inferences/📝️text/🛰️.proto"),
        include_str!("../../../🧬️schema/💡️inferences/📝️text/📖️.grammar.semio"),
        include_str!("../../../🧬️schema/💡️inferences/💾️binary/🔠️.abnf"),
        include_str!("../../../🧬️schema/💡️inferences/💾️binary/🥋️.ksy"),
        include_str!("../../../🧬️schema/💡️inferences/💾️binary/🌶️.spicy"),
        include_str!("../../../🧬️schema/💡️inferences/💾️binary/📡️.protocol.semio"),
    ];
    for facet in facets {
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
