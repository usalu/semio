fn identifier(name: &str) -> bool {
    name.len() <= 128 && !name.is_empty() && name.bytes().enumerate().all(|(index, byte)| if index == 0 { byte.is_ascii_uppercase() } else { byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_' })
}
/// 🛬 Header, complex-position and scalar-domain work retain explicit source coordinates throughout admission.
impl RetainedMachine {
    fn header_value<'a>(&self, frame: Frame, env: &'a MachineEnvironment<'_>) -> Result<&'a IfcValue, ValueError> {
        let header = &env.source.ok_or_else(|| refusal("express.runtime-source"))?.header;
        let roots = match frame.a {
            0 => &header.file_description,
            1 => &header.file_name,
            _ => &header.file_schema,
        };
        let mut value = roots.get(frame.b).ok_or_else(|| refusal("express.header-coordinate"))?;
        for child in &self.store().scratch[frame.d..frame.d + frame.c] {
            value = match value {
                IfcValue::Aggregate(items) => items.get(*child),
                IfcValue::TypedValue(typed) => typed.items.get(*child),
                _ => None,
            }
            .ok_or_else(|| refusal("express.header-path"))?;
        }
        Ok(value)
    }
    fn header(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage == 0 {
            if frame.a == NONE {
                frame.a = 0;
                frame.b = 0;
            }
            if frame.a == 3 {
                return Ok(());
            }
            let header = &env.source.ok_or_else(|| refusal("express.runtime-source"))?.header;
            let length = match frame.a {
                0 => header.file_description.len(),
                1 => header.file_name.len(),
                _ => header.file_schema.len(),
            };
            if frame.b == length {
                frame.a += 1;
                frame.b = 0;
                return self.push(frame);
            }
            let child = Frame { a: frame.a, b: frame.b, c: 0, d: self.store().scratch.len(), stage: 10, ..Frame::new(Kind::Header, frame.scope, frame.depth + 1) };
            frame.b += 1;
            self.push(frame)?;
            return self.push(child);
        }
        let value = self.header_value(frame, env)?;
        if frame.stage == 10 {
            match value {
                IfcValue::Reference(_) => return Err(refusal("express.header-reference")),
                IfcValue::Real(value) if !value.is_finite() => return Err(refusal("express.header-real")),
                IfcValue::Enum(name) if !identifier(name) => return Err(refusal("express.header-enumeration")),
                IfcValue::TypedValue(typed) if !identifier(&typed.name) => return Err(refusal("express.header-typed-name")),
                IfcValue::Binary(_) => {
                    frame.stage = 13;
                    frame.position = 0;
                    return self.push(frame);
                }
                IfcValue::Aggregate(_) | IfcValue::TypedValue(_) => {
                    frame.stage = 11;
                    frame.position = 0;
                    return self.push(frame);
                }
                _ => return Ok(()),
            }
        }
        if frame.stage == 13 {
            let IfcValue::Binary(text) = value else { return Err(refusal("express.header-binary")) };
            if text.is_empty() {
                return Err(refusal("express.header-binary"));
            }
            let end = (frame.position + 64).min(text.len());
            for (offset, byte) in text.as_bytes()[frame.position..end].iter().copied().enumerate() {
                let index = frame.position + offset;
                if index == 0 && !matches!(byte, b'0'..=b'3') || index > 0 && !matches!(byte, b'0'..=b'9' | b'A'..=b'F') {
                    return Err(refusal("express.header-binary"));
                }
            }
            if end < text.len() {
                frame.position = end;
                return self.push(frame);
            }
            if text.len() == 1 && text != "0" || text.len() > 1 && u8::from_str_radix(&text[1..2], 16).unwrap_or(255) >= 1u8 << (4 - (text.as_bytes()[0] - b'0')) {
                return Err(refusal("express.header-binary"));
            }
            return Ok(());
        }
        if frame.stage == 12 {
            self.store_mut().scratch.truncate(frame.d + frame.c);
            frame.stage = 11;
            return self.push(frame);
        }
        let count = match value {
            IfcValue::Aggregate(items) => items.len(),
            IfcValue::TypedValue(typed) => typed.items.len(),
            _ => return Err(refusal("express.header-container")),
        };
        if frame.position == count {
            return Ok(());
        }
        self.store_mut().scratch.push(frame.position);
        frame.position += 1;
        frame.stage = 12;
        self.push(frame)?;
        self.push(Frame { a: frame.a, b: frame.b, c: frame.c + 1, d: frame.d, stage: 10, ..Frame::new(Kind::Header, frame.scope, frame.depth + 1) })
    }
    fn normalize(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let arena = env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?;
        let overlay = env.overlay.ok_or_else(|| refusal("express.runtime-overlay"))?;
        let snapshot = env.source.ok_or_else(|| refusal("express.runtime-source"))?;
        if frame.stage == 0 {
            if self.store().domains.len() < arena.nodes.len() {
                self.store_mut().domains.push(DomainSlot { expression: None, name: None });
                return self.push(frame);
            }
            frame.stage = 1;
            frame.position = 0;
            frame.b = 0;
            frame.d = self.store().scratch.len();
            return self.push(frame);
        }
        if frame.position == overlay.len() {
            return Ok(());
        }
        let entity = overlay.entity(snapshot, env.mutations, frame.position)?;
        if entity.complex.len() >= 32 {
            return Err(refusal("express.complex-chain"));
        }
        if frame.stage == 1 {
            let name = if frame.b == 0 { entity.name.as_str() } else { entity.complex.get(frame.b - 1).ok_or_else(|| refusal("express.complex-component"))?.name.as_str() };
            let declaration = metadata::entity(name).ok_or_else(|| refusal("express.unknown-entity"))?;
            let index = metadata::ENTITIES.binary_search_by_key(&declaration.key, |entity| entity.key).map_err(|_| refusal("express.unknown-entity"))?;
            if self.store().scratch[frame.d..].contains(&index) {
                return Err(refusal("express.complex-duplicate"));
            }
            self.store_mut().scratch.push(index);
            frame.entity = Some(match frame.entity {
                None => declaration,
                Some(previous) if metadata::is_subtype(declaration.key, previous.key) => declaration,
                Some(previous) if metadata::is_subtype(previous.key, declaration.key) => previous,
                _ => return Err(refusal("express.complex-chain")),
            });
            if frame.b == entity.complex.len() {
                frame.b = 0;
                frame.c = 0;
                frame.owner = frame.entity.unwrap().key;
                frame.stage = 2;
            } else {
                frame.b += 1;
            }
            return self.push(frame);
        }
        let declaration = frame.entity.ok_or_else(|| refusal("express.complex-chain"))?;
        if frame.stage == 2 {
            if declaration.abstract_entity {
                return Err(refusal("express.abstract-entity"));
            }
            if !entity.complex.is_empty() && !frame.owner.is_empty() {
                let current = metadata::entity(frame.owner).ok_or_else(|| refusal("express.complex-chain"))?;
                let index = metadata::ENTITIES.binary_search_by_key(&current.key, |entity| entity.key).map_err(|_| refusal("express.complex-chain"))?;
                if !self.store().scratch[frame.d..].contains(&index) {
                    return Err(refusal("express.complex-chain"));
                }
                frame.c += 1;
                frame.owner = current.supertype.unwrap_or("");
                return self.push(frame);
            }
            if !entity.complex.is_empty() && frame.c != entity.complex.len() + 1 {
                return Err(refusal("express.complex-chain"));
            }
            if declaration.all_attributes.len() > 32 {
                return Err(refusal("express.released-attribute-capacity"));
            }
            self.store_mut().originals.push(Constructed { declaration, fields: [None; 32], length: declaration.all_attributes.len(), version: 0, view: None });
            self.store_mut().scratch.truncate(frame.d);
            frame.b = 0;
            frame.stage = 3;
            return self.push(frame);
        }
        let Some(attribute) = declaration.all_attributes.get(frame.b) else {
            frame.position += 1;
            frame.b = 0;
            frame.c = 0;
            frame.entity = None;
            frame.stage = 1;
            return self.push(frame);
        };
        let (component, argument) = if entity.complex.is_empty() {
            (0, frame.b)
        } else {
            let component = std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part| part.name.as_str())).position(|name| name.eq_ignore_ascii_case(attribute.declared_by)).ok_or_else(|| refusal("express.complex-owner"))?;
            let owner = metadata::entity(attribute.declared_by).ok_or_else(|| refusal("express.complex-owner"))?;
            let argument = owner.attributes.iter().position(|direct| direct.name.eq_ignore_ascii_case(attribute.name)).ok_or_else(|| refusal("express.complex-field"))?;
            (component, argument)
        };
        let offset = if component == 0 { 0 } else { entity.args.len() + entity.complex[..component - 1].iter().map(|part| part.args.len()).sum::<usize>() };
        let received = arena.entities.get(frame.position).ok_or_else(|| refusal("express.source-row"))?;
        let field = arena.fields.get(received.first_field + offset + argument).ok_or_else(|| refusal("express.source-field"))?;
        if field.root.row != frame.position || field.root.component != component || field.root.argument != argument {
            return Err(refusal("express.source-field-coordinate"));
        }
        if attribute.derived && !matches!(env.source_value(field.value)?, IfcValue::Derived) {
            return Err(refusal("express.derived-marker"));
        }
        let value = self.make(Value::Source(field.value));
        self.store_mut().originals[frame.position].fields[frame.b] = Some((attribute.name, value));
        frame.b += 1;
        self.push(frame)
    }
    fn source_node(&self, handle: Handle) -> Result<usize, ValueError> {
        if let Value::Source(node) = self.store().values[handle] { Ok(node) } else { Err(refusal("express.source-domain-value")) }
    }
    fn domain(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        let value = env.source_value(frame.a)?;
        if matches!(value, IfcValue::Unset) && frame.identity {
            return Ok(());
        }
        if matches!(value, IfcValue::Unset | IfcValue::Derived) {
            return Err(refusal("express.domain-required"));
        }
        let expression = frame.type_expression;
        let name = expression.and_then(|expression| expression.name).unwrap_or(frame.name);
        if frame.stage == 20 {
            let IfcValue::String(text) = value else { return Err(refusal("express.domain-string")) };
            if let Some(character) = text.get(frame.position..).and_then(|text| text.chars().next()) {
                frame.position += character.len_utf8();
                frame.i += 1;
                return self.push(frame);
            }
            let expression = expression.ok_or_else(|| refusal("express.domain-expression"))?;
            if expression.width.is_some_and(|width| if expression.fixed { frame.i != width as i128 } else { frame.i > width as i128 }) {
                return Err(refusal("express.domain-width"));
            }
            return Ok(());
        }
        if frame.stage == 30 {
            let result = self.pop()?;
            if self.truth(result, env)? != Logical::True {
                return Err(refusal("express.domain-select"));
            }
            if let IfcValue::TypedValue(_) = value {
                let arena = env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?;
                let child = arena.nodes[frame.a].first_edge.and_then(|edge| arena.edges.get(edge)).ok_or_else(|| refusal("express.domain-typed"))?.value;
                return self.push(Frame { a: child, name: frame.owner, ..Frame::new(Kind::Domain, frame.scope, frame.depth + 1) });
            }
            return Ok(());
        }
        if frame.stage == 40 {
            let IfcValue::Enum(actual) = value else { return Err(refusal("express.domain-enumeration")) };
            let declaration = metadata::defined_type(name).ok_or_else(|| refusal("express.domain-enumeration"))?;
            if let Some(member) = declaration.members.get(frame.position) {
                if *member == actual {
                    return Ok(());
                }
                frame.position += 1;
                return self.push(frame);
            }
            return Err(refusal("express.domain-enumeration"));
        }
        if frame.stage == 1 {
            let expression = expression.ok_or_else(|| refusal("express.domain-expression"))?;
            if let Some(edge) = frame.cursor.next {
                let arena = env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?;
                let edge = arena.edges[edge];
                frame.cursor.next = edge.next;
                self.push(frame)?;
                return self.push(Frame { a: edge.value, type_expression: Some(expression.item.ok_or_else(|| refusal("express.domain-expression"))?), identity: expression.optional, ..Frame::new(Kind::Domain, frame.scope, frame.depth + 1) });
            }
            return Ok(());
        }
        if self.store().domains[frame.a].expression.is_none() {
            self.store_mut().domains[frame.a].expression = expression;
        }
        if self.store().domains[frame.a].name.is_none() && !name.is_empty() {
            self.store_mut().domains[frame.a].name = Some(name);
        }
        if expression.is_some_and(|expression| expression.kind == "aggregate") {
            let expression = expression.unwrap();
            let IfcValue::Aggregate(items) = value else { return Err(refusal("express.domain-aggregate")) };
            let size = items.len() as i128;
            let valid = if expression.aggregate == Some("ARRAY") {
                match (expression.lower, expression.upper) {
                    (Some(lower), Some(upper)) => size == upper as i128 - lower as i128 + 1,
                    _ => true,
                }
            } else {
                expression.lower.is_none_or(|lower| size >= lower as i128) && expression.upper.is_none_or(|upper| size <= upper as i128)
            };
            if !valid {
                return Err(refusal("express.domain-aggregate-bounds"));
            }
            frame.cursor = Cursor { source: true, next: env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?.nodes[frame.a].first_edge };
            frame.stage = 1;
            return self.push(frame);
        }
        if metadata::entity(name).is_some() {
            let IfcValue::Reference(id) = value else { return Err(refusal("express.domain-reference")) };
            let row = env.overlay.and_then(|overlay| overlay.find(*id)).ok_or_else(|| refusal("express.domain-reference"))?;
            if !metadata::is_subtype(self.entity_declaration(Entity::Original(row))?.key, name) {
                return Err(refusal("express.domain-reference"));
            }
            return Ok(());
        }
        if let Some(declaration) = metadata::defined_type(name) {
            match declaration.kind {
                "enum" => {
                    frame.stage = 40;
                    frame.position = 0;
                    return self.push(frame);
                }
                "select" => {
                    let actual = match value {
                        IfcValue::Reference(id) => self.entity_declaration(Entity::Original(env.overlay.and_then(|overlay| overlay.find(*id)).ok_or_else(|| refusal("express.domain-reference"))?))?.key,
                        IfcValue::TypedValue(typed) if typed.items.len() == 1 => {
                            metadata::defined_type(&typed.name).map(|declaration| declaration.key).or_else(|| metadata::entity(&typed.name).map(|declaration| declaration.key)).ok_or_else(|| refusal("express.domain-select"))?
                        }
                        _ => return Err(refusal("express.domain-select")),
                    };
                    if matches!(value, IfcValue::TypedValue(_)) {
                        self.store_mut().domains[frame.a].name = Some(actual);
                    }
                    frame.owner = actual;
                    frame.stage = 30;
                    self.push(frame)?;
                    return self.push(Frame { name, owner: actual, ..Frame::new(Kind::Select, frame.scope, frame.depth + 1) });
                }
                _ => {
                    let expression = declaration.type_expr.ok_or_else(|| refusal("express.domain-expression"))?;
                    if let IfcValue::TypedValue(typed) = value {
                        if typed.name.eq_ignore_ascii_case(name) {
                            if typed.items.len() != 1 {
                                return Err(refusal("express.domain-typed"));
                            }
                            let arena = env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?;
                            let child = arena.nodes[frame.a].first_edge.and_then(|edge| arena.edges.get(edge)).ok_or_else(|| refusal("express.domain-typed"))?.value;
                            return self.push(Frame { a: child, type_expression: Some(expression), ..Frame::new(Kind::Domain, frame.scope, frame.depth + 1) });
                        }
                    }
                    frame.type_expression = Some(expression);
                    frame.name = "";
                    return self.push(frame);
                }
            }
        }
        match (name, value) {
            ("STRING", IfcValue::String(_)) => {
                if expression.is_some_and(|expression| expression.width.is_some()) {
                    frame.stage = 20;
                    frame.position = 0;
                    frame.i = 0;
                    self.push(frame)?;
                }
            }
            ("INTEGER" | "REAL" | "NUMBER", IfcValue::Integer(_)) => {}
            ("REAL" | "NUMBER", IfcValue::Real(value)) if value.is_finite() => {}
            ("BOOLEAN", IfcValue::Enum(value)) if ["T", "F"].contains(&value.as_str()) => {}
            ("LOGICAL", IfcValue::Enum(value)) if ["T", "F", "U"].contains(&value.as_str()) => {}
            ("BINARY", IfcValue::Binary(value)) => {
                let length = value.len().saturating_sub(1) as i128 * 4 - value.as_bytes().first().map_or(0, |byte| byte.saturating_sub(b'0')) as i128;
                if expression.and_then(|expression| expression.width).is_some_and(|width| if expression.unwrap().fixed { length != width as i128 } else { length > width as i128 }) {
                    return Err(refusal("express.domain-width"));
                }
            }
            _ => return Err(refusal("express.domain-type")),
        }
        Ok(())
    }
    fn select_domain(&mut self, mut frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
        if frame.stage == 1 {
            let result = self.pop()?;
            if self.truth(result, env)? == Logical::True {
                self.result(boolean(true));
                return Ok(());
            }
            frame.position += 1;
            frame.stage = 0;
        }
        if frame.name == frame.owner || metadata::is_subtype(frame.owner, frame.name) {
            self.result(boolean(true));
            return Ok(());
        }
        let Some(declaration) = metadata::defined_type(frame.name).filter(|declaration| declaration.kind == "select") else {
            self.result(boolean(false));
            return Ok(());
        };
        let Some(member) = declaration.members.get(frame.position).copied() else {
            self.result(boolean(false));
            return Ok(());
        };
        frame.stage = 1;
        self.push(frame)?;
        self.push(Frame { name: member, owner: frame.owner, ..Frame::new(Kind::Select, frame.scope, frame.depth + 1) })
    }
    fn source_aggregate_expression(&self, node: usize) -> Result<Option<&'static metadata::StaticTypeExpression>, ValueError> {
        let Some(domain) = self.store().domains.get(node) else { return Ok(None) };
        let mut expression = domain.expression;
        for _ in 0..MAXIMUM_DEPTH {
            let Some(current) = expression else { return Ok(None) };
            if current.kind == "aggregate" {
                return Ok(Some(current));
            }
            expression = current.name.and_then(metadata::defined_type).and_then(|declaration| declaration.type_expr);
        }
        Err(refusal("express.type-cycle"))
    }
}
