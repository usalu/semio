//! 🧹️ The exact original native family handoff, retained inline until physical grants.
use super::Brep;
impl Brep {
    /// 🚪️ Moves the same original family inline, retaining its arenas and live index until grants.
    pub fn detach_retirement(&mut self)->semio_framework_value::retirement::controlled::ControlledRetirement<Self> {
        semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(self)).unwrap_or_else(|_|panic!("original BRep owner requires typed retirement"))
    }
}
