/// 🧭 Each expression continuation schedules children instead of recursively evaluating semantic work.
impl RetainedMachine {
    fn execute(&mut self, frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        match frame.kind {
            Kind::Eval => {
                self.push(Frame::new(Kind::RootRead, frame.scope, frame.depth))?;
                self.push(Frame { kind: Kind::EvalInner, ..frame })
            }
            Kind::EvalInner => self.expression(frame, env),
            Kind::RootRead => self.root_read(frame, env),
            Kind::Dependency => self.dependency(frame),
            Kind::PreparedQuery => self.prepared_query(frame, env),
            Kind::Lookup => self.lookup(frame, env),
            Kind::Unary => self.unary(frame, env),
            Kind::BinaryLeft | Kind::BinaryRight | Kind::Equal | Kind::EqualSequence | Kind::Contains | Kind::AggregateBinary | Kind::Match | Kind::TextCompare | Kind::CompareResult => self.operation(frame, env),
            Kind::Aggregate | Kind::AggregateRepeat => self.aggregate_expression(frame, env),
            Kind::Query => self.query(frame, env),
            Kind::Index | Kind::Slice => self.index_slice(frame, env),
            Kind::TextCopy => self.text_copy(frame, env),
            Kind::CallArgs | Kind::Call | Kind::Function | Kind::Declare | Kind::Coerce | Kind::Body | Kind::Statement | Kind::Assign | Kind::Case | Kind::Repeat => self.statement(frame, env),
            Kind::Attribute | Kind::TypeOf | Kind::UsedIn => self.population_operation(frame, env),
            Kind::Header => self.header(frame, env),
            Kind::Normalize => self.normalize(frame, env),
            Kind::Domain => self.domain(frame, env),
            Kind::Select => self.select_domain(frame, env),
            Kind::Population | Kind::TypeRules | Kind::Bound | Kind::InverseRule | Kind::UniqueRule | Kind::Global | Kind::Record | Kind::AggregateUnique => self.population_rules(frame, env),
        }
    }
    fn expression(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let expression = frame.expression.ok_or_else(|| refusal("express.ast"))?;
        if expression.name.is_some_and(|name| name.len() > 1024) || expression.operator.is_some_and(|operator| operator.len() > 16) {
            return Err(refusal("express.ast-budget"));
        }
        use ExpressExpressionKind::*;
        match expression.kind {
            Unknown => self.result(Value::Missing),
            Integer | Real => {
                let text = expression.text.unwrap_or("");
                if text.len() > 128 {
                    return Err(refusal("express.numeric-budget"));
                }
                if expression.kind == Integer {
                    self.result(Value::Integer(text.parse().map_err(|_| refusal("express.integer-overflow"))?));
                } else {
                    let value: f64 = text.parse().map_err(|_| refusal("express.numeric-literal"))?;
                    if !value.is_finite() {
                        return Err(refusal("express.numeric-overflow"));
                    }
                    self.result(Value::Real(value));
                }
            }
            String => self.result(Value::String(Text::Static(expression.text.unwrap_or("")))),
            Logical => self.result(Value::Logical(match expression.text.unwrap_or("") {
                "true" => self::Logical::True,
                "false" => self::Logical::False,
                "unknown" => self::Logical::Unknown,
                _ => return Err(refusal("express.ast")),
            })),
            Identifier => {
                frame.kind = Kind::Lookup;
                frame.name = expression.name.unwrap_or("");
                frame.a = frame.scope;
                frame.position = self.store().scopes[frame.scope].bindings.unwrap_or(NONE);
                self.push(frame)?;
            }
            Aggregate => {
                frame.kind = Kind::Aggregate;
                frame.a = self.make(Value::Aggregate(self::Aggregate::empty(AggregateKind::List)));
                self.push(frame)?;
            }
            Unary | Qualify | Attribute => {
                frame.kind = if expression.kind == Unary { Kind::Unary } else { Kind::Attribute };
                frame.stage = if expression.kind == Qualify { 100 } else { 0 };
                self.push(frame)?;
                self.push(Frame::eval(expression.operands.first().ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1))?;
            }
            Binary => {
                frame.kind = Kind::BinaryLeft;
                self.push(frame)?;
                self.push(Frame::eval(expression.operands.first().ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1))?;
            }
            Call => {
                frame.kind = Kind::CallArgs;
                frame.name = expression.operands.first().and_then(|operand| operand.name).ok_or_else(|| refusal("express.call"))?;
                frame.a = self.make(Value::Aggregate(self::Aggregate::empty(AggregateKind::List)));
                frame.position = 1;
                if frame.name == "NVL" {
                    if expression.operands.len() != 3 {
                        return Err(refusal("express.call-arity"));
                    }
                    frame.stage = 10;
                }
                self.push(frame)?;
            }
            Query => {
                frame.kind = Kind::Query;
                self.push(frame)?;
            }
            Index | Slice | Interval => {
                frame.kind = if expression.kind == Index { Kind::Index } else { Kind::Slice };
                frame.stage = if expression.kind == Interval { 100 } else { 0 };
                self.push(frame)?;
            }
        }
        let _ = env;
        Ok(())
    }
    fn lookup(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage < 2 {
            if frame.position != NONE {
                let binding = self.store().bindings[frame.position];
                if binding.name.eq_ignore_ascii_case(frame.name) {
                    if frame.stage == 0 {
                        self.store_mut().results.push(binding.value);
                        return Ok(());
                    }
                    if matches!(self.raw(binding.value, env)?, Raw::Entity(_)) {
                        frame.kind = Kind::Attribute;
                        frame.name = frame.expression.and_then(|expression| expression.name).unwrap_or("");
                        frame.a = binding.value;
                        frame.stage = 1;
                        frame.identity = true;
                        return self.push(frame);
                    }
                    frame.stage = 2;
                    frame.position = 0;
                    frame.b = 0;
                } else {
                    frame.position = binding.next.unwrap_or(NONE);
                    return self.push(frame);
                }
            } else if let Some(parent) = self.store().scopes[frame.scope].parent {
                frame.scope = parent;
                frame.position = self.store().scopes[parent].bindings.unwrap_or(NONE);
                return self.push(frame);
            } else if frame.stage == 0 {
                frame.stage = 1;
                frame.scope = frame.a;
                frame.position = self.store().scopes[frame.scope].bindings.unwrap_or(NONE);
                frame.name = "SELF";
                return self.push(frame);
            } else {
                frame.stage = 2;
                frame.position = 0;
                frame.b = 0;
            }
        }
        let name = frame.expression.and_then(|expression| expression.name).unwrap_or(frame.name);
        if name == "PI" {
            self.result(Value::Real(std::f64::consts::PI));
            return Ok(());
        }
        if metadata::entity(name).is_some() || metadata::defined_type(name).is_some() {
            self.result(Value::Enum(Text::Static(name)));
            return Ok(());
        }
        if let Some(declaration) = metadata::TYPES.get(frame.position) {
            if declaration.kind == "enum" {
                if let Some(member) = declaration.members.get(frame.b) {
                    if *member == name {
                        self.result(Value::Enum(Text::Static(name)));
                        return Ok(());
                    }
                    frame.b += 1;
                    return self.push(frame);
                }
            }
            frame.position += 1;
            frame.b = 0;
            return self.push(frame);
        }
        Err(refusal("express.unbound"))
    }
    fn unary(&mut self, frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let value = self.pop()?;
        let raw = self.raw(value, env)?;
        match frame.expression.and_then(|expression| expression.operator).unwrap_or("") {
            "NOT" => {
                let truth = self.truth(value, env)?;
                self.result(Value::Logical(match truth {
                    Logical::True => Logical::False,
                    Logical::False => Logical::True,
                    _ => Logical::Unknown,
                }));
            }
            "+" if matches!(raw, Raw::Missing | Raw::Integer(_) | Raw::Real(_)) => self.store_mut().results.push(value),
            "-" => self.result(match raw {
                Raw::Missing => Value::Missing,
                Raw::Integer(value) => Value::Integer(value.checked_neg().ok_or_else(|| refusal("express.integer-overflow"))?),
                Raw::Real(value) => Value::Real(-value),
                _ => return Err(refusal("express.operand")),
            }),
            _ => return Err(refusal("express.operator")),
        }
        Ok(())
    }
    fn aggregate_expression(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let expression = frame.expression.ok_or_else(|| refusal("express.ast"))?;
        if matches!(frame.kind, Kind::AggregateRepeat) {
            if frame.i > 0 {
                self.append(frame.a, frame.b)?;
                frame.i -= 1;
                return self.push(frame);
            }
            frame.kind = Kind::Aggregate;
            frame.position += 1;
            frame.stage = 0;
        }
        let Some(entry) = expression.entries.get(frame.position) else {
            self.store_mut().results.push(frame.a);
            return Ok(());
        };
        match frame.stage {
            0 => {
                frame.stage = 1;
                self.push(frame)?;
                self.push(Frame::eval(entry.value, frame.scope, frame.depth + 1))?;
            }
            1 => {
                frame.b = self.pop()?;
                if let Some(repeat) = entry.repeat {
                    frame.stage = 2;
                    self.push(frame)?;
                    self.push(Frame::eval(repeat, frame.scope, frame.depth + 1))?;
                } else {
                    frame.kind = Kind::AggregateRepeat;
                    frame.i = 1;
                    self.push(frame)?;
                }
            }
            _ => {
                let repeat = self.pop()?;
                if matches!(self.raw(repeat, env)?, Raw::Missing) {
                    self.result(Value::Missing);
                    return Ok(());
                }
                frame.i = self.integer(repeat, env)?.filter(|count| *count >= 0).ok_or_else(|| refusal("express.repeat-count"))?;
                frame.kind = Kind::AggregateRepeat;
                self.push(frame)?;
            }
        }
        Ok(())
    }
    fn query(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let expression = frame.expression.ok_or_else(|| refusal("express.ast"))?;
        match frame.stage {
            0 => {
                frame.stage = 1;
                self.push(frame)?;
                self.push(Frame::eval(expression.operands.first().ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1))?;
            }
            1 => {
                frame.a = self.pop()?;
                if matches!(self.raw(frame.a, env)?, Raw::Missing) {
                    self.result(Value::Missing);
                    return Ok(());
                }
                let (cursor, kind, lower) = if let Raw::Aggregate { cursor, kind, lower, .. } = self.raw(frame.a, env)? { (cursor, kind, lower) } else { (Cursor::default(), AggregateKind::List, 1) };
                frame.cursor = cursor;
                frame.b = self.make(Value::Aggregate(Aggregate { lower, ..Aggregate::empty(kind) }));
                frame.stage = 2;
                self.push(frame)?;
            }
            2 => {
                if let Some(value) = self.item(&mut frame.cursor, env)? {
                    frame.c = value;
                    let scope = self.scope(Some(frame.scope));
                    self.bind(scope, expression.name.unwrap_or(""), value, None);
                    frame.stage = 3;
                    self.push(frame)?;
                    self.push(Frame::eval(expression.operands.get(1).ok_or_else(|| refusal("express.ast"))?, scope, frame.depth + 1))?;
                } else {
                    self.store_mut().results.push(frame.b);
                }
            }
            _ => {
                let predicate = self.pop()?;
                if self.truth(predicate, env)? == Logical::True {
                    self.append(frame.b, frame.c)?;
                }
                frame.stage = 2;
                self.push(frame)?;
            }
        }
        Ok(())
    }
    fn index_slice(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let expression = frame.expression.ok_or_else(|| refusal("express.ast"))?;
        if frame.stage == 100 {
            frame.stage = 101;
            self.push(frame)?;
            return self.push(Frame::eval(expression.operands.get(1).ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1));
        }
        if frame.stage == 101 {
            frame.b = self.pop()?;
            frame.stage = 102;
            self.push(frame)?;
            return self.push(Frame::eval(expression.operands.first().ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1));
        }
        if frame.stage == 102 {
            frame.a = self.pop()?;
            frame.stage = 103;
            self.push(frame)?;
            return self.push(Frame::eval(expression.operands.get(2).ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1));
        }
        if frame.stage == 103 {
            frame.c = self.pop()?;
            let operator = expression.operator.unwrap_or("");
            let mut operators = operator.split(',');
            frame.kind = Kind::BinaryRight;
            frame.name = operators.next().unwrap_or("");
            frame.stage = 100;
            let last = Frame { a: frame.b, b: frame.c, name: operators.next().unwrap_or(""), stage: 101, ..frame };
            self.push(last)?;
            return self.binary(frame.name, frame.a, frame.b, frame.scope, frame.depth, env);
        }
        if frame.stage < 3 {
            if frame.stage > 0 {
                let value = self.pop()?;
                if frame.stage == 1 {
                    frame.a = value;
                } else {
                    frame.b = value;
                }
            }
            let wanted = if matches!(frame.kind, Kind::Index) { 2 } else { 3 };
            if frame.stage < wanted {
                let index = frame.stage;
                frame.stage += 1;
                self.push(frame)?;
                return self.push(Frame::eval(expression.operands.get(index).ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1));
            }
        } else if frame.stage == 3 && matches!(frame.kind, Kind::Slice) {
            frame.c = self.pop()?;
        }
        if frame.stage <= 3 {
            let Some(low) = self.integer(frame.b, env)? else {
                self.result(Value::Missing);
                return Ok(());
            };
            let high = if matches!(frame.kind, Kind::Index) {
                low
            } else {
                let Some(high) = self.integer(frame.c, env)? else {
                    self.result(Value::Missing);
                    return Ok(());
                };
                high
            };
            let (cursor, lower, length) = match self.raw(frame.a, env)? {
                Raw::Aggregate { cursor, lower, length, .. } => (cursor, lower, Some(length)),
                Raw::String(_) => (Cursor::default(), 1, None),
                _ => {
                    self.result(Value::Missing);
                    return Ok(());
                }
            };
            let (Some(low_offset), Some(high_offset)) = (low.checked_sub(lower), high.checked_sub(lower)) else {
                self.result(Value::Missing);
                return Ok(());
            };
            if low < lower || high < low || length.is_some_and(|length| high_offset >= length as i128) {
                self.result(Value::Missing);
                return Ok(());
            }
            frame.i = low_offset;
            frame.j = high_offset;
            frame.k = 0;
            frame.position = 0;
            frame.cursor = cursor;
            frame.stage = if length.is_some() { 4 } else { 5 };
            if matches!(frame.kind, Kind::Slice) && length.is_some() {
                frame.b = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::List)));
            }
            return self.push(frame);
        }
        if frame.stage == 4 {
            if let Some(value) = self.item(&mut frame.cursor, env)? {
                if frame.k >= frame.i && frame.k <= frame.j {
                    if matches!(frame.kind, Kind::Index) {
                        self.store_mut().results.push(value);
                        return Ok(());
                    }
                    self.append(frame.b, value)?;
                }
                frame.k += 1;
                self.push(frame)?;
            } else if matches!(frame.kind, Kind::Index) {
                self.result(Value::Missing);
            } else {
                self.store_mut().results.push(frame.b);
            }
        } else {
            let Raw::String(text) = self.raw(frame.a, env)? else { return Err(refusal("express.runtime-string")) };
            let text = self.text(text, env)?;
            if let Some(character) = text.get(frame.position..).and_then(|text| text.chars().next()) {
                if frame.k == frame.i {
                    frame.b = frame.position;
                }
                frame.position += character.len_utf8();
                if frame.k == frame.j {
                    frame.c = frame.position;
                }
                frame.k += 1;
                self.push(frame)?;
            } else if frame.k <= frame.j {
                self.result(Value::Missing);
            } else {
                frame.kind = Kind::TextCopy;
                frame.name = "slice";
                frame.i = (frame.c - frame.b) as i128;
                frame.stage = 0;
                frame.position = 0;
                self.push(frame)?;
            }
        }
        Ok(())
    }
    fn text_copy(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage == 0 {
            let length = usize::try_from(frame.i).map_err(|_| refusal("express.text-budget"))?;
            if length > MAXIMUM_NODES {
                return Err(refusal("express.node-budget"));
            }
            frame.d = self.store().texts.len();
            self.store_mut().texts.push(String::with_capacity(length));
            frame.stage = 1;
            return self.push(frame);
        }
        let mut bytes = [0u8; 64];
        let total = frame.i as usize;
        if frame.position == total {
            self.result(Value::String(Text::Owned(frame.d)));
            return Ok(());
        }
        let (handle, offset, limit) = if frame.name == "slice" {
            (frame.a, frame.b + frame.position, frame.c)
        } else {
            let Raw::String(left) = self.raw(frame.a, env)? else { return Err(refusal("express.operand")) };
            let left = self.text(left, env)?.len();
            if frame.position < left { (frame.a, frame.position, left) } else { (frame.b, frame.position - left, total - left) }
        };
        let Raw::String(text) = self.raw(handle, env)? else { return Err(refusal("express.operand")) };
        let text = self.text(text, env)?;
        let mut end = (offset + 64).min(limit);
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        let length = end - offset;
        bytes[..length].copy_from_slice(&text.as_bytes()[offset..end]);
        self.store_mut().texts[frame.d].push_str(std::str::from_utf8(&bytes[..length]).map_err(|_| refusal("express.runtime-text"))?);
        frame.position += length;
        self.push(frame)
    }
}
