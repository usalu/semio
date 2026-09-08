//! 🧭️ Kernel-only M5 grammar, protocol and fixture-discovery conformance.
//! Fleet-owned real-example laws live in the dedicated fixture-sweep test package.

//#region 🔖️ExampleAssetDiscovery
/// @emoji 🖼️ Path-agnostic example-asset discovery for M5 pilots: prefers
/// `📚️examples/<slug>/🖼️assets/*.<kind>.semio`, soft-falls back to legacy plural kind dirs.
#[cfg(test)]
#[path = "../🔬️example-asset-discovery/🦀️.rs"]
mod example_asset_discovery;
//#endregion 🔖️ExampleAssetDiscovery

//#region 🧭️PilotResolve
/// 🧭️ Path-agnostic example-asset resolution for M5 pilots.
/// Prefers `📚️examples/<slug>/🖼️assets/*.<kind>.semio`; falls back to any `.semio` under the
/// slug tree (legacy `🗣️dsls`/`🎒️packs`/…) so mid-migration does not break compile-time includes.
#[cfg(test)]
#[path = "../🔬️pilot-resolve/🦀️.rs"]
mod pilot_resolve;
//#endregion 🧭️PilotResolve

//#region 🔖️M5AutoDiscovery
/// @emoji 🧭️ P2-M3: auto-discovers m5 grammar/protocol conformance pilots by walking the repo's
/// plugin tree at test time (see `discovery_roots` below for exactly which roots — NOT a blind
/// `✏️s/🔌️plugins/**`, a scoping decision made empirically during this wave, see `p2-m3-report.md`),
/// replacing the pre-P2-M3 hardcoded one-`#[test]`-per-pilot list (6 `include_str!` grammar tests +
/// 7 `include_str!` protocol tests, hand-added one at a time). This is the ownership keystone for
/// every future STDIO fan-out wave (P1-P3/FG1-FG4 per the plan — the only kind of fan-out wave this
/// program ever dispatches): a new stdio standard lands its own `🧬️schema/📸️snapshot/📝️text/
/// 📖️.grammar.semio` + sibling `.dsl.semio` fixture (or `🧬️schema/📸️snapshot/💾️binary/
/// 📡️.protocol.semio` + `.pack.semio`, or `🧬️schema/🧬️mutations/💾️binary/
/// 📡️.protocol.semio` + `.spr.semio`, matching dag's pre-existing 7th hardcoded pilot
/// check) and is enrolled automatically — ZERO edits to this framework file for discovery itself.
/// The one thing an FG-wave DOES still touch here is the shrink-only stdio exemption list below,
/// and only to graduate its OWN standard, once.
#[cfg(test)]
#[path = "../🔬️m5-auto-discovery/🦀️.rs"]
mod m5_auto_discovery;
//#endregion 🔖️M5AutoDiscovery

//#region 🔖️M5SoftSkip
/// @emoji 🛟 Soft-skip helpers for M5 pilot laws when a facet has not exported a usable
/// `COMPONENT_GRAMMAR_SEMIO` / `COMPONENT_PROTOCOL_SEMIO` yet (empty or stub text). Keeps the
/// fixture-sweep compiling without plugin crate fan-in; example payloads are FS-discovered.
#[cfg(test)]
#[path = "../🔬️m5-soft-skip/🦀️.rs"]
mod m5_soft_skip;
//#endregion 🔖️M5SoftSkip

//#region 🔖️M5HandcraftedGrammar
/// @emoji 📖️ P2-M3: m5 grammar conformance over EVERY auto-discovered `🧬️schema/📸️snapshot/📝️text/
/// 📖️.grammar.semio` under `✏️s/🔌️plugins` (see [`super::m5_auto_discovery`]) — replaces the
/// pre-P2-M3 hardcoded 6-pilot `include_str!` list. One `#[test]` fn iterates every discovered pair
/// and asserts each individually with a labeled failure message (chosen over N generated `#[test]`
/// fns — this dialect's test infra has no `#[test_case]`-style macro, and one aggregating fn keeps
/// per-artifact failures legible without inventing a codegen mechanism this wave doesn't need).
/// stdio standards still on [`super::m5_auto_discovery::STDIO_CONFORMANCE_GRADUATED`]'s exempt side
/// fail SOFT (logged, not asserted); every non-stdio artifact (today: lowpoly/dag/cad/en1992/note/
/// fem2d — the plan's own 6 pilots) and any graduated stdio standard fails HARD.
#[cfg(test)]
#[path = "../🔬️m5-handcrafted-grammar-conformance/🦀️.rs"]
mod m5_handcrafted_grammar_conformance;
//#endregion 🔖️M5HandcraftedGrammar

//#region 🔖️M5HandcraftedProtocol
/// @emoji 📡️ P2-M3: m5 protocol conformance over EVERY auto-discovered pack/spr protocol facet
/// (see [`super::m5_auto_discovery`]) via [`verify_protocol_source`]/[`walk_protocol`] — replaces
/// the pre-P2-M3 hardcoded 7-pilot `include_str!` list (6 pack + dag's 1 spr). Same hard/soft split
/// as [`super::m5_handcrafted_grammar_conformance`]: stdio standards still on
/// `STDIO_CONFORMANCE_GRADUATED`'s exempt side fail soft; every non-stdio artifact and any graduated
/// stdio standard fails hard.
#[cfg(test)]
#[path = "../🔬️m5-handcrafted-protocol-conformance/🦀️.rs"]
mod m5_handcrafted_protocol_conformance;
//#endregion 🔖️M5HandcraftedProtocol

//#region 🔖️M5CrossArtifactRejection
/// @emoji ⚔️ P2-M3: cross-artifact anti-genericness generalized over EVERY auto-discovered non-stdio
/// grammar+fixture pair (previously hardcoded to exactly one pair, lowpoly-vs-dag) — every distinct
/// pair's grammar must reject the other's shipped fixture body, both directions. stdio is excluded
/// entirely here (not merely soft): most stdio grammars are still ABNF-dialect/placeholder stubs per
/// the P2-W0 recon, so a stub-vs-stub non-rejection is not a meaningful anti-genericness signal yet
/// — stdio standards join this check the same way they join hard conformance, by graduating on
/// `STDIO_CONFORMANCE_GRADUATED`.
#[cfg(test)]
#[path = "../🔬️m5-cross-artifact-rejection/🦀️.rs"]
mod m5_cross_artifact_rejection;
//#endregion 🔖️M5CrossArtifactRejection

//#region 🔖️M5ProductionCoverage
/// @emoji 📊️ P2-M3: production coverage ([`Recognizer::uncovered_productions`]) over EVERY
/// auto-discovered snapshot grammar+fixture pair — previously hardcoded to 4 of the 6 non-stdio
/// pilots (lowpoly/dag/cad/en1992; note/fem2d were never enrolled here, a pre-P2-M3 gap discovery
/// closes for free). Soft-skips missing/stub specs and unparseable grammars (parse failures are
/// grammar_conformance's failure to surface, not this diagnostic's); logs uncovered names without
/// failing the gate hard on THEM (advisory, per the original design). The recognize-must-succeed
/// assertion mirrors `m5_handcrafted_grammar_conformance`'s own hard/soft split — note/fem2d joining
/// this check means fem2d's pre-existing grammar_conformance failure now also surfaces here (same
/// underlying bug, not a new one; documented in `p2-m3-report.md`).
#[cfg(test)]
#[path = "../🔬️m5-production-coverage/🦀️.rs"]
mod m5_production_coverage;
//#endregion 🔖️M5ProductionCoverage

//#region 🔖️M5SemioEnvelopeProtocol
/// @emoji 🧬️ P2-M3 deliverable 3: the `wrap_binary` SEMIO envelope (`0x89 'S' 'E' 'M' 0D 0A 1A 0A`
/// magic + u32le token-length + token + payload — real byte layout confirmed by reading
/// `wrap_binary`/`unwrap_binary`/`BINARY_MAGIC` directly, `🧰️framework/🛍️products/💻️os/🔨️modules/
/// 🧬️semio/🦀️.rs:120-134`) is uniform across every artifact and described ONCE here — a
/// framework-level `.protocol.semio` file, colocated with the real `wrap_binary` implementation it
/// describes (`🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/📡️protocol/📡️.protocol.semio`),
/// per the plan's target architecture table. Per-artifact protocol files describe only the
/// post-unwrap payload (`chain bytes` below stops at "the rest," honestly — an artifact-specific
/// protocol file is meant to walk exactly that trailing region on its own, once cross-artifact `use`
/// resolution is real; confirmed STILL non-functional on the protocol side today, see the M3 report
/// — so this file is NOT `use`d by anything yet, it stands alone as a real, parseable, walkable
/// artifact with its own conformance proof below, matching the mission's explicit fallback).
#[cfg(test)]
#[path = "../🔬️m5-semio-envelope-protocol/🦀️.rs"]
mod m5_semio_envelope_protocol;
//#endregion 🔖️M5SemioEnvelopeProtocol
