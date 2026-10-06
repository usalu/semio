use crate::standards::v_utf_8::subsets::any::io::text::mutations::*;
use protocol::OpText;

#[test]
fn generic_framing_refuses_malformed_text_without_panicking() {
    for frame in ["txt-mutation unknown payload=0", "txt-mutation unknown payload=!!", "txt-mutation unknown payload=ff", "txt-mutation unknown payload=7b7d"] {
        assert!(TxtMutation::parse_op(frame).is_err(), "{frame}");
    }
}
