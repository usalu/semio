//! 🥣️ Companion tests of number: the unit laws of the algebra, primes, rational, interval, modular and traits modules, run against the public API of `semio-framework-number`.
//!
//! 🧭️ Each module's laws sit below a wrapper of that module that restates its imports; the natural and integer laws stay in the lib because they read private items.
#![cfg(test)]

#[path = "🧮️algebra/🦀️.rs"]
mod algebra;

#[path = "🔑️primes/🦀️.rs"]
mod primes;

#[path = "➗️rational/🦀️.rs"]
mod rational;

#[path = "📏️interval/🦀️.rs"]
mod interval;

#[path = "🕰️modular/🦀️.rs"]
mod modular;

#[path = "💍️traits/🦀️.rs"]
mod traits;
