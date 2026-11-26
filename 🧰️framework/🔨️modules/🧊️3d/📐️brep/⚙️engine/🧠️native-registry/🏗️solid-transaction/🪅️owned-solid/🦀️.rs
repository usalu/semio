//! 🪅️ Analytic topology publication directly into a private, independently retireable body.

use super::*;
use semio_framework_value::retirement::RetireOwned;

/// 🧱️ Holds primitive topology and the journal until the solid can be donated with its entire body.
pub struct OwnedSolidPublication {
    body: Body,
    plan: Option<SolidPublication>,
    root: Option<SolidId>,
}

impl OwnedSolidPublication {
    /// 🌱️ Validates and admits an analytic solid without allocating any topology backing.
    pub fn new(owner: usize, primitive: SolidPrimitive, handle: impl FnOnce(u64) -> [u8; 64]) -> Result<Self, ValueError> {
        primitive.validate().map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        let mut body = Body::new();
        let plan = SolidPublication::begin(owner, SolidBoundary::Analytic(primitive), &mut body.labels, handle)?;
        Ok(Self { body, plan: Some(plan), root: None })
    }

    /// 📊️ Reads the live topology journal extent without scanning any arena.
    pub fn progress(&self) -> f32 {
        self.plan.as_ref().map_or(1.0, |plan| 0.9 * plan.inserted.len() as f32 / (plan.offsets.solid + 1) as f32)
    }

    /// 🪜️ Writes one portion of topology or closes its journal under the original allocation effect.
    pub fn step(&mut self, owner: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<(Body, SolidId)>, Progress), ValueError> {
        let Some(plan) = self.plan.as_mut() else { return Ok((None, Progress::default())); };
        if !plan.owns(owner) {
            return Err(ValueError::invariant("owned solid publication requires its original source"));
        }
        if !plan.topology_ready() {
            return plan.advance(&mut self.body, effect).map(|receipt| (None, receipt));
        }
        if plan.is_open() {
            if effect.grant().maximum_items == 0 || effect.grant().maximum_depth == 0 || !effect.performed(1)? {
                return Ok((None, Progress::default()));
            }
            self.root = Some(plan.product(owner)?.0);
            plan.mark_published();
            return Ok((None, item()));
        }
        let receipt = plan.close(&mut self.body, effect)?.progress();
        if !plan.terminal_is_empty() {
            return Ok((None, receipt));
        }
        let root = self.root.take().ok_or_else(|| ValueError::invariant("owned solid root disappeared before donation"))?;
        self.plan.take();
        Ok((Some((take(&mut self.body), root)), receipt))
    }
}

type RetirementParts = (Body, Vec<EntityRef>, Vec<FaceId>, Vec<ShellId>);

impl RetireOwned for OwnedSolidPublication {
    type RetirementCursor = <RetirementParts as RetireOwned>::RetirementCursor;

    fn retirement_seed(mut self) -> Self::RetirementCursor {
        let mut scratch = self.plan.take();
        let (journal, faces, shells) = scratch.as_mut().map_or_else(Default::default, |plan| {
            (take(&mut plan.inserted), take(&mut plan.shell_faces), take(&mut plan.inner_shells))
        });
        (self.body, journal, faces, shells).retirement_seed()
    }

    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Self::RetirementCursor>()) }
    fn controlled_retirement_supported() -> bool { RetirementParts::controlled_retirement_supported() }
}
