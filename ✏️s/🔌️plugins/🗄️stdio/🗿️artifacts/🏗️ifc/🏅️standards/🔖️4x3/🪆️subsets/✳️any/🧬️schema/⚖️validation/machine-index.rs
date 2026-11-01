/// 🧭 Entity identity and rule-owner indexing every evaluation lane shares.
impl RetainedMachine {
    pub fn source_aggregate_expression(&self, node: usize) -> Result<Option<&'static metadata::StaticTypeExpression>, ValueError> {
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
    pub fn indexed_rule(&self, owner: &'static str, lane: usize, position: usize, inherited: bool) -> Option<usize> {
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
    pub fn canonical_entity(&self, entity: Entity) -> Entity {
        match entity {
            Entity::View(index) => Entity::Original(self.store().views[index].original),
            _ => entity,
        }
    }
    pub fn dependent_result(&mut self, value: Handle, entity: Entity, scope: usize, depth: usize) -> Result<(), ValueError> {
        let memo = self.store().scopes[scope].memo.unwrap_or(NONE);
        self.push(Frame { a: value, b: memo, record_entity: Some(entity), ..Frame::new(Kind::Dependency, scope, depth) })
    }
}
