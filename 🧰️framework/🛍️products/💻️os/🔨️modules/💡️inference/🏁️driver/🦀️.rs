//! 🏁️ Driver of the inference cache: the step, whole-field, update and diff entry points and the event, commit and drive helpers behind them. See [`super`].

use super::*;

//#region 🔖️Driver
pub(super) fn encode<T:ToValue>(value:&T)->Vec<u8>{semio_framework_pack_json::to_json_string(value).into_bytes()}

fn key_text<K: ToValue>(key: &K) -> String {
    String::from_utf8_lossy(&encode(key)).into_owned()
}

/// ⏩️ Advances original graph, typed parent, compute, publication and close events within one actual caller slice.
/// The source snapshot and ordinal value owner remain retained across denied turns. Cache publication requires a funded producer.
pub fn infer_field_step<P, F: OriginalInferredField<P>>(snapshot: &P, source:&F::Source, _cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values:&mut InferenceValues<F::Value>,cx:&mut semio_framework_job::StepContext<'_>)->Result<InferenceStepReport,InferenceWorkFailure>{
    let before=cx.fuel_remaining();let computed=cursor.next;
    loop{
        let work=cx.original_work_units_spent();original_field_event::<P,F>(snapshot,source,cursor,values,cx)?;
        if cursor.is_done()||cx.original_work_units_spent()==work||cx.should_yield()||cx.retained_grant().maximum_items==0{break}
    }
    Ok(InferenceStepReport{done:cursor.is_done(),fuel_used:usize::try_from(before.checked_sub(cx.fuel_remaining()).expect("original inference CPU is monotone")).unwrap_or(usize::MAX),computed:cursor.next-computed,hits:0,progress:cursor.fraction()})
}

pub(super) fn original_field_metadata(cx:&mut semio_framework_job::StepContext<'_>,copy:usize)->Result<bool,InferenceWorkFailure>{
    let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}
    cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}

fn original_field_event<P,F:OriginalInferredField<P>>(snapshot:&P,source:&F::Source,cursor:&mut InferenceCursor<F::Key>,values:&mut InferenceValues<F::Value>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),InferenceWorkFailure>{
    use semio_framework_value::{ValueError,ValueRefusalKind};
    let changed=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inferred field source, parent or output changed");
    if cursor.cancelled||cx.is_cancelled(){return Err(InferenceError::Cancelled.into())}
        F::validate_source(snapshot,source,cx)?;
    let field_source=std::ptr::from_ref(source).cast::<()>()as usize;
    if cursor.original_field_source.is_some_and(|original|original!=field_source){return Err(changed().into())}
if let Some(scope)=cursor.original_scope.as_ref(){if !scope.matches(cx)?{return Err(changed().into())}}
    let identity=(cx.operation().0,cx.generation().0,F::source_identity(snapshot,source));
    if let Some(original)=cursor.original_caller{if original!=identity{return Err(changed().into())}}
    if cx.should_yield(){return Ok(())}
    if cursor.original_caller.is_none(){
        let scope=OriginalInferenceScope::capture(cx)?;
        if !original_field_metadata(cx,0)?{return Ok(())}
        cursor.original_caller=Some(identity);cursor.original_field_source=Some(field_source);cursor.original_scope=Some(scope);
        if F::plan_source_extent(snapshot,source).is_some(){cursor.original_plan=Some(OriginalInferencePlan::default())}
        return Ok(())
    }
    let Some(plan)=cursor.original_plan.as_mut()else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original inferred field has no borrowed graph producer").into())};
    if !plan.complete(){plan.step::<P,F>(snapshot,source,cx)?;return Ok(())}
    if cursor.next>=plan.len(){return Ok(())}
    let extent=F::plan_source_extent(snapshot,source).ok_or_else(changed)?.0;
    if !values.admit_source_step(cx)?{return Ok(())}
    if !values.initialize_step(extent,cx)?{return Ok(())}
    let ordinal=plan.source_ordinal(cursor.next).ok_or_else(changed)?;
    let key=F::plan_source_key(snapshot,source,ordinal).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original inferred field has no borrowed typed key producer"))?;
    match cursor.original_phase{
        OriginalInferencePhase::CountParents=>{
            if !original_field_metadata(cx,size_of::<usize>())?{return Ok(())}
            if cursor.next>=plan.cyclic_start()||cursor.original_edge==plan.edge_count(){cursor.original_edge=0;cursor.original_phase=OriginalInferencePhase::ReserveParents}
            else{let(_,child)=plan.edge_source_ordinals(cursor.original_edge).ok_or_else(changed)?;if child==ordinal{cursor.original_parent_count=cursor.original_parent_count.checked_add(1).ok_or_else(changed)?}cursor.original_edge+=1}
        }
        OriginalInferencePhase::ReserveParents=>{
            if !values.reserve_parents_step(cursor.original_parent_count,cx)?{return Ok(())}
            if original_field_metadata(cx,0)?{if cursor.next>=plan.cyclic_start(){cursor.original_edge=plan.edge_count()}cursor.original_phase=OriginalInferencePhase::CopyParents}
        }
        OriginalInferencePhase::CopyParents=>{
            if values.parent.is_some(){if values.push_parent_step(cx)?{cursor.original_edge+=1}return Ok(())}
            if cursor.original_edge==plan.edge_count(){if original_field_metadata(cx,0)?{cursor.original_phase=OriginalInferencePhase::Compute}return Ok(())}
            let(parent,child)=plan.edge_source_ordinals(cursor.original_edge).ok_or_else(changed)?;
            if child!=ordinal{if original_field_metadata(cx,0)?{cursor.original_edge+=1}return Ok(())}
            let original=values.rows.get(parent).and_then(Option::as_ref).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original inferred field parent is missing from funded plan order"))?;
            F::clone_original_parent(original,&mut values.parent,cx)?;
        }
        OriginalInferencePhase::Compute=>match if cursor.next>=plan.cyclic_start(){F::compute_cyclic_step(snapshot,source,key,&mut cursor.pending,cx)?}else{F::compute_step(snapshot,source,key,&values.parents,&mut cursor.pending,cx)?}{
            ComputeStep::Working{progress,..}=>cursor.progress=progress,
            ComputeStep::Done{value,..}=>{values.output=Some(value);cursor.progress=1.0;cursor.original_phase=OriginalInferencePhase::ClosePending}
        },
        OriginalInferencePhase::ClosePending=>{if cursor.close_original_pending_step(cx)?&&original_field_metadata(cx,0)?{cursor.original_phase=OriginalInferencePhase::CloseParents}},
        OriginalInferencePhase::CloseParents=>{if values.close_parents_step(cx)?&&original_field_metadata(cx,0)?{cursor.original_phase=OriginalInferencePhase::Publish}},
        OriginalInferencePhase::Publish=>{if values.publish_output_step(ordinal,cx)?{cursor.original_phase=OriginalInferencePhase::Advance}},
        OriginalInferencePhase::Advance=>{if original_field_metadata(cx,size_of::<usize>())?{cursor.next+=1;cursor.progress=0.0;cursor.original_edge=0;cursor.original_parent_count=0;cursor.original_phase=OriginalInferencePhase::CountParents}}
    }
    Ok(())
}

fn missing<K: ToValue>(key: &K, parent: &K) -> InferenceError {
    InferenceError::MissingParent { key: key_text(key), parent: key_text(parent) }
}

fn commit<K: Clone + Eq + std::hash::Hash + Ord + ToValue, V>(values: &mut BTreeMap<K, V>, hashes: &mut HashMap<K, DepHash>, track: Option<&mut Track<K, V>>, step: &InferenceStep<K>, hash: Option<DepHash>, value: V) {
    let old = values.insert(step.key.clone(), value);
    match (track, hash) {
        (Some(track), Some(hash)) => track.record(step, hash, old),
        (None, Some(hash)) => {
            hashes.insert(step.key.clone(), hash);
        }
        _ => {}
    }
}

fn drive<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values:&mut BTreeMap<F::Key,F::Value>,hashing:bool,mut track:Option<&mut Track<F::Key,F::Value>>)->Result<InferenceStepReport,InferenceWorkFailure> {
    if cursor.cancelled {
        return Err(InferenceError::Cancelled.into());
    
    }
    if cursor.plan.is_none(){cursor.plan=Some(F::plan(snapshot));}
    let InferenceCursor { plan, next, hashes, pending, progress, .. } = &mut *cursor;
    let plan = plan.as_deref().expect("the plan is set above");

    let mut report = InferenceStepReport::default();

    while *next < plan.len() {
        
        let step = &plan[*next];
        if pending.is_none() && track.as_deref_mut().is_some_and(|track| track.carries::<P, F>(snapshot, step, values)) {
            *next += 1;
            continue;
        }
        let dep_hash = if hashing {
            let mut parent_hashes: Vec<DepHash> = Vec::with_capacity(step.parents.len());
            for parent in &step.parents {
                let hash = match track.as_deref() {
                    Some(track) => track.hash_of(parent),
                    None => hashes.get(parent).copied(),
                };
                parent_hashes.push(hash.ok_or_else(|| missing(&step.key, parent))?);
            }
            let input = encode(&F::dep_input(snapshot, &step.key, &step.parents));
            Some(if step.parents.is_empty() { DepHash::root(F::FIELD_ID, F::SCHEMA_VERSION, &input) } else { DepHash::chain(F::FIELD_ID, F::SCHEMA_VERSION, &input, &parent_hashes) })
        } else {
            None
        };

        if pending.is_none() {
            if let Some((hash, track)) = dep_hash.zip(track.as_deref_mut()) {
                if track.confirms(step, hash, values) {
                    *next += 1;
                    continue;
                }
            }
            if let Some((hash, value)) = dep_hash.zip(cache.as_deref_mut()).and_then(|(hash, cache)| cache.get::<F::Value>(hash).map(|value| (hash, value))) {
                commit(values, hashes, track.as_deref_mut(), step, Some(hash), value);
                *next += 1;
                report.hits += 1;
                continue;
            }
        }

        let mut parent_values: Vec<F::Value> = Vec::with_capacity(step.parents.len());
        for parent in &step.parents {
            parent_values.push(values.get(parent).cloned().ok_or_else(|| missing(&step.key, parent))?);
        }
        let outcome=Ok::<_,InferenceWorkFailure>(ComputeStep::Done{value:F::compute(snapshot,&step.key,&parent_values),fuel_used:1});
        let spent=1;
        match outcome {
            Err(fault) => {
                return Err(fault);
            }
            Ok(ComputeStep::Working { fuel_used, progress: partial }) => {
                *progress = partial;
                let _=fuel_used;report.fuel_used+=spent;
                break;
            }
            Ok(ComputeStep::Done { value, fuel_used }) => {
                *pending = None;
                *progress = 0.0;
                if let Some((hash, cache)) = dep_hash.zip(cache.as_deref_mut()).filter(|(_, cache)| cache.enabled()) {
                    cache.insert(hash, value.clone(), F::value_bytes(&value));
                }
                commit(values, hashes, track.as_deref_mut(), step, dep_hash, value);
                *next += 1;
                report.computed += 1;
                let _=fuel_used;
                report.fuel_used += spent;
                
            }
        }
    }

    report.done = *next >= plan.len();
    if let Some(track) = track.filter(|_| report.done) {
        track.sweep(values, plan);
    }
    report.progress = cursor.fraction();
    Ok(report)
}

/// 🧮️ Computes a complete reference projection; interactive production uses [`infer_field_step`].
pub fn try_infer_field<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>) -> Result<BTreeMap<F::Key, F::Value>, InferenceError> {
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    let hashing = cache.as_deref().is_some_and(InferenceCache::enabled);
    loop {
        if drive::<P,F>(snapshot,cache.as_deref_mut(),&mut cursor,&mut values,hashing,None).map_err(|original|match original{InferenceWorkFailure::Driver(primary)=>primary,_=>unreachable!("explicit pure inference does not invoke an original caller producer")})?.done {
            return Ok(values);
        }
    }
}

/// ⏩ The plain driver: [`try_infer_field`] for fields whose plan is a topological order and whose compute is total; a violation of that contract is a loud failure, never a silently misaligned compute.
pub fn infer_field<P, F: InferredField<P>>(snapshot: &P, cache: Option<&mut InferenceCache>) -> BTreeMap<F::Key, F::Value> {
    try_infer_field::<P, F>(snapshot, cache).unwrap_or_else(|error| panic!("{error}"))
}

/// ⏭️ One slice of an [`InferenceSession::begin_update`]: walks the plan of the new snapshot with the same fuel and cancellation as [`infer_field_step`], but an entity that neither the diff
/// ([`InferredField::touched_by`]) nor a moved parent reaches is carried with its previous value and hash (no dependency evaluation, no hashing), and an entity whose dependency hash did not move keeps its previous value
/// (no compute); only the rest is looked up in `cache` or computed. Entities that left the plan are removed when the plan is done.
pub fn update_field_cold<P, F: InferredField<P>>(snapshot: &P, cache: &mut InferenceCache, update:&mut FieldUpdate<F::Key,F::Value>)->Result<InferenceStepReport,InferenceWorkFailure> {
    if update.gated {
        return Ok(InferenceStepReport { done: true, progress: 1.0, ..InferenceStepReport::default() });
    }
    let report = drive::<P,F>(snapshot,Some(cache),&mut update.cursor,&mut update.values,true,Some(&mut update.track))?;
    update.computed += report.computed;
    update.hits += report.hits;
    Ok(report)
}

/// ⏩ The unbounded diff-driven update: [`InferenceSession::begin_update`], [`update_field_cold`] to the end and [`InferenceSession::finish_update`]. The values stay in the session ([`InferenceSession::field_values`]);
/// the delta names the entities that moved, so a consumer copies only those. Fails like [`try_infer_field`]; the session then distrusts its values and the next update examines every entity.
pub fn infer_field_delta<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> Result<FieldDelta<F::Key, F::Value>, InferenceError>
where
    F: InferredField<P>,
    D: semio_framework_os_command::DiffRegions,
{
    let mut update = session.begin_update::<P, F, D>(diff, cache.enabled());
    let outcome = loop {
        match drive::<P,F>(snapshot,Some(cache),&mut update.cursor,&mut update.values,true,Some(&mut update.track)).map_err(|original|match original{InferenceWorkFailure::Driver(primary)=>primary,_=>unreachable!("explicit pure inference update does not invoke an original caller producer")}) {
            Ok(report) if report.done => break Ok(()),
            Ok(_) => {}
            Err(error) => break Err(error),
        }
    };
    let delta = session.finish_update(update);
    outcome.map(|()| delta)
}

/// ⏩ Diff-gated variant: if `diff.touches()` doesn't intersect `F::reads()`, returns the session's
/// previous full result for this field unchanged (tier-1 gate) instead of walking the plan at all.
/// Otherwise it updates the session through [`infer_field_delta`] and returns a copy of the values.
pub async fn infer_field_after_diff<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> BTreeMap<F::Key, F::Value>
where
    F: InferredField<P>,
    D: semio_framework_os_command::DiffRegions,
{
    infer_field_delta::<P, F, D>(snapshot, diff, session, cache).unwrap_or_else(|error| panic!("{error}"));
    session.field_values::<P, F>().cloned().unwrap_or_default()
}
//#endregion 🔖️Driver
