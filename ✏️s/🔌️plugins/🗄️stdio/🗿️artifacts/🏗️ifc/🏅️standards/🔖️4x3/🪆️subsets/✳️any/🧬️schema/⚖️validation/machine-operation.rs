fn logical_and(a: Logical, b: Logical) -> Logical {
    if a == Logical::False || b == Logical::False {
        Logical::False
    } else if a == Logical::True && b == Logical::True {
        Logical::True
    } else {
        Logical::Unknown
    }
}
fn logical_or(a: Logical, b: Logical) -> Logical {
    if a == Logical::True || b == Logical::True {
        Logical::True
    } else if a == Logical::False && b == Logical::False {
        Logical::False
    } else {
        Logical::Unknown
    }
}
fn inverse(value: Logical) -> Logical {
    match value {
        Logical::True => Logical::False,
        Logical::False => Logical::True,
        _ => Logical::Unknown,
    }
}
fn boolean(value: bool) -> Value {
    Value::Logical(if value { Logical::True } else { Logical::False })
}
fn number(raw: Raw) -> Result<f64, ValueError> {
    match raw {
        Raw::Real(value) if value.is_finite() => Ok(value),
        Raw::Integer(integer) => {
            let value = integer as f64;
            if value >= 2f64.powi(127) || value < -2f64.powi(127) || value as i128 != integer {
                return Err(refusal("express.integer-precision"));
            }
            Ok(value)
        }
        _ => Err(refusal("express.numeric-operand")),
    }
}
fn numeric_order(left: Raw, right: Raw) -> Result<Ordering, ValueError> {
    match (left, right) {
        (Raw::Integer(a), Raw::Integer(b)) => Ok(a.cmp(&b)),
        (Raw::Real(a), Raw::Real(b)) => a.partial_cmp(&b).ok_or_else(|| refusal("express.numeric-overflow")),
        (Raw::Integer(a), Raw::Real(b)) => {
            if !b.is_finite() {
                return Err(refusal("express.numeric-overflow"));
            }
            if b >= 2f64.powi(127) {
                return Ok(Ordering::Less);
            }
            if b < -2f64.powi(127) {
                return Ok(Ordering::Greater);
            }
            let order = a.cmp(&(b.trunc() as i128));
            Ok(if order != Ordering::Equal {
                order
            } else if b.fract() > 0.0 {
                Ordering::Less
            } else if b.fract() < 0.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            })
        }
        (Raw::Real(_), Raw::Integer(_)) => Ok(numeric_order(right, left)?.reverse()),
        _ => Err(refusal("express.numeric-operand")),
    }
}
/// ⚖️ Equality, matching and aggregate work are explicit resumable operations, including each uncertain matching edge.
impl RetainedMachine {
    fn binary(&mut self, operator: &'static str, left: Handle, right: Handle, scope: usize, depth: usize, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if !["AND", "OR", "XOR", "=", "<>", ":=:", ":<>:", "IN", "||", "+", "-", "*", "/", "**", "DIV", "MOD", "<", "<=", ">", ">="].contains(&operator) {
            return Err(refusal("express.unsupported-operator"));
        }
        if ["AND", "OR", "XOR"].contains(&operator) {
            let a = self.truth(left, env)?;
            let b = self.truth(right, env)?;
            self.result(Value::Logical(match operator {
                "AND" => logical_and(a, b),
                "OR" => logical_or(a, b),
                _ => {
                    if a == Logical::Unknown || b == Logical::Unknown {
                        Logical::Unknown
                    } else if a != b {
                        Logical::True
                    } else {
                        Logical::False
                    }
                }
            }));
            return Ok(());
        }
        if ["=", "<>", ":=:", ":<>:"].contains(&operator) {
            self.push(Frame { name: if ["<>", ":<>:"].contains(&operator) { "NOT" } else { "PASS" }, ..Frame::new(Kind::CompareResult, scope, depth) })?;
            return self.push(Frame { a: left, b: right, identity: operator.contains(':'), ..Frame::new(Kind::Equal, scope, depth + 1) });
        }
        let a = self.raw(left, env)?;
        let b = self.raw(right, env)?;
        if operator == "IN" && !matches!(b, Raw::Missing | Raw::Aggregate { .. }) {
            return Err(refusal("express.aggregate-operand"));
        }
        if matches!(a, Raw::Missing) || matches!(b, Raw::Missing) {
            self.result(if ["IN", "<", "<=", ">", ">="].contains(&operator) { Value::Logical(Logical::Unknown) } else { Value::Missing });
            return Ok(());
        }
        if operator == "IN" {
            let Raw::Aggregate { cursor, .. } = b else { return Err(refusal("express.aggregate-operand")) };
            return self.push(Frame { a: left, cursor, truth: Logical::False, ..Frame::new(Kind::Contains, scope, depth) });
        }
        if operator == "||" && matches!((a, b), (Raw::Entity(_), Raw::Entity(_))) {
            return self.entity_concat(left, right, env);
        }
        if matches!(a, Raw::Aggregate { .. }) || matches!(b, Raw::Aggregate { .. }) {
            return self.push(Frame { a: left, b: right, name: operator, ..Frame::new(Kind::AggregateBinary, scope, depth) });
        }
        if ["<", "<=", ">", ">="].contains(&operator) {
            let ordering = match (a, b) {
                (Raw::Logical(a), Raw::Logical(b)) => Some((a as u8).cmp(&(b as u8))),
                (Raw::String(_), Raw::String(_)) | (Raw::Enum(_), Raw::Enum(_)) => None,
                _ => Some(numeric_order(a, b)?),
            };
            if let Some(ordering) = ordering {
                self.result(boolean(match operator {
                    "<" => ordering == Ordering::Less,
                    "<=" => ordering != Ordering::Greater,
                    ">" => ordering == Ordering::Greater,
                    _ => ordering != Ordering::Less,
                }));
            } else {
                self.push(Frame { name: operator, ..Frame::new(Kind::CompareResult, scope, depth) })?;
                self.push(Frame { a: left, b: right, ..Frame::new(Kind::TextCompare, scope, depth) })?;
            }
            return Ok(());
        }
        if let (Raw::String(a), Raw::String(b)) = (a, b) {
            if operator == "+" {
                let length = self.text(a, env)?.len().checked_add(self.text(b, env)?.len()).ok_or_else(|| refusal("express.text-budget"))?;
                return self.push(Frame { a: left, b: right, name: "concat", i: length as i128, ..Frame::new(Kind::TextCopy, scope, depth) });
            }
        }
        if let (Raw::Integer(a), Raw::Integer(b)) = (a, b) {
            if ["/", "DIV", "MOD"].contains(&operator) && b == 0 {
                self.result(Value::Missing);
                return Ok(());
            }
            let exact = match operator {
                "+" => Some(a.checked_add(b)),
                "-" => Some(a.checked_sub(b)),
                "*" => Some(a.checked_mul(b)),
                "DIV" => Some(a.checked_div(b)),
                "MOD" => Some(a.checked_rem(b)),
                "**" if b >= 0 => Some(a.checked_pow(u32::try_from(b).map_err(|_| refusal("express.integer-overflow"))?)),
                _ => None,
            };
            if let Some(exact) = exact {
                self.result(Value::Integer(exact.ok_or_else(|| refusal("express.integer-overflow"))?));
                return Ok(());
            }
        }
        let a = number(a)?;
        let b = number(b)?;
        if operator == "/" && b == 0.0 {
            self.result(Value::Missing);
            return Ok(());
        }
        let value = match operator {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "**" => a.powf(b),
            _ => return Err(refusal("express.unsupported-operator")),
        };
        if !value.is_finite() {
            return Err(refusal("express.numeric-overflow"));
        }
        self.result(Value::Real(value));
        Ok(())
    }
    fn operation(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        match frame.kind {
            Kind::BinaryLeft => {
                frame.a = self.pop()?;
                let operator = frame.expression.and_then(|expression| expression.operator).unwrap_or("");
                let truth = self.truth(frame.a, env)?;
                if operator == "AND" && truth == Logical::False || operator == "OR" && truth == Logical::True {
                    self.result(Value::Logical(truth));
                    return Ok(());
                }
                frame.kind = Kind::BinaryRight;
                self.push(frame)?;
                self.push(Frame::eval(frame.expression.and_then(|expression| expression.operands.get(1)).ok_or_else(|| refusal("express.ast"))?, frame.scope, frame.depth + 1))
            }
            Kind::BinaryRight => {
                if frame.stage == 101 {
                    frame.d = self.pop()?;
                    frame.stage = 102;
                    self.push(frame)?;
                    return self.binary(frame.name, frame.a, frame.b, frame.scope, frame.depth, env);
                }
                if frame.stage == 102 {
                    let last = self.pop()?;
                    return self.binary("AND", frame.d, last, frame.scope, frame.depth, env);
                }
                let right = self.pop()?;
                self.binary(frame.expression.and_then(|expression| expression.operator).unwrap_or(frame.name), frame.a, right, frame.scope, frame.depth, env)
            }
            Kind::CompareResult => {
                let result = self.pop()?;
                let value = if frame.name == "PASS" {
                    self.truth(result, env)?
                } else if frame.name == "NOT" {
                    inverse(self.truth(result, env)?)
                } else {
                    let order = self.integer(result, env)?.ok_or_else(|| refusal("express.runtime-ordering"))?;
                    if match frame.name {
                        "=" => order == 0,
                        "<" => order < 0,
                        "<=" => order <= 0,
                        ">" => order > 0,
                        _ => order >= 0,
                    } {
                        Logical::True
                    } else {
                        Logical::False
                    }
                };
                self.result(Value::Logical(value));
                Ok(())
            }
            Kind::TextCompare => {
                let text = |raw| match raw {
                    Raw::String(text) | Raw::Enum(text) | Raw::Binary(text) => Ok(text),
                    _ => Err(refusal("express.operand")),
                };
                let a = self.text(text(self.raw(frame.a, env)?)?, env)?;
                let b = self.text(text(self.raw(frame.b, env)?)?, env)?;
                let end = (frame.position + 64).min(a.len().min(b.len()));
                let order = a.as_bytes()[frame.position..end].cmp(&b.as_bytes()[frame.position..end]);
                if order != Ordering::Equal {
                    self.result(Value::Integer(if order == Ordering::Less { -1 } else { 1 }));
                } else if end == a.len().min(b.len()) {
                    self.result(Value::Integer(match a.len().cmp(&b.len()) {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    }));
                } else {
                    frame.position = end;
                    self.push(frame)?;
                }
                Ok(())
            }
            Kind::Equal => self.equal(frame, env),
            Kind::EqualSequence => {
                if frame.stage == 1 {
                    let result = self.pop()?;
                    let truth = self.truth(result, env)?;
                    if truth == Logical::False {
                        self.result(Value::Logical(Logical::False));
                        return Ok(());
                    }
                    if truth == Logical::Unknown {
                        frame.truth = truth;
                    }
                }
                if frame.name == "entity" {
                    return self.entity_equal_next(frame, env);
                }
                match (self.item(&mut frame.cursor, env)?, self.item(&mut frame.other, env)?) {
                    (Some(a), Some(b)) => {
                        frame.stage = 1;
                        self.push(frame)?;
                        self.push(Frame { a, b, identity: frame.identity, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) })?;
                    }
                    (None, None) => self.result(Value::Logical(frame.truth)),
                    _ => self.result(Value::Logical(Logical::False)),
                }
                Ok(())
            }
            Kind::Contains => {
                if frame.stage == 1 {
                    let result = self.pop()?;
                    let truth = self.truth(result, env)?;
                    if truth == Logical::True {
                        self.result(Value::Logical(truth));
                        return Ok(());
                    }
                    if truth == Logical::Unknown {
                        frame.truth = truth;
                    }
                }
                if let Some(candidate) = self.item(&mut frame.cursor, env)? {
                    frame.stage = 1;
                    self.push(frame)?;
                    self.push(Frame { a: frame.a, b: candidate, identity: true, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) })?;
                } else {
                    self.result(Value::Logical(frame.truth));
                }
                Ok(())
            }
            Kind::Match => self.matching(frame, env),
            Kind::AggregateBinary => self.aggregate_binary(frame, env),
            _ => Err(refusal("express.runtime-operation")),
        }
    }
    fn equal(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage == 0 && (matches!(self.raw(frame.a, env)?, Raw::Entity(_)) || matches!(self.raw(frame.b, env)?, Raw::Entity(_))) {
            frame.stage = 1;
            self.push(frame)?;
            self.store_mut().results.push(frame.a);
            return self.push(Frame::new(Kind::RootRead, frame.scope, frame.depth + 1));
        }
        if frame.stage == 1 {
            frame.a = self.pop()?;
            frame.stage = 2;
            self.push(frame)?;
            self.store_mut().results.push(frame.b);
            return self.push(Frame::new(Kind::RootRead, frame.scope, frame.depth + 1));
        }
        if frame.stage == 2 { frame.b = self.pop()?; }
        frame.stage = 0;
        let a = self.raw(frame.a, env)?;
        let b = self.raw(frame.b, env)?;
        let value = match (a, b) {
            (Raw::Missing, _) | (_, Raw::Missing) => Value::Logical(Logical::Unknown),
            (Raw::Integer(_) | Raw::Real(_), Raw::Integer(_) | Raw::Real(_)) => boolean(numeric_order(a, b)? == Ordering::Equal),
            (Raw::Logical(a), Raw::Logical(b)) => boolean(a == b),
            (Raw::String(_), Raw::String(_)) | (Raw::Binary(_), Raw::Binary(_)) | (Raw::Enum(_), Raw::Enum(_)) => {
                self.push(Frame { name: "=", ..Frame::new(Kind::CompareResult, frame.scope, frame.depth) })?;
                frame.kind = Kind::TextCompare;
                return self.push(frame);
            }
            (Raw::Entity(a), Raw::Entity(b)) => {
                if self.canonical_entity(a) == self.canonical_entity(b) {
                    boolean(true)
                } else if frame.identity {
                    boolean(false)
                } else {
                    if self.entity_declaration(a)?.key != self.entity_declaration(b)?.key {
                        self.result(boolean(false));
                        return Ok(());
                    }
                    frame.kind = Kind::EqualSequence;
                    frame.name = "entity";
                    frame.position = 0;
                    return self.push(frame);
                }
            }
            (Raw::Aggregate { cursor, kind: a, length: al, .. }, Raw::Aggregate { cursor: other, kind: b, length: bl, .. }) => {
                if al != bl {
                    boolean(false)
                } else if matches!(a, AggregateKind::Set | AggregateKind::Bag) || matches!(b, AggregateKind::Set | AggregateKind::Bag) {
                    frame.kind = Kind::Match;
                    return self.push(frame);
                } else {
                    frame.kind = Kind::EqualSequence;
                    frame.cursor = cursor;
                    frame.other = other;
                    return self.push(frame);
                }
            }
            _ => boolean(false),
        };
        self.result(value);
        Ok(())
    }
    fn matching(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage == 0 {
            let (Raw::Aggregate { cursor, length: left, .. }, Raw::Aggregate { cursor: other, length: right, .. }) = (self.raw(frame.a, env)?, self.raw(frame.b, env)?) else { return Err(refusal("express.aggregate-operand")) };
            if left > right {
                self.result(boolean(false));
                return Ok(());
            }
            if left == 0 {
                self.result(boolean(true));
                return Ok(());
            }
            let cells = left.checked_mul(right).and_then(|count| count.checked_add(left + right + 2 * right + 3 * MAXIMUM_DEPTH)).ok_or_else(|| refusal("express.node-budget"))?;
            if cells > MAXIMUM_NODES {
                return Err(refusal("express.node-budget"));
            }
            frame.i = left as i128;
            frame.j = right as i128;
            frame.c = self.store().scratch.len();
            frame.cursor = cursor;
            frame.other = other;
            frame.stage = 1;
            frame.position = 0;
            frame.d = cells;
            return self.push(frame);
        }
        let left = frame.i as usize;
        let right = frame.j as usize;
        let matrix = frame.c + left + right;
        let matched = matrix + left * right;
        let seen = matched + right;
        let stack = seen + right;
        let next = stack + MAXIMUM_DEPTH;
        let edges = next + MAXIMUM_DEPTH;
        match frame.stage {
            1 => {
                let value = if frame.position < left { self.item(&mut frame.cursor, env)? } else { self.item(&mut frame.other, env)? };
                if let Some(value) = value {
                    self.store_mut().scratch.push(value);
                    frame.position += 1;
                }
                if frame.position == left + right {
                    frame.stage = 2;
                    frame.position = 0;
                }
                self.push(frame)
            }
            2 => {
                if self.store().scratch.len() < frame.c + frame.d {
                    let at = self.store().scratch.len();
                    self.store_mut().scratch.push(if at >= matched && at < seen { NONE } else { 0 });
                    self.push(frame)
                } else {
                    frame.stage = 3;
                    frame.position = 0;
                    self.push(frame)
                }
            }
            3 => {
                if frame.position == left * right {
                    frame.stage = 6;
                    frame.k = 0;
                    frame.position = 0;
                    frame.truth = Logical::True;
                    return self.push(frame);
                }
                let a = self.store().scratch[frame.c + frame.position / right];
                let b = self.store().scratch[frame.c + left + frame.position % right];
                frame.stage = 4;
                self.push(frame)?;
                self.push(Frame { a, b, identity: frame.identity, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) })
            }
            4 => {
                let result = self.pop()?;
                let value = self.truth(result, env)? as usize;
                self.store_mut().scratch[matrix + frame.position] = value;
                frame.position += 1;
                frame.stage = 3;
                self.push(frame)
            }
            6 => {
                if frame.k as usize == left {
                    self.store_mut().scratch.truncate(frame.c);
                    self.result(Value::Logical(frame.truth));
                    return Ok(());
                }
                if frame.position < right {
                    self.store_mut().scratch[seen + frame.position] = 0;
                    frame.position += 1;
                    return self.push(frame);
                }
                self.store_mut().scratch[stack] = frame.k as usize;
                self.store_mut().scratch[next] = 0;
                frame.position = 0;
                frame.stage = 7;
                self.push(frame)
            }
            7 => {
                let depth = frame.position;
                let row = self.store().scratch[stack + depth];
                let candidate = self.store().scratch[next + depth];
                if candidate == right {
                    if depth > 0 {
                        frame.position -= 1;
                        return self.push(frame);
                    }
                    if frame.truth == Logical::True {
                        frame.truth = Logical::Unknown;
                        frame.stage = 9;
                        frame.position = 0;
                        frame.k = 0;
                        return self.push(frame);
                    }
                    self.store_mut().scratch.truncate(frame.c);
                    self.result(boolean(false));
                    return Ok(());
                }
                self.store_mut().scratch[next + depth] += 1;
                let comparison = self.store().scratch[matrix + row * right + candidate];
                if self.store().scratch[seen + candidate] != 0 || comparison != Logical::True as usize && !(frame.truth == Logical::Unknown && comparison == Logical::Unknown as usize) {
                    return self.push(frame);
                }
                self.store_mut().scratch[seen + candidate] = 1;
                let previous = self.store().scratch[matched + candidate];
                if previous == NONE {
                    self.store_mut().scratch[matched + candidate] = row;
                    frame.stage = 8;
                } else {
                    if depth + 1 == MAXIMUM_DEPTH {
                        return Err(refusal("express.depth"));
                    }
                    self.store_mut().scratch[edges + depth] = candidate;
                    self.store_mut().scratch[stack + depth + 1] = previous;
                    self.store_mut().scratch[next + depth + 1] = 0;
                    frame.position += 1;
                }
                self.push(frame)
            }
            8 => {
                if frame.position > 0 {
                    frame.position -= 1;
                    let row = self.store().scratch[stack + frame.position];
                    let edge = self.store().scratch[edges + frame.position];
                    self.store_mut().scratch[matched + edge] = row;
                } else {
                    frame.k += 1;
                    frame.stage = 6;
                }
                self.push(frame)
            }
            9 => {
                if frame.position < right {
                    self.store_mut().scratch[matched + frame.position] = NONE;
                    frame.position += 1;
                } else {
                    frame.position = 0;
                    frame.stage = 6;
                }
                self.push(frame)
            }
            _ => Err(refusal("express.runtime-matching")),
        }
    }

    fn aggregate_binary(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage == 0 {
            let a = self.raw(frame.a, env)?;
            let b = self.raw(frame.b, env)?;
            let left_kind = if let Raw::Aggregate { kind, .. } = a { kind } else { AggregateKind::List };
            let right_kind = if let Raw::Aggregate { kind, .. } = b { kind } else { AggregateKind::List };
            let kind = if frame.name == "*" && (left_kind == AggregateKind::Set || right_kind == AggregateKind::Set) {
                AggregateKind::Set
            } else if matches!(a, Raw::Aggregate { .. }) {
                left_kind
            } else {
                right_kind
            };
            if !matches!(a, Raw::Aggregate { .. }) {
                let value = frame.a;
                frame.a = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::List)));
                self.append(frame.a, value)?;
            }
            if !matches!(b, Raw::Aggregate { .. }) {
                let value = frame.b;
                frame.b = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::List)));
                self.append(frame.b, value)?;
            }
            if ["<=", ">="].contains(&frame.name) && (left_kind == AggregateKind::Bag || right_kind == AggregateKind::Bag) {
                if frame.name == ">=" {
                    std::mem::swap(&mut frame.a, &mut frame.b);
                }
                frame.kind = Kind::Match;
                frame.identity = true;
                return self.push(frame);
            }
            if frame.name == ">=" {
                std::mem::swap(&mut frame.a, &mut frame.b);
            }
            let Raw::Aggregate { cursor, .. } = self.raw(frame.a, env)? else { return Err(refusal("express.aggregate-operand")) };
            let Raw::Aggregate { cursor: other, .. } = self.raw(frame.b, env)? else { return Err(refusal("express.aggregate-operand")) };
            frame.cursor = cursor;
            frame.other = other;
            frame.c = self.make(Value::Aggregate(Aggregate::empty(kind)));
            frame.d = self.store().scratch.len();
            frame.identity = if frame.name == "+" {
                kind == AggregateKind::Set
            } else if frame.name == "*" {
                left_kind == AggregateKind::Bag || right_kind == AggregateKind::Bag
            } else {
                matches!(left_kind, AggregateKind::Bag | AggregateKind::Set)
            };
            frame.stage = 1;
            frame.position = 0;
            return self.push(frame);
        }
        if ["<=", ">="].contains(&frame.name) {
            if frame.stage == 2 {
                let result = self.pop()?;
                match self.truth(result, env)? {
                    Logical::False => {
                        self.result(boolean(false));
                        return Ok(());
                    }
                    Logical::Unknown => frame.truth = Logical::Unknown,
                    _ => {}
                }
            }
            if let Some(value) = self.item(&mut frame.cursor, env)? {
                let Raw::Aggregate { cursor, .. } = self.raw(frame.b, env)? else { return Err(refusal("express.aggregate-operand")) };
                frame.stage = 2;
                self.push(frame)?;
                self.push(Frame { a: value, cursor, truth: Logical::False, ..Frame::new(Kind::Contains, frame.scope, frame.depth + 1) })?;
            } else {
                self.result(Value::Logical(frame.truth));
            }
            return Ok(());
        }
        if frame.name == "+" {
            if frame.stage == 1 {
                if let Some(value) = self.item(&mut frame.cursor, env)? {
                    self.append(frame.c, value)?;
                } else {
                    frame.stage = 2;
                }
                return self.push(frame);
            }
            if frame.stage == 3 {
                let result = self.pop()?;
                match self.truth(result, env)? {
                    Logical::Unknown => return Err(refusal("express.aggregate-indeterminate")),
                    Logical::False => self.append(frame.c, frame.a)?,
                    _ => {}
                }
                frame.stage = 2;
                return self.push(frame);
            }
            if let Some(value) = self.item(&mut frame.other, env)? {
                if frame.identity {
                    frame.a = value;
                    frame.stage = 3;
                    let Raw::Aggregate { cursor, .. } = self.raw(frame.c, env)? else { return Err(refusal("express.aggregate-builder")) };
                    self.push(frame)?;
                    self.push(Frame { a: value, cursor, truth: Logical::False, ..Frame::new(Kind::Contains, frame.scope, frame.depth + 1) })?;
                } else {
                    self.append(frame.c, value)?;
                    self.push(frame)?;
                }
            } else {
                self.store_mut().results.push(frame.c);
            }
            return Ok(());
        }
        if !["-", "*"].contains(&frame.name) {
            return Err(refusal("express.aggregate-operator"));
        }
        if frame.stage == 1 {
            let value = if frame.name == "-" { self.item(&mut frame.cursor, env)? } else { self.item(&mut frame.other, env)? };
            if let Some(value) = value {
                self.store_mut().scratch.push(value);
            } else {
                frame.stage = 2;
            }
            return self.push(frame);
        }
        if frame.stage == 2 {
            let value = if frame.name == "-" { self.item(&mut frame.other, env)? } else { self.item(&mut frame.cursor, env)? };
            if let Some(value) = value {
                frame.a = value;
                frame.position = 0;
                frame.stage = 4;
                return self.push(frame);
            }
            if frame.name == "*" {
                self.store_mut().scratch.truncate(frame.d);
                self.store_mut().results.push(frame.c);
                return Ok(());
            }
            frame.position = 0;
            frame.stage = 7;
            return self.push(frame);
        }
        if frame.stage == 4 {
            let index = frame.d + frame.position;
            if index == self.store().scratch.len() {
                frame.stage = 2;
                return self.push(frame);
            }
            let candidate = self.store().scratch[index];
            frame.stage = 5;
            self.push(frame)?;
            return self.push(Frame { a: frame.a, b: candidate, identity: true, ..Frame::new(Kind::Equal, frame.scope, frame.depth + 1) });
        }
        if frame.stage == 5 {
            let result = self.pop()?;
            match self.truth(result, env)? {
                Logical::Unknown => return Err(refusal("express.aggregate-indeterminate")),
                Logical::False => {
                    frame.position += 1;
                    frame.stage = 4;
                    return self.push(frame);
                }
                Logical::True => {
                    if frame.name == "*" {
                        self.append(frame.c, frame.a)?;
                    }
                    if frame.identity {
                        let end = self.store().scratch.len() - 1;
                        let last = self.store().scratch[end];
                        self.store_mut().scratch[frame.d + frame.position] = last;
                        self.store_mut().scratch.pop();
                        frame.stage = 2;
                    } else if frame.name == "-" {
                        frame.stage = 6;
                    } else {
                        frame.stage = 2;
                    }
                    return self.push(frame);
                }
            }
        }
        if frame.stage == 6 {
            let index = frame.d + frame.position;
            if index + 1 < self.store().scratch.len() {
                let value = self.store().scratch[index + 1];
                self.store_mut().scratch[index] = value;
                frame.position += 1;
            } else {
                self.store_mut().scratch.pop();
                frame.stage = 2;
            }
            return self.push(frame);
        }
        if frame.stage == 7 {
            if let Some(value) = self.store().scratch.get(frame.d + frame.position).copied() {
                self.append(frame.c, value)?;
                frame.position += 1;
                self.push(frame)?;
            } else {
                self.store_mut().scratch.truncate(frame.d);
                self.store_mut().results.push(frame.c);
            }
            return Ok(());
        }
        Err(refusal("express.runtime-aggregate"))
    }
}
