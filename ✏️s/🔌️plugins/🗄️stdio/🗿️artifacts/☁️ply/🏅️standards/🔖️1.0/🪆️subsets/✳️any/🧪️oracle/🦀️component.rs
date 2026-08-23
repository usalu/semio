//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `ply-rs` reference implementation so the subject's own mutation has an independent
//! result to be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. PLY 1.0 `any` has no sibling subset in this
//! artifact, so nothing here is shared.
//!
//! REFERENCE JUDGMENT (fleet brief §6): `ply-rs` 0.1.3's source was read directly out of the local
//! cargo registry cache (`src/writer/mod.rs`, `src/parser/mod.rs`) before this module was written.
//! It genuinely reads AND writes ascii and both binary endiannesses of the same `Ply<DefaultElement>`
//! model, with its own real round-trip test (`tests/write.rs::read_write_ply` asserts header+payload
//! equality after a write/read cycle) — a normal differential oracle, the first branch of §6, not the
//! read-only or no-oracle fallback.
//!
//! WORKED-AROUND DEFECT: `Writer::__write_binary_element`'s `PropertyType::List` arm writes
//! `element_def.count` (the ELEMENT's total row count) as the per-row list-length prefix instead of
//! that row's own list length — confirmed by reading the source and reproduced standalone: writing
//! this subset's real 16,128-face fixture to `binary_little_endian` and reading it back with the SAME
//! crate fails with "Couldn't find a list element at index 114" (16128 truncates to 0 as the declared
//! `uchar` index type, corrupting every subsequent read). [`write_binary_payload`] below writes binary
//! payloads itself — reusing `Writer::write_header` (unaffected) and the crate's own `PropertyAccess`
//! getters, encoding each value with `to_le_bytes`/`to_be_bytes` from the standard library alone —
//! while every ASCII write and every read (ascii or binary) still routes through the crate unmodified,
//! since only that one binary write path is wrong. Not hidden by loosening the projection: the
//! `set-format` scenario genuinely converts this real document to `binaryLittleEndian` and back.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`PlyMutation::KINDS`).

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use ply_rs::parser::Parser;
    use ply_rs::ply::{Addable, DefaultElement, ElementDef, Encoding, KeyMap, Ply, PropertyAccess, PropertyDef, PropertyType, ScalarType};
    use ply_rs::writer::Writer;
    use semio_repo_test_host::Json;
    use std::io::Cursor;

    //#region 🔖️Encoding
    fn encoding_from_str(value: &str) -> Result<Encoding, String> {
        Ok(match value {
            "ascii" => Encoding::Ascii,
            "binaryLittleEndian" => Encoding::BinaryLittleEndian,
            "binaryBigEndian" => Encoding::BinaryBigEndian,
            other => return Err(format!("unknown ply format {other:?}")),
        })
    }

    fn encoding_name(value: &Encoding) -> &'static str {
        match value {
            Encoding::Ascii => "ascii",
            Encoding::BinaryLittleEndian => "binaryLittleEndian",
            Encoding::BinaryBigEndian => "binaryBigEndian",
        }
    }

    fn scalar_type_from_str(value: &str) -> Result<ScalarType, String> {
        Ok(match value {
            "char" => ScalarType::Char,
            "uChar" => ScalarType::UChar,
            "short" => ScalarType::Short,
            "uShort" => ScalarType::UShort,
            "int" => ScalarType::Int,
            "uInt" => ScalarType::UInt,
            "float" => ScalarType::Float,
            "double" => ScalarType::Double,
            other => return Err(format!("unknown ply scalar type {other:?}")),
        })
    }

    fn scalar_type_name(value: &ScalarType) -> &'static str {
        match value {
            ScalarType::Char => "char",
            ScalarType::UChar => "uChar",
            ScalarType::Short => "short",
            ScalarType::UShort => "uShort",
            ScalarType::Int => "int",
            ScalarType::UInt => "uInt",
            ScalarType::Float => "float",
            ScalarType::Double => "double",
        }
    }
    //#endregion 🔖️Encoding

    //#region 🔖️JsonValue
    /// 🔎️ Owned mutation-params grammar this module speaks: a property's TYPE always comes from its
    /// owning element's own declaration (looked up at apply time), so a scalar value is a bare JSON
    /// number and a list value is a bare JSON number array — no self-describing tag needed.
    fn usize_field(value: &Json, key: &str) -> usize {
        match value.get(key) {
            Some(Json::Number(number)) => number.max(0.0) as usize,
            _ => 0,
        }
    }

    fn number_of(value: &Json) -> Result<f64, String> {
        match value {
            Json::Number(number) => Ok(*number),
            other => Err(format!("expected a number, found {other:?}")),
        }
    }

    fn value_to_property(value: &Json, data_type: &PropertyType) -> Result<ply_rs::ply::Property, String> {
        use ply_rs::ply::Property;
        match data_type {
            PropertyType::Scalar(scalar_type) => {
                let n = number_of(value)?;
                Ok(match scalar_type {
                    ScalarType::Char => Property::Char(n as i8),
                    ScalarType::UChar => Property::UChar(n as u8),
                    ScalarType::Short => Property::Short(n as i16),
                    ScalarType::UShort => Property::UShort(n as u16),
                    ScalarType::Int => Property::Int(n as i32),
                    ScalarType::UInt => Property::UInt(n as u32),
                    ScalarType::Float => Property::Float(n as f32),
                    ScalarType::Double => Property::Double(n as f64),
                })
            }
            PropertyType::List(_, item_type) => {
                let Json::Array(items) = value else { return Err(format!("expected an array for a list property, found {value:?}")) };
                let numbers = items.iter().map(number_of).collect::<Result<Vec<f64>, String>>()?;
                Ok(match item_type {
                    ScalarType::Char => Property::ListChar(numbers.iter().map(|n| *n as i8).collect()),
                    ScalarType::UChar => Property::ListUChar(numbers.iter().map(|n| *n as u8).collect()),
                    ScalarType::Short => Property::ListShort(numbers.iter().map(|n| *n as i16).collect()),
                    ScalarType::UShort => Property::ListUShort(numbers.iter().map(|n| *n as u16).collect()),
                    ScalarType::Int => Property::ListInt(numbers.iter().map(|n| *n as i32).collect()),
                    ScalarType::UInt => Property::ListUInt(numbers.iter().map(|n| *n as u32).collect()),
                    ScalarType::Float => Property::ListFloat(numbers.iter().map(|n| *n as f32).collect()),
                    ScalarType::Double => Property::ListDouble(numbers.iter().map(|n| *n as f64).collect()),
                })
            }
        }
    }

    fn property_to_json(value: &ply_rs::ply::Property) -> Json {
        use ply_rs::ply::Property;
        match value {
            Property::Char(v) => Json::Number(*v as f64),
            Property::UChar(v) => Json::Number(*v as f64),
            Property::Short(v) => Json::Number(*v as f64),
            Property::UShort(v) => Json::Number(*v as f64),
            Property::Int(v) => Json::Number(*v as f64),
            Property::UInt(v) => Json::Number(*v as f64),
            Property::Float(v) => Json::Number(*v as f64),
            Property::Double(v) => Json::Number(*v),
            Property::ListChar(v) => Json::Array(v.iter().map(|x| Json::Number(*x as f64)).collect()),
            Property::ListUChar(v) => Json::Array(v.iter().map(|x| Json::Number(*x as f64)).collect()),
            Property::ListShort(v) => Json::Array(v.iter().map(|x| Json::Number(*x as f64)).collect()),
            Property::ListUShort(v) => Json::Array(v.iter().map(|x| Json::Number(*x as f64)).collect()),
            Property::ListInt(v) => Json::Array(v.iter().map(|x| Json::Number(*x as f64)).collect()),
            Property::ListUInt(v) => Json::Array(v.iter().map(|x| Json::Number(*x as f64)).collect()),
            Property::ListFloat(v) => Json::Array(v.iter().map(|x| Json::Number(*x as f64)).collect()),
            Property::ListDouble(v) => Json::Array(v.iter().map(|x| Json::Number(*x)).collect()),
        }
    }

    fn property_def_from_json(value: &Json) -> Result<PropertyDef, String> {
        let name = value.str("name");
        if name.is_empty() {
            return Err("property requires a non-empty name".to_string());
        }
        match value.str("form").as_str() {
            "scalar" => Ok(PropertyDef::new(name, PropertyType::Scalar(scalar_type_from_str(&value.str("kind"))?))),
            "list" => Ok(PropertyDef::new(name, PropertyType::List(scalar_type_from_str(&value.str("countKind"))?, scalar_type_from_str(&value.str("valueKind"))?))),
            other => Err(format!("unknown property form {other:?}")),
        }
    }

    fn property_def_to_json(value: &PropertyDef) -> Json {
        match &value.data_type {
            PropertyType::Scalar(scalar_type) => {
                Json::Object(vec![("name".to_string(), Json::String(value.name.clone())), ("form".to_string(), Json::String("scalar".to_string())), ("kind".to_string(), Json::String(scalar_type_name(scalar_type).to_string()))])
            }
            PropertyType::List(index_type, item_type) => Json::Object(vec![
                ("name".to_string(), Json::String(value.name.clone())),
                ("form".to_string(), Json::String("list".to_string())),
                ("countKind".to_string(), Json::String(scalar_type_name(index_type).to_string())),
                ("valueKind".to_string(), Json::String(scalar_type_name(item_type).to_string())),
            ]),
        }
    }

    fn row_from_json(value: &Json, element_def: &ElementDef) -> Result<DefaultElement, String> {
        let values = value.array("values");
        if values.len() != element_def.properties.len() {
            return Err(format!("row for element {:?} expects {} values, got {}", element_def.name, element_def.properties.len(), values.len()));
        }
        let mut row = DefaultElement::new();
        for ((property_name, property_def), cell) in element_def.properties.iter().zip(values.iter()) {
            row.set_property(property_name.clone(), value_to_property(cell, &property_def.data_type)?);
        }
        Ok(row)
    }

    fn row_to_json(row: &DefaultElement, element_def: &ElementDef) -> Json {
        Json::Object(vec![("values".to_string(), Json::Array(element_def.properties.iter().map(|(property_name, _)| row.get(property_name).map(property_to_json).unwrap_or(Json::Null)).collect()))])
    }

    fn element_from_json(value: &Json) -> Result<(ElementDef, Vec<DefaultElement>), String> {
        let name = value.str("name");
        if name.is_empty() {
            return Err("element requires a non-empty name".to_string());
        }
        let mut def = ElementDef::new(name);
        for property in value.array("properties") {
            def.properties.add(property_def_from_json(&property)?);
        }
        let rows = value.array("rows").iter().map(|row| row_from_json(row, &def)).collect::<Result<Vec<_>, String>>()?;
        def.count = rows.len();
        Ok((def, rows))
    }

    fn element_to_json(name: &str, element_def: &ElementDef, rows: &[DefaultElement]) -> Json {
        Json::Object(vec![
            ("name".to_string(), Json::String(name.to_string())),
            ("count".to_string(), Json::Number(rows.len() as f64)),
            ("properties".to_string(), Json::Array(element_def.properties.iter().map(|(_, property_def)| property_def_to_json(property_def)).collect())),
            ("rows".to_string(), Json::Array(rows.iter().map(|row| row_to_json(row, element_def)).collect())),
        ])
    }

    fn ply_from_json(value: &Json) -> Result<Ply<DefaultElement>, String> {
        let mut ply = Ply::<DefaultElement>::new();
        ply.header.encoding = encoding_from_str(&value.str("format"))?;
        ply.header.comments = value
            .array("comments")
            .into_iter()
            .map(|comment| match comment {
                Json::String(text) => text,
                _ => String::new(),
            })
            .collect();
        for element in value.array("elements") {
            let (def, rows) = element_from_json(&element)?;
            let name = def.name.clone();
            ply.header.elements.add(def);
            ply.payload.insert(name, rows);
        }
        Ok(ply)
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️KeyMapReorder
    /// 🧭️ `KeyMap` (`LinkedHashMap`) has no positional insert — `AddElement`/`RemoveElement` rebuild
    /// the map in the desired order instead, which is the only place either header's `elements` and
    /// the payload's own key order can be edited.
    fn keymap_insert_at<V: Clone>(map: &KeyMap<V>, index: usize, key: String, value: V) -> KeyMap<V> {
        let mut items: Vec<(String, V)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let at = index.min(items.len());
        items.insert(at, (key, value));
        let mut out = KeyMap::new();
        for (k, v) in items {
            out.insert(k, v);
        }
        out
    }

    fn keymap_remove<V: Clone>(map: &KeyMap<V>, key: &str) -> KeyMap<V> {
        let mut out = KeyMap::new();
        for (k, v) in map.iter() {
            if k != key {
                out.insert(k.clone(), v.clone());
            }
        }
        out
    }
    //#endregion 🔖️KeyMapReorder

    //#region 🔖️BinaryWriteWorkaround
    /// 💾️ See this file's own module doc comment for the confirmed `ply-rs` defect this works
    /// around: writes every element's payload directly with `PropertyAccess` getters and
    /// `to_le_bytes`/`to_be_bytes`, computing each list property's length-prefix from the ROW's own
    /// list (not the element's row count, which is what the crate's binary writer wrongly uses).
    fn push_bytes<const N: usize>(out: &mut Vec<u8>, little_endian: [u8; N], big_endian: [u8; N], little: bool) {
        out.extend_from_slice(if little { &little_endian } else { &big_endian });
    }

    fn write_binary_scalar(out: &mut Vec<u8>, row: &DefaultElement, key: &String, scalar_type: &ScalarType, little: bool) {
        match scalar_type {
            ScalarType::Char => out.push(row.get_char(key).unwrap_or(0) as u8),
            ScalarType::UChar => out.push(row.get_uchar(key).unwrap_or(0)),
            ScalarType::Short => push_bytes(out, row.get_short(key).unwrap_or(0).to_le_bytes(), row.get_short(key).unwrap_or(0).to_be_bytes(), little),
            ScalarType::UShort => push_bytes(out, row.get_ushort(key).unwrap_or(0).to_le_bytes(), row.get_ushort(key).unwrap_or(0).to_be_bytes(), little),
            ScalarType::Int => push_bytes(out, row.get_int(key).unwrap_or(0).to_le_bytes(), row.get_int(key).unwrap_or(0).to_be_bytes(), little),
            ScalarType::UInt => push_bytes(out, row.get_uint(key).unwrap_or(0).to_le_bytes(), row.get_uint(key).unwrap_or(0).to_be_bytes(), little),
            ScalarType::Float => push_bytes(out, row.get_float(key).unwrap_or(0.0).to_le_bytes(), row.get_float(key).unwrap_or(0.0).to_be_bytes(), little),
            ScalarType::Double => push_bytes(out, row.get_double(key).unwrap_or(0.0).to_le_bytes(), row.get_double(key).unwrap_or(0.0).to_be_bytes(), little),
        }
    }

    fn list_value_len(row: &DefaultElement, key: &String, item_type: &ScalarType) -> usize {
        match item_type {
            ScalarType::Char => row.get_list_char(key).map(<[i8]>::len).unwrap_or(0),
            ScalarType::UChar => row.get_list_uchar(key).map(<[u8]>::len).unwrap_or(0),
            ScalarType::Short => row.get_list_short(key).map(<[i16]>::len).unwrap_or(0),
            ScalarType::UShort => row.get_list_ushort(key).map(<[u16]>::len).unwrap_or(0),
            ScalarType::Int => row.get_list_int(key).map(<[i32]>::len).unwrap_or(0),
            ScalarType::UInt => row.get_list_uint(key).map(<[u32]>::len).unwrap_or(0),
            ScalarType::Float => row.get_list_float(key).map(<[f32]>::len).unwrap_or(0),
            ScalarType::Double => row.get_list_double(key).map(<[f64]>::len).unwrap_or(0),
        }
    }

    fn write_binary_index(out: &mut Vec<u8>, index_type: &ScalarType, length: usize, little: bool) {
        match index_type {
            ScalarType::Char => out.push(length as i8 as u8),
            ScalarType::UChar => out.push(length as u8),
            ScalarType::Short => push_bytes(out, (length as i16).to_le_bytes(), (length as i16).to_be_bytes(), little),
            ScalarType::UShort => push_bytes(out, (length as u16).to_le_bytes(), (length as u16).to_be_bytes(), little),
            ScalarType::Int => push_bytes(out, (length as i32).to_le_bytes(), (length as i32).to_be_bytes(), little),
            ScalarType::UInt => push_bytes(out, (length as u32).to_le_bytes(), (length as u32).to_be_bytes(), little),
            ScalarType::Float | ScalarType::Double => {}
        }
    }

    fn write_binary_list_items(out: &mut Vec<u8>, row: &DefaultElement, key: &String, item_type: &ScalarType, little: bool) {
        match item_type {
            ScalarType::Char => row.get_list_char(key).unwrap_or(&[]).iter().for_each(|v| out.push(*v as u8)),
            ScalarType::UChar => row.get_list_uchar(key).unwrap_or(&[]).iter().for_each(|v| out.push(*v)),
            ScalarType::Short => row.get_list_short(key).unwrap_or(&[]).iter().for_each(|v| push_bytes(out, v.to_le_bytes(), v.to_be_bytes(), little)),
            ScalarType::UShort => row.get_list_ushort(key).unwrap_or(&[]).iter().for_each(|v| push_bytes(out, v.to_le_bytes(), v.to_be_bytes(), little)),
            ScalarType::Int => row.get_list_int(key).unwrap_or(&[]).iter().for_each(|v| push_bytes(out, v.to_le_bytes(), v.to_be_bytes(), little)),
            ScalarType::UInt => row.get_list_uint(key).unwrap_or(&[]).iter().for_each(|v| push_bytes(out, v.to_le_bytes(), v.to_be_bytes(), little)),
            ScalarType::Float => row.get_list_float(key).unwrap_or(&[]).iter().for_each(|v| push_bytes(out, v.to_le_bytes(), v.to_be_bytes(), little)),
            ScalarType::Double => row.get_list_double(key).unwrap_or(&[]).iter().for_each(|v| push_bytes(out, v.to_le_bytes(), v.to_be_bytes(), little)),
        }
    }

    fn write_binary_payload(ply: &Ply<DefaultElement>, little: bool) -> Vec<u8> {
        let mut out = Vec::new();
        for (element_name, element_def) in &ply.header.elements {
            let rows = ply.payload.get(element_name).map(|rows| rows.as_slice()).unwrap_or(&[]);
            for row in rows {
                for (property_name, property_def) in &element_def.properties {
                    match &property_def.data_type {
                        PropertyType::Scalar(scalar_type) => write_binary_scalar(&mut out, row, property_name, scalar_type, little),
                        PropertyType::List(index_type, item_type) => {
                            let length = list_value_len(row, property_name, item_type);
                            write_binary_index(&mut out, index_type, length, little);
                            write_binary_list_items(&mut out, row, property_name, item_type, little);
                        }
                    }
                }
            }
        }
        out
    }
    //#endregion 🔖️BinaryWriteWorkaround

    //#region 🔖️Serialize
    fn serialize(ply: &mut Ply<DefaultElement>) -> Result<Vec<u8>, String> {
        ply.make_consistent().map_err(|error| format!("ply-rs consistency check failed: {error}"))?;
        let writer = Writer::<DefaultElement>::new();
        match ply.header.encoding {
            Encoding::Ascii => {
                let mut out = Vec::new();
                writer.write_ply_unchecked(&mut out, ply).map_err(|error| format!("ply-rs could not write ascii output: {error}"))?;
                Ok(out)
            }
            Encoding::BinaryLittleEndian | Encoding::BinaryBigEndian => {
                let mut out = Vec::new();
                writer.write_header(&mut out, &ply.header).map_err(|error| format!("ply-rs could not write the header: {error}"))?;
                out.extend(write_binary_payload(ply, matches!(ply.header.encoding, Encoding::BinaryLittleEndian)));
                Ok(out)
            }
        }
    }
    //#endregion 🔖️Serialize

    //#region 🔖️Forward
    /// 🦠️ Applies `{kind, params}` to `input`, parsed and re-serialized entirely through `ply-rs`'s
    /// own `Ply<DefaultElement>` model (see this file's module doc comment for the one worked-around
    /// binary-writer defect). An unrecognised kind is an error, never a silent no-op.
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let parser = Parser::<DefaultElement>::new();
        let mut cursor = Cursor::new(input);
        let mut ply = parser.read_ply(&mut cursor).map_err(|error| format!("ply-rs could not parse the input: {error}"))?;
        match kind {
            "" => return Err("mutation spec carries no `kind`".to_string()),
            "no-mutation" => {}
            "set-snapshot" => {
                ply = ply_from_json(params.get("snapshot").ok_or("set-snapshot requires a snapshot field")?)?;
            }
            "set-format" => {
                ply.header.encoding = encoding_from_str(&params.str("format"))?;
            }
            "insert-comment" => {
                let index = usize_field(params, "index").min(ply.header.comments.len());
                ply.header.comments.insert(index, params.str("comment"));
            }
            "remove-comment" => {
                let index = usize_field(params, "index");
                if index < ply.header.comments.len() {
                    ply.header.comments.remove(index);
                }
            }
            "add-element" => {
                let element = params.get("element").ok_or("add-element requires an element field")?;
                let (def, rows) = element_from_json(element)?;
                let name = def.name.clone();
                let index = usize_field(params, "index").min(ply.header.elements.len());
                ply.header.elements = keymap_insert_at(&ply.header.elements, index, name.clone(), def);
                ply.payload = keymap_insert_at(&ply.payload, index, name, rows);
            }
            "remove-element" => {
                let name = params.str("name");
                ply.header.elements = keymap_remove(&ply.header.elements, &name);
                ply.payload = keymap_remove(&ply.payload, &name);
            }
            "insert-row" => {
                let element_name = params.str("elementName");
                let element_def = ply.header.elements.get(&element_name).cloned().ok_or_else(|| format!("no element named {element_name:?}"))?;
                let row = row_from_json(params.get("row").ok_or("insert-row requires a row field")?, &element_def)?;
                if !ply.payload.contains_key(&element_name) {
                    ply.payload.insert(element_name.clone(), Vec::new());
                }
                let rows = ply.payload.get_mut(&element_name).expect("just inserted");
                let index = usize_field(params, "index").min(rows.len());
                rows.insert(index, row);
            }
            "remove-row" => {
                let element_name = params.str("elementName");
                let index = usize_field(params, "index");
                if let Some(rows) = ply.payload.get_mut(&element_name) {
                    if index < rows.len() {
                        rows.remove(index);
                    }
                }
            }
            "set-row-property" => {
                let element_name = params.str("elementName");
                let row_index = usize_field(params, "rowIndex");
                let property_name = params.str("propertyName");
                let element_def = ply.header.elements.get(&element_name).cloned().ok_or_else(|| format!("no element named {element_name:?}"))?;
                let property_def = element_def.properties.get(&property_name).cloned().ok_or_else(|| format!("no property named {property_name:?}"))?;
                let value = value_to_property(&params.get("value").cloned().unwrap_or(Json::Null), &property_def.data_type)?;
                if let Some(row) = ply.payload.get_mut(&element_name).and_then(|rows| rows.get_mut(row_index)) {
                    row.set_property(property_name, value);
                }
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        serialize(&mut ply)
    }
    //#endregion 🔖️Forward

    //#region 🔖️Projection
    /// 👁️ The independent projection both producers' results are compared through: wire format,
    /// in-order comments, and each name-keyed element's own ordered property declarations and rows.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let parser = Parser::<DefaultElement>::new();
        let mut cursor = Cursor::new(bytes);
        let ply = parser.read_ply(&mut cursor).map_err(|error| format!("independent reader could not parse the document: {error}"))?;
        let elements = ply.header.elements.iter().map(|(name, def)| element_to_json(name, def, ply.payload.get(name).map(|rows| rows.as_slice()).unwrap_or(&[]))).collect();
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String(encoding_name(&ply.header.encoding).to_string())),
            ("comments".to_string(), Json::Array(ply.header.comments.iter().map(|comment| Json::String(comment.clone())).collect())),
            ("elements".to_string(), Json::Array(elements)),
        ]))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    let empty = Json::Object(Vec::new());
    let params = spec.get("params").unwrap_or(&empty);
    oracles::apply_mutation(input, &kind, params)
}

/// 👁️ This subset's own semantic projection, read independently through `ply-rs`. @see
/// [`oracles::project`].
#[cfg(feature = "oracles")]
pub fn project_ply(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_ply(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
