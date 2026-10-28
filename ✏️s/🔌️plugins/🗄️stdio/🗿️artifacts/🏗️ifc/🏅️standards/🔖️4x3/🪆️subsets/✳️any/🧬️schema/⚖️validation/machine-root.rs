/// 🪟 Procedural views keep source identity while fields, dependency stamps and children stay original-owned.
impl RetainedMachine {
    fn canonical_entity(&self, entity: Entity) -> Entity {
        match entity {
            Entity::View(index) => Entity::Original(self.store().views[index].original),
            _ => entity,
        }
    }
    fn root_read(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let root = self.store().scopes[frame.scope].root;
        if frame.stage == 0 {
            frame.a = self.pop()?;
            frame.b = match self.raw(frame.a, env)? {
                Raw::Entity(Entity::Original(row)) => row,
                Raw::Entity(Entity::View(index)) if self.store().views[index].root != root => self.store().views[index].original,
                _ => {
                    self.store_mut().results.push(frame.a);
                    return Ok(());
                }
            };
            frame.c = self.store().originals[frame.b].view.unwrap_or(NONE);
            frame.stage = 1;
            return self.push(frame);
        }
        if let Some(view) = self.store().views.get(frame.c).copied() {
            if view.root == root {
                self.result(Value::Local { base: frame.a, view: frame.c });
            } else {
                frame.c = view.next.unwrap_or(NONE);
                self.push(frame)?;
            }
            return Ok(());
        }
        let original = self.store().originals[frame.b];
        let index = self.store().views.len();
        self.store_mut().views.push(LocalView { original: frame.b, root, next: original.view, record: Constructed { view: None, ..original } });
        self.store_mut().originals[frame.b].view = Some(index);
        self.result(Value::Local { base: frame.a, view: index });
        Ok(())
    }
    fn dependent_result(&mut self, value: Handle, entity: Entity, scope: usize, depth: usize) -> Result<(), ValueError> {
        let memo = self.store().scopes[scope].memo.unwrap_or(NONE);
        self.push(Frame { a: value, b: memo, record_entity: Some(entity), ..Frame::new(Kind::Dependency, scope, depth) })
    }
    fn dependency(&mut self, mut frame: Frame) -> Result<(), ValueError> {
        let Some(mut memo) = self.store().memos.get(frame.b).copied() else {
            self.store_mut().results.push(frame.a);
            return Ok(());
        };
        let entity = frame.record_entity.ok_or_else(|| refusal("express.runtime-dependency"))?;
        let version = self.entity_record(entity)?.version;
        let next = self.store().dependencies.len();
        self.store_mut().dependencies.push(Dependency { entity, version, next: memo.dependencies });
        memo.dependencies = Some(next);
        self.store_mut().memos[frame.b] = memo;
        frame.b = memo.parent.unwrap_or(NONE);
        self.push(frame)
    }
    fn prepared_query(&mut self, frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        #[cfg(test)]
        if let Some(Initial::Prepared(expression, bindings)) = self.initial {
            let mut frame = frame;
            if let Some(&(name, id)) = bindings.get(frame.position) {
                if name.len() > 1024 {
                    return Err(refusal("express.ast-budget"));
                }
                let row = env.overlay.ok_or_else(|| refusal("express.runtime-overlay"))?.find(id).ok_or_else(|| refusal("express.local-row"))?;
                self.entity_record(Entity::Original(row))?;
                let value = self.make(Value::Entity(Entity::Original(row)));
                self.bind(frame.scope, name, value, None);
                frame.position += 1;
                return self.push(frame);
            }
            return self.push(Frame::eval(expression, frame.scope, frame.depth));
        }
        let _ = (frame, env);
        Err(refusal("express.runtime-query-authority"))
    }
    #[cfg(test)]
    fn test_prepared_query(&mut self, expression: &'static ExpressExpression, bindings: &'static [(&'static str, u64)], env: &MachineEnvironment<'_>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        env.identity()?;
        if bindings.len() > 32 || env.source.is_none() {
            return Err(refusal("express.runtime-query-authority").into());
        }
        if self.store.is_none() {
            return self.birth(Initial::Prepared(expression, bindings), cx);
        }
        if self.closing || self.failed || self.environment != Some(env.identity()?) || !matches!(self.initial, Some(Initial::Prepared(..))) || !self.initialized || !self.store().tasks.is_empty() {
            return Err(refusal("express.runtime-query-not-ready").into());
        }
        if let Some((index, demand)) = self.next_growth()? {
            if !admitted(cx, demand)? {
                return Ok(false);
            }
            self.reserve_table(index);
            debit(cx, demand)?;
            return Ok(false);
        }
        let demand = fixed(size_of::<Initial>() + 2 * size_of::<Frame>() + 2 * size_of::<Scope>());
        if !admitted(cx, demand)? {
            return Ok(false);
        }
        self.initial = Some(Initial::Prepared(expression, bindings));
        let scope = self.scope(None);
        self.push(Frame { expression: Some(expression), ..Frame::new(Kind::PreparedQuery, scope, 1) })?;
        debit(cx, demand)?;
        Ok(true)
    }
}
