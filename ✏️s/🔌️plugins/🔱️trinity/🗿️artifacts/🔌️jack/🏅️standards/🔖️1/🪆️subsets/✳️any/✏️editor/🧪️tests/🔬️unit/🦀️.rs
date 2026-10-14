//! 🧪️ The intrinsic editor command bridge stays independent of concrete member kinds.
use super::*;
#[test]fn intrinsic_command_codec_preserves_literal_query(){use protocol::OpBinary;let value=TrinityJackCommand::RunQuery{query:Some("MATCH (a:Piece) RETURN a".into()),results_window_id:"literal@results".into()};let wire=value.encode_op().unwrap();assert_eq!(TrinityJackCommand::decode_op(&wire).unwrap(),value);}
