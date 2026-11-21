//! 🔤️ Paid traversal of an original snapshot's borrowed JSON view.
use super::*;

pub(super) fn advance_projection<S: StructuredMediaSource>(
    request: &Request<S>,
    buffers: &mut Buffers<S::Projection>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    cx.set_stage("structured-media-project");
    let (media_type, schema) = S::descriptor(&request.port)
        .ok_or_else(|| ValueError::invariant("structured media export received a foreign port"))?;
    cx.original_source_parent()?.validate_borrowed_context(cx)?;
    if let Some(source) = buffers.source {
        if !source.matches(cx) {
            return Err(ValueError::invariant("structured media projection changed original Source").into());
        }
    } else {
        let bytes = size_of::<Option<OriginalSourceBinding>>();
        if !cx.retained_work_can_enter(header(bytes)) {
            return Ok(());
        }
        buffers.source = Some(OriginalSourceBinding::borrow(cx));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        return Ok(());
    }
    if let Some(message) = buffers.failure {
        let bytes = request.port.len() + message.len();
        let demand = RetirementDemand { capacity_bytes: bytes, ..header(size_of::<ResultPage>() + bytes) };
        if !cx.retained_work_can_enter(demand) {
            return Ok(());
        }
        buffers.result = Some(Err(MediaError::Payload(request.port.clone(), message.into())));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        return Ok(());
    }
    if !buffers.prepared {
        return prepare::<S>(request, buffers, cx);
    }
    if buffers.json.is_none() {
        return write::<S>(request, buffers, cx);
    }
    let demand = RetirementDemand {
        capacity_bytes: schema.len(),
        ..header(size_of::<(MediaType, String, Vec<u8>)>() + schema.len())
    };
    if !cx.retained_work_can_enter(demand) {
        return Ok(());
    }
    buffers.encoded = Some((media_type, schema.to_owned(), buffers.json.take().unwrap()));
    cx.consume_retained(RetainedCloneProgress {
        copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: schema.len(), ..Default::default()
    })?;
    cx.consume_fuel(1);
    Ok(())
}

fn prepare<S: StructuredMediaSource>(
    request: &Request<S>,
    buffers: &mut Buffers<S::Projection>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let bytes = S::PREPARATION_COPY_BYTES + size_of::<Option<&'static str>>() + size_of::<bool>();
    if !cx.retained_work_can_enter(header(bytes)) {
        return Ok(());
    }
    let grant = cx.retained_grant();
    let Some((result, performed)) = cx.with_original_source_effect(grant, |cx, effect| -> Result<_, ValueError> {
        let actions = JSON_ACTIONS_PER_TURN
            .min(effect.remaining_work_units()?.try_into().unwrap_or(usize::MAX))
            .min(grant.maximum_items)
            .min(grant.maximum_copy_bytes / bytes);
        for _ in 0..actions {
            if cx.should_yield() || !effect.performed(1)? {
                break;
            }
            match S::prepare(&request.port, request.snapshot_read.get(), &mut buffers.projection) {
                Ok(true) => { buffers.prepared = true; break; }
                Ok(false) => (),
                Err(message) => { buffers.failure = Some(message); break; }
            }
        }
        Ok(())
    })? else { return Ok(()) };
    cx.consume_retained(RetainedCloneProgress {
        copied_items: performed as usize, copied_bytes: bytes * performed as usize, ..Default::default()
    })?;
    cx.consume_fuel(performed as u64);
    Ok(result?)
}

fn write<S: StructuredMediaSource>(
    request: &Request<S>,
    buffers: &mut Buffers<S::Projection>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let projection = &buffers.projection;
    write_borrowed(
        || S::borrow(&request.port, request.snapshot_read.get(), projection),
        &mut buffers.cursor, &mut buffers.json, &mut buffers.failure, S::OVERSIZE, cx,
    )
}

fn write_borrowed<V: JsonWriteSource>(
    source: impl FnOnce() -> Result<V, ValueError>,
    cursor: &mut JsonBorrowedWriteCursor,
    output: &mut Option<Vec<u8>>,
    failure: &mut Option<&'static str>,
    oversize: &'static str,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let mut grant = cx.retained_grant();
    let metadata = size_of::<Option<Vec<u8>>>().max(size_of::<Option<&'static str>>());
    if cx.should_yield() || grant.maximum_copy_bytes < metadata {
        return Ok(());
    }
    grant.maximum_copy_bytes -= metadata;
    let total = cx.original_total_retained_grant();
    let Some((result, performed)) = cx.with_original_source_effect(grant, |cx, effect| {
        let source = source()?;
        let actions = JSON_ACTIONS_PER_TURN.min(effect.remaining_work_units()?.try_into().unwrap_or(usize::MAX));
        let mut live = |_| !cx.is_cancelled();
        let mut native = NativeEncodeControl::new_retained(&mut live);
        let result = cursor.step_with_demand(&source, actions, &mut native, effect.grant());
        let mut progress = cursor.normal_step_progress();
        if progress.retained_capacity_bytes != 0 {
            *effect.actual() = Some(progress.retained_capacity_bytes);
        }
        if !effect.performed(cursor.last_turn_work().max(1))? {
            return Err(ValueError::invariant("structured JSON exceeded its admitted work"));
        }
        let result = match result {
            Ok((Some(bytes), _)) => {
                *output = Some(bytes.into_bytes());
                progress.copied_bytes += size_of::<Option<Vec<u8>>>();
                Ok(())
            }
            Ok((None, Some(demand))) if demand.copy_bytes > total.maximum_copy_bytes.saturating_sub(metadata)
                || demand.capacity_bytes > total.maximum_capacity_bytes
                || demand.depth > total.maximum_depth => Err(ValueError::literal(ValueRefusalKind::WorkLimit, "structured JSON cannot fund its next source phase")),
            Ok(_) => Ok(()),
            Err(error) if error.kind == ValueRefusalKind::OwnershipLimit && error.message == "controlled JSON output exceeds caller limit" => {
                *failure = Some(oversize);
                progress.copied_items = progress.copied_items.max(1);
                progress.copied_bytes += size_of::<Option<&'static str>>();
                Ok(())
            }
            Err(error) => Err(error),
        };
        Ok::<_, ValueError>((result, progress))
    })? else { return Ok(()) };
    let (body, progress) = result?;
    cx.consume_retained(progress)?;
    cx.consume_fuel(performed as u64);
    Ok(body?)
}
