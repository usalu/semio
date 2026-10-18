/// 🧮️ Oracle JSON helpers and expected rows: the collection and object length probes, the operation applier, the scalar digest, the canonical JSON and the expected page, history and handback rows.

fn flow_oracle_collection_len(document: &semio_framework_pack_json::Value, key: &str) -> usize {
    document.get(key).and_then(semio_framework_pack_json::Value::as_array).expect("oracle collection").len()
}

fn flow_oracle_object_len(document: &semio_framework_pack_json::Value, key: &str) -> usize {
    document.get(key).and_then(semio_framework_pack_json::Value::as_object).expect("oracle object").len()
}

fn flow_oracle_id_position(values: &[semio_framework_pack_json::Value], id: &str) -> usize {
    values.iter().position(|value| value.get("id").and_then(semio_framework_pack_json::Value::as_str) == Some(id)).expect("oracle retained id")
}

fn flow_oracle_apply_operation(feature: &str, input: &semio_framework_pack_json::Value, document: &mut semio_framework_pack_json::Value) {
    match feature {
        "addWidget" => {
            let index = flow_oracle_usize(input, "index");
            document.get_mut("widgets").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle widgets").insert(index, input.get("widget").expect("oracle widget input").clone());
        }
        "removeWidget" => {
            let id = input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle widget id");
            let widgets = document.get_mut("widgets").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle widgets");
            let index = flow_oracle_id_position(widgets, id);
            widgets.remove(index);
        }
        "moveWidget" => {
            let id = input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle widget id");
            let target = flow_oracle_usize(input, "index");
            let widgets = document.get_mut("widgets").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle widgets");
            let index = flow_oracle_id_position(widgets, id);
            let widget = widgets.remove(index);
            widgets.insert(target, widget);
        }
        "patchWidget" => {
            let id = input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle widget id");
            let widgets = document.get_mut("widgets").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle widgets");
            let index = flow_oracle_id_position(widgets, id);
            widgets[index] = input.get("widget").expect("oracle widget patch").clone();
        }
        "addSynapse" => {
            let index = flow_oracle_usize(input, "index");
            document.get_mut("synapses").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle synapses").insert(index, input.get("synapse").expect("oracle synapse input").clone());
        }
        "removeSynapse" => {
            let id = input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle synapse id");
            let synapses = document.get_mut("synapses").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle synapses");
            let index = flow_oracle_id_position(synapses, id);
            synapses.remove(index);
        }
        "moveSynapse" => {
            let id = input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle synapse id");
            let target = flow_oracle_usize(input, "index");
            let synapses = document.get_mut("synapses").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle synapses");
            let index = flow_oracle_id_position(synapses, id);
            let synapse = synapses.remove(index);
            synapses.insert(target, synapse);
        }
        "patchSynapse" => {
            let id = input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle synapse id");
            let synapses = document.get_mut("synapses").and_then(semio_framework_pack_json::Value::as_array_mut).expect("oracle synapses");
            let index = flow_oracle_id_position(synapses, id);
            synapses[index] = input.get("synapse").expect("oracle synapse patch").clone();
        }
        "setLayout" => {
            let id = input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle layout id").to_owned();
            let layout = input.get("layout").expect("oracle layout input").clone();
            document.get_mut("layout").and_then(semio_framework_pack_json::Value::as_object_mut).expect("oracle layout").insert(id, layout);
        }
        "replaceDocument" => *document = input.get("document").expect("oracle replacement").clone(),
        _ => panic!("unsupported oracle operation {feature}"),
    }
}

fn flow_oracle_scalar_digest(document: &semio_framework_pack_json::Value) -> u64 {
    let schema = document.get("schema").and_then(semio_framework_pack_json::Value::as_str).expect("oracle schema");
    let camera = document.get("camera").expect("oracle camera");
    14_695_981_039_346_656_037
        ^ u64::try_from(schema.len()).expect("oracle schema bytes").rotate_left(3)
        ^ u64::try_from(flow_oracle_collection_len(document, "widgets")).expect("oracle widget count").rotate_left(11)
        ^ u64::try_from(flow_oracle_collection_len(document, "synapses")).expect("oracle synapse count").rotate_left(23)
        ^ u64::try_from(flow_oracle_object_len(document, "layout")).expect("oracle layout count").rotate_left(37)
        ^ camera.get("x").and_then(semio_framework_pack_json::Value::as_f64).expect("oracle camera x").to_bits()
        ^ camera.get("y").and_then(semio_framework_pack_json::Value::as_f64).expect("oracle camera y").to_bits().rotate_left(17)
        ^ camera.get("zoom").and_then(semio_framework_pack_json::Value::as_f64).expect("oracle camera zoom").to_bits().rotate_left(31)
}

fn flow_oracle_canonical_json(value: &semio_framework_pack_json::Value) -> String {
    fn append(value: &semio_framework_pack_json::Value, output: &mut String) {
        match value {
            semio_framework_pack_json::Value::Null => output.push_str("null"),
            semio_framework_pack_json::Value::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
            semio_framework_pack_json::Value::Number(value) => output.push_str(&format!("f64:{:016x}", value.as_f64().to_bits())),
            semio_framework_pack_json::Value::String(value) => output.push_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(value.clone()))),
            semio_framework_pack_json::Value::Array(values) => {
                output.push('[');
                for value in values {
                    append(value, output);
                    output.push(',');
                }
                output.push(']');
            }
            semio_framework_pack_json::Value::Object(values) => {
                let mut keys: Vec<&str> = values.iter().map(|(key, _)| key).collect();
                keys.sort();
                output.push('{');
                for key in keys {
                    output.push_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(key.to_string())));
                    output.push(':');
                    append(values.get(key).expect("oracle value"), output);
                    output.push(',');
                }
                output.push('}');
            }
        }
    }
    let mut output = String::new();
    append(value, &mut output);
    output
}

fn flow_oracle_u64(value: &semio_framework_pack_json::Value, key: &str) -> u64 {
    let value = value.get(key).expect("oracle numeric field");
    value.as_u64().or_else(|| value.as_str().and_then(|text| text.parse().ok())).expect("oracle u64 field")
}

fn flow_oracle_usize(value: &semio_framework_pack_json::Value, key: &str) -> usize {
    usize::try_from(flow_oracle_u64(value, key)).expect("oracle usize field")
}

fn flow_oracle_expected_page(value: &semio_framework_pack_json::Value) -> FlowOraclePage {
    FlowOraclePage {
        sequence: flow_oracle_u64(value, "sequence"),
        operation: flow_oracle_u64(value, "operation"),
        session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("oracle session generation"),
        revision: flow_oracle_u64(value, "revision"),
        parent_revision: flow_oracle_u64(value, "parentRevision"),
        document_generation: flow_oracle_u64(value, "documentGeneration"),
        widget_count: u32::try_from(flow_oracle_u64(value, "widgetCount")).expect("oracle widget count"),
        synapse_count: u32::try_from(flow_oracle_u64(value, "synapseCount")).expect("oracle synapse count"),
        layout_count: u32::try_from(flow_oracle_u64(value, "layoutCount")).expect("oracle layout count"),
        semantic_digest: flow_oracle_u64(value, "semanticDigest"),
    }
}

fn flow_oracle_expected_history(value: &semio_framework_pack_json::Value) -> FlowOracleHistory {
    FlowOracleHistory { undo_owners: flow_oracle_usize(value, "undoOwners"), redo_owners: flow_oracle_usize(value, "redoOwners") }
}

fn flow_oracle_expected_handback(template: &semio_framework_pack_json::Value, page: &FlowOraclePage, history: &FlowOracleHistory, document_versions: usize, active_document_version: usize) -> FlowOracleHandback {
    let credits = template.get("credits").expect("oracle terminal credits");
    FlowOracleHandback {
        credits: [
            flow_oracle_usize(credits, "operations"),
            flow_oracle_usize(credits, "pages"),
            flow_oracle_usize(credits, "items"),
            flow_oracle_usize(credits, "bytes"),
            flow_oracle_usize(credits, "outputs"),
            flow_oracle_usize(credits, "events"),
            flow_oracle_usize(credits, "controls"),
        ],
        active_operations: flow_oracle_usize(template, "activeOperations"),
        leased_pages: flow_oracle_usize(template, "leasedPages"),
        undo_owners: history.undo_owners,
        redo_owners: history.redo_owners,
        retired_action_owners: flow_oracle_usize(template, "retiredActionOwners"),
        retired_surface_owners: flow_oracle_usize(template, "retiredSurfaceOwners"),
        revision: page.revision,
        parent_revision: page.parent_revision,
        document_generation: page.document_generation,
        document_digest: page.semantic_digest,
        document_versions,
        active_document_version,
        edit_owner: template.get("editOwner").and_then(semio_framework_pack_json::Value::as_u64),
        document_retained: template.get("documentRetained").and_then(semio_framework_pack_json::Value::as_bool).expect("oracle document retained"),
        closing: template.get("closing").and_then(semio_framework_pack_json::Value::as_bool).expect("oracle closing"),
    }
}
