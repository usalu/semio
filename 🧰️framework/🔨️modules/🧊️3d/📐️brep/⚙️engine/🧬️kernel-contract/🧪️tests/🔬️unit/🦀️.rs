//! 🔬️ Kernel contract: 1 test covering brep error contract is owned and stable.
use super::*;


#[test]
fn brep_error_contract_is_owned_and_stable() {
    let errors = [(BrepError::InvalidInput("mesh".into()), "invalid input: mesh"), (BrepError::MissingHandle("a1".into()), "missing handle: a1"), (BrepError::Operation("split".into()), "operation failed: split")];
    for (error, message) in errors {
        assert_eq!(error.to_string(), message);
        assert!(std::error::Error::source(&error).is_none());
    }
}
