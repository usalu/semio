//! 💾️ Direct binary-codec identity for edit-before-fixture / EditBeforeFixture.

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "edit-before-fixture");
