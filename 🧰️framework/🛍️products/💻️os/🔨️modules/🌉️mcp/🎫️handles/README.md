# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🎫️ Handle table + idempotency store (`📌️sol-P1b-packet.md` §2.2, `📋️master.md` §2.4) — every
`prep_`/`txn_`/`undo_`/`job_`/`appr_`/`cont_`/`sess_` handle a later packet (P6 mutation protocol,
P10 shell) mints flows through here. **Authorization is never derivable from possession of a
handle alone**: [`HandleTable::resolve`] takes the requesting session and returns
`PERMISSION_DENIED` on owner mismatch even though the handle itself exists and has not expired —
this is the security property `mod quick`'s `cross_session_resolve_is_permission_denied_not_a_leak`
test exists to pin down, not a nicety. Zero dependency on the kernel/plugin/channel/actor crates,
matching this crate's own root doc.
