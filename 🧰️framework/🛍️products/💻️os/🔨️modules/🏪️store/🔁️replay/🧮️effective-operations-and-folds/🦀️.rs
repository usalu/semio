//! 🧮️ Effective operations and folds: the pair merge, the effective operation with its drop, the supersession aware effective operations, the replacement admission and the effective edit, operation and recorded folds. See [`super`].

use super::*;

/// 🫱️ What merging a persisted pair into a live store found ([`ArtifactStore::merge_persisted_history`]): `merged` events the
/// store took from the pair, and `ahead` events the store holds that the pair lacks — what a writer persists again for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PairMerge {
    pub merged: usize,
    pub ahead: usize,
}

/// 🎭️ One forward operation as every fold site reads it: the recorded original, its decoded replacement, or nothing
/// when withdrawn or when its supersession breaks the supersede law ([`admit_replacement`]). A decoded replacement
/// retires itself through [`Mutation::retire_cold`].
pub struct EffectiveOperation<'a, P, Mutation>
where
    Mutation: super::Mutation<P>,
{
    pub op_index: u32,
    pub mutation_id: MutationId,
    pub original: &'a Mutation,
    pub replacement: Option<Mutation>,
    pub withdrawn: bool,
    pub fault: Option<semio_framework_os_spr::MutationMessage>,
    pub marker: PhantomData<fn() -> P>,
}

impl<P, Mutation> EffectiveOperation<'_, P, Mutation>
where
    Mutation: super::Mutation<P>,
{
    /// ▶️ The operation a fold applies, `None` when withdrawn or faulted.
    pub fn operation(&self) -> Option<&Mutation> {
        if self.withdrawn || self.fault.is_some() {
            return None;
        }
        Some(self.replacement.as_ref().unwrap_or(self.original))
    }

    /// 🔀️ Whether a supersession replaced or withdrew the recorded forward (validly or not).
    pub fn superseded(&self) -> bool {
        self.withdrawn || self.replacement.is_some() || self.fault.is_some()
    }

    /// 🚫️ Whether a valid supersession withdrew the operation.
    pub fn withdrawn(&self) -> bool {
        self.withdrawn
    }

    /// 🧯️ The `Fatal` message of a supersession breaking the supersede law; the operation then folds as a no-op.
    pub fn fault(&self) -> Option<&semio_framework_os_spr::MutationMessage> {
        self.fault.as_ref()
    }
}

impl<P, Mutation> Drop for EffectiveOperation<'_, P, Mutation>
where
    Mutation: super::Mutation<P>,
{
    fn drop(&mut self) {
        if let Some(replacement) = self.replacement.take() {
            super::Mutation::<P>::retire_cold(replacement);
        }
    }
}

/// 🔍️ The effective input of `operation` — operation `op_index` of its edit, identified as `mutation_id` — under
/// `supersessions` in a document of `schema`. Total: a supersession breaking the supersede law ([`admit_replacement`])
/// folds the operation as a no-op carrying one `Fatal` `mutation.invariant` message that targets the operation —
/// identically at every fold site, so a garbage or foreign replacement never aborts a projection.
pub fn effective_operation<'a, P, Mutation>(operation: &'a Mutation, op_index: u32, mutation_id: MutationId, schema: &str, supersessions: &EffectiveSupersessions) -> EffectiveOperation<'a, P, Mutation>
where
    Mutation: super::Mutation<P> + OpBinary,
{
    let supersession = supersessions.get(&mutation_id);
    effective_operation_with_supersession::<P, Mutation>(operation, op_index, mutation_id, schema, supersession)
}

pub fn effective_operation_with_supersession<'a, P, Mutation>(operation: &'a Mutation, op_index: u32, mutation_id: MutationId, schema: &str, supersession: Option<&protocol::EffectiveSupersession>) -> EffectiveOperation<'a, P, Mutation>
where
    Mutation: super::Mutation<P> + OpBinary,
{
    let mut effective = EffectiveOperation { op_index, mutation_id, original: operation, replacement: None, withdrawn: false, fault: None, marker: PhantomData };
    let Some(supersession) = supersession else { return effective };
    match admit_replacement::<P, Mutation>(operation, &supersession.replacement, schema) {
        Ok(Some(replacement)) => effective.replacement = Some(replacement),
        Ok(None) => effective.withdrawn = true,
        Err(reason) => {
            let mut fault =
                semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", format!("supersession {} of {} {reason}; the operation folds as a no-op", supersession.transition_id, effective.mutation_id.0)).at([effective.mutation_id.0.clone()]);
            fault.op_index = Some(op_index);
            effective.fault = Some(fault);
        }
    }
    effective
}

/// 🛂️ The supersede law every input obeys — authored, ingested, loaded or folded. A withdrawal is admitted for every
/// operation. A replacement is an operation of `schema` in its canonical encoding that plans no foreign steps; an operation
/// that plans foreign steps takes no replacement but its own recorded input — what the undo of its withdrawal authors —
/// because any other input would plan steps the other documents never received. Answers the decoded replacement (`None` for
/// a withdrawal) or why the input breaks the law.
pub fn admit_replacement<P, Mutation>(original: &Mutation, replacement: &protocol::InputReplacement, schema: &str) -> Result<Option<Mutation>, String>
where
    Mutation: super::Mutation<P> + OpBinary,
{
    let protocol::InputReplacement::Input { schema: named, payload } = replacement else { return Ok(None) };
    if named != schema {
        return Err(format!("names schema {named}, not {schema}"));
    }
    let decoded = Mutation::decode_op(payload).map_err(|error| format!("does not decode: {error}"))?;
    let refusal = match decoded.encode_op() {
        Err(error) => Some(format!("does not re-encode: {error}")),
        Ok(recoded) if recoded != *payload => Some("is not canonically encoded".to_string()),
        Ok(recoded) if original.may_emit_foreign_steps() => (!original.encode_op().is_ok_and(|recorded| recorded == recoded)).then(|| "replaces the input of an operation that plans foreign steps".to_string()),
        Ok(_) if decoded.may_emit_foreign_steps() => Some("plans foreign steps".to_string()),
        Ok(_) => None,
    };
    match refusal {
        None => Ok(Some(decoded)),
        Some(reason) => {
            super::Mutation::<P>::retire_cold(decoded);
            Err(reason)
        }
    }
}

/// 📜️ `edit`'s effective forwards in operation order ([`effective_operation`]). The single accessor every fold site
/// reads an edit through.
pub fn effective_forwards<'a, P, Mutation>(edit: &'a Edit<Mutation>, schema: &'a str, supersessions: &'a EffectiveSupersessions) -> impl Iterator<Item = EffectiveOperation<'a, P, Mutation>> + 'a
where
    P: 'a,
    Mutation: super::Mutation<P> + OpBinary + 'a,
{
    let ids = semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit);
    edit.forwards.iter().zip(ids).enumerate().map(move |(index, (operation, mutation_id))| effective_operation::<P, Mutation>(operation, index as u32, mutation_id, schema, supersessions))
}

/// 🔂️ Folds the first `take` effective forwards of `edit` into `projection` with keep-and-record semantics
/// ([`fold_operation`]) — the one loop every messageless fold site shares.
pub fn fold_effective_edit<P, Mutation>(projection: &mut impl FoldProjection<P, Mutation>, edit: &Edit<Mutation>, schema: &str, supersessions: &EffectiveSupersessions, take: usize)
where
    Mutation: super::Mutation<P> + OpBinary,
{
    if supersessions.is_empty() {
        for (index, operation) in edit.forwards.iter().enumerate().take(take) {
            if let (Some(next), _) = fold_operation::<P, Mutation>(projection.projection(), operation, index as u32) {
                projection.advance(next);
            }
        }
        return;
    }
    for effective in effective_forwards::<P, Mutation>(edit, schema, supersessions).take(take) {
        if let Some(operation) = effective.operation() {
            if let (Some(next), _) = fold_operation::<P, Mutation>(projection.projection(), operation, effective.op_index) {
                projection.advance(next);
            }
        }
    }
}

/// 🛡️ Applies an already computed outcome through [`protocol::apply_diff`]: the next projection with the outcome intact, or —
/// on an apply refusal — the unchanged `base`, an empty diff and one `Fatal` message. The builder-side twin of the fatal
/// message production dispatch records.
pub fn apply_outcome<P, D>(base: &P, outcome: semio_framework_os_spr::MutationOutcome<D>) -> (P, semio_framework_os_spr::MutationOutcome<D>)
where
    P: Clone,
    D: MutationDiff<P>,
{
    let (diff, mut messages) = outcome.into_parts();
    match semio_framework_os_spr::apply_diff(&diff, base) {
        Ok(next) => (next, semio_framework_os_spr::MutationOutcome::new(diff).absorb_messages(messages)),
        Err(error) => {
            messages.push(semio_framework_os_spr::MutationMessage::fatal(error.code, error.message).at(error.target));
            D::retire_cold(diff);
            (base.clone(), semio_framework_os_spr::MutationOutcome::new(D::default()).absorb_messages(messages))
        }
    }
}

/// 🎬️ One operation that never drops a projection: the next projection when its diff applied, and its op-stamped
/// messages — an apply refusal becomes one `Fatal` message and folds as a no-op. The keep-and-record step every fold
/// site, the retained initializers included, takes.
pub fn fold_operation<P, Mutation>(state: &P, operation: &Mutation, op_index: u32) -> (Option<P>, Vec<semio_framework_os_spr::MutationMessage>)
where
    Mutation: super::Mutation<P>,
{
    let (applied, mut messages) = apply_operation::<P, Mutation>(state, operation, op_index);
    match applied {
        Ok(next) => (Some(next), messages),
        Err(error) => {
            let mut fatal = semio_framework_os_spr::MutationMessage::fatal(error.code, error.message).at(error.target);
            fatal.op_index = Some(op_index);
            messages.push(fatal);
            (None, messages)
        }
    }
}

/// 📼️ Folds `applied[from..to]` onto `base` under `supersessions` with keep-and-record semantics, handing out the
/// projection after every prefix length below `to` that is a multiple of `stride` (aliases of the running state, no
/// copies). A failure retires the running state and everything recorded.
pub fn fold_recorded<P, Mutation>(
    base: Arc<P>,
    applied: &[String],
    from: usize,
    to: usize,
    edits: &ArtifactHistoryLedger<Edit<Mutation>>,
    schema: &str,
    supersessions: &EffectiveSupersessions,
    stride: usize,
) -> Result<(Arc<P>, Vec<(usize, Arc<P>)>), VcsError>
where
    Mutation: super::Mutation<P> + OpBinary,
{
    let mut running = base;
    let mut recorded: Vec<(usize, Arc<P>)> = Vec::new();
    let mut failure = None;
    for (offset, edit_id) in applied[from..to].iter().enumerate() {
        let Some(edit) = edits.find_near(|edit| edit.id == *edit_id) else {
            failure = Some(VcsError::UnknownEdit(edit_id.clone()));
            break;
        };
        fold_effective_edit(&mut running, edit, schema, supersessions, usize::MAX);
        let length = from + offset + 1;
        if length < to && length % stride == 0 {
            recorded.push((length, Arc::clone(&running)));
        }
    }
    if let Some(error) = failure {
        retire_shared_projection::<P, Mutation>(running);
        for (_, snapshot) in recorded {
            retire_shared_projection::<P, Mutation>(snapshot);
        }
        return Err(error);
    }
    Ok((running, recorded))
}
