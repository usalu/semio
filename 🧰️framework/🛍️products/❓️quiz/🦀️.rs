//! ❓️ Quiz product façade — render-independent declarative quizzes and their pure core: the contract
//! types, bit-exact randomness, the solution-free sheet of a run, validation, partial-credit scoring,
//! badges, the learner lifecycle as `decide`/`evolve` pairs and the read models a proctor serves.
//! Every function is pure and shared vocabulary with the TypeScript core (`@semio-tech/quiz`).
//!
//! @see README.md — the domain model
//! @see 🧬️schema/🔣️.json — the normative contract

pub use crate::badges::*;
pub use crate::challenge::*;
pub use crate::lifecycle::*;
pub use crate::presence::*;
pub use crate::randomness::*;
pub use crate::schema::*;
pub use crate::scoring::*;
pub use crate::sheet::*;
pub use crate::validation::*;
pub use crate::views::*;
