//! 🔢️ Arbitrary-precision integers and rationals, modular arithmetic, primality/factorization,
//! certified interval arithmetic, the abstract-algebra trait hierarchy (`Ring` through `Field`)
//! every exact numeric/symbolic consumer is generic over, and (in [`algebra`]) the generic
//! `MatG`/`VecG` matrix/vector types built on that hierarchy. Framework-tier vocabulary — the
//! numeric analogue of `semio_framework_geometry`'s "framework-internal vocabulary every framework
//! crate may name without reaching a plugin": `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚖️predicates`
//! escalates to [`rational::Rational`] for exact-arithmetic tie-breaking, a framework-tier call
//! site that cannot depend on any plugin crate. Also consumed, on the plugin side, by
//! `➗️mathematical`'s polynomial/CAS inferences (`number::MatG`/`number::VecG` for exact linear
//! solving in partial-fraction decomposition, `math::polynomial`'s generic polynomials, and
//! `math::cas`'s expression coefficients) and `📸️remodel`'s algebra internals (`number::MatG`/
//! `number::VecG`, plus its own `lll_reduce` free function built on them). Relocated verbatim from
//! `🧰️framework/🔨️modules/🧮️math/🔢️number` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MATHEND — see that wave's report
//! for the domain-neutral-exemption justification. `algebra` (`MatG`/`VecG`) joined it verbatim
//! from `📸️remodel`'s `➕️algebra-internals` in the same ticket's wave FIXALG, after wave M3d's
//! sole-consumer census of that plugin was outrun mid-ticket by wave M3a giving `➗️mathematical`'s
//! `cas-internals` a second consumer — see wave FIXALG's report for the cross-wave hazard.
// #region 🔖️Traits
#[path = "💍️traits/🦀️.rs"]
pub mod traits;
// #endregion 🔖️Traits

// #region 🔖️Natural
#[path = "🧱️natural/🦀️.rs"]
pub mod natural;
// #endregion 🔖️Natural

// #region 🔖️Integer
#[path = "➕️integer/🦀️.rs"]
pub mod integer;
// #endregion 🔖️Integer

// #region 🔖️Rational
#[path = "➗️rational/🦀️.rs"]
pub mod rational;
// #endregion 🔖️Rational

// #region 🔖️Modular
#[path = "🕰️modular/🦀️.rs"]
pub mod modular;
// #endregion 🔖️Modular

// #region 🔖️Primes
#[path = "🔑️primes/🦀️.rs"]
pub mod primes;
// #endregion 🔖️Primes

// #region 🔖️Interval
#[path = "📏️interval/🦀️.rs"]
pub mod interval;
// #endregion 🔖️Interval
// #region 🔖️Algebra
#[path = "🧮️algebra/🦀️.rs"]
pub mod algebra;
// #endregion 🔖️Algebra

pub use algebra::{MatG, VecG};
pub use integer::{Integer, Sign};
pub use interval::Interval;
pub use modular::ModInt;
pub use natural::Natural;
pub use rational::Rational;
pub use traits::{field_div_rem, field_gcd, CommutativeRing, EuclideanDomain, Field, GcdDomain, IntegralDomain, Ring};
