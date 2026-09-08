
//! 🧬️ THE carrier law this whole pilot exists to prove (design.md §3, mission step 5):
//! `s.stdio.txt@utf-8/*`'s native `Text` `IoPayload` is the raw external file text, verbatim
//! — decode→encode must reproduce arbitrary text exactly, and the encoded payload must NOT
//! carry the old `semio stdio.txt.dsl v1` preamble line (`ArtifactDsl::print_dsl` emitted
//! before this fix — see `📸️snapshot/🦀️.rs`).
use crate::TxtSnapshot;
use store::ArtifactDsl;

#[semio_framework_async_macros::async_test]
async fn carrier_native_is_raw() {
    for text in ["", "hello\n", "a\r\nb\r\nc", "just one line, no newline", "Hello, \u{4e16}\u{754c}!\n\u{1f389}"] {
        let decoded = TxtSnapshot::parse_dsl(text).expect("decode");
        let encoded = decoded.print_dsl();
        assert_eq!(encoded, text, "carrier round trip must be verbatim for {text:?}");
        assert!(!encoded.starts_with("semio "), "carrier payload must not carry a .semio preamble: {encoded:?}");
    }
}
