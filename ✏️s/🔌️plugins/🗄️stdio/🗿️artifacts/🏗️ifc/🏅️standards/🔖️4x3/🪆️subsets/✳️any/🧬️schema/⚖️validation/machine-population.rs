/// 📖 Calls, constructors and contextual attributes retain the same semantic arena and immutable source authority.
impl RetainedMachine {
    fn call(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let Raw::Aggregate { cursor, length, .. } = self.raw(frame.a, env)? else { return Err(refusal("express.runtime-arguments")) };
        if frame.name.len() > 1024 {
            return Err(refusal("express.ast-budget"));
        }
        if frame.stage == 1 {
            let Raw::String(text) = self.raw(frame.b, env)? else { return Err(refusal("express.operand")) };
            if let Some(character) = self.text(text, env)?.get(frame.position..).and_then(|text| text.chars().next()) {
                frame.position += character.len_utf8();
                frame.i += 1;
                self.push(frame)?;
            } else {
                self.result(Value::Integer(frame.i));
            }
            return Ok(());
        }
        if frame.stage == 20 {
            let declaration = self.store().entities[frame.b].declaration;
            if let Some(attribute) = declaration.attributes.get(frame.position) {
                let value = self.item(&mut frame.cursor, env)?.ok_or_else(|| refusal("express.call-arity"))?;
                let entity = &mut self.store_mut().entities[frame.b];
                if entity.length == entity.fields.len() {
                    return Err(refusal("express.constructor-field-budget"));
                }
                entity.fields[entity.length] = Some((attribute.name, value));
                entity.length += 1;
                frame.position += 1;
                self.push(frame)?;
            } else {
                self.result(Value::Entity(Entity::Constructed(frame.b)));
            }
            return Ok(());
        }
        let expected = match frame.name {
            "NVL" | "USEDIN" => Some(2),
            "EXISTS" | "TYPEOF" | "SIZEOF" | "HIINDEX" | "LOINDEX" | "LENGTH" | "BLENGTH" | "ABS" | "SQRT" => Some(1),
            _ => None,
        };
        if expected.is_some_and(|expected| expected != length) {
            return Err(refusal("express.call-arity"));
        }
        let mut arguments = cursor;
        let a = if let Some(value) = self.item(&mut arguments, env)? { value } else { self.make(Value::Missing) };
        let b = if let Some(value) = self.item(&mut arguments, env)? { value } else { self.make(Value::Missing) };
        let raw = self.raw(a, env)?;
        match frame.name {
            "NVL" => self.store_mut().results.push(if matches!(raw, Raw::Missing) { b } else { a }),
            "EXISTS" => self.result(boolean(!matches!(raw, Raw::Missing))),
            "TYPEOF" => {
                frame.kind = Kind::TypeOf;
                frame.a = a;
                frame.stage = 0;
                self.push(frame)?;
            }
            "USEDIN" => {
                frame.kind = Kind::UsedIn;
                frame.a = a;
                frame.b = b;
                frame.stage = 0;
                self.push(frame)?;
            }
            "SIZEOF" | "HIINDEX" | "LOINDEX" | "LENGTH" | "BLENGTH" | "ABS" | "SQRT" if matches!(raw, Raw::Missing) => self.result(Value::Missing),
            "SIZEOF" | "HIINDEX" | "LOINDEX" => {
                let Raw::Aggregate { length, lower, .. } = raw else { return Err(refusal("express.aggregate-operand")) };
                self.result(Value::Integer(match frame.name {
                    "SIZEOF" => length as i128,
                    "LOINDEX" => lower,
                    _ => lower.checked_add(length as i128).and_then(|value| value.checked_sub(1)).ok_or_else(|| refusal("express.integer-overflow"))?,
                }));
            }
            "LENGTH" => {
                if !matches!(raw, Raw::String(_)) {
                    return Err(refusal("express.operand"));
                }
                frame.b = a;
                frame.stage = 1;
                frame.i = 0;
                self.push(frame)?;
            }
            "BLENGTH" => {
                let Raw::Binary(text) = raw else { return Err(refusal("express.operand")) };
                let text = self.text(text, env)?;
                self.result(Value::Integer(text.len().saturating_sub(1) as i128 * 4 - text.as_bytes().first().map_or(0, |byte| byte.saturating_sub(b'0')) as i128));
            }
            "ABS" => self.result(match raw {
                Raw::Integer(value) => Value::Integer(value.checked_abs().ok_or_else(|| refusal("express.integer-overflow"))?),
                _ => Value::Real(number(raw)?.abs()),
            }),
            "SQRT" => {
                let value = number(raw)?;
                self.result(if value < 0.0 { Value::Missing } else { Value::Real(value.sqrt()) });
            }
            _ => {
                if let Some(function) = self.program.functions.iter().find(|function| function.name.eq_ignore_ascii_case(frame.name)) {
                    frame.kind = Kind::Function;
                    frame.function = Some(function);
                    return self.push(frame);
                }
                if let Some(declaration) = metadata::entity(frame.name) {
                    if length != declaration.attributes.len() {
                        return Err(refusal("express.call-arity"));
                    }
                    frame.b = self.store().entities.len();
                    self.store_mut().entities.push(Constructed { declaration, fields: [None; 32], length: 0, version: 0, view: None });
                    frame.cursor = cursor;
                    frame.stage = 20;
                    return self.push(frame);
                }
                if let Some(declaration) = metadata::defined_type(frame.name) {
                    if length != 1 {
                        return Err(refusal("express.call-arity"));
                    }
                    self.result(Value::Tagged { base: a, domain: Some(declaration.key), aggregate: None });
                } else {
                    return Err(refusal("express.unsupported-call"));
                }
            }
        }
        Ok(())
    }
    fn entity_record(&self, entity: Entity) -> Result<&Constructed, ValueError> {
        match entity {
            Entity::Constructed(index) => self.store().entities.get(index),
            Entity::Original(index) => self.store().originals.get(index),
            Entity::View(index) => self.store().views.get(index).map(|view| &view.record),
        }
        .ok_or_else(|| refusal("express.population-not-initialized"))
    }
    fn entity_declaration(&self, entity: Entity) -> Result<&'static metadata::StaticEntity, ValueError> {
        Ok(self.entity_record(entity)?.declaration)
    }
    fn entity_concat(&mut self, left: Handle, right: Handle, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let (Raw::Entity(a), Raw::Entity(b)) = (self.raw(left, env)?, self.raw(right, env)?) else { return Err(refusal("express.entity-operand")) };
        let a = *self.entity_record(a)?;
        let b = *self.entity_record(b)?;
        let declaration = if metadata::is_subtype(b.declaration.key, a.declaration.key) {
            b.declaration
        } else if metadata::is_subtype(a.declaration.key, b.declaration.key) {
            a.declaration
        } else {
            return Err(refusal("express.constructor-chain"));
        };
        let mut combined = Constructed { declaration, version: 0, view: None, ..a };
        for field in b.fields[..b.length].iter().flatten() {
            if let Some(previous) = combined.fields[..combined.length].iter_mut().flatten().find(|previous| previous.0.eq_ignore_ascii_case(field.0)) {
                *previous = *field;
            } else {
                if combined.length == combined.fields.len() {
                    return Err(refusal("express.constructor-field-budget"));
                }
                combined.fields[combined.length] = Some(*field);
                combined.length += 1;
            }
        }
        let index = self.store().entities.len();
        self.store_mut().entities.push(combined);
        self.result(Value::Entity(Entity::Constructed(index)));
        Ok(())
    }
    fn entity_equal_next(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let (Raw::Entity(a), Raw::Entity(b)) = (self.raw(frame.a, env)?, self.raw(frame.b, env)?) else { return Err(refusal("express.entity-operand")) };
        let a = self.entity_record(a)?;
        let b = self.entity_record(b)?;
        if a.length != b.length {
            self.result(boolean(false));
            return Ok(());
        }
        let Some((name, value)) = a.fields.get(frame.position).copied().flatten() else {
            self.result(Value::Logical(frame.truth));
            return Ok(());
        };
        let Some((_, other)) = b.fields[..b.length].iter().flatten().find(|(field, _)| field.eq_ignore_ascii_case(name)).copied() else {
            self.result(boolean(false));
            return Ok(());
        };
        frame.position += 1;
        frame.stage = 1;
        self.push(frame)?;
        self.push(Frame { a: value, b: other, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) })
    }
    fn first_domain(&self, mut value: Handle, env: &MachineEnvironment<'_>) -> Result<Option<&'static str>, ValueError> {
        for _ in 0..MAXIMUM_DEPTH {
            match self.store().values[value] {
                Value::Tagged { domain: Some(domain), .. } => return Ok(Some(domain)),
                Value::Tagged { base, .. } | Value::Qualified { base, .. } | Value::Local { base, .. } => value = base,
                Value::Source(node) => {
                    return Ok(if let IfcValue::TypedValue(typed) = env.source_value(node)? { metadata::defined_type(&typed.name).map(|declaration| declaration.key) } else { self.store().domains.get(node).and_then(|domain| domain.name) });
                }
                _ => return Ok(None),
            }
        }
        Err(refusal("express.depth"))
    }
    fn qualifier(&self, mut value: Handle) -> Result<Option<&'static str>, ValueError> {
        for _ in 0..MAXIMUM_DEPTH {
            match self.store().values[value] {
                Value::Qualified { owner, .. } => return Ok(Some(owner)),
                Value::Tagged { base, .. } | Value::Local { base, .. } => value = base,
                _ => return Ok(None),
            }
        }
        Err(refusal("express.depth"))
    }
    fn retag(&mut self, source: Handle, result: Handle, env: &MachineEnvironment<'_>) -> Result<Handle, ValueError> {
        Ok(if let Some(domain) = self.first_domain(source, env)? { self.make(Value::Tagged { base: result, domain: Some(domain), aggregate: None }) } else { result })
    }
    fn primitive_names(&self, value: Handle, env: &MachineEnvironment<'_>) -> Result<&'static [&'static str], ValueError> {
        Ok(match self.raw(value, env)? {
            Raw::Missing | Raw::Entity(_) => &[],
            Raw::Integer(_) => &["INTEGER", "REAL", "NUMBER"],
            Raw::Real(_) => &["REAL", "NUMBER"],
            Raw::Logical(Logical::Unknown) => &["LOGICAL"],
            Raw::Logical(_) => &["LOGICAL", "BOOLEAN"],
            Raw::String(_) => &["STRING"],
            Raw::Binary(_) => &["BINARY"],
            Raw::Enum(_) => &["ENUM"],
            Raw::Aggregate { kind, .. } => match kind {
                AggregateKind::List => &["LIST"],
                AggregateKind::Set => &["SET"],
                AggregateKind::Bag => &["BAG"],
                AggregateKind::Array => &["ARRAY"],
            },
        })
    }
    fn population_operation(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if matches!(frame.kind, Kind::TypeOf) {
            if frame.stage == 0 {
                frame.b = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::Set)));
                if matches!(self.raw(frame.a, env)?, Raw::Missing) {
                    self.store_mut().results.push(frame.b);
                    return Ok(());
                }
                if let Some(qualifier) = self.qualifier(frame.a)? {
                    frame.name = qualifier;
                    frame.identity = true;
                } else if let Raw::Entity(entity) = self.raw(frame.a, env)? {
                    frame.name = self.entity_declaration(entity)?.key;
                } else {
                    frame.name = self.first_domain(frame.a, env)?.unwrap_or("");
                }
                frame.stage = 1;
                frame.position = 0;
                return self.push(frame);
            }
            if frame.stage == 1 {
                let candidate = if !frame.name.is_empty() {
                    let candidate = frame.name;
                    frame.name = if frame.identity {
                        ""
                    } else if let Some(entity) = metadata::entity(candidate) {
                        entity.supertype.unwrap_or("")
                    } else {
                        metadata::defined_type(candidate).and_then(|declaration| declaration.type_expr).filter(|expression| expression.kind == "named").and_then(|expression| expression.name).unwrap_or("")
                    };
                    candidate
                } else {
                    let primitive = if frame.identity { None } else { self.primitive_names(frame.a, env)?.get(frame.position).copied() };
                    let Some(name) = primitive else {
                        frame.stage = 4;
                        frame.d = NONE;
                        return self.push(frame);
                    };
                    frame.position += 1;
                    name
                };
                frame.stage = 2;
                self.push(frame)?;
                if candidate.starts_with("IFC") && !candidate.contains('.') {
                    let a = self.make(Value::String(Text::Static("IFC4X3_ADD2.")));
                    let b = self.make(Value::String(Text::Static(candidate)));
                    self.push(Frame { a, b, name: "concat", i: ("IFC4X3_ADD2.".len() + candidate.len()) as i128, ..Frame::new(Kind::TextCopy, frame.scope, frame.depth + 1) })?;
                } else {
                    self.result(Value::String(Text::Static(candidate)));
                }
                return Ok(());
            }
            if frame.stage == 2 {
                frame.c = self.pop()?;
                let Raw::Aggregate { cursor, .. } = self.raw(frame.b, env)? else { return Err(refusal("express.aggregate-builder")) };
                frame.stage = 3;
                self.push(frame)?;
                return self.push(Frame { a: frame.c, cursor, truth: Logical::False, ..Frame::new(Kind::Contains, frame.scope, frame.depth + 1) });
            }
            if frame.stage == 4 {
                let Value::Aggregate(aggregate) = self.store().values[frame.b] else { return Err(refusal("express.aggregate-builder")) };
                let next = if frame.d == NONE { aggregate.first } else { self.store().edges[frame.d].next };
                let Some(next) = next else {
                    self.store_mut().results.push(frame.b);
                    return Ok(());
                };
                frame.d = next;
                frame.a = self.store().edges[next].value;
                frame.position = 0;
                frame.c = 0;
                frame.stage = 5;
                return self.push(frame);
            }
            if frame.stage == 5 {
                let Raw::String(seed) = self.raw(frame.a, env)? else { return Err(refusal("express.typeof-seed")) };
                let seed = self.text(seed, env)?;
                let parents = metadata::select_parents(seed.strip_prefix("IFC4X3_ADD2.").unwrap_or(seed));
                let Some(&parent) = parents.get(frame.position) else {
                    frame.stage = 4;
                    return self.push(frame);
                };
                frame.name = parent;
                frame.stage = 6;
                self.push(frame)?;
                let a = self.make(Value::String(Text::Static("IFC4X3_ADD2.")));
                let b = self.make(Value::String(Text::Static(parent)));
                return self.push(Frame { a, b, name: "concat", i: ("IFC4X3_ADD2.".len() + parent.len()) as i128, ..Frame::new(Kind::TextCopy, frame.scope, frame.depth + 1) });
            }
            if frame.stage == 6 {
                let candidate = self.pop()?;
                let Raw::Aggregate { cursor, .. } = self.raw(frame.b, env)? else { return Err(refusal("express.aggregate-builder")) };
                frame.stage = 7;
                frame.i = candidate as i128;
                self.push(frame)?;
                return self.push(Frame { a: candidate, cursor, truth: Logical::False, ..Frame::new(Kind::Contains, frame.scope, frame.depth + 1) });
            }
            if frame.stage == 7 {
                let result = self.pop()?;
                if self.truth(result, env)? == Logical::False {
                    self.append(frame.b, frame.i as usize)?;
                }
                frame.stage = 8;
                return self.push(frame);
            }
            if frame.stage == 8 {
                frame.position += 1;
                frame.c = 0;
                frame.stage = 5;
                return self.push(frame);
            }
            let result = self.pop()?;
            if self.truth(result, env)? == Logical::False {
                self.append(frame.b, frame.c)?;
            }
            frame.stage = 1;
            return self.push(frame);
        }
        if matches!(frame.kind, Kind::UsedIn) {
            return self.used_in(frame, env);
        }
        if frame.stage == 100 {
            let value = self.pop()?;
            if matches!(self.raw(value, env)?, Raw::Entity(_)) {
                self.result(Value::Qualified { base: value, owner: frame.expression.and_then(|expression| expression.name).unwrap_or("") });
            } else {
                self.store_mut().results.push(value);
            }
            return Ok(());
        }
        if frame.stage == 0 {
            frame.a = self.pop()?;
            frame.stage = 1;
            frame.name = frame.expression.and_then(|expression| expression.name).unwrap_or(frame.name);
        }
        let raw = self.raw(frame.a, env)?;
        if let Raw::Enum(owner) = raw {
            let owner = self.text(owner, env)?;
            if let Some(declaration) = metadata::defined_type(owner).filter(|declaration| declaration.kind == "enum") {
                if let Some(member) = declaration.members.get(frame.position) {
                    if member.eq_ignore_ascii_case(frame.name) {
                        let value = self.make(Value::Enum(Text::Static(member)));
                        self.result(Value::Tagged { base: value, domain: Some(declaration.key), aggregate: None });
                    } else {
                        frame.position += 1;
                        self.push(frame)?;
                    }
                    return Ok(());
                }
            }
            self.result(Value::Missing);
            return Ok(());
        }
        let Raw::Entity(entity) = raw else {
            self.result(Value::Missing);
            return Ok(());
        };
        if frame.stage == 1 {
            frame.c = 0;
            frame.d = 0;
            frame.stage = 2;
        }
        if frame.stage == 2 {
            if let Some(owner) = self.owner_at(frame.a, frame.c, env)? {
                if let Some(index) = self.indexed_rule(owner, 1, frame.d, false) {
                    let rule = &self.program.derived[index];
                    if rule.owner.eq_ignore_ascii_case(owner) && rule.label.eq_ignore_ascii_case(frame.name) {
                        frame.b = index;
                        frame.position = 0;
                        frame.stage = 3;
                    } else {
                        frame.d += 1;
                    }
                } else {
                    frame.c += 1;
                    frame.d = 0;
                }
            } else {
                frame.stage = 5;
                frame.position = 0;
            }
            return self.push(frame);
        }
        if frame.stage == 3 {
            if let Some(memo) = self.store().memos.get(frame.position).copied() {
                if memo.entity == self.canonical_entity(entity) && memo.root == self.store().scopes[frame.scope].root && memo.rule == frame.b {
                    if memo.active {
                        return Err(refusal("express.derived-cycle"));
                    }
                    if memo.value.is_some() && memo.cacheable {
                        frame.c = frame.position;
                        frame.d = memo.dependencies.unwrap_or(NONE);
                        frame.stage = 6;
                        return self.push(frame);
                    }
                }
                frame.position += 1;
                return self.push(frame);
            }
            frame.d = self.store().memos.len();
            let memo = Memo { entity: self.canonical_entity(entity), rule: frame.b, value: None, active: true, root: self.store().scopes[frame.scope].root, parent: self.store().scopes[frame.scope].memo, dependencies: None, cacheable: false };
            self.store_mut().memos.push(memo);
            let scope = self.detached_scope(frame.scope);
            self.store_mut().scopes[scope].memo = Some(frame.d);
            self.bind(scope, "SELF", frame.a, None);
            frame.stage = 4;
            self.push(frame)?;
            return self.push(Frame::eval(self.program.derived[frame.b].expression, scope, frame.depth + 1));
        }
        if frame.stage == 4 {
            let mut value = self.pop()?;
            let rule = &self.program.derived[frame.b];
            if let Some(expression) =
                metadata::entity(rule.owner).and_then(|entity| entity.derived.iter().find(|attribute| attribute.name.eq_ignore_ascii_case(rule.label))).map(|attribute| &attribute.type_expr).filter(|expression| expression.kind == "aggregate")
            {
                if matches!(self.raw(value, env)?, Raw::Aggregate { .. }) {
                    let kind = aggregate_kind(expression.aggregate);
                    let lower = if kind == AggregateKind::Array { expression.lower.unwrap_or(1) as i128 } else { 1 };
                    value = self.make(Value::Tagged { base: value, domain: None, aggregate: Some((kind, lower)) });
                }
            }
            let cacheable = !matches!(self.raw(value, env)?, Raw::Entity(_) | Raw::Aggregate { .. });
            self.store_mut().memos[frame.d].cacheable = cacheable;
            self.store_mut().memos[frame.d].value = Some(value);
            self.store_mut().memos[frame.d].active = false;
            self.store_mut().results.push(value);
            return Ok(());
        }
        if frame.stage == 6 {
            if let Some(dependency) = self.store().dependencies.get(frame.d).copied() {
                if self.entity_record(dependency.entity)?.version != dependency.version {
                    frame.position += 1;
                    frame.stage = 3;
                } else {
                    frame.d = dependency.next.unwrap_or(NONE);
                }
                return self.push(frame);
            }
            frame.d = self.store().memos[frame.c].dependencies.unwrap_or(NONE);
            frame.stage = 7;
            return self.push(frame);
        }
        if frame.stage == 7 {
            if let Some(dependency) = self.store().dependencies.get(frame.d).copied() {
                frame.d = dependency.next.unwrap_or(NONE);
                frame.stage = 8;
                self.push(frame)?;
                let value = self.make(Value::Missing);
                return self.dependent_result(value, dependency.entity, frame.scope, frame.depth + 1);
            }
            let value = self.store().memos[frame.c].value.ok_or_else(|| refusal("express.runtime-memo"))?;
            self.store_mut().results.push(value);
            return Ok(());
        }
        if frame.stage == 8 {
            self.pop()?;
            frame.stage = 7;
            return self.push(frame);
        }
        if frame.stage == 5 {
            if let Some(index) = self.indexed_rule(self.entity_declaration(entity)?.key, 3, frame.position, true) {
                let inverse = &self.program.inverse[index];
                if inverse.name.eq_ignore_ascii_case(frame.name) && metadata::is_subtype(self.entity_declaration(entity)?.key, inverse.owner) {
                    frame.kind = Kind::UsedIn;
                    frame.name = "inverse";
                    frame.b = index;
                    frame.position = 0;
                    frame.stage = 0;
                    return self.push(frame);
                }
                frame.position += 1;
                return self.push(frame);
            }
            let record = self.entity_record(entity)?;
            if let Some((_, value)) = record.fields[..record.length].iter().flatten().find(|(name, _)| name.eq_ignore_ascii_case(frame.name)).copied() {
                return self.dependent_result(value, entity, frame.scope, frame.depth + 1);
            } else if frame.identity {
                return Err(refusal("express.unbound"));
            } else {
                self.result(Value::Missing);
            }
            return Ok(());
        }
        Err(refusal("express.runtime-attribute"))
    }
    fn owner_at(&self, value: Handle, mut position: usize, env: &MachineEnvironment<'_>) -> Result<Option<&'static str>, ValueError> {
        let qualifier = self.qualifier(value)?;
        if let Some(qualifier) = qualifier {
            if position == 0 {
                return Ok(Some(qualifier));
            }
            position -= 1;
        }
        let Raw::Entity(entity) = self.raw(value, env)? else { return Ok(None) };
        let mut declaration = Some(self.entity_declaration(entity)?);
        for _ in 0..32 {
            let Some(current) = declaration else { return Ok(None) };
            if qualifier.is_none_or(|qualifier| !qualifier.eq_ignore_ascii_case(current.key)) {
                if position == 0 {
                    return Ok(Some(current.key));
                }
                position -= 1;
            }
            declaration = current.supertype.and_then(metadata::entity);
        }
        Err(refusal("express.type-cycle"))
    }
    fn used_in(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let inverse = matches!(frame.name, "inverse" | "inverse-count").then(|| &self.program.inverse[frame.b]);
        if frame.stage == 0 {
            frame.c = self.make(Value::Aggregate(Aggregate::empty(if inverse.is_some() { AggregateKind::Set } else { AggregateKind::Bag })));
            frame.stage = 1;
            return self.push(frame);
        }
        if frame.stage == 2 {
            let result = self.pop()?;
            if self.truth(result, env)? == Logical::False {
                self.append(frame.c, frame.d)?;
            }
            frame.stage = 1;
            return self.push(frame);
        }
        let Raw::Entity(entity) = self.raw(frame.a, env)? else {
            self.store_mut().results.push(frame.c);
            return Ok(());
        };
        let Entity::Original(target) = self.canonical_entity(entity) else {
            self.store_mut().results.push(frame.c);
            return Ok(());
        };
        let arena = env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?;
        let Some(node) = arena.nodes.get(frame.position) else {
            if frame.name == "inverse-count" {
                let Raw::Aggregate { length, .. } = self.raw(frame.c, env)? else { return Err(refusal("express.aggregate-builder")) };
                self.result(Value::Integer(length as i128));
            } else if inverse.is_some_and(|inverse| !inverse.aggregate) {
                let Raw::Aggregate { mut cursor, .. } = self.raw(frame.c, env)? else { return Err(refusal("express.aggregate-builder")) };
                if let Some(value) = self.item(&mut cursor, env)? {
                    self.store_mut().results.push(value);
                } else {
                    self.result(Value::Missing);
                }
            } else {
                self.store_mut().results.push(frame.c);
            }
            return Ok(());
        };
        frame.position += 1;
        if let super::receive::ReceivedKind::Reference(actual) = node.kind {
            if actual == target {
                let owner = Entity::Original(node.root.row);
                let declaration = self.entity_declaration(owner)?;
                let source = env.overlay.ok_or_else(|| refusal("express.runtime-overlay"))?.entity(env.source.ok_or_else(|| refusal("express.runtime-source"))?, env.mutations, node.root.row)?;
                let fields = if source.complex.is_empty() {
                    declaration.all_attributes
                } else {
                    let name = if node.root.component == 0 { source.name.as_str() } else { source.complex[node.root.component - 1].name.as_str() };
                    metadata::entity(name).ok_or_else(|| refusal("express.runtime-component"))?.attributes
                };
                let attribute = fields.get(node.root.argument).ok_or_else(|| refusal("express.runtime-source-field"))?.name;
                let accepted = if let Some(inverse) = inverse {
                    inverse.attribute.eq_ignore_ascii_case(attribute) && metadata::is_subtype(declaration.key, inverse.entity_type)
                } else {
                    let role = if let Raw::String(text) = self.raw(frame.b, env)? { self.text(text, env)? } else { "" };
                    if role.len() > 1024 {
                        return Err(refusal("express.role-budget"));
                    }
                    let mut parts = role.rsplit('.');
                    let field = parts.next().filter(|value| !value.is_empty());
                    let owner = parts.next();
                    field.is_none_or(|field| field.eq_ignore_ascii_case(attribute)) && owner.is_none_or(|owner| metadata::is_subtype(declaration.key, owner))
                };
                if accepted {
                    frame.d = self.make(Value::Entity(owner));
                    if inverse.is_some() {
                        let Raw::Aggregate { cursor, .. } = self.raw(frame.c, env)? else { return Err(refusal("express.aggregate-builder")) };
                        frame.stage = 2;
                        self.push(frame)?;
                        return self.push(Frame { a: frame.d, cursor, truth: Logical::False, ..Frame::new(Kind::Contains, frame.scope, frame.depth + 1) });
                    }
                    self.append(frame.c, frame.d)?;
                }
            }
        }
        self.push(frame)
    }
}

fn aggregate_kind(kind: Option<&str>) -> AggregateKind {
    match kind {
        Some("ARRAY") => AggregateKind::Array,
        Some("SET") => AggregateKind::Set,
        Some("BAG") => AggregateKind::Bag,
        _ => AggregateKind::List,
    }
}
