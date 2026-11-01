/// 🪟 Procedural views keep source identity while fields, dependency stamps and children stay original-owned.
pub fn root_read(this: &mut RetainedMachine, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
    let root = this.store().scopes[frame.scope].root;
    if frame.stage == 0 {
        frame.a = this.pop()?;
        frame.b = match this.raw(frame.a, env)? {
            Raw::Entity(Entity::Original(row)) => row,
            Raw::Entity(Entity::View(index)) if this.store().views[index].root != root => this.store().views[index].original,
            _ => {
                this.store_mut().results.push(frame.a);
                return Ok(());
            }
        };
        frame.c = this.store().originals[frame.b].view.unwrap_or(NONE);
        frame.stage = 1;
        return this.push(frame);
    }
    if let Some(view) = this.store().views.get(frame.c).copied() {
        if view.root == root {
            this.result(Value::Local { base: frame.a, view: frame.c });
        } else {
            frame.c = view.next.unwrap_or(NONE);
            this.push(frame)?;
        }
        return Ok(());
    }
    let original = this.store().originals[frame.b];
    let index = this.store().views.len();
    this.store_mut().views.push(LocalView { original: frame.b, root, next: original.view, record: Constructed { view: None, ..original } });
    this.store_mut().originals[frame.b].view = Some(index);
    this.result(Value::Local { base: frame.a, view: index });
    Ok(())
}
pub fn dependency(this: &mut RetainedMachine, mut frame: Frame) -> Result<(), ValueError> {
    let Some(mut memo) = this.store().memos.get(frame.b).copied() else {
        this.store_mut().results.push(frame.a);
        return Ok(());
    };
    let entity = frame.record_entity.ok_or_else(|| refusal("express.runtime-dependency"))?;
    let version = this.entity_record(entity)?.version;
    let next = this.store().dependencies.len();
    this.store_mut().dependencies.push(Dependency { entity, version, next: memo.dependencies });
    memo.dependencies = Some(next);
    this.store_mut().memos[frame.b] = memo;
    frame.b = memo.parent.unwrap_or(NONE);
    this.push(frame)
}
pub fn prepared_query(this: &mut RetainedMachine, frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
    #[cfg(test)]
    if let Some(Initial::Prepared(expression, bindings)) = this.initial {
        let mut frame = frame;
        if let Some(&(name, id)) = bindings.get(frame.position) {
            if name.len() > 1024 {
                return Err(refusal("express.ast-budget"));
            }
            let row = env.overlay.ok_or_else(|| refusal("express.runtime-overlay"))?.find(id).ok_or_else(|| refusal("express.local-row"))?;
            this.entity_record(Entity::Original(row))?;
            let value = this.make(Value::Entity(Entity::Original(row)));
            this.bind(frame.scope, name, value, None);
            frame.position += 1;
            return this.push(frame);
        }
        return this.push(Frame::eval(expression, frame.scope, frame.depth));
    }
    let _ = (frame, env);
    Err(refusal("express.runtime-query-authority"))
}
#[cfg(test)]
pub fn test_prepared_query(this: &mut RetainedMachine, expression: &'static ExpressExpression, bindings: &'static [(&'static str, u64)], env: &MachineEnvironment<'_>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    env.identity()?;
    if bindings.len() > 32 || env.source.is_none() {
        return Err(refusal("express.runtime-query-authority").into());
    }
    if this.store.is_none() {
        return this.birth(Initial::Prepared(expression, bindings), cx);
    }
    if this.closing || this.failed || this.environment != Some(env.identity()?) || !matches!(this.initial, Some(Initial::Prepared(..))) || !this.initialized || !this.store().tasks.is_empty() {
        return Err(refusal("express.runtime-query-not-ready").into());
    }
    if let Some((index, demand)) = this.next_growth()? {
        if !admitted(cx, demand)? {
            return Ok(false);
        }
        this.reserve_table(index);
        debit(cx, demand)?;
        return Ok(false);
    }
    let demand = fixed(size_of::<Initial>() + 2 * size_of::<Frame>() + 2 * size_of::<Scope>());
    if !admitted(cx, demand)? {
        return Ok(false);
    }
    this.initial = Some(Initial::Prepared(expression, bindings));
    let scope = this.scope(None);
    this.push(Frame { expression: Some(expression), ..Frame::new(Kind::PreparedQuery, scope, 1) })?;
    debit(cx, demand)?;
    Ok(true)
}
