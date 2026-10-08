//! ✂️ Range-text operations — the Rust twin of `✂️text-splice/🟦️.ts`. A splice names what its author saw around
//! the replaced run, so it lands where its author meant it in any text other authors changed meanwhile, the same way in every
//! language. Positions and lengths count Unicode scalar values (`char`s).
//!
//! @see ../🧬️schema/✂️text-splice/🔣️.json

/// ✂️ One range-text operation as its author saw it: at scalar `start` of the author's text, `deleted` was replaced by
/// `insert`; `before`/`after` are up to [`TEXT_SPLICE_CONTEXT_SCALARS`] scalars around the replaced run.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSplice {
    pub start: u32,
    pub deleted: String,
    pub insert: String,
    pub before: String,
    pub after: String,
}

/// 📍️ Where a splice lands in a concrete text: the run it replaces there, and whether its `deleted` run was already gone
/// (`clamped`: nothing is deleted, the insert still lands) or its context was nowhere to be found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocatedTextSplice {
    pub start: u32,
    pub delete_length: u32,
    pub clamped: bool,
}

/// 🧾️ A splice applied to a text: the new text, where it landed, and the inverse splice that restores the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppliedTextSplice {
    pub text: String,
    pub located: LocatedTextSplice,
    pub inverse: TextSplice,
}

/// 📏️ Context scalars a splice carries on each side.
pub const TEXT_SPLICE_CONTEXT_SCALARS: usize = 32;

/// ⌨️ The host side of the typing-run protocol (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING; owner constants
/// `🛠️tool-machine` `TYPING_BUFFER_ARG`/`TYPING_COMMIT_ARG`/`TYPING_IDLE_MS`, pinned against the typing-law fixture by this
/// module's laws, twin of the TS `TEXT_EDITOR_TYPING_*`): every live delivery names the buffer it types into, a commit signal
/// names its reason and carries no edit, and a host ends its run this long after its last delivery.
pub const TEXT_EDITOR_TYPING_BUFFER_ARG: &str = "typing";
pub const TEXT_EDITOR_TYPING_COMMIT_ARG: &str = "typingCommit";
pub const TEXT_EDITOR_TYPING_IDLE_MS: u64 = 750;

/// 🔗️ Two-sided context patterns shorter than this many scalars per side are never searched: two scalars of context match all
/// over a real document, one-sided context of the same length near the author's position is the better witness.
pub const TEXT_SPLICE_MIN_TWO_SIDED_SCALARS: usize = 4;

/// 🔎️ Every start index of `needle` in `hay` (overlapping); an empty needle matches at every position.
fn occurrences(hay: &[char], needle: &[char]) -> Vec<usize> {
    if needle.is_empty() {
        return (0..=hay.len()).collect();
    }
    if needle.len() > hay.len() {
        return Vec::new();
    }
    (0..=hay.len() - needle.len()).filter(|&index| hay[index..index + needle.len()] == *needle).collect()
}

/// 🎯️ The candidate nearest to `expected`; ties take the lowest position.
fn nearest(positions: impl IntoIterator<Item = usize>, expected: usize) -> Option<usize> {
    positions.into_iter().min_by_key(|&position| (position.abs_diff(expected), position))
}

fn joined(parts: &[&[char]]) -> Vec<char> {
    parts.iter().flat_map(|part| part.iter().copied()).collect()
}

impl TextSplice {
    /// 📍️ Locates the splice in `text`, deterministically and identically to the TS twin:
    /// 1. the run with context on both sides, `k` = the longer side's length down to [`TEXT_SPLICE_MIN_TWO_SIDED_SCALARS`];
    /// 2. the run with context on one side, `k` = the longer side's length down to 1 (both sides of one `k` compete);
    /// 3. the run alone (only when it is not empty);
    /// 4. a run that is gone deletes nothing (`clamped`): steps 1–2 again with an empty run find the insertion point;
    /// 5. otherwise the author's `start`, clamped to the text (`clamped` when the splice carried any context).
    /// Every step takes the match nearest to the author's `start`, ties to the lowest position.
    pub fn locate(&self, text: &str) -> LocatedTextSplice {
        let hay: Vec<char> = text.chars().collect();
        let (before, deleted, after): (Vec<char>, Vec<char>, Vec<char>) = (self.before.chars().collect(), self.deleted.chars().collect(), self.after.chars().collect());
        let start = self.start as usize;
        let longest = before.len().max(after.len());
        let anchored = |run: &[char]| -> Option<usize> {
            let mut k = longest;
            while k >= TEXT_SPLICE_MIN_TWO_SIDED_SCALARS {
                let head = &before[before.len() - k.min(before.len())..];
                let tail = &after[..k.min(after.len())];
                if head.is_empty() || tail.is_empty() {
                    break;
                }
                if let Some(found) = nearest(occurrences(&hay, &joined(&[head, run, tail])).into_iter().map(|index| index + head.len()), start) {
                    return Some(found);
                }
                k -= 1;
            }
            for k in (1..=longest).rev() {
                let head = &before[before.len() - k.min(before.len())..];
                let tail = &after[..k.min(after.len())];
                let left = if head.is_empty() { Vec::new() } else { occurrences(&hay, &joined(&[head, run])).into_iter().map(|index| index + head.len()).collect() };
                let right = if tail.is_empty() { Vec::new() } else { occurrences(&hay, &joined(&[run, tail])) };
                if let Some(found) = nearest(left.into_iter().chain(right), start) {
                    return Some(found);
                }
            }
            None
        };
        let exact = anchored(&deleted).or_else(|| if deleted.is_empty() { None } else { nearest(occurrences(&hay, &deleted), start) });
        if let Some(found) = exact {
            return LocatedTextSplice { start: found as u32, delete_length: deleted.len() as u32, clamped: false };
        }
        if !deleted.is_empty() {
            if let Some(found) = anchored(&[]) {
                return LocatedTextSplice { start: found as u32, delete_length: 0, clamped: true };
            }
        }
        LocatedTextSplice { start: start.min(hay.len()) as u32, delete_length: 0, clamped: !deleted.is_empty() || !before.is_empty() || !after.is_empty() }
    }

    /// ✂️ Applies the splice to `text` where it locates, answering the new text and the inverse splice (the removed run back in
    /// place of the inserted one, with `context` scalars of the new text around it).
    pub fn splice_into(&self, text: &str, context: usize) -> AppliedTextSplice {
        let located = self.locate(text);
        let hay: Vec<char> = text.chars().collect();
        let insert: Vec<char> = self.insert.chars().collect();
        let (at, length) = (located.start as usize, located.delete_length as usize);
        let removed: String = hay[at..at + length].iter().collect();
        let next: Vec<char> = joined(&[&hay[..at], &insert, &hay[at + length..]]);
        let end = at + insert.len();
        AppliedTextSplice {
            text: next.iter().collect(),
            located,
            inverse: TextSplice {
                start: located.start,
                deleted: self.insert.clone(),
                insert: removed,
                before: next[at.saturating_sub(context)..at].iter().collect(),
                after: next[end..(end + context).min(next.len())].iter().collect(),
            },
        }
    }

    /// ⌨️ The ONE splice an editor sends for its own change `previous → next`: the common scalar prefix and suffix bound the
    /// changed run, the context is what the author saw around it; `None` when nothing changed.
    pub fn from_edit(previous: &str, next: &str, context: usize) -> Option<Self> {
        let (a, b): (Vec<char>, Vec<char>) = (previous.chars().collect(), next.chars().collect());
        let prefix = a.iter().zip(&b).take_while(|(left, right)| left == right).count();
        if prefix == a.len() && prefix == b.len() {
            return None;
        }
        let suffix = a[prefix..].iter().rev().zip(b[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
        Some(Self {
            start: prefix as u32,
            deleted: a[prefix..a.len() - suffix].iter().collect(),
            insert: b[prefix..b.len() - suffix].iter().collect(),
            before: a[prefix.saturating_sub(context)..prefix].iter().collect(),
            after: a[a.len() - suffix..(a.len() - suffix + context).min(a.len())].iter().collect(),
        })
    }

    /// 🧷️ A zero-width splice marking scalar `position` of `text` — how a caret or a selection end travels through other
    /// authors' splices: locating the marker in the new text is the position's new place.
    pub fn marker(text: &str, position: usize, context: usize) -> Self {
        let hay: Vec<char> = text.chars().collect();
        let at = position.min(hay.len());
        Self { start: at as u32, deleted: String::new(), insert: String::new(), before: hay[at.saturating_sub(context)..at].iter().collect(), after: hay[at..(at + context).min(hay.len())].iter().collect() }
    }

    /// 🔗️ The ONE splice of a typing run: `self` (the run so far) took the author's text `T0` to `T1`, `next` (an edit of `T1`,
    /// same author, same coordinates) takes `T1` to `T2`. `next` must lie inside the run's window `before + insert + after` of
    /// `T1` and agree with it wherever both carry text, and the two changes must touch once each is placed anywhere it may
    /// equivalently sit among repeated scalars (a canonical splice sits rightmost, the caret may sit further left). The run
    /// then grows to the union of both changes, untrimmed — a scalar the run deleted and typed again stays inside it, so typing
    /// on at the caret keeps touching it; `Cancelled` when the run no longer changes anything, `Disjoint` when the caret
    /// jumped away. Placements are tried nearest first, left before right, identically in the TS twin.
    pub fn then(&self, next: &TextSplice, context: usize) -> TextSpliceComposition {
        let chars = |text: &str| -> Vec<char> { text.chars().collect() };
        let (before, deleted, insert, after) = (chars(&self.before), chars(&self.deleted), chars(&self.insert), chars(&self.after));
        let (next_before, next_deleted, next_insert, next_after) = (chars(&next.before), chars(&next.deleted), chars(&next.insert), chars(&next.after));
        let window = joined(&[&before, &insert, &after]);
        let origin = i64::from(self.start) - before.len() as i64;
        let at = i64::from(next.start) - origin;
        if at < 0 || at as usize + next_deleted.len() > window.len() {
            return TextSpliceComposition::Disjoint;
        }
        let (at, end) = (at as usize, at as usize + next_deleted.len());
        let (head, tail) = (&window[..at], &window[end..]);
        let (seen_before, seen_after) = (next_before.len().min(head.len()), next_after.len().min(tail.len()));
        if window[at..end] != next_deleted[..] || head[head.len() - seen_before..] != next_before[next_before.len() - seen_before..] || tail[..seen_after] != next_after[..seen_after] {
            return TextSpliceComposition::Disjoint;
        }
        let runs = placements(&window, before.len(), &insert, &deleted);
        let nexts = placements(&window, at, &next_deleted, &next_insert);
        let Some(((run_at, _, run_deleted), (next_at, _, next_typed))) =
            runs.iter().find_map(|run| nexts.iter().find(|candidate| candidate.0 <= run.0 + insert.len() && candidate.0 + next_deleted.len() >= run.0).map(|candidate| (run, candidate)))
        else {
            return TextSpliceComposition::Disjoint;
        };
        let original = joined(&[&window[..*run_at], &run_deleted[..], &window[run_at + insert.len()..]]);
        let typed = joined(&[&window[..*next_at], &next_typed[..], &window[next_at + next_deleted.len()..]]);
        if original == typed {
            return TextSpliceComposition::Cancelled;
        }
        let (lo, hi) = ((*run_at).min(*next_at), (run_at + insert.len()).max(next_at + next_deleted.len()));
        let (original_hi, typed_hi) = (hi - insert.len() + deleted.len(), hi - next_deleted.len() + next_insert.len());
        let left = &next_before[..next_before.len() - seen_before];
        let leading = joined(&[left, &window[..lo]]);
        let trailing = joined(&[&original[original_hi..], &next_after[seen_after..]]);
        TextSpliceComposition::Composed(Self {
            start: (origin + lo as i64) as u32,
            deleted: original[lo..original_hi].iter().collect(),
            insert: typed[lo..typed_hi].iter().collect(),
            before: leading[leading.len().saturating_sub(context)..].iter().collect(),
            after: trailing[..trailing.len().min(context)].iter().collect(),
        })
    }
}

/// 🪜️ Every equivalent placement of one change inside `window` (`T1`): `inside` is its `T1` side starting at `at`, `outside` its
/// other side; shifting it by one scalar `c` is the same change while both sides end (left) or start (right) with `c`.
/// Ordered by distance from `at`, left before right.
fn placements(window: &[char], at: usize, inside: &[char], outside: &[char]) -> Vec<(usize, Vec<char>, Vec<char>)> {
    let shift = |(position, inside, outside): &(usize, Vec<char>, Vec<char>), leftward: bool| -> Option<(usize, Vec<char>, Vec<char>)> {
        let scalar = if leftward { *window.get(position.checked_sub(1)?)? } else { *window.get(position + inside.len())? };
        let matches = |side: &Vec<char>| side.is_empty() || (if leftward { side.last() } else { side.first() }) == Some(&scalar);
        if (inside.is_empty() && outside.is_empty()) || !matches(inside) || !matches(outside) {
            return None;
        }
        let rotate = |side: &Vec<char>| if side.is_empty() { Vec::new() } else if leftward { joined(&[&[scalar], &side[..side.len() - 1]]) } else { joined(&[&side[1..], &[scalar]]) };
        Some((if leftward { position - 1 } else { position + 1 }, rotate(inside), rotate(outside)))
    };
    let current = (at, inside.to_vec(), outside.to_vec());
    let lefts: Vec<_> = std::iter::successors(shift(&current, true), |previous| shift(previous, true)).collect();
    let rights: Vec<_> = std::iter::successors(shift(&current, false), |previous| shift(previous, false)).collect();
    let mut ordered = vec![current];
    for index in 0..lefts.len().max(rights.len()) {
        ordered.extend(lefts.get(index).cloned());
        ordered.extend(rights.get(index).cloned());
    }
    ordered
}

/// 🔗️ How a typed splice joins its run ([`TextSplice::then`]): one net splice, a run that changed nothing, or a caret that
/// jumped away (the run ends and a new one begins).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextSpliceComposition {
    Composed(TextSplice),
    Cancelled,
    Disjoint,
}

/// 🔁️ An editor host's view after the guest published `remote`: every splice the guest has not applied yet, in the order the
/// host sent them, folded onto `remote`, and the local selection (scalar `anchor`/`caret` in `local`) carried along.
pub fn rebase_text_edits(remote: &str, unapplied: &[TextSplice], local: &str, anchor: usize, caret: usize) -> (String, usize, usize) {
    let text = unapplied.iter().fold(remote.to_string(), |text, splice| splice.splice_into(&text, TEXT_SPLICE_CONTEXT_SCALARS).text);
    let place = |position: usize| TextSplice::marker(local, position, TEXT_SPLICE_CONTEXT_SCALARS).locate(&text).start as usize;
    let (anchor, caret) = (place(anchor), place(caret));
    (text, anchor, caret)
}

//#region ✂️DraftChangeSet
/// ✂️ One range of an explicit draft's change set: in the coordinates of the text the draft STARTED from (ascending, never overlapping),
/// `delete` scalars at `offset` were replaced by `insert`. Twin of the TS `DraftChangeV1`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DraftChange {
    pub offset: usize,
    pub delete: usize,
    pub insert: String,
}

/// ✂️ The edit argument a window declares to receive the draft's change set (JSON list of [`DraftChange`]) instead of its text.
pub const DRAFT_SPLICES_ARGUMENT: &str = "splices";

enum DraftPiece {
    Keep { base: usize, length: usize },
    Change { base: usize, remove: usize, insert: Vec<char> },
}

impl DraftPiece {
    fn base_end(&self) -> usize {
        match self {
            Self::Keep { base, length } => base + length,
            Self::Change { base, remove, .. } => base + remove,
        }
    }
}

/// ⌨️ The change set after one more editor step `previous -> next` (a typed run, a paste, an undo, a redo): the step is the editor's own
/// change (the scalars both texts share at either end bound it), folded into the ranges so far; every range stays in the coordinates of the
/// starting text. Twin of the TS `composeDraftStepV1`.
pub fn compose_draft_step(changes: &[DraftChange], previous: &str, next: &str) -> Vec<DraftChange> {
    let Some(step) = TextSplice::from_edit(previous, next, 0) else { return changes.to_vec() };
    let current_length = previous.chars().count();
    let start = step.start as usize;
    let end = start + step.deleted.chars().count();
    let inserted: Vec<char> = step.insert.chars().collect();
    let mut pieces = Vec::new();
    let (mut base_position, mut covered) = (0usize, 0usize);
    for change in changes {
        if change.offset > base_position {
            pieces.push(DraftPiece::Keep { base: base_position, length: change.offset - base_position });
            covered += change.offset - base_position;
        }
        let insert: Vec<char> = change.insert.chars().collect();
        covered += insert.len();
        pieces.push(DraftPiece::Change { base: change.offset, remove: change.delete, insert });
        base_position = change.offset + change.delete;
    }
    if current_length > covered {
        pieces.push(DraftPiece::Keep { base: base_position, length: current_length - covered });
    }
    let mut out: Vec<DraftPiece> = Vec::new();
    let (mut base_now, mut placed, mut at) = (0usize, false, 0usize);
    macro_rules! push {
        ($piece:expr) => {{
            let piece = $piece;
            base_now = piece.base_end();
            out.push(piece);
        }};
    }
    for piece in pieces {
        let length = match &piece {
            DraftPiece::Keep { length, .. } => *length,
            DraftPiece::Change { insert, .. } => insert.len(),
        };
        let (from, to) = (at, at + length);
        at = to;
        match piece {
            DraftPiece::Keep { base, .. } => {
                let left_end = from.max(to.min(start));
                if left_end > from {
                    push!(DraftPiece::Keep { base, length: left_end - from });
                }
                if !placed && start >= from && start <= to {
                    out.push(DraftPiece::Change { base: base_now, remove: 0, insert: inserted.clone() });
                    placed = true;
                }
                let (middle_from, middle_to) = (from.max(start), to.min(end));
                if middle_to > middle_from {
                    push!(DraftPiece::Change { base: base + (middle_from - from), remove: middle_to - middle_from, insert: Vec::new() });
                }
                let right_from = from.max(end);
                if to > right_from {
                    push!(DraftPiece::Keep { base: base + (right_from - from), length: to - right_from });
                }
            }
            DraftPiece::Change { base, remove, insert } => {
                let left_length = length.min(start.saturating_sub(from));
                let right_from = length.min(end.saturating_sub(from));
                if !placed && start >= from && start <= to {
                    push!(DraftPiece::Change { base, remove, insert: insert[..left_length].to_vec() });
                    out.push(DraftPiece::Change { base: base_now, remove: 0, insert: inserted.clone() });
                    placed = true;
                    if right_from < length {
                        push!(DraftPiece::Change { base: base + remove, remove: 0, insert: insert[right_from..].to_vec() });
                    }
                } else {
                    let kept: Vec<char> = insert[..left_length].iter().chain(&insert[right_from..]).copied().collect();
                    push!(DraftPiece::Change { base, remove, insert: kept });
                }
            }
        }
    }
    if !placed {
        out.push(DraftPiece::Change { base: base_now, remove: 0, insert: inserted });
    }
    let mut merged: Vec<(usize, usize, Vec<char>)> = Vec::new();
    let mut open = false;
    for piece in out {
        match piece {
            DraftPiece::Keep { .. } => open = false,
            DraftPiece::Change { base, remove, insert } => match merged.last_mut() {
                Some(last) if open => {
                    last.1 += remove;
                    last.2.extend(insert);
                }
                _ => {
                    merged.push((base, remove, insert));
                    open = true;
                }
            },
        }
    }
    merged.into_iter().filter(|(_, remove, insert)| *remove > 0 || !insert.is_empty()).map(|(offset, delete, insert)| DraftChange { offset, delete, insert: insert.into_iter().collect() }).collect()
}


fn draft_change_node<'a>(changes:&'a[DraftChange],path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'a>,protocol::value::ValueError>{
    use protocol::value::{Number,ValueError,ValueRefusalKind};
    use semio_framework_pack_json::JsonWriteNode;
    let fault=||ValueError::literal(ValueRefusalKind::InvariantViolated,"draft wire path leaves original change owner");
    match path {
        []=>Ok(JsonWriteNode::Array(changes.len())),
        [index] if *index<changes.len()=>Ok(JsonWriteNode::Object(3)),
        [index,field]=>{
            let change=changes.get(*index).ok_or_else(fault)?;
            match field {
                0|1=>{let value=if *field==0{change.offset}else{change.delete};let value=u64::try_from(value).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"draft range exceeds wire integer"))?;Ok(JsonWriteNode::Number(Number::UInt(value)))},
                2=>Ok(JsonWriteNode::String(&change.insert)),
                _=>Err(fault()),
            }
        },
        _=>Err(fault()),
    }
}

fn draft_change_key(changes:&[DraftChange],path:&[usize],index:usize)->Result<&'static str,protocol::value::ValueError>{
    if matches!(path,[row]if *row<changes.len()){
        return ["offset","delete","insert"].get(index).copied().ok_or_else(||protocol::value::ValueError::literal(protocol::value::ValueRefusalKind::InvariantViolated,"draft wire field leaves original change owner"));
    }
    Err(protocol::value::ValueError::literal(protocol::value::ValueRefusalKind::InvariantViolated,"draft wire key leaves original change owner"))
}

struct DraftChangesBorrowed<'a>(&'a[DraftChange]);
impl semio_framework_pack_json::JsonWriteSource for DraftChangesBorrowed<'_>{
    fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,protocol::value::ValueError>{draft_change_node(self.0,path)}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,protocol::value::ValueError>{draft_change_key(self.0,path,index)}
}

/// 📡️ Writes original ranges through the caller's borrowed sink, preserving its accepted prefix on refusal.
pub fn write_draft_changes_json_into<E:From<protocol::value::ValueError>,F>(changes:&[DraftChange],maximum_bytes:u64,maximum_depth:usize,maximum_items:u64,output:&mut F,control:&mut protocol::value::NativeEncodeControl<'_>)->Result<u64,E>
where F:FnMut(&[u8],&mut protocol::value::NativeEncodeControl<'_>)->Result<(),E>{
    semio_framework_pack_json::write_json_source_into(&DraftChangesBorrowed(changes),maximum_bytes,maximum_depth,maximum_items,output,control)
}

struct DraftChangesOwner(Vec<DraftChange>);
impl semio_framework_pack_json::JsonWriteSource for DraftChangesOwner{
    fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,protocol::value::ValueError>{draft_change_node(&self.0,path)}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,protocol::value::ValueError>{draft_change_key(&self.0,path,index)}
}
impl protocol::value::retirement::RetireOwned for DraftChange{
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor>{use protocol::value::retirement::RetireOwned;(self.offset,self.delete,self.insert).retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{use protocol::value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes::<usize>(),deferred_birth_bytes::<usize>(),deferred_birth_bytes::<String>()])}
    fn controlled_retirement_supported()->bool{true}
}
impl protocol::value::retirement::RetireOwned for DraftChangesOwner{
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor>{use protocol::value::retirement::RetireOwned;self.0.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{use protocol::value::retirement::RetireOwned;self.0.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
}

/// 📄️ Retains the original typed changes and canonical JSON frontier across caller-controlled frame hops.
pub struct DraftChangesJsonCursor{writer:semio_framework_pack_json::JsonWriteCursor<DraftChangesOwner>}
impl DraftChangesJsonCursor{
    /// 🌱️ Moves existing ranges without allocation, refusing structural policy before consuming their owner.
    pub fn new(changes:Vec<DraftChange>,maximum_items:usize,maximum_depth:usize)->Result<Self,(protocol::value::ValueError,Vec<DraftChange>)>{
        use protocol::value::{ValueError,ValueRefusalKind};
        if maximum_depth<if changes.is_empty(){1}else{3}{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"draft wire exceeds caller depth"),changes));}
        if changes.len()>maximum_items||(!changes.is_empty()&&maximum_items<3){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"draft wire exceeds caller item policy"),changes));}
        Ok(Self{writer:semio_framework_pack_json::JsonWriteCursor::new(DraftChangesOwner(changes))})
    }
    /// ⏱️ Uses one cumulative admission for bounded source traversal and physical wire output.
    pub fn step(&mut self,maximum_units:usize,control:&mut protocol::value::NativeEncodeControl<'_>)->Result<Option<String>,protocol::value::ValueError>{self.writer.step(maximum_units,control)}
    /// 📊️ Reports measured or emitted bytes without scanning retained source fields.
    pub fn progress(&self)->(usize,bool){self.writer.progress()}
    /// 📤️ Returns the same typed ranges once the complete wire owner has transferred.
    pub fn take_changes(&mut self)->Option<Vec<DraftChange>>{self.writer.take_source().map(|owner|owner.0)}
}
impl protocol::value::retirement::RetireOwned for DraftChangesJsonCursor{
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor>{use protocol::value::retirement::RetireOwned;self.writer.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{use protocol::value::retirement::RetireOwned;self.writer.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
}

/// ✂️ Applies a change set (ranges in the coordinates of `text`) to `text`; `None` when a range leaves the text or overlaps the previous one.
pub fn apply_draft_changes(text: &str, changes: &[DraftChange]) -> Option<String> {
    let hay: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut position = 0usize;
    for change in changes {
        if change.offset < position || change.offset + change.delete > hay.len() {
            return None;
        }
        out.extend(&hay[position..change.offset]);
        out.push_str(&change.insert);
        position = change.offset + change.delete;
    }
    out.extend(&hay[position..]);
    Some(out)
}
//#endregion ✂️DraftChangeSet

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/✂️text-splice/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
