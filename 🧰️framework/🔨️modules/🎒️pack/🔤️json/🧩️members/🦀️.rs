/// 🧩️ Explicit authority for repeated decoded JSON object member names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonMemberPolicy {
    Reject,
    Replace,
}
