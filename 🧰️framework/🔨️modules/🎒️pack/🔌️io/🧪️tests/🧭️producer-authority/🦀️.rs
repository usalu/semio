//! 🧪️ Actual atomic writer rejects invalid caller paths before native IO.
use super::*;
use semio_framework_pack_error::PackError;
use semio_framework_value::ValueRefusalKind;

#[test]
fn actual_atomic_writer_missing_filename_retains_invalid_value_authority(){
        let (context, _cancellation, _progress) = crate::io::caller_test_policy::context();
 let refusal=write_atomic(std::path::Path::new(""),b"untouched", &context).unwrap_err();
 assert_eq!(refusal.refusal_kind(),Some(ValueRefusalKind::InvalidValue));
 assert!(matches!(refusal,PackError::Refusal(semio_framework_pack_error::PackRefusal::ValueRefusal(_))));
 eprintln!("[DEBUG] Pack atomic writer invalid path refused before file IO");
}
