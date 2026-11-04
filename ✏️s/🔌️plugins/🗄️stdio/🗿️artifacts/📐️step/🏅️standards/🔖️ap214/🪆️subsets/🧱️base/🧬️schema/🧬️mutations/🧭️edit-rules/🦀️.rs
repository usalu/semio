//! 🧭️ The details-pane vocabulary of the STEP editor: which JSON-pointer edit raises which concrete kind.
//!
//! The header slots, the name and arguments of an entity and the entity list each have a kind, so [`EDIT_RULES`] is complete: an entity is
//! addressed by its id, an argument by its position, and an edit below an argument replaces that argument. Complex constituents, the entity
//! ids and the document schema have no kind and are refused.

#[cfg(test)] use super::*;
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector, SnapshotEditError, SnapshotEditEvent, TRANSFORM_PATH};

const ENTITY: &[Selector] = &[Selector::Field { payload: "id", field: "id" }];
const ARGUMENT: &[Selector] = &[Selector::Field { payload: "id", field: "id" }, Selector::Index("argIndex")];

/// 📚 Every pointer a STEP edit resolves to one kind.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/header/fileDescription", "set-file-description", "fileDescription"),
        EntityRule::new("/header/fileName", "set-file-name", "fileName"),
        EntityRule::new("/header/fileSchema", "set-file-schema", "fileSchema"),
        EntityRule::new("/entities/*/name", "set-entity-name", "name").selecting(ENTITY),
        EntityRule::new("/entities/*/args/*", "set-entity-arg", "value").selecting(ARGUMENT),
    ],
    inserts: &[InsertRule::new("/entities", "insert-entity", "entity").at("index"), InsertRule::new("/entities/*/args", "insert-entity-arg", "value").at("argIndex").selecting(ENTITY)],
    removes: &[RemoveRule::by_key("/entities", "remove-entity", "id", "id"), RemoveRule::by_index("/entities/*/args", "remove-entity-arg", "argIndex").selecting(ENTITY)],
};

/// 📐️ The transform gesture: every `CARTESIAN_POINT` coordinate list is moved, every `DIRECTION` ratio list is turned, each by one `set-entity-arg`;
/// coordinates stay in the file's own length unit and every other argument, entity and header slot is untouched. `None` for any other edit.
pub fn special(event: &SnapshotEditEvent, snapshot: &super::StepSnapshot) -> Result<Option<Vec<super::StepMutation>>, semio_s_artifact_stdio_contract::editing::SnapshotEditError> {
    use super::set_entity_arg::SetEntityArg;
    use super::StepMutation;
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepValue;
    let Some(transform) = semio_s_artifact_stdio_contract::editing::Transform::from_event(event) else { return Ok(None) };
    let transform = transform.map_err(|fault| SnapshotEditError::new("snapshot-edit.transform-invalid", TRANSFORM_PATH, fault.message.to_string()))?;
    let number = |value: &StepValue| match value { StepValue::Real(real) => Some(*real), StepValue::Integer(integer) => Some(*integer as f64), _ => None };
    let mut leaves = Vec::new();
    for entity in &snapshot.entities {
        let moves = match entity.name.as_str() { "CARTESIAN_POINT" => true, "DIRECTION" => false, _ => continue };
        let Some(StepValue::Aggregate(items)) = entity.args.get(1) else { continue };
        let Some(input) = items.iter().map(number).collect::<Option<Vec<f64>>>() else { continue };
        if !(2..=3).contains(&input.len()) { continue; }
        let padded = [input[0], input[1], input.get(2).copied().unwrap_or(0.0)];
        let output = if moves { transform.point(padded) } else { transform.direction(padded) };
        let value = StepValue::Aggregate(output.iter().take(input.len()).map(|coordinate| StepValue::Real(*coordinate)).collect());
        leaves.push(StepMutation::SetEntityArg(SetEntityArg { id: entity.id, arg_index: 1, value }));
    }
    if leaves.is_empty() {
        return Err(SnapshotEditError::new("snapshot-edit.transform-empty", TRANSFORM_PATH, "the document holds no CARTESIAN_POINT or DIRECTION entity to transform"));
    }
    Ok(Some(leaves))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
