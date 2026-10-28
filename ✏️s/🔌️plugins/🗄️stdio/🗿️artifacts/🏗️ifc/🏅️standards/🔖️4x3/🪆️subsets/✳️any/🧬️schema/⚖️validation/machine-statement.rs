/// 🔁 Functions and statements retain their actual scopes, return flows and per-iteration cursor state.
impl RetainedMachine {
    fn statement(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        match frame.kind {
            Kind::CallArgs => {
                if frame.stage == 100 {
                    let Some(Initial::Call(_, arguments)) = self.initial else { return Err(refusal("express.runtime-call-input")) };
                    if let Some(value) = arguments.get(frame.position) {
                        let value = self.make(Value::Integer(*value));
                        self.append(frame.a, value)?;
                        frame.position += 1;
                        self.push(frame)?;
                    } else {
                        frame.kind = Kind::Call;
                        frame.stage = 0;
                        self.push(frame)?;
                    }
                    return Ok(());
                }
                let expression = frame.expression.ok_or_else(|| refusal("express.ast"))?;
                if frame.stage == 10 {
                    frame.stage = 11;
                    self.push(frame)?;
                    return self.push(Frame::eval(&expression.operands[1], frame.scope, frame.depth + 1));
                }
                if frame.stage == 11 {
                    let value = self.pop()?;
                    if matches!(self.raw(value, env)?, Raw::Missing) {
                        self.push(Frame::eval(&expression.operands[2], frame.scope, frame.depth + 1))?;
                    } else {
                        self.store_mut().results.push(value);
                    }
                    return Ok(());
                }
                if frame.stage == 1 {
                    let value = self.pop()?;
                    self.append(frame.a, value)?;
                    frame.position += 1;
                    frame.stage = 0;
                }
                if let Some(operand) = expression.operands.get(frame.position) {
                    frame.stage = 1;
                    self.push(frame)?;
                    self.push(Frame::eval(operand, frame.scope, frame.depth + 1))?;
                } else {
                    frame.kind = Kind::Call;
                    frame.stage = 0;
                    self.push(frame)?;
                }
                Ok(())
            }
            Kind::Call => self.call(frame, env),
            Kind::Function => self.function(frame, env),
            Kind::Declare => self.declare(frame, env),
            Kind::Coerce => self.coerce(frame, env),
            Kind::Body => {
                if self.flow != Flow::None {
                    return Ok(());
                }
                if let Some(statement) = frame.body.get(frame.position) {
                    frame.position += 1;
                    self.push(frame)?;
                    self.push(Frame { statement: Some(statement), ..Frame::new(Kind::Statement, frame.scope, frame.depth + 1) })?;
                }
                Ok(())
            }
            Kind::Statement => {
                let statement = frame.statement.ok_or_else(|| refusal("express.ast"))?;
                let expression = |index| statement.expressions.get(index).ok_or_else(|| refusal("express.ast"));
                use ExpressStatementKind::*;
                match statement.kind {
                    Null => {}
                    Escape => self.flow = Flow::Escape,
                    Skip => self.flow = Flow::Skip,
                    Return | Call => {
                        if frame.stage == 0 {
                            frame.stage = 1;
                            self.push(frame)?;
                            self.push(Frame::eval(expression(0)?, frame.scope, frame.depth + 1))?;
                        } else {
                            let value = self.pop()?;
                            if statement.kind == Return {
                                self.flow = Flow::Return(value);
                            }
                        }
                    }
                    Assign => {
                        if frame.stage == 0 {
                            frame.stage = 1;
                            self.push(frame)?;
                            self.push(Frame::eval(expression(1)?, frame.scope, frame.depth + 1))?;
                        } else {
                            frame.a = self.pop()?;
                            frame.expression = Some(expression(0)?);
                            frame.kind = Kind::Assign;
                            frame.stage = 0;
                            self.push(frame)?;
                        }
                    }
                    Block => self.push(Frame { body: statement.body, ..Frame::new(Kind::Body, frame.scope, frame.depth + 1) })?,
                    If => {
                        if frame.stage == 0 {
                            frame.stage = 1;
                            self.push(frame)?;
                            self.push(Frame::eval(expression(0)?, frame.scope, frame.depth + 1))?;
                        } else {
                            let value = self.pop()?;
                            let body = if self.truth(value, env)? == Logical::True { statement.body } else { statement.otherwise };
                            self.push(Frame { body, ..Frame::new(Kind::Body, frame.scope, frame.depth + 1) })?;
                        }
                    }
                    Case => {
                        frame.kind = Kind::Case;
                        self.push(frame)?;
                    }
                    Repeat => {
                        frame.kind = Kind::Repeat;
                        self.push(frame)?;
                    }
                }
                Ok(())
            }
            Kind::Case => {
                let statement = frame.statement.ok_or_else(|| refusal("express.ast"))?;
                match frame.stage {
                    0 => {
                        frame.stage = 1;
                        self.push(frame)?;
                        self.push(Frame::eval(statement.expressions.first().ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1))?;
                    }
                    1 => {
                        frame.a = self.pop()?;
                        frame.stage = 2;
                        frame.b = 0;
                        self.push(frame)?;
                    }
                    2 => {
                        let Some(case) = statement.cases.get(frame.position) else {
                            return self.push(Frame { body: statement.otherwise, ..Frame::new(Kind::Body, frame.scope, frame.depth + 1) });
                        };
                        if let Some(label) = case.labels.get(frame.b) {
                            frame.stage = 3;
                            self.push(frame)?;
                            self.push(Frame::eval(label, frame.scope, frame.depth + 1))?;
                        } else {
                            frame.position += 1;
                            frame.b = 0;
                            self.push(frame)?;
                        }
                    }
                    3 => {
                        let label = self.pop()?;
                        frame.stage = 4;
                        self.push(frame)?;
                        self.push(Frame { a: frame.a, b: label, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) })?;
                    }
                    _ => {
                        let result = self.pop()?;
                        if self.truth(result, env)? == Logical::True {
                            self.push(Frame { body: statement.cases[frame.position].body, ..Frame::new(Kind::Body, frame.scope, frame.depth + 1) })?;
                        } else {
                            frame.b += 1;
                            frame.stage = 2;
                            self.push(frame)?;
                        }
                    }
                }
                Ok(())
            }
            Kind::Repeat => self.repeat(frame, env),
            Kind::Assign => self.assign(frame, env),
            _ => Err(refusal("express.runtime-statement")),
        }
    }
    fn coerce(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let Raw::Aggregate { lower, .. } = self.raw(frame.a, env)? else {
            self.store_mut().results.push(frame.a);
            return Ok(());
        };
        let source = frame.name.trim();
        let kind = if source.get(..4).is_some_and(|value| value.eq_ignore_ascii_case("LIST")) {
            AggregateKind::List
        } else if source.get(..3).is_some_and(|value| value.eq_ignore_ascii_case("SET")) {
            AggregateKind::Set
        } else if source.get(..3).is_some_and(|value| value.eq_ignore_ascii_case("BAG")) {
            AggregateKind::Bag
        } else if source.get(..5).is_some_and(|value| value.eq_ignore_ascii_case("ARRAY")) {
            AggregateKind::Array
        } else {
            self.store_mut().results.push(frame.a);
            return Ok(());
        };
        frame.i = if frame.stage == 1 {
            let result = self.pop()?;
            self.integer(result, env)?.ok_or_else(|| refusal("express.bound"))?
        } else {
            lower
        };
        if frame.stage == 0 && kind == AggregateKind::Array {
            if let Some(bounds) = source.split('[').nth(1).and_then(|part| part.split(']').next()) {
                let bound = bounds.split(':').next().unwrap_or("").trim();
                if bound.len() > 128 {
                    return Err(refusal("express.bound"));
                }
                if let Ok(lower) = bound.parse::<i128>() {
                    frame.i = lower;
                } else {
                    frame.stage = 1;
                    self.push(frame)?;
                    return self.push(Frame { name: bound, a: frame.scope, position: self.store().scopes[frame.scope].bindings.unwrap_or(NONE), ..Frame::new(Kind::Lookup, frame.scope, frame.depth + 1) });
                }
            }
        }
        self.result(Value::Tagged { base: frame.a, domain: None, aggregate: Some((kind, frame.i)) });
        Ok(())
    }
    fn declare(&mut self, mut frame: Frame, _env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let Some(declaration) = frame.declarations.get(frame.position) else { return Ok(()) };
        match frame.stage {
            0 => {
                if let Some(initial) = declaration.initial {
                    frame.stage = 1;
                    self.push(frame)?;
                    self.push(Frame::eval(initial, frame.scope, frame.depth + 1))?;
                } else {
                    frame.a = self.make(Value::Missing);
                    frame.stage = 2;
                    frame.b = 0;
                    self.push(frame)?;
                }
            }
            1 => {
                frame.a = self.pop()?;
                frame.b = 0;
                frame.stage = 2;
                self.push(frame)?;
            }
            2 => {
                if declaration.names.get(frame.b).is_some() {
                    frame.stage = 3;
                    self.push(frame)?;
                    self.push(Frame { a: frame.a, name: declaration.type_source, ..Frame::new(Kind::Coerce, frame.scope, frame.depth + 1) })?;
                } else {
                    frame.position += 1;
                    frame.stage = 0;
                    self.push(frame)?;
                }
            }
            _ => {
                let value = self.pop()?;
                self.bind(frame.scope, declaration.names[frame.b], value, Some(declaration.type_source));
                frame.b += 1;
                frame.stage = 2;
                self.push(frame)?;
            }
        }
        Ok(())
    }
    fn function(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let function = frame.function.ok_or_else(|| refusal("express.runtime-function"))?;
        match frame.stage {
            0 => {
                let Raw::Aggregate { cursor, .. } = self.raw(frame.a, env)? else { return Err(refusal("express.runtime-arguments")) };
                frame.b = self.detached_scope(frame.scope);
                frame.cursor = cursor;
                frame.position = 0;
                frame.c = 0;
                frame.stage = 1;
                self.push(frame)?;
            }
            1 => {
                if let Some(parameter) = function.parameters.get(frame.position) {
                    if parameter.names.get(frame.c).is_some() {
                        let value = self.item(&mut frame.cursor, env)?.ok_or_else(|| refusal("express.call-arity"))?;
                        frame.stage = 2;
                        self.push(frame)?;
                        self.push(Frame { a: value, name: parameter.type_source, ..Frame::new(Kind::Coerce, frame.scope, frame.depth + 1) })?;
                    } else {
                        frame.position += 1;
                        frame.c = 0;
                        self.push(frame)?;
                    }
                } else {
                    if frame.cursor.next.is_some() {
                        return Err(refusal("express.call-arity"));
                    }
                    frame.stage = 3;
                    self.push(frame)?;
                    self.push(Frame { declarations: function.locals, ..Frame::new(Kind::Declare, frame.b, frame.depth + 1) })?;
                }
            }
            2 => {
                let value = self.pop()?;
                self.bind(frame.b, function.parameters[frame.position].names[frame.c], value, None);
                frame.c += 1;
                frame.stage = 1;
                self.push(frame)?;
            }
            3 => {
                self.flow = Flow::None;
                frame.stage = 4;
                self.push(frame)?;
                self.push(Frame { body: function.body, ..Frame::new(Kind::Body, frame.b, frame.depth + 1) })?;
            }
            _ => {
                let value = if let Flow::Return(value) = self.flow { value } else { self.make(Value::Missing) };
                self.flow = Flow::None;
                self.push(Frame { a: value, name: function.result_type, ..Frame::new(Kind::Coerce, frame.b, frame.depth + 1) })?;
            }
        }
        Ok(())
    }
    fn repeat(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let statement = frame.statement.ok_or_else(|| refusal("express.ast"))?;
        if frame.stage < 4 {
            if statement.name.is_none() {
                frame.i = 1;
                frame.k = 1;
                frame.stage = 4;
                return self.push(frame);
            }
            if frame.stage > 0 {
                let value = self.pop()?;
                let Some(value) = self.integer(value, env)? else { return Ok(()) };
                match frame.stage {
                    1 => frame.i = value,
                    2 => frame.j = value,
                    _ => {
                        if value == 0 {
                            return Ok(());
                        }
                        frame.k = value;
                    }
                }
            }
            if frame.stage < 3 {
                let index = frame.stage;
                frame.stage += 1;
                self.push(frame)?;
                return self.push(Frame::eval(statement.expressions.get(index).ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1));
            }
            frame.stage = 4;
            return self.push(frame);
        }
        if frame.stage == 4 {
            if frame.position >= 1_000_000 {
                return Err(refusal("express.iteration-budget"));
            }
            frame.position += 1;
            self.iterations += 1;
            if let Some(name) = statement.name {
                if if frame.k > 0 { frame.i > frame.j } else { frame.i < frame.j } {
                    return Ok(());
                }
                let value = self.make(Value::Integer(frame.i));
                if frame.d == NONE {
                    frame.d = self.bind(frame.scope, name, value, None);
                } else {
                    self.store_mut().bindings[frame.d].value = value;
                }
            }
            if let Some(condition) = statement.while_condition {
                frame.stage = 5;
                self.push(frame)?;
                return self.push(Frame::eval(condition, frame.scope, frame.depth + 1));
            }
            frame.stage = 6;
            self.push(frame)?;
            return self.push(Frame { body: statement.body, ..Frame::new(Kind::Body, frame.scope, frame.depth + 1) });
        }
        if frame.stage == 5 {
            let result = self.pop()?;
            if self.truth(result, env)? != Logical::True {
                return Ok(());
            }
            frame.stage = 6;
            self.push(frame)?;
            return self.push(Frame { body: statement.body, ..Frame::new(Kind::Body, frame.scope, frame.depth + 1) });
        }
        if frame.stage == 6 {
            match self.flow {
                Flow::Return(_) => return Ok(()),
                Flow::Escape => {
                    self.flow = Flow::None;
                    return Ok(());
                }
                _ => self.flow = Flow::None,
            }
            if let Some(condition) = statement.until_condition {
                frame.stage = 7;
                self.push(frame)?;
                return self.push(Frame::eval(condition, frame.scope, frame.depth + 1));
            }
        } else {
            let value = self.pop()?;
            if self.truth(value, env)? == Logical::True {
                return Ok(());
            }
        }
        if statement.name.is_some() {
            frame.i = frame.i.checked_add(frame.k).ok_or_else(|| refusal("express.integer-overflow"))?;
        }
        frame.stage = 4;
        self.push(frame)
    }
    fn assign(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let target = frame.expression.ok_or_else(|| refusal("express.ast"))?;
        if target.kind == ExpressExpressionKind::Identifier {
            if frame.stage == 11 {
                let value = self.pop()?;
                self.store_mut().bindings[frame.b].value = value;
                return Ok(());
            }
            if frame.stage == 0 {
                frame.b = frame.scope;
                frame.position = self.store().scopes[frame.scope].bindings.unwrap_or(NONE);
                frame.stage = 10;
                return self.push(frame);
            }
            if frame.position != NONE {
                let binding = self.store().bindings[frame.position];
                if binding.name.eq_ignore_ascii_case(target.name.unwrap_or("")) {
                    if let Some(declaration) = binding.declaration {
                        frame.b = frame.position;
                        frame.stage = 11;
                        self.push(frame)?;
                        return self.push(Frame { a: frame.a, name: declaration, ..Frame::new(Kind::Coerce, frame.scope, frame.depth + 1) });
                    }
                    self.store_mut().bindings[frame.position].value = frame.a;
                    return Ok(());
                }
                frame.position = binding.next.unwrap_or(NONE);
                return self.push(frame);
            }
            if let Some(parent) = self.store().scopes[frame.b].parent {
                frame.b = parent;
                frame.position = self.store().scopes[parent].bindings.unwrap_or(NONE);
                return self.push(frame);
            }
            self.bind(frame.scope, target.name.unwrap_or(""), frame.a, None);
            return Ok(());
        }
        if target.kind == ExpressExpressionKind::Attribute {
            if frame.stage == 0 {
                frame.stage = 1;
                self.push(frame)?;
                return self.push(Frame::eval(target.operands.first().ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1));
            }
            let owner = self.pop()?;
            let Raw::Entity(identity) = self.raw(owner, env)? else { return Err(refusal("express.assignment-identity")) };
            let entity = match identity {
                Entity::Constructed(index) => &mut self.store_mut().entities[index],
                Entity::View(index) => &mut self.store_mut().views[index].record,
                Entity::Original(_) => return Err(refusal("express.assignment-identity")),
            };
            let version = entity.version.checked_add(1).ok_or_else(|| refusal("express.runtime-mutation-version"))?;
            if let Some(field) = entity.fields[..entity.length].iter_mut().flatten().find(|(name, _)| name.eq_ignore_ascii_case(target.name.unwrap_or(""))) {
                field.1 = frame.a;
            } else {
                if entity.length == entity.fields.len() {
                    return Err(refusal("express.constructor-field-budget"));
                }
                entity.fields[entity.length] = Some((target.name.unwrap_or(""), frame.a));
                entity.length += 1;
            }
            entity.version = version;
            return Ok(());
        }
        if target.kind != ExpressExpressionKind::Index {
            return Err(refusal("express.assignment"));
        }
        match frame.stage {
            0 => {
                frame.stage = 1;
                self.push(frame)?;
                self.push(Frame::eval(target.operands.first().ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1))?;
            }
            1 => {
                frame.b = self.pop()?;
                frame.stage = 2;
                self.push(frame)?;
                self.push(Frame::eval(target.operands.get(1).ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1))?;
            }
            2 => {
                let index = self.pop()?;
                let Some(index) = self.integer(index, env)? else { return Err(refusal("express.assignment-index")) };
                let Raw::Aggregate { cursor, kind, lower, length } = self.raw(frame.b, env)? else { return Err(refusal("express.assignment")) };
                let at = index.checked_sub(lower).and_then(|index| usize::try_from(index).ok()).filter(|index| *index < length).ok_or_else(|| refusal("express.assignment-index"))?;
                frame.i = at as i128;
                frame.position = 0;
                frame.cursor = cursor;
                frame.c = self.make(Value::Aggregate(Aggregate { lower, ..Aggregate::empty(kind) }));
                frame.stage = 3;
                self.push(frame)?;
            }
            _ => {
                if let Some(value) = self.item(&mut frame.cursor, env)? {
                    self.append(frame.c, if frame.position as i128 == frame.i { frame.a } else { value })?;
                    frame.position += 1;
                    self.push(frame)?;
                } else {
                    frame.a = self.retag(frame.b, frame.c, env)?;
                    frame.expression = Some(target.operands.first().ok_or_else(|| refusal("express.ast"))?);
                    frame.stage = 0;
                    frame.depth += 1;
                    self.push(frame)?;
                }
            }
        }
        Ok(())
    }
}
