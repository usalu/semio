/// ⚖️ Population lanes retain their own cursor, scopes and rule-result continuations.
impl RetainedMachine {
    fn indexed_rule(&self, owner: &'static str, lane: usize, position: usize, inherited: bool) -> Option<usize> {
        if std::ptr::eq(self.program, super::ifc4x3_express_program()) {
            let row = if inherited { super::program_index::entity(owner) } else { super::program_index::owner(owner) }?;
            let indices = match lane {
                0 => row.rules,
                1 => row.derived,
                2 => row.bounds,
                3 => row.inverse,
                _ => row.unique,
            };
            indices.get(position).copied()
        } else {
            let length = match lane {
                0 => self.program.rules.len(),
                1 => self.program.derived.len(),
                2 => self.program.bounds.len(),
                3 => self.program.inverse.len(),
                _ => self.program.unique.len(),
            };
            (position < length).then_some(position)
        }
    }
    fn accepts_owner(&self, concrete: &str, owner: &str, inherited: bool) -> bool {
        concrete == owner || inherited && metadata::is_subtype(concrete, owner)
    }
    fn population_id(&self, row: usize, env: &MachineEnvironment<'_>) -> Result<Option<u64>, ValueError> {
        if row == NONE {
            return Ok(None);
        }
        Ok(Some(env.overlay.ok_or_else(|| refusal("express.runtime-overlay"))?.entity(env.source.ok_or_else(|| refusal("express.runtime-source"))?, env.mutations, row)?.id))
    }
    fn record(&mut self, frame: Frame, value: Handle, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let outcome = match self.raw(value, env)? {
            Raw::Logical(Logical::True) => RetainedRuleOutcome::True,
            Raw::Logical(Logical::False) => RetainedRuleOutcome::False,
            Raw::Logical(Logical::Unknown) | Raw::Missing => RetainedRuleOutcome::Unknown,
            _ => return Err(refusal("express.rule-result")),
        };
        let kind = match frame.position {
            0 => RetainedRuleKind::Where,
            1 => RetainedRuleKind::Global,
            2 => RetainedRuleKind::Bound,
            3 => RetainedRuleKind::Inverse,
            4 => RetainedRuleKind::Unique,
            _ => RetainedRuleKind::Admission,
        };
        let record = RetainedRuleRecord {
            owner: Some(frame.owner),
            rule: Some(frame.name),
            kind,
            outcome,
            entity_id: self.population_id(frame.a, env)?,
            offset: frame.expression.map(|expression| expression.offset),
            code: (outcome == RetainedRuleOutcome::False).then_some(match kind {
                RetainedRuleKind::Bound => "express.bound",
                RetainedRuleKind::Inverse => "express.inverse",
                RetainedRuleKind::Unique => "express.unique",
                _ => "express.rule",
            }),
        };
        self.store_mut().records.push(record);
        if outcome == RetainedRuleOutcome::False {
            self.population_valid = false;
            self.store_mut().diagnostics.push(record);
        }
        if matches!(self.initial, Some(Initial::Local(_))) && outcome == RetainedRuleOutcome::Unknown {
            self.population_complete = false;
            self.population_valid = false;
        }
        Ok(())
    }
    fn schedule_rule(&mut self, rule: &'static ExpressRule, scope: usize, row: usize, kind: usize, depth: usize) -> Result<(), ValueError> {
        let scope = if kind == 1 { scope } else { self.root_scope(scope) };
        self.push(Frame { a: row, owner: rule.owner, name: rule.label, position: kind, expression: Some(rule.expression), ..Frame::new(Kind::Record, scope, depth) })?;
        self.push(Frame::eval(rule.expression, scope, depth + 1))
    }
    fn population_rules(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        match frame.kind {
            Kind::Record => {
                let value = self.pop()?;
                return self.record(frame, value, env);
            }
            Kind::TypeRules => return self.type_rule_lane(frame, env),
            Kind::Bound => return self.bound_lane(frame, env),
            Kind::InverseRule => return self.inverse_lane(frame, env),
            Kind::UniqueRule => return self.unique_lane(frame, env),
            Kind::Global => return self.global_lane(frame, env),
            Kind::AggregateUnique => return self.aggregate_unique(frame, env),
            _ => {}
        }
        if frame.stage == 0 {
            if frame.a == NONE {
                frame.a = 0;
                frame.b = 0;
            }
            let Some(record) = self.store().originals.get(frame.a).copied() else {
                if frame.identity {
                    return Err(refusal("express.local-row"));
                }
                frame.stage = 2;
                frame.position = 0;
                return self.push(frame);
            };
            let Some(attribute) = record.declaration.all_attributes.get(frame.b) else {
                if frame.identity {
                    frame.stage = 2;
                    frame.position = 0;
                } else {
                    frame.a += 1;
                }
                frame.b = 0;
                return self.push(frame);
            };
            let value = record.fields[frame.b].ok_or_else(|| refusal("express.source-field"))?.1;
            frame.b += 1;
            self.push(frame)?;
            if !attribute.derived {
                self.push(Frame { a: self.source_node(value)?, type_expression: Some(&attribute.type_expr), identity: attribute.optional, ..Frame::new(Kind::Domain, frame.scope, frame.depth + 1) })?;
            }
            return Ok(());
        }
        if frame.stage == 2 {
            let arena = env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?;
            if frame.position == arena.nodes.len() {
                if !frame.identity {
                    frame.a = 0;
                }
                frame.b = 0;
                frame.stage = 3;
                return self.push(frame);
            }
            let node = frame.position;
            frame.position += 1;
            self.push(frame)?;
            if (!frame.identity || arena.nodes[node].root.row == frame.a) && self.source_aggregate_expression(node)?.is_some_and(|expression| expression.unique || expression.aggregate == Some("SET")) {
                self.push(Frame { a: node, ..Frame::new(Kind::AggregateUnique, frame.scope, frame.depth + 1) })?;
            }
            return Ok(());
        }
        if frame.stage == 3 {
            let Some(record) = self.store().originals.get(frame.a).copied() else {
                if frame.identity {
                    return Err(refusal("express.local-row"));
                }
                frame.stage = 20;
                frame.position = 0;
                return self.push(frame);
            };
            frame.scope = self.scope(None);
            frame.d = self.make(Value::Entity(Entity::Original(frame.a)));
            self.bind(frame.scope, "SELF", frame.d, None);
            frame.owner = record.declaration.key;
            frame.position = 0;
            frame.stage = 10;
            return self.push(frame);
        }
        if frame.stage == 10 {
            if let Some(index) = self.indexed_rule(frame.owner, 0, frame.position, true) {
                let rule = &self.program.rules[index];
                frame.position += 1;
                self.push(frame)?;
                if self.accepts_owner(frame.owner, rule.owner, true) {
                    self.schedule_rule(rule, frame.scope, frame.a, 0, frame.depth + 1)?;
                }
                return Ok(());
            }
            frame.stage = 11;
            frame.position = 0;
            return self.push(frame);
        }
        if frame.stage == 11 {
            let record = self.store().originals[frame.a];
            if let Some(attribute) = record.declaration.all_attributes.get(frame.position) {
                let value = record.fields[frame.position].ok_or_else(|| refusal("express.source-field"))?.1;
                frame.position += 1;
                self.push(frame)?;
                if !attribute.derived {
                    self.push(Frame { a: value, b: frame.a, type_expression: Some(&attribute.type_expr), ..Frame::new(Kind::TypeRules, frame.scope, frame.depth + 1) })?;
                }
                return Ok(());
            }
            if frame.identity {
                self.population_finished = true;
                return Ok(());
            }
            frame.stage = 12;
            frame.position = 0;
            return self.push(frame);
        }
        if frame.stage == 12 {
            if let Some(index) = self.indexed_rule(frame.owner, 1, frame.position, true) {
                let rule = &self.program.derived[index];
                frame.position += 1;
                frame.stage = 15;
                self.push(frame)?;
                if self.accepts_owner(frame.owner, rule.owner, true) {
                    let scope = self.root_scope(frame.scope);
                    self.push(Frame { a: frame.d, name: rule.label, stage: 1, ..Frame::new(Kind::Attribute, scope, frame.depth + 1) })?;
                } else {
                    self.result(Value::Missing);
                }
                return Ok(());
            }
            frame.stage = 13;
            frame.position = 0;
            return self.push(frame);
        }
        if frame.stage == 15 {
            self.pop()?;
            frame.stage = 12;
            return self.push(frame);
        }
        if frame.stage == 13 {
            if let Some(index) = self.indexed_rule(frame.owner, 2, frame.position, true) {
                let rule = &self.program.bounds[index];
                frame.position += 1;
                self.push(frame)?;
                if self.accepts_owner(frame.owner, rule.owner, true) {
                    self.push(Frame { a: frame.a, b: index, c: frame.d, ..Frame::new(Kind::Bound, frame.scope, frame.depth + 1) })?;
                }
                return Ok(());
            }
            frame.stage = 14;
            frame.position = 0;
            return self.push(frame);
        }
        if frame.stage == 14 {
            if let Some(index) = self.indexed_rule(frame.owner, 3, frame.position, true) {
                let inverse = &self.program.inverse[index];
                frame.position += 1;
                self.push(frame)?;
                if self.accepts_owner(frame.owner, inverse.owner, true) {
                    self.push(Frame { a: frame.a, b: index, c: frame.d, ..Frame::new(Kind::InverseRule, frame.scope, frame.depth + 1) })?;
                }
                return Ok(());
            }
            frame.a += 1;
            frame.stage = 3;
            return self.push(frame);
        }
        if frame.stage == 20 {
            if frame.position < self.program.unique.len() {
                let index = frame.position;
                frame.position += 1;
                self.push(frame)?;
                return self.push(Frame { b: index, ..Frame::new(Kind::UniqueRule, frame.scope, frame.depth + 1) });
            }
            frame.stage = 21;
            frame.position = 0;
            return self.push(frame);
        }
        if frame.stage == 21 {
            if frame.position < self.program.global_rules.len() {
                let index = frame.position;
                frame.position += 1;
                self.push(frame)?;
                return self.push(Frame { a: index, ..Frame::new(Kind::Global, frame.scope, frame.depth + 1) });
            }
            self.population_finished = true;
            return Ok(());
        }
        Err(refusal("express.population-stage"))
    }
    fn aggregate_unique(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage == 0 {
            let value = self.make(Value::Source(frame.a));
            let Raw::Aggregate { cursor, .. } = self.raw(value, env)? else { return Err(refusal("express.domain-aggregate")) };
            frame.cursor = cursor;
            frame.other = cursor;
            frame.stage = 1;
            return self.push(frame);
        }
        if frame.stage == 3 {
            let value = self.pop()?;
            if self.truth(value, env)? == Logical::True {
                return Err(refusal("express.domain-aggregate-unique"));
            }
            frame.stage = 2;
            return self.push(frame);
        }
        if frame.stage == 1 {
            let Some(left) = self.item(&mut frame.cursor, env)? else {
                return Ok(());
            };
            frame.b = left;
            frame.other = frame.cursor;
            frame.stage = 2;
            return self.push(frame);
        }
        if let Some(right) = self.item(&mut frame.other, env)? {
            frame.stage = 3;
            self.push(frame)?;
            self.push(Frame { a: frame.b, b: right, identity: true, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) })?;
        } else {
            frame.stage = 1;
            self.push(frame)?;
        }
        Ok(())
    }
    fn type_rule_lane(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if matches!(self.raw(frame.a, env)?, Raw::Missing) {
            return Ok(());
        }
        if frame.stage == 0 {
            let expression = frame.type_expression.ok_or_else(|| refusal("express.type-expression"))?;
            if expression.kind == "aggregate" {
                let Raw::Aggregate { cursor, .. } = self.raw(frame.a, env)? else { return Err(refusal("express.type-aggregate")) };
                frame.cursor = cursor;
                frame.stage = 3;
                return self.push(frame);
            }
            frame.name = expression.name.unwrap_or("");
            if metadata::entity(frame.name).is_some() {
                return Ok(());
            }
            if metadata::defined_type(frame.name).is_some_and(|declaration| declaration.kind == "select") {
                frame.name = self.first_domain(frame.a, env)?.unwrap_or("");
            }
            if frame.name.is_empty() {
                return Ok(());
            }
            frame.stage = 1;
            frame.position = 0;
            return self.push(frame);
        }
        if frame.stage == 1 {
            if let Some(index) = self.indexed_rule(frame.name, 0, frame.position, false) {
                let rule = &self.program.rules[index];
                frame.position += 1;
                self.push(frame)?;
                if rule.owner == frame.name {
                    let scope = self.scope(None);
                    self.bind(scope, "SELF", frame.a, None);
                    self.schedule_rule(rule, scope, frame.b, 0, frame.depth + 1)?;
                }
                return Ok(());
            }
            if let Some(expression) = metadata::defined_type(frame.name).and_then(|declaration| declaration.type_expr) {
                frame.type_expression = Some(expression);
                frame.stage = 0;
                frame.depth += 1;
                return self.push(frame);
            }
            return Ok(());
        }
        if let Some(value) = self.item(&mut frame.cursor, env)? {
            self.push(frame)?;
            self.push(Frame { a: value, b: frame.b, type_expression: frame.type_expression.and_then(|expression| expression.item), ..Frame::new(Kind::TypeRules, frame.scope, frame.depth + 1) })?;
        }
        Ok(())
    }
    fn bound_lane(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let rule = &self.program.bounds[frame.b];
        if frame.stage == 0 {
            frame.scope = self.root_scope(frame.scope);
            if rule.label.len() > 1024 {
                return Err(refusal("express.bound-path"));
            }
            let attribute = rule.label.split('.').next().ok_or_else(|| refusal("express.bound-path"))?;
            frame.stage = 1;
            self.push(frame)?;
            return self.push(Frame { a: frame.c, name: attribute, stage: 1, ..Frame::new(Kind::Attribute, frame.scope, frame.depth + 1) });
        }
        if frame.stage == 1 {
            frame.c = self.pop()?;
            frame.stage = 2;
            self.push(frame)?;
            return self.push(Frame::eval(rule.expression, frame.scope, frame.depth + 1));
        }
        if frame.stage == 2 {
            frame.d = self.pop()?;
            frame.stage = 3;
            frame.position = 1;
            let builder = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::List)));
            self.append(builder, frame.c)?;
            frame.c = builder;
            return self.push(frame);
        }
        if frame.stage == 3 {
            let segment = rule.label.split('.').nth(frame.position).ok_or_else(|| refusal("express.bound-path"))?;
            if segment.eq_ignore_ascii_case("item") {
                let Raw::Aggregate { cursor, .. } = self.raw(frame.c, env)? else { return Err(refusal("express.bound-values")) };
                frame.cursor = cursor;
                frame.other = Cursor::default();
                frame.c = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::List)));
                frame.stage = 4;
                return self.push(frame);
            }
            if !segment.eq_ignore_ascii_case("upper") && !segment.eq_ignore_ascii_case("lower") {
                return Err(refusal("express.bound-path"));
            }
            frame.identity = segment.eq_ignore_ascii_case("upper");
            let Raw::Aggregate { cursor, .. } = self.raw(frame.c, env)? else { return Err(refusal("express.bound-values")) };
            frame.cursor = cursor;
            frame.stage = 5;
            return self.push(frame);
        }
        if frame.stage == 4 {
            if let Some(value) = self.item(&mut frame.other, env)? {
                self.append(frame.c, value)?;
                return self.push(frame);
            }
            if let Some(value) = self.item(&mut frame.cursor, env)? {
                if let Raw::Aggregate { cursor, .. } = self.raw(value, env)? {
                    frame.other = cursor;
                }
                return self.push(frame);
            }
            frame.position += 1;
            frame.stage = 3;
            return self.push(frame);
        }
        if frame.stage == 5 {
            let Some(value) = self.item(&mut frame.cursor, env)? else {
                return Ok(());
            };
            let actual = match self.raw(value, env)? {
                Raw::Aggregate { lower, length, .. } => Value::Integer(if frame.identity { lower.checked_add(length as i128).and_then(|value| value.checked_sub(1)).ok_or_else(|| refusal("express.integer-overflow"))? } else { lower }),
                _ => Value::Missing,
            };
            let actual = self.make(actual);
            frame.stage = 6;
            self.push(frame)?;
            return self.push(Frame { a: actual, b: frame.d, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) });
        }
        let result = self.pop()?;
        self.record(Frame { a: frame.a, owner: rule.owner, name: rule.label, position: 2, ..Frame::new(Kind::Record, frame.scope, frame.depth) }, result, env)?;
        frame.stage = 5;
        self.push(frame)
    }
    fn inverse_lane(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let inverse = &self.program.inverse[frame.b];
        if frame.stage == 0 {
            frame.scope = self.root_scope(frame.scope);
            frame.stage = 1;
            self.push(frame)?;
            return self.push(Frame { a: frame.c, b: frame.b, name: "inverse-count", ..Frame::new(Kind::UsedIn, frame.scope, frame.depth + 1) });
        }
        if frame.stage == 1 {
            frame.c = self.pop()?;
            frame.position = 0;
            frame.stage = 2;
            return self.push(frame);
        }
        if frame.stage == 4 {
            let value = self.pop()?;
            self.record(Frame { a: frame.a, owner: inverse.owner, name: inverse.name, position: 3, ..Frame::new(Kind::Record, frame.scope, frame.depth) }, value, env)?;
            frame.stage = 2;
            return self.push(frame);
        }
        if frame.stage == 3 {
            let expected = self.pop()?;
            let actual = frame.c;
            frame.stage = 4;
            self.push(frame)?;
            return self.binary(if frame.position == 2 { ">=" } else { "<=" }, actual, expected, frame.scope, frame.depth + 1, env);
        }
        if frame.position == 0 {
            frame.position = 1;
            if !inverse.aggregate {
                let one = self.make(Value::Integer(1));
                frame.stage = 4;
                self.push(frame)?;
                return self.push(Frame { a: frame.c, b: one, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) });
            }
            return self.push(frame);
        }
        let expression = match frame.position {
            1 => inverse.lower,
            2 => inverse.upper,
            _ => return Ok(()),
        };
        frame.position += 1;
        if let Some(expression) = expression.filter(|expression| expression.kind != ExpressExpressionKind::Unknown) {
            frame.stage = 3;
            self.push(frame)?;
            return self.push(Frame::eval(expression, frame.scope, frame.depth + 1));
        }
        self.push(frame)
    }
    fn unique_lane(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let rule = &self.program.unique[frame.b];
        if frame.stage == 0 {
            frame.a = 0;
            frame.c = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::List)));
            frame.stage = 1;
            return self.push(frame);
        }
        if frame.stage == 1 {
            let Some(record) = self.store().originals.get(frame.a).copied() else {
                return Ok(());
            };
            if !metadata::is_subtype(record.declaration.key, rule.owner) {
                frame.a += 1;
                return self.push(frame);
            }
            frame.scope = self.scope(None);
            let value = self.make(Value::Entity(Entity::Original(frame.a)));
            self.bind(frame.scope, "SELF", value, None);
            frame.stage = 2;
            self.push(frame)?;
            return self.push(Frame::eval(rule.expression, frame.scope, frame.depth + 1));
        }
        if frame.stage == 2 {
            frame.d = self.pop()?;
            let Raw::Aggregate { cursor, .. } = self.raw(frame.c, env)? else { return Err(refusal("express.unique-history")) };
            frame.cursor = cursor;
            frame.truth = Logical::True;
            frame.stage = 3;
            return self.push(frame);
        }
        if frame.stage == 4 {
            let value = self.pop()?;
            match self.truth(value, env)? {
                Logical::True => {
                    frame.truth = Logical::False;
                    frame.cursor.next = None;
                }
                Logical::Unknown if frame.truth != Logical::False => frame.truth = Logical::Unknown,
                _ => {}
            };
            frame.stage = 3;
            return self.push(frame);
        }
        if let Some(previous) = self.item(&mut frame.cursor, env)? {
            frame.stage = 4;
            self.push(frame)?;
            return self.push(Frame { a: frame.d, b: previous, identity: true, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) });
        }
        let result = self.make(Value::Logical(frame.truth));
        self.record(Frame { a: frame.a, owner: rule.owner, name: rule.label, position: 4, ..Frame::new(Kind::Record, frame.scope, frame.depth) }, result, env)?;
        self.append(frame.c, frame.d)?;
        frame.a += 1;
        frame.stage = 1;
        self.push(frame)
    }
    fn global_lane(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let global = &self.program.global_rules[frame.a];
        if frame.stage == 0 {
            frame.scope = self.scope(None);
            frame.b = 0;
            frame.stage = 1;
            return self.push(frame);
        }
        if frame.stage == 1 {
            if global.populations.get(frame.b).is_none() {
                frame.stage = 3;
                self.push(frame)?;
                return self.push(Frame { declarations: global.locals, ..Frame::new(Kind::Declare, frame.scope, frame.depth + 1) });
            }
            frame.c = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::Set)));
            frame.position = 0;
            frame.stage = 2;
            return self.push(frame);
        }
        if frame.stage == 2 {
            if let Some(record) = self.store().originals.get(frame.position).copied() {
                if metadata::is_subtype(record.declaration.key, global.populations[frame.b]) {
                    let value = self.make(Value::Entity(Entity::Original(frame.position)));
                    self.append(frame.c, value)?;
                }
                frame.position += 1;
                return self.push(frame);
            }
            self.bind(frame.scope, global.populations[frame.b], frame.c, None);
            frame.b += 1;
            frame.stage = 1;
            return self.push(frame);
        }
        if frame.stage == 3 {
            self.flow = Flow::None;
            frame.stage = 4;
            frame.position = 0;
            self.push(frame)?;
            return self.push(Frame { body: global.body, ..Frame::new(Kind::Body, frame.scope, frame.depth + 1) });
        }
        if self.flow != Flow::None {
            return Err(refusal("express.global-flow"));
        }
        if let Some(rule) = global.rules.get(frame.position) {
            frame.position += 1;
            self.push(frame)?;
            return self.schedule_rule(rule, frame.scope, NONE, 1, frame.depth + 1);
        }
        Ok(())
    }
}
