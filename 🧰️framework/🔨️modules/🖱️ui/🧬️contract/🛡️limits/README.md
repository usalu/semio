# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🛡️ Quotas, `validate_snapshot`, and the one shared transactional `apply_patch`.

🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
below is plain sync by owner ruling U1.

`apply_patch` is the single most important function in the crate: the React DOM store, the GPU
frontend, and every future renderer all apply patches through this exact code, which is what makes
them agree. It is totally transactional — ops apply to a shadow draft, never to the caller's state,
and on ANY rejection the caller's state is byte-for-byte unchanged. Untrusted plugin documents flow
through here, so [`UiDocumentLimits`] are a security boundary, not a nicety: an oversized or
malformed document is rejected before it costs more than an O(1) or O(node count) check.
