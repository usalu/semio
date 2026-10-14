//! 🎚️ Explicit config field-set representation expansion.
#[macro_export]
macro_rules! field_set_mutations_text {
    ($set:ident { $($variant:ident $kind:literal),+ $(,)? }) => {
        impl $crate::__pack_json::ArtifactCanonicalJsonTree for $set {
            fn canonical_tree_node(&self) -> Result<$crate::__pack_json::ArtifactCanonicalJsonNode<'_>, $crate::__value::ValueError> { Ok($crate::__pack_json::ArtifactCanonicalJsonNode::Object(2)) }
            fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn $crate::__pack_json::ArtifactCanonicalJsonTree, $crate::__value::ValueError> {
                match (self, ordinal) {
                    $((Self::$variant(_), 0) => { static KIND: &str = $kind; Ok(&KIND) }, (Self::$variant(value), 1) => Ok(value),)+
                    _ => Err($crate::__value::ValueError::literal($crate::__value::ValueRefusalKind::InvariantViolated, "field set canonical child is absent")),
                }
            }
            fn canonical_tree_key(&self, ordinal: usize) -> Result<$crate::__pack_json::ArtifactCanonicalJsonText<'_>, $crate::__value::ValueError> {
                match ordinal { 0 => Ok("kind".into()), 1 => Ok("value".into()), _ => Err($crate::__value::ValueError::literal($crate::__value::ValueRefusalKind::InvariantViolated, "field set canonical key is absent")) }
            }
        }
        impl $crate::OpText for $set {
            fn parse_op(line: &str) -> Result<Self, $crate::__diagnostic::TextError> {
                $crate::__pack_json::from_json_str(line, $crate::__pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::__diagnostic::TextError::from_value_error(error, $crate::__diagnostic::TextSpan::at(1, 1)))
            }

            fn print_op(&self) -> String {
                $crate::__pack_json::to_json_string(self)
            }
        }

    };
}
