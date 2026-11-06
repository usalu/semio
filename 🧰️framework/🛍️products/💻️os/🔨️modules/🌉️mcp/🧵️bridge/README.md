# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🧵️ `ShellBridge` frame codec (`📋️master.md` §2.2 verbatim variant list, `📌️sol-P1b-packet.md`
§2.4) — hand-rolled binary framing (`tag: u8` + fields in declaration order), `BRIDGE_VERSION = 1`.
This file is the Rust SSOT; `🟦️.ts` is the TS twin, and `🧫️fixtures/📨️frames.json` is the
anti-drift mechanism proving both codecs agree byte-for-byte (`mod quick`'s
`every_fixture_round_trips_through_the_rust_codec` test; the TS side of the same fixtures is
exercised by a foreground `bun run` script, not wired into this crate's own test run).

Field types are hand-picked for a self-contained wire format: this facet deliberately does NOT
depend on the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket's `os_spr::wire` primitives
(`📡️spr/🧵️channel/🦀️.rs` is their exclusive, mid-rewrite territory per
`📌️important.md`'s collision matrix) — see this packet's report §7 for the full rationale. Opaque
payloads (`ShellState.state`, `ShellStatePatch.patch`, `ShellCommand.command`,
`AppCommand.command`, each `AppFrames.frames[i]`) are carried as length-prefixed byte blobs; what
goes inside them (the real packed `ShellState`) is `💻️os/🔨️modules/🖥️shell`'s concern (P9), not
this codec's.

The live bridge is owned by the retained HTTP transport through `🔖️BridgeHandle`; the Axum
`🔖️BridgeServer` remains a `cfg(test)` differential oracle only.
