//! 🪞️ A bounded canonical facade borrows native payloads through retained preimage metadata.
use super::*;

pub(super) struct BorrowedInverse<'a> {
    pub view: &'a IfcInverseWire,
    pub mutations: &'a [IfcMutation],
}
impl BorrowedInverse<'_> {
    fn entity(&self) -> Result<&IfcEntity, ValueError> {
        self.view.overlay.entity(self.view.overlay.before.origin.ok_or_else(|| refusal("IFC inverse entity origin missing"))?, self.view.snapshot()?, self.mutations)
    }
    fn name(&self) -> Result<&String, ValueError> {
        match self.view.overlay.before.name {
            Some(row) => match self.mutations.get(row) {
                Some(IfcMutation::SetEntityName(value)) => Ok(&value.name),
                _ => Err(refusal("IFC inverse name owner missing")),
            },
            None => Ok(&self.entity()?.name),
        }
    }
    fn complex(&self) -> Result<&Vec<IfcComplexType>, ValueError> {
        match self.view.overlay.before.complex {
            Some(row) => match self.mutations.get(row) {
                Some(IfcMutation::SetEntityComplex(value)) => Ok(&value.complex),
                _ => Err(refusal("IFC inverse complex owner missing")),
            },
            None => Ok(&self.entity()?.complex),
        }
    }
    fn child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        let snapshot = self.view.snapshot()?;
        match (self.view.inverse, ordinal) {
            (Inverse::Description, 0) => match self.view.overlay.header {
                Some(row) => match &self.mutations[row] {
                    IfcMutation::SetFileDescription(value) => Ok(&value.values),
                    _ => Err(refusal("IFC inverse header owner missing")),
                },
                None => Ok(&snapshot.header.file_description),
            },
            (Inverse::FileName, 0) => match self.view.overlay.header {
                Some(row) => match &self.mutations[row] {
                    IfcMutation::SetFileName(value) => Ok(&value.values),
                    _ => Err(refusal("IFC inverse header owner missing")),
                },
                None => Ok(&snapshot.header.file_name),
            },
            (Inverse::Schema, 0) => match self.view.overlay.header {
                Some(row) => match &self.mutations[row] {
                    IfcMutation::SetFileSchema(value) => Ok(&value.values),
                    _ => Err(refusal("IFC inverse header owner missing")),
                },
                None => Ok(&snapshot.header.file_schema),
            },
            (Inverse::Insert, 0) => Ok(&self.view.position),
            (Inverse::Remove | Inverse::Name | Inverse::Argument | Inverse::InsertArgument | Inverse::RemoveArgument | Inverse::Complex, 0) => Ok(&self.view.id),
            (Inverse::Name, 1) => Ok(self.name()?),
            (Inverse::Argument | Inverse::InsertArgument | Inverse::RemoveArgument, 1) => Ok(&self.view.index),
            (Inverse::Argument | Inverse::InsertArgument, 2) => Ok(self.view.overlay.value(self.view.overlay.argument.ok_or_else(|| refusal("IFC inverse argument owner missing"))?, snapshot, self.mutations)?),
            (Inverse::Complex, 1) => Ok(self.complex()?),
            _ => Err(refusal("IFC inverse child missing")),
        }
    }
    fn descendant(&self, path: &[usize]) -> Result<&dyn Tree, ValueError> {
        let (first, rest) = path.split_first().ok_or_else(|| refusal("IFC inverse descendant missing"))?;
        let mut tree = self.child(first.checked_sub(1).ok_or_else(|| refusal("IFC inverse tag has no descendant"))?)?;
        for ordinal in rest {
            tree = tree.canonical_tree_child(*ordinal)?;
        }
        Ok(tree)
    }
    fn entity_descendant(&self, path: &[usize]) -> Result<&dyn Tree, ValueError> {
        let (first, rest) = path.split_first().ok_or_else(|| refusal("IFC inverse entity descendant missing"))?;
        let (mut tree, rest): (&dyn Tree, &[usize]) = match *first {
            0 => (&self.entity()?.id, rest),
            1 => (self.name()?, rest),
            2 => {
                let (index, rest) = rest.split_first().ok_or_else(|| refusal("IFC inverse argument descendant missing"))?;
                let owner = *self.view.overlay.arguments.get(*index).ok_or_else(|| refusal("IFC inverse argument index missing"))?;
                (self.view.overlay.value(owner, self.view.snapshot()?, self.mutations)?, rest)
            }
            3 => (self.complex()?, rest),
            _ => return Err(refusal("IFC inverse entity field missing")),
        };
        for ordinal in rest {
            tree = tree.canonical_tree_child(*ordinal)?;
        }
        Ok(tree)
    }
}
impl store::ArtifactCanonicalJson for BorrowedInverse<'_> {
    fn canonical_json_original_owner(&self) -> Option<&dyn store::ArtifactCanonicalJson> {
        Some(self.view)
    }
    fn canonical_json_node(&self, path: &[usize]) -> Result<Node<'_>, ValueError> {
        match path {
            [] => Ok(Node::Object(self.view.fields().len() + 1)),
            [0] => Ok(Node::String(self.view.tag())),
            [2, rest @ ..] if matches!(self.view.inverse, Inverse::Insert) => match rest {
                [] => Ok(Node::Object(3 + usize::from(!self.complex()?.is_empty()))),
                [2] => Ok(Node::Array(self.view.overlay.arguments.len())),
                _ => self.entity_descendant(rest)?.canonical_tree_node(),
            },
            _ => self.descendant(path)?.canonical_tree_node(),
        }
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<Text<'_>, ValueError> {
        match path {
            [] => {
                if index == 0 {
                    Ok("mutation".into())
                } else {
                    self.view.fields().get(index - 1).map(|key| Text::from(*key)).ok_or_else(|| refusal("IFC inverse field missing"))
                }
            }
            [2, rest @ ..] if matches!(self.view.inverse, Inverse::Insert) => {
                if rest.is_empty() {
                    ["id", "name", "args", "complex"].get(index).filter(|_| index < 3 || !self.complex().map_or(true, Vec::is_empty)).map(|key| Text::from(*key)).ok_or_else(|| refusal("IFC inverse entity key missing"))
                } else {
                    self.entity_descendant(rest)?.canonical_tree_key(index)
                }
            }
            _ => self.descendant(path)?.canonical_tree_key(index),
        }
    }
}
