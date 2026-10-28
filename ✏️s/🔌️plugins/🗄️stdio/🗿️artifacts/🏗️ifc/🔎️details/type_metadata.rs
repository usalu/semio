//! 🧬️ Immutable type nodes expose exact declaration identity without owned catalog copies.
#[derive(Clone, Copy)]
pub struct IfcTypeProjection {
    pub kind: &'static str,
    pub name: Option<&'static str>,
    pub width: Option<i64>,
    pub fixed: bool,
    pub aggregate: Option<&'static str>,
    pub lower: Option<i64>,
    pub upper: Option<i64>,
    pub lower_expression: Option<&'static str>,
    pub upper_expression: Option<&'static str>,
    pub unique: bool,
    pub optional: bool,
    pub item: Option<&'static dyn IfcDetailsTypeMetadata>,
}
pub trait IfcDetailsTypeMetadata: Sync {
    fn projection(&'static self) -> IfcTypeProjection;
}
impl IfcDetailsTypeMetadata for super::IfcTypeExpression {
    fn projection(&'static self) -> IfcTypeProjection {
        IfcTypeProjection {
            kind: &self.kind,
            name: self.name.as_deref(),
            width: self.width,
            fixed: self.fixed,
            aggregate: self.aggregate.as_deref(),
            lower: self.lower,
            upper: self.upper,
            lower_expression: self.lower_expression.as_deref(),
            upper_expression: self.upper_expression.as_deref(),
            unique: self.unique,
            optional: self.optional,
            item: self.item.as_deref().map(|item| item as &dyn IfcDetailsTypeMetadata),
        }
    }
}
#[derive(Default)]
pub(super) struct Comparison {
    nodes: Option<(&'static dyn IfcDetailsTypeMetadata, &'static dyn IfcDetailsTypeMetadata)>,
    phase: u8,
    field: usize,
    offset: usize,
    depth: usize,
}
impl Comparison {
    pub(super) fn start(&mut self, left: &'static dyn IfcDetailsTypeMetadata, right: &'static dyn IfcDetailsTypeMetadata) {
        *self = Self { nodes: Some((left, right)), ..Default::default() };
    }
    pub(super) fn advance(&mut self) -> Result<Option<bool>, semio_framework_value::ValueError> {
        let (left, right) = self.nodes.ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "IFC type metadata comparison missing"))?;
        let left = left.projection();
        let right = right.projection();
        match self.phase {
            0 => {
                if (left.width, left.fixed, left.lower, left.upper, left.unique, left.optional, left.item.is_some()) != (right.width, right.fixed, right.lower, right.upper, right.unique, right.optional, right.item.is_some()) {
                    return Ok(Some(false));
                }
                self.phase = 1;
            }
            1 if self.field < 5 => {
                let field = |value: IfcTypeProjection| match self.field {
                    0 => Some(value.kind),
                    1 => value.name,
                    2 => value.aggregate,
                    3 => value.lower_expression,
                    _ => value.upper_expression,
                };
                match (field(left), field(right)) {
                    (None, None) => {
                        self.field += 1;
                        self.offset = 0;
                    }
                    (Some(left), Some(right)) if left.len() == right.len() => {
                        let end = self.offset.saturating_add(64).min(left.len());
                        if left.as_bytes()[self.offset..end] != right.as_bytes()[self.offset..end] {
                            return Ok(Some(false));
                        }
                        self.offset = end;
                        if end == left.len() {
                            self.field += 1;
                            self.offset = 0;
                        }
                    }
                    _ => return Ok(Some(false)),
                }
            }
            _ => match (left.item, right.item) {
                (None, None) => return Ok(Some(true)),
                (Some(left), Some(right)) => {
                    if self.depth == 128 {
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "IFC type metadata depth exceeded"));
                    }
                    self.nodes = Some((left, right));
                    self.depth += 1;
                    self.phase = 0;
                    self.field = 0;
                    self.offset = 0;
                }
                _ => return Ok(Some(false)),
            },
        }
        Ok(None)
    }
    pub(super) fn metadata(&self) -> [usize; 6] {
        [self.nodes.map_or(0, |(left, _)| left as *const _ as *const () as usize), self.nodes.map_or(0, |(_, right)| right as *const _ as *const () as usize), self.phase as usize, self.field, self.offset, self.depth]
    }
}
