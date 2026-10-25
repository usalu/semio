//! 🧾️ Build and SDL of the repo GraphQL schema: the schema assembly, the reachable type walk, the SDL rendering and the schema inventory. See [`super`].

use super::*;

//#region 🏗️SchemaRoots
/// 🗺️ Assembles the executable schema from every per-aggregate fragment.
pub fn build_schema() -> Schema {
    let mut types: BTreeMap<String, NamedType> = BTreeMap::new();
    for scalar in SCALARS {
        types.insert((*scalar).to_string(), NamedType::Scalar((*scalar).to_string()));
    }
    let fragments = [
        schema_enums(),
        schema_metrics(),
        schema_repo(),
        schema_technology(),
        schema_bundle(),
        schema_folder(),
        schema_file(),
        schema_section(),
        schema_definition(),
        schema_statute(),
        schema_breach(),
        schema_policy(),
        schema_interaction(),
        schema_goal(),
        schema_draft(),
        schema_todo(),
        schema_ticket(),
        schema_checkpoint(),
        schema_contributor(),
        schema_inputs(),
        schema_query(),
        schema_mutation(),
    ];
    for fragment in fragments {
        for declared in fragment {
            types.insert(declared.name().to_string(), declared);
        }
    }
    Schema { types, query: "Query".to_string(), mutation: Some("Mutation".to_string()) }
}
//#endregion 🏗️SchemaRoots

//#region 📜️Sdl
/// 📜️ Collects every type reachable from the operation roots.
fn reachable(schema: &Schema) -> Vec<String> {
    let mut pending: Vec<String> = std::iter::once(schema.query.clone()).chain(schema.mutation.clone()).collect();
    let mut seen: Vec<String> = Vec::new();
    while let Some(name) = pending.pop() {
        if seen.contains(&name) {
            continue;
        }
        seen.push(name.clone());
        let Some(declared) = schema.types.get(&name) else { continue };
        let push_field = |field: &Field, pending: &mut Vec<String>| {
            pending.push(field.ty.type_name().to_string());
            for argument in &field.args {
                pending.push(argument.ty.type_name().to_string());
            }
        };
        match declared {
            NamedType::Object(object) => {
                pending.extend(object.interfaces.clone());
                for field in &object.fields {
                    push_field(field, &mut pending);
                }
            }
            NamedType::Interface(interface) => {
                for field in &interface.fields {
                    push_field(field, &mut pending);
                }
            }
            NamedType::Union(union) => pending.extend(union.types.clone()),
            NamedType::InputObject(input) => pending.extend(input.fields.iter().map(|(_, kind)| kind.type_name().to_string())),
            NamedType::Scalar(_) | NamedType::Enum(_) => {}
        }
    }
    seen.sort();
    seen
}

/// 🖌️ Renders one field in SDL.
fn render_field(field: &Field) -> String {
    let arguments = if field.args.is_empty() {
        String::new()
    } else {
        format!("({})", field.args.iter().map(|argument| format!("{}: {}", argument.name, argument.ty.render())).collect::<Vec<_>>().join(", "))
    };
    format!("  {}{}: {}", field.name, arguments, field.ty.render())
}

/// 🧬️ Renders the schema as the SDL both implementations must serve.
///
/// Deterministic by construction: reachable types only, sorted by name, fields in declaration
/// order — so the dump is a comparable artifact rather than a map walk.
pub fn render_sdl(schema: &Schema) -> String {
    let mut out = String::new();
    out.push_str("schema {\n");
    out.push_str(&format!("  query: {}\n", schema.query));
    if let Some(mutation) = &schema.mutation {
        out.push_str(&format!("  mutation: {mutation}\n"));
    }
    out.push_str("}\n");
    for name in reachable(schema) {
        let Some(declared) = schema.types.get(&name) else { continue };
        out.push('\n');
        match declared {
            NamedType::Scalar(scalar) => {
                if !["String", "Int", "Boolean", "ID"].contains(&scalar.as_str()) {
                    out.push_str(&format!("scalar {scalar}\n"));
                }
            }
            NamedType::Object(object) => {
                let implements = if object.interfaces.is_empty() { String::new() } else { format!(" implements {}", object.interfaces.join(" & ")) };
                out.push_str(&format!("type {}{} {{\n", object.name, implements));
                for field in &object.fields {
                    out.push_str(&render_field(field));
                    out.push('\n');
                }
                out.push_str("}\n");
            }
            NamedType::Interface(interface) => {
                out.push_str(&format!("interface {} {{\n", interface.name));
                for field in &interface.fields {
                    out.push_str(&render_field(field));
                    out.push('\n');
                }
                out.push_str("}\n");
            }
            NamedType::Union(union) => out.push_str(&format!("union {} = {}\n", union.name, union.types.join(" | "))),
            NamedType::Enum(enumeration) => {
                out.push_str(&format!("enum {} {{\n", enumeration.name));
                for (member, _) in &enumeration.values {
                    out.push_str(&format!("  {member}\n"));
                }
                out.push_str("}\n");
            }
            NamedType::InputObject(input) => {
                out.push_str(&format!("input {} {{\n", input.name));
                for (field, kind) in &input.fields {
                    out.push_str(&format!("  {}: {}\n", field, kind.render()));
                }
                out.push_str("}\n");
            }
        }
    }
    let trimmed = out.trim_end();
    format!("{trimmed}\n")
}

/// 📋️ The language-neutral inventory of the served schema: what a conforming reader finds in the
/// SDL, in the one order both implementations can agree on.
pub fn schema_inventory(schema: &Schema) -> Json {
    let field_row = |field: &Field| {
        json!({
            "name": field.name,
            "type": field.ty.render(),
            "args": field.args.iter().map(|argument| json!({ "name": argument.name, "type": argument.ty.render() })).collect::<Vec<_>>(),
        })
    };
    let types = reachable(schema)
        .into_iter()
        .filter_map(|name| schema.types.get(&name))
        .filter(|declared| !matches!(declared, NamedType::Scalar(scalar) if ["String", "Int", "Boolean", "ID"].contains(&scalar.as_str())))
        .map(|declared| match declared {
            NamedType::Scalar(scalar) => json!({ "name": scalar, "kind": "scalar" }),
            NamedType::Object(object) => json!({ "name": object.name, "kind": "object", "interfaces": object.interfaces, "fields": object.fields.iter().map(&field_row).collect::<Vec<_>>() }),
            NamedType::Interface(interface) => json!({ "name": interface.name, "kind": "interface", "fields": interface.fields.iter().map(&field_row).collect::<Vec<_>>() }),
            NamedType::Union(union) => json!({ "name": union.name, "kind": "union", "possibleTypes": union.types }),
            NamedType::Enum(enumeration) => json!({ "name": enumeration.name, "kind": "enum", "values": enumeration.values.iter().map(|(member, _)| member.clone()).collect::<Vec<_>>() }),
            NamedType::InputObject(input) => json!({ "name": input.name, "kind": "input", "fields": input.fields.iter().map(|(field, kind)| json!({ "name": field, "type": kind.render() })).collect::<Vec<_>>() }),
        })
        .collect::<Vec<_>>();
    json!({ "query": schema.query, "mutation": schema.mutation, "types": types })
}
//#endregion 📜️Sdl
