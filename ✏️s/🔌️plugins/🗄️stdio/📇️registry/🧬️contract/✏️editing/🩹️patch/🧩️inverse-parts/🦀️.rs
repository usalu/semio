//! 🧩️ The bounded inverse planner: JSON length measures, shells and inverse parts.

use super::*;

//#region 🧩️InverseParts
/// 📏️ The length of `text` as a canonical JSON string (the framework writer's escapes).
fn text_json_len(text: &str) -> usize {
    2 + text
        .chars()
        .map(|character| match character {
            '"' | '\\' | '\u{0008}' | '\u{000C}' | '\n' | '\r' | '\t' => 2,
            control if (control as u32) < 0x20 => 6,
            other => other.len_utf8(),
        })
        .sum::<usize>()
}

fn digits(value: u64) -> usize {
    value.checked_ilog10().map_or(1, |exponent| exponent as usize + 1)
}

/// 📏️ The exact length of `value`'s canonical JSON (the text [`SnapshotPatch`] encodes it as), computed without writing it.
pub(super) fn json_len(value: &DslValue) -> usize {
    let separators = |count: usize| count.saturating_sub(1);
    match value {
        DslValue::Null | DslValue::Bool(true) => 4,
        DslValue::Bool(false) => 5,
        DslValue::Number(semio_framework_value::Number::UInt(number)) => digits(*number),
        DslValue::Number(semio_framework_value::Number::Int(number)) => digits(number.unsigned_abs()) + usize::from(*number < 0),
        DslValue::Number(semio_framework_value::Number::Float(_)) => semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(value)).len(),
        DslValue::String(text) => text_json_len(text),
        DslValue::Bytes(bytes) => 2 + bytes.iter().map(|byte| digits(u64::from(*byte))).sum::<usize>() + separators(bytes.len()),
        DslValue::Array(items) => 2 + items.iter().map(json_len).sum::<usize>() + separators(items.len()),
        DslValue::Object(members) => 2 + members.iter().map(|(key, member)| text_json_len(key) + 1 + json_len(member)).sum::<usize>() + separators(members.len()),
    }
}

/// 🐚️ The smallest stand-in of `value` that keeps its kind and, for objects, every key: containers empty, text empty.
fn minimal_len(value: &DslValue) -> usize {
    match value {
        DslValue::String(_) | DslValue::Bytes(_) | DslValue::Array(_) => 2,
        DslValue::Object(members) => 2 + members.iter().map(|(key, member)| text_json_len(key) + 1 + minimal_len(member)).sum::<usize>() + members.len().saturating_sub(1),
        scalar => json_len(scalar),
    }
}

/// 🐚️ How a placed stand-in differs from the value it stands for: whole, emptied (refilled by appends), or an object whose
/// first `placed` members stand in by their own shells (the rest appended as members).
enum Shell {
    Whole,
    Emptied,
    Object { members: Vec<Shell>, placed: usize },
}

/// 🐚️ The stand-in of `value` within `room` bytes of canonical JSON: the value itself when it fits; otherwise its kind emptied,
/// or an object keeping, in order, each member whole while the rest still fit as their minimal stand-ins, each other member
/// as its own shell, and — when even the minimal stand-ins do not fit — only the members before the first that does not.
fn shell(value: &DslValue, room: usize) -> (DslValue, Shell) {
    if json_len(value) <= room {
        return (value.clone(), Shell::Whole);
    }
    match value {
        DslValue::String(_) => (DslValue::String(String::new()), Shell::Emptied),
        DslValue::Bytes(_) => (DslValue::Bytes(Vec::new()), Shell::Emptied),
        DslValue::Array(_) => (DslValue::Array(Vec::new()), Shell::Emptied),
        DslValue::Object(members) => {
            let entry = |position: usize, key: &str| text_json_len(key) + 1 + usize::from(position > 0);
            let mut rest = members.iter().enumerate().map(|(position, (key, member))| entry(position, key) + minimal_len(member)).sum::<usize>();
            let (mut used, mut placed, mut shells) = (2usize, Vec::new(), Vec::new());
            for (position, (key, member)) in members.iter().enumerate() {
                let minimal = minimal_len(member);
                rest -= entry(position, key) + minimal;
                let Some(available) = room.checked_sub(used + entry(position, key) + rest).filter(|available| *available >= minimal) else { break };
                let (stand_in, member_shell) = shell(member, available);
                used += entry(position, key) + json_len(&stand_in);
                placed.push((key.clone(), stand_in));
                shells.push(member_shell);
            }
            let count = placed.len();
            (DslValue::Object(placed), Shell::Object { members: shells, placed: count })
        }
        scalar => (scalar.clone(), Shell::Whole),
    }
}

/// 🧱️ The units a splice appends: items, octets, text or members.
enum Units<'v> {
    Items(&'v [DslValue]),
    Octets(&'v [u8]),
    Text(&'v str),
    Members(&'v [(String, DslValue)]),
}

impl<'v> Units<'v> {
    fn of(value: &'v DslValue) -> Option<Self> {
        Some(match value {
            DslValue::Array(items) => Self::Items(items),
            DslValue::Bytes(bytes) => Self::Octets(bytes),
            DslValue::String(text) => Self::Text(text),
            DslValue::Object(members) => Self::Members(members),
            _ => return None,
        })
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::Items(items) => items.len(),
            Self::Octets(bytes) => bytes.len(),
            Self::Text(text) => text.len(),
            Self::Members(members) => members.len(),
        }
    }
}

pub(super) struct InversePlanner {
    pub(super) budget: usize,
    pub(super) parts: Vec<SnapshotPatch>,
}

impl InversePlanner {
    fn limit() -> SnapshotEditError {
        SnapshotEditError::new("snapshot-edit.inverse-limit", "", "the exact inverse exceeds its bounded number of parts")
    }

    pub(super) fn push(&mut self, part: SnapshotPatch) -> Result<(), SnapshotEditError> {
        if self.parts.len() >= SNAPSHOT_PATCH_MAX_INVERSE_PARTS {
            return Err(Self::limit());
        }
        self.parts.push(part);
        Ok(())
    }

    /// 📏️ The room a splice part at `path`/`offset`/`remove` leaves for its value (counting a `continued` marker).
    fn room(&self, path: &str, offset: usize, remove: usize) -> Result<usize, SnapshotEditError> {
        let envelope = SnapshotPatch::Splice { path: path.into(), offset: offset as u64, remove: remove as u64, value: DslValue::Null, continued: true };
        let bytes = envelope.encode_op().map_err(|error| SnapshotEditError::new("snapshot-edit.patch-limit", path, error.to_string()))?.len() - 4;
        self.budget.checked_sub(bytes).ok_or_else(Self::limit)
    }

    pub(super) fn split<S: ToValue + FromValue + Clone>(&mut self, snapshot: &S, patch: &SnapshotPatch, inverse: SnapshotPatch) -> Result<(), SnapshotEditError> {
        match inverse {
            SnapshotPatch::Set { path, value } => {
                let segments = decode_pointer(&path)?;
                match segments.split_last() {
                    Some((key, parent)) => {
                        let position = match shape(snapshot, parent)? {
                            ValueShape::Object { .. } => object_key_index(snapshot, &segments)?.ok_or_else(Self::limit)?,
                            _ => array_index(key, usize::MAX, &path, false)?,
                        };
                        self.place(snapshot, parent, key, position, 1, &value)
                    }
                    None => {
                        let SnapshotPatch::Set { value: current, .. } = patch else { return Err(Self::limit()) };
                        self.root(current, &value)
                    }
                }
            }
            SnapshotPatch::Insert { path, value, index: position } => {
                let segments = decode_pointer(&path)?;
                let (key, parent) = segments.split_last().ok_or_else(Self::limit)?;
                let position = match position {
                    Some(position) => index(position, &segments)?,
                    None => array_index(key, usize::MAX, &path, false)?,
                };
                self.place(snapshot, parent, key, position, 0, &value)
            }
            SnapshotPatch::Splice { path, offset, remove, value, .. } => {
                let units = Units::of(&value).ok_or_else(Self::limit)?;
                self.append(&decode_pointer(&path)?, offset as usize, remove as usize, units)
            }
            SnapshotPatch::Remove { .. } | SnapshotPatch::Move { .. } | SnapshotPatch::Rename { .. } => Err(SnapshotEditError::new("snapshot-edit.patch-limit", "", "a pointer-only inverse exceeds the patch budget")),
        }
    }

    /// 📍️ Places the shell of `value` as member `key` of `parent` (replacing `remove` units at `position`), then refills it.
    fn place<S: ToValue>(&mut self, snapshot: &S, parent: &[String], key: &str, position: usize, remove: usize, value: &DslValue) -> Result<(), SnapshotEditError> {
        let object = matches!(shape(snapshot, parent)?, ValueShape::Object { .. });
        let wrapper = if object { 2 + text_json_len(key) + 1 } else { 2 };
        let room = self.room(&pointer(parent), position, remove)?.checked_sub(wrapper).ok_or_else(Self::limit)?;
        let (stand_in, placed) = shell(value, room);
        let wrapped = if object { DslValue::Object(vec![(key.to_string(), stand_in)]) } else { DslValue::Array(vec![stand_in]) };
        self.push(SnapshotPatch::Splice { path: pointer(parent), offset: position as u64, remove: remove as u64, value: wrapped, continued: true })?;
        self.fill(&child(parent, key), value, &placed)
    }

    /// 🌳️ Restores a replaced document root: a splice of the whole root when the kind is unchanged, else a root `set` of the shell.
    fn root(&mut self, current: &DslValue, value: &DslValue) -> Result<(), SnapshotEditError> {
        let same_kind = std::mem::discriminant(current) == std::mem::discriminant(value);
        match (Units::of(current), Units::of(value)) {
            (Some(current), Some(Units::Members(members))) if same_kind => {
                let room = self.room("", 0, current.len())?;
                let (stand_in, placed) = shell(value, room);
                self.push(SnapshotPatch::Splice { path: String::new(), offset: 0, remove: current.len() as u64, value: stand_in, continued: true })?;
                self.fill_object(&[], members, &placed)
            }
            (Some(current), Some(units)) if same_kind => self.append(&[], 0, current.len(), units),
            _ => {
                let envelope = SnapshotPatch::Set { path: String::new(), value: DslValue::Null }.encode_op().map_err(|error| SnapshotEditError::new("snapshot-edit.patch-limit", "", error.to_string()))?.len() - 4;
                let (stand_in, placed) = shell(value, self.budget.checked_sub(envelope).ok_or_else(Self::limit)?);
                self.push(SnapshotPatch::Set { path: String::new(), value: stand_in })?;
                self.fill(&[], value, &placed)
            }
        }
    }

    fn fill(&mut self, path: &[String], value: &DslValue, placed: &Shell) -> Result<(), SnapshotEditError> {
        match (placed, value) {
            (Shell::Whole, _) => Ok(()),
            (Shell::Emptied, value) => self.append(path, 0, 0, Units::of(value).ok_or_else(Self::limit)?),
            (Shell::Object { .. }, DslValue::Object(members)) => self.fill_object(path, members, placed),
            _ => Err(Self::limit()),
        }
    }

    fn fill_object(&mut self, path: &[String], members: &[(String, DslValue)], placed: &Shell) -> Result<(), SnapshotEditError> {
        let Shell::Object { members: shells, placed } = placed else { return Ok(()) };
        for ((key, member), member_shell) in members.iter().zip(shells) {
            self.fill(&child(path, key.clone()), member, member_shell)?;
        }
        if *placed < members.len() {
            self.append(path, *placed, 0, Units::Members(&members[*placed..]))?;
        }
        Ok(())
    }

    /// ➕️ Appends `units` at `offset` of the container at `path` (the first part also removing `remove` units), packing each
    /// part greedily up to the budget; an item or member too large for a part of its own goes in as its shell and is refilled
    /// right after its part.
    fn append(&mut self, path: &[String], offset: usize, remove: usize, units: Units<'_>) -> Result<(), SnapshotEditError> {
        let text = pointer(path);
        let (mut start, mut remove, mut next) = (offset, remove, 0usize);
        while next < units.len() || remove > 0 {
            let room = self.room(&text, start, remove)?.checked_sub(2).ok_or_else(Self::limit)?;
            let mut used = 0usize;
            let first = next;
            let (value, refills) = match &units {
                Units::Text(content) => {
                    let mut end = next;
                    for character in content[next..].chars() {
                        let size = text_json_len(character.encode_utf8(&mut [0; 4])) - 2;
                        if used + size > room {
                            break;
                        }
                        used += size;
                        end += character.len_utf8();
                    }
                    next = end;
                    (DslValue::String(content[first..end].to_string()), Vec::new())
                }
                Units::Octets(bytes) => {
                    while next < bytes.len() && used + usize::from(next > first) + digits(u64::from(bytes[next])) <= room {
                        used += usize::from(next > first) + digits(u64::from(bytes[next]));
                        next += 1;
                    }
                    (DslValue::Bytes(bytes[first..next].to_vec()), Vec::new())
                }
                Units::Items(items) => {
                    let (mut batch, mut refills) = (Vec::new(), Vec::new());
                    while next < items.len() {
                        let (stand_in, placed) = shell(&items[next], room);
                        let size = usize::from(!batch.is_empty()) + json_len(&stand_in);
                        if !batch.is_empty() && used + size > room {
                            break;
                        }
                        used += size;
                        refills.push((child(path, (start + batch.len()).to_string()), &items[next], placed));
                        batch.push(stand_in);
                        next += 1;
                    }
                    (DslValue::Array(batch), refills)
                }
                Units::Members(members) => {
                    let (mut batch, mut refills) = (Vec::new(), Vec::new());
                    while next < members.len() {
                        let (key, member) = &members[next];
                        let (stand_in, placed) = shell(member, room.saturating_sub(text_json_len(key) + 1));
                        let size = usize::from(!batch.is_empty()) + text_json_len(key) + 1 + json_len(&stand_in);
                        if !batch.is_empty() && used + size > room {
                            break;
                        }
                        used += size;
                        refills.push((child(path, key.clone()), member, placed));
                        batch.push((key.clone(), stand_in));
                        next += 1;
                    }
                    (DslValue::Object(batch), refills)
                }
            };
            if next == first && remove == 0 {
                return Err(SnapshotEditError::new("snapshot-edit.patch-limit", text.as_str(), "a unit of the inverse cannot be split within the patch budget"));
            }
            self.push(SnapshotPatch::Splice { path: text.clone(), offset: start as u64, remove: remove as u64, value, continued: true })?;
            start += next - first;
            remove = 0;
            for (refill_path, member, placed) in refills {
                self.fill(&refill_path, member, &placed)?;
            }
        }
        Ok(())
    }
}
//#endregion 🧩️InverseParts
