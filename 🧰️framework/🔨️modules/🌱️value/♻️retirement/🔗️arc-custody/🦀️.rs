//! 🔗️ Shared immutable owners retain Arc backing custody until aliases and weak leases close.

semio_framework_value_retirement_contract::shared_controlled_owner!();

#[cfg(test)]
#[path="🔐️unique/🧪️tests/🦀️.rs"]
mod shared_unique_tests;
