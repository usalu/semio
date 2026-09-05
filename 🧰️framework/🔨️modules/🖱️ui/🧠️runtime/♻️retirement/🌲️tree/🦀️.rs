//! 🌲️ Single retained runtime tree owner with concrete, non-overwriting source transfers.

#[derive(Default)]
pub(super) struct SurfaceTreeRetireCursor {
    owner: Option<ui_contract::BuiltTreeRetirement>,
}

impl SurfaceTreeRetireCursor {
    pub(super) fn try_begin_node(&mut self, source: &mut Option<crate::TreeNode>) -> bool {
        if self.owner.is_some() { return false; }
        let Some(node) = source.take() else { return false; };
        self.owner = Some(ui_contract::BuiltTreeRetirement::new(node));
        true
    }

    pub(super) fn try_begin_tree(&mut self, source: &mut Option<crate::ComponentTree>) -> bool {
        if self.owner.is_some() { return false; }
        let Some(tree) = source.take() else { return false; };
        self.owner = Some(ui_contract::BuiltTreeRetirement::new(tree.root));
        true
    }

    pub(super) fn try_begin_held(&mut self, source: &mut Option<(Option<usize>, crate::TreeNode)>) -> bool {
        if self.owner.is_some() { return false; }
        let Some((_, node)) = source.take() else { return false; };
        self.owner = Some(ui_contract::BuiltTreeRetirement::new(node));
        true
    }

    pub(super) fn close_step(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        let Some(owner) = self.owner.as_mut() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
        let step = owner.close_step(items, bytes)?;
        if step.complete { self.owner.take(); }
        Ok(step)
    }

    pub(super) fn step(&mut self) -> bool {
        self.close_step(1, super::SURFACE_COMPONENT_COPY_WORK_BYTES).expect("runtime tree retains exact retirement authority").complete
    }

    pub(super) fn is_empty(&self) -> bool { self.owner.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
