//! 🏁️ Driver of the inference cache: the step, whole-field, update and diff entry points and the event, commit and drive helpers behind them. See [`super`].

use super::*;

//#region 🔖️Driver
pub(super) fn encode<T:ToValue>(value:&T)->Vec<u8>{semio_framework_pack_json::to_json_string(value).into_bytes()}

fn key_text<K: ToValue>(key: &K) -> String {
    String::from_utf8_lossy(&encode(key)).into_owned()
}

/// ⏩️ Advances original graph, typed parent, compute, publication and close events within one actual caller slice.
/// The source snapshot and ordinal value owner remain retained across denied turns. Cache publication requires a funded producer.
pub fn infer_field_step<P, F: OriginalInferredField<P>>(snapshot: &P, source:&F::Source, mut cache: Option<&mut OriginalInferenceCache<F::Value>>, cursor: &mut InferenceCursor<F::Key>, values:&mut InferenceValues<F::Value>,cx:&mut semio_framework_job::StepContext<'_>)->Result<InferenceStepReport,InferenceWorkFailure>{
    cache = cache.filter(|cache| cache.enabled());
    let before = cx.fuel_remaining();
    let completed = cursor.next;
    let hits = cursor.original_hits;
    loop{
        let work=cx.original_work_units_spent();original_field_event::<P,F>(snapshot,source,cache.as_deref_mut(),cursor,values,cx)?;
        if cursor.is_done()||cx.original_work_units_spent()==work||cx.should_yield()||cx.retained_grant().maximum_items==0{break}
    }
    let hits = cursor.original_hits - hits;
    Ok(InferenceStepReport{done:cursor.is_done(),fuel_used:usize::try_from(before.saturating_sub(cx.fuel_remaining())).unwrap_or(usize::MAX),computed:cursor.next-completed-hits,hits,progress:cursor.fraction()})
}

pub(super) fn original_field_metadata(cx:&mut semio_framework_job::StepContext<'_>,copy:usize)->Result<bool,InferenceWorkFailure>{
    let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}
    cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}

fn original_field_event<P,F:OriginalInferredField<P>>(snapshot:&P,source:&F::Source,cache:Option<&mut OriginalInferenceCache<F::Value>>,cursor:&mut InferenceCursor<F::Key>,values:&mut InferenceValues<F::Value>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),InferenceWorkFailure>{
    use semio_framework_value::{ValueError,ValueRefusalKind};
    let changed=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inferred field source, parent or output changed");
    if cursor.cancelled||cx.is_cancelled(){return Err(InferenceError::Cancelled.into())}
        F::validate_source(snapshot,source,cx)?;
    let field_source=std::ptr::from_ref(source).cast::<()>()as usize;
    if cursor.original_field_source.is_some_and(|original|original!=field_source){return Err(changed().into())}
if let Some(scope)=cursor.original_scope.as_ref(){if !scope.matches(cx)?{return Err(changed().into())}}
    let identity=(cx.operation().0,cx.generation().0,F::source_identity(snapshot,source));
    let cache_identity = cache.as_deref().map(|cache| std::ptr::from_ref(cache).addr());
    if let Some(original)=cursor.original_caller{if original!=identity || cursor.original_cache != cache_identity{return Err(changed().into())}}
    if cx.should_yield(){return Ok(())}
    if cursor.original_caller.is_none(){
        let scope=OriginalInferenceScope::capture(cx)?;
        if !original_field_metadata(cx,0)?{return Ok(())}
        cursor.original_caller=Some(identity);cursor.original_field_source=Some(field_source);cursor.original_scope=Some(scope);
        cursor.original_cache = cache_identity;
        if cache_identity.is_none() { cursor.original_phase = OriginalInferencePhase::CountParents; }
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
        OriginalInferencePhase::Fingerprint => {
            if cache.is_none() || cursor.next >= plan.cyclic_start() {
                if original_field_metadata(cx, 0)? { cursor.original_phase = OriginalInferencePhase::CountParents; }
                return Ok(());
            }
            match F::original_dependency_step(snapshot, source, ordinal, &mut cursor.pending, cx)? {
                OriginalDependencyStep::Pending => {}
                OriginalDependencyStep::Ready(hash) => {
                    cursor.original_fingerprint = Some(hash);
                    cursor.original_phase = OriginalInferencePhase::CloseFingerprint;
                }
                OriginalDependencyStep::Uncacheable => {
                    if original_field_metadata(cx, 0)? { cursor.original_phase = OriginalInferencePhase::CloseFingerprint; }
                }
            }
        }
        OriginalInferencePhase::CloseFingerprint => {
            if cursor.close_original_pending_step(cx)? && original_field_metadata(cx, 0)? {
                cursor.original_phase = OriginalInferencePhase::FoldParents;
            }
        }
        OriginalInferencePhase::FoldParents => {
            if cursor.original_fingerprint.is_none() || cursor.original_edge == plan.edge_count() {
                if original_field_metadata(cx, 0)? {
                    cursor.original_edge = 0;
                    cursor.original_phase = OriginalInferencePhase::FindCached;
                }
                return Ok(());
            }
            let (parent, child) = plan.edge_source_ordinals(cursor.original_edge).ok_or_else(changed)?;
            if child == ordinal && cx.retained_grant().maximum_depth < values.rows.next_get_depth_demand(parent).map_err(ValueError::from)? {
                return Ok(());
            }
            if original_field_metadata(cx, if child == ordinal { 64 } else { 0 })? {
                if child == ordinal {
                    cursor.original_fingerprint = cursor.original_fingerprint.zip(values.fingerprint(parent)).map(|(hash, parent)| hash.with_parent(parent));
                }
                cursor.original_edge += 1;
            }
        }
        OriginalInferencePhase::FindCached => {
            let Some((cache, hash)) = cache.zip(cursor.original_fingerprint) else {
                if original_field_metadata(cx, 0)? { cursor.original_phase = OriginalInferencePhase::CountParents; }
                return Ok(());
            };
            let index = cursor.original_cache_lookup;
            if index == cache.extent() {
                if original_field_metadata(cx, 0)? {
                    cache.record_lookup(false);
                    cursor.original_phase = OriginalInferencePhase::CountParents;
                }
            } else if cx.retained_grant().maximum_depth >= cache.lookup_depth(index)? && original_field_metadata(cx, 2 * size_of::<DepHash>())? {
                match cache.fingerprint(index) {
                    Some(cached) if cached == hash => cursor.original_phase = OriginalInferencePhase::CopyCached,
                    cached => {
                        if cached.is_none() && cursor.original_cache_vacancy.is_none() { cursor.original_cache_vacancy = Some(index); }
                        cursor.original_cache_lookup += 1;
                    }
                }
            }
        }
        OriginalInferencePhase::CopyCached => {
            let cache = cache.ok_or_else(changed)?;
            let index = cursor.original_cache_lookup;
            if cx.retained_grant().maximum_depth >= cache.lookup_depth(index)?
                && F::clone_original_parent(cache.value(index).ok_or_else(changed)?, &mut values.output, cx)?
            {
                cache.record_lookup(true);
                cursor.original_row_hit = true;
                cursor.original_phase = OriginalInferencePhase::Publish;
            }
        }
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
            let original=values.rows.get(parent).and_then(|row|row.0.as_ref()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original inferred field parent is missing from funded plan order"))?;
            F::clone_original_parent(original,&mut values.parent,cx)?;
        }
        OriginalInferencePhase::Compute=>match if cursor.next>=plan.cyclic_start(){F::compute_cyclic_step(snapshot,source,key,&mut cursor.pending,cx)?}else{F::compute_step(snapshot,source,key,&values.parents,&mut cursor.pending,cx)?}{
            ComputeStep::Working{progress,..}=>cursor.progress=progress,
            ComputeStep::Done{value,..}=>{values.output=Some(value);cursor.progress=1.0;cursor.original_phase=OriginalInferencePhase::ClosePending}
        },
        OriginalInferencePhase::ClosePending=>{if cursor.close_original_pending_step(cx)?&&original_field_metadata(cx,0)?{cursor.original_phase=OriginalInferencePhase::CloseParents}},
        OriginalInferencePhase::CloseParents => {
            if values.close_parents_step(cx)? && original_field_metadata(cx, 0)? {
                cursor.original_phase = if cursor.original_fingerprint.is_some() { OriginalInferencePhase::CacheOutput } else { OriginalInferencePhase::Publish };
            }
        }
        OriginalInferencePhase::CacheOutput => {
            let output = values.output.as_ref().ok_or_else(changed)?;
            let entry = cache.zip(cursor.original_fingerprint).and_then(|(cache, hash)| {
                let bytes = F::original_cache_weight(output)?;
                cache.admits(bytes).then_some((cache, hash, bytes))
            });
            if let Some((cache, hash, bytes)) = entry {
                if values.parent.is_none() {
                    F::clone_original_parent(output, &mut values.parent, cx)?;
                } else if cache.insert_step(hash, bytes, &mut values.parent, &mut cursor.original_cache_vacancy, cx)? {
                    cursor.original_phase = OriginalInferencePhase::Publish;
                }
            } else if original_field_metadata(cx, 0)? {
                cursor.original_phase = OriginalInferencePhase::Publish;
            }
        }
        OriginalInferencePhase::Publish=>{if values.publish_output_step(ordinal,cursor.original_fingerprint,cx)?{cursor.original_phase=OriginalInferencePhase::Advance}},
        OriginalInferencePhase::Advance => {
            if original_field_metadata(cx, size_of::<usize>())? {
                cursor.next += 1;
                cursor.original_hits += usize::from(cursor.original_row_hit);
                cursor.progress = 0.0;
                cursor.original_edge = 0;
                cursor.original_parent_count = 0;
                cursor.original_fingerprint = None;
                cursor.original_cache_lookup = 0;
                cursor.original_cache_vacancy = None;
                cursor.original_row_hit = false;
                cursor.original_phase = if cache_identity.is_some() && cursor.next < plan.cyclic_start() { OriginalInferencePhase::Fingerprint } else { OriginalInferencePhase::CountParents };
            }
        }
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

fn drive<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values:&mut BTreeMap<F::Key,F::Value>,hashing:bool,mut track:Option<&mut Track<F::Key,F::Value>>,maximum_computes:usize)->Result<InferenceStepReport,InferenceWorkFailure> {
    if cursor.cancelled {
        return Err(InferenceError::Cancelled.into());
    
    }
    if cursor.plan.is_none(){cursor.plan=Some(F::plan(snapshot));}
    let InferenceCursor { plan, next, hashes, pending, progress, .. } = &mut *cursor;
    let plan = plan.as_deref().expect("the plan is set above");

    let mut report = InferenceStepReport::default();

    while *next < plan.len() && report.computed < maximum_computes {
        
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
        if drive::<P,F>(snapshot,cache.as_deref_mut(),&mut cursor,&mut values,hashing,None,usize::MAX).map_err(|original|match original{InferenceWorkFailure::Driver(primary)=>primary,_=>unreachable!("explicit pure inference does not invoke an original caller producer")})?.done {
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
    step_field_cold::<P,F>(snapshot, cache, update, usize::MAX)
}

/// ⏲️ Advances a reference update by up to the requested number of uncached node computations.
pub fn step_field_cold<P,F:InferredField<P>>(snapshot:&P,cache:&mut InferenceCache,update:&mut FieldUpdate<F::Key,F::Value>,maximum_computes:usize)->Result<InferenceStepReport,InferenceWorkFailure> {
    if update.gated {
        return Ok(InferenceStepReport { done: true, progress: 1.0, ..InferenceStepReport::default() });
    }
    let report = drive::<P,F>(snapshot,Some(cache),&mut update.cursor,&mut update.values,true,Some(&mut update.track),maximum_computes)?;
    update.computed += report.computed;
    update.hits += report.hits;
    Ok(report)
}

/// ⏩ The unbounded diff-driven update: [`InferenceSession::begin_update`], [`update_field_cold`] to the end and [`InferenceSession::finish_update`]. The values stay in the session ([`InferenceSession::field_values`]);
/// the delta names the entities that moved, so a consumer copies only those. Fails like [`try_infer_field`]; the session then distrusts its values and the next update examines every entity.
pub fn infer_field_delta<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> Result<FieldDelta<F::Key, F::Value>, InferenceError>
where
    F: InferredField<P>,
    D: semio_os_command::DiffRegions,
{
    let mut update = session.begin_update::<P, F, D>(diff, cache.enabled());
    let outcome = loop {
        match update_field_cold::<P,F>(snapshot,cache,&mut update).map_err(|original|match original{InferenceWorkFailure::Driver(primary)=>primary,_=>unreachable!("explicit pure inference update does not invoke an original caller producer")}) {
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
    D: semio_os_command::DiffRegions,
{
    infer_field_delta::<P, F, D>(snapshot, diff, session, cache).unwrap_or_else(|error| panic!("{error}"));
    session.field_values::<P, F>().cloned().unwrap_or_default()
}
//#endregion 🔖️Driver
