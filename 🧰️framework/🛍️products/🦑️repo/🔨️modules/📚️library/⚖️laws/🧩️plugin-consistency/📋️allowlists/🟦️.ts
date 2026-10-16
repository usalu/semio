/** 📃️ Wave 4 plugin consistency policy: shrink-only allowlists and exemptions.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */


//#region 🔧️PolicyAllowlists
/**
 * 🎫️ Wave 4 V1 duplication audit (`.🧬semio/🦑️repo/🎫️tickets/26/07/18/WAVE-4-V1-DUPLICATION-HUNTER-AUDIT`): both crates
 * resolve a second "terminology" axis (native/reuse) the SDK's locale-only `app_labels!`/`LocaleLabels`
 * primitive can't express. Flagged for a Wave-4 decision (extend the primitive to two axes, or formally
 * accept the gap) — tracked here as a low-priority, non-failing breach until that decision lands.
 * Keyed `<pluginId>#<StructName>` via `policyScopeKey` — the struct name alone already disambiguates
 * puzzle's three apps (Puzzle2dLabels/Puzzle3dLabels/Puzzle5dLabels), so no app segment is needed.
 */
export const POLICY_LABELS_TWO_AXIS_ALLOWLIST = new Set<string>(["cad#CadLabels", "puzzle#Puzzle2dLabels", "puzzle#Puzzle3dLabels", "puzzle#Puzzle5dLabels"]);

/**
 * 🎫️ Wave 4 V1 duplication audit: puzzle's 3d/5d `tree_item_with_action` redefinitions add an `icon_id`
 * param the SDK's description-based primitive can't express (icon rendering) — documented real gap,
 * tracked here as a low-priority, non-failing breach rather than a should-fix duplicate. Keyed
 * `<pluginId>#<appId>` via `policyScopeKey` + `policyAppIdFromCrateDir` — the original `puzzle/plugin/rs#d3`
 * form assumed one shared crate with `pub mod d3 { … }`/`pub mod d5 { … }` wrappers; today (and in the
 * future taxonomy) each app is its own crate, so the app id in the crate's own path is the disambiguator,
 * not an enclosing `pub mod`.
 */
export const POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST = new Set<string>(["puzzle#3d", "puzzle#5d"]);

/**
 * 🎫️ Wave 4 V3 coupling audit (`.🧬semio/🦑️repo/🎫️tickets/26/07/18/WAVE-4-APP-TO-APP-COUPLING-AND-FRAMEWORK-IDENTITY-LEAK-AUDIT`):
 * these crates are neutral shared domain/library crates that also happen to ship their own minimal
 * playground app (documented via each crate's `AGENTS.md`) — depending on them is not app-to-app coupling.
 */
export const POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST = new Set<string>(["flow", "trinity_jack", "trinity_ram", "mathematical_graph_drawing", "mathematical_geometry", "infinite_board_port_directed", "infinite_board_port_directed_dag"]);

/**
 * 🎫️ dsl/ derive-engine migration lock step: technologies whose example/*.json fixture has not yet
 * been converted to its own DSL-text extension (e.g. `.puzzle2d`, `.flow`). Tracked here as a documented,
 * still-open follow-up rather than a silent exception — remove an entry once that technology's fixture
 * is migrated (see an already-migrated sibling crate's `*_fixture` round-trip test for the pattern).
 */
export const POLICY_JSON_FIXTURE_ALLOWLIST = new Set<string>([]);

/** 🛡️Path prefixes exempt from the no-JSON-fixture rule structurally — not a technology document fixture (e.g. `coda/`'s `example/` holds a whole simulated ACC project's run/iteration artifacts, unrelated to the dsl/ migration). */
export const POLICY_JSON_FIXTURE_PATH_PREFIX_ALLOWLIST = ["coda/"];

/**
 * 🎫️ pack/ binary-document-layer rollout lock step (`.🧬semio/🦑️repo/🎫️tickets/26/07/27/
 * PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS`): every `*.rs` file below that already calls
 * `assert_dsl_round_trip(`/`assert_document_text_round_trip(` but does not yet ALSO call
 * `assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(` on the same fixtures — seeded at
 * wave 1 with every file that fails the check today (wave 0 only built the `pack` crate family itself;
 * wave 1 only wired `vcs`/`dsl_derive`/`framework`, proving the mechanism on `vcs/rs/lib.rs`'s own
 * `VcsSnapshot` fixture, which is why `vcs/rs/lib.rs` is NOT in this list). Wave 2's per-app-family
 * agents add the pack-equivalence assertions beside each technology's existing DSL round-trip tests and
 * remove that file from this list; wave 3 verifies it has shrunk to empty. Still empty today, but keyed
 * by `policyNormalizeRelPath` (canonical `<pluginId>/<component>`) like its siblings below, so the first
 * entry ever added here is already taxonomy-move-proof.
 */
export const POLICY_PACK_COMPLETENESS_ALLOWLIST = new Set<string>([]);

/**
 * 🎫️ CW7 command-envelope law lock step (`.🧬semio/🦑️repo/🎫️tickets/26/07/27/
 * INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING`): every `*.rs` file that already calls
 * `assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(` but does not yet ALSO call
 * `vcs::test_support::assert_command_envelope_round_trip` (added in CW7) on the same fixtures — seeded
 * at CW8 with every file that fails the check today, exactly like `POLICY_PACK_COMPLETENESS_ALLOWLIST`
 * was seeded for the dsl/pack lock step (`vcs/rs/lib.rs` itself proves the mechanism on its own
 * `VcsSnapshot`/`LossyMutation` fixtures, which is why it is NOT in this list). Remove an entry
 * once that file adds the command-envelope-round-trip call. Keyed by `policyNormalizeRelPath` (canonical
 * `<pluginId>/<component>`, not the raw repo-relative path) so a plugin's move to the taxonomy layout
 * never requires touching this list — matched against every discovered file via `policyNormalizeRelPath`
 * at lookup time, so both the legacy and future path shape resolve to the same entry.
 */
export const POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST = new Set<string>([
  "architect/spine",
  "os/dsl",
  "products/os",
  "lowpoly/spr",
  "os/workflow",
  "animate/presentation/spr",
  "gis/artifact",
  "mathematical/equation/artifact",
  "note/artifact",
  "raster/artifact",
  "reasoning/artifact",
  "space/artifact",
  "vcs/artifact",
]);

/**
 * 🎫️ dsl/ derive-engine migration lock step: known generic bridges whose `DocumentDsl`/`OpText` coverage
 * cannot be seen as a `#[derive(dsl::Dsl...)]` attribute directly on the type (a blanket/generic impl
 * elsewhere covers them) — accepted as DSL-complete by `policyDslCompletenessBreaches` without a
 * hand-rolled-impl grep hit under that exact type name.
 * - `Value` (`serde_json::Value`): blanket `impl DocumentDsl for serde_json::Value` in `vcs/rs/lib.rs`
 *   (the schema-less escape hatch for a technology whose `DocumentApp::Snapshot` predates its own
 *   typed DSL derive — see `puzzle_2d`'s `🔖️ValueBridge` region).
 * - `SetDocumentMutation` (`norm_core::SetDocumentMutation<D>`): one hand-rolled generic
 *   `impl<D: DocumentDsl> OpText for SetDocumentMutation<D>` in `norm/core/rs/lib.rs`, shared by every
 *   norm family's `Mutation` type instead of a per-family derive.
 */
export const POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST = new Set<string>(["Value", "SetDocumentMutation"]);

/**
 * 🎫️ W1 grammar-engine wave (`.claude/plans/the-final-goal-for-jolly-spindle.md` `## Master wave
 * plan` `W1 — Grammar engine`, design ruling B-R4): every `*.rs` file that defines a real
 * `impl protocol::MutationDiff<...>` diff type but does not yet ALSO give that type a
 * `protocol::DiffCodec` impl (via `#[derive(dsl::DslDiff)]` or a hand-rolled impl) — seeded at W1
 * with every file that fails the check today. `DiffCodec` is deliberately NOT (yet) a hard
 * supertrait bound of `MutationDiff` (see that trait's doc comment in `protocol_command/rs/lib.rs`)
 * — this allowlist is the shrinking-list enforcement mechanism instead, exactly like
 * `POLICY_PACK_COMPLETENESS_ALLOWLIST`/`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST` before it.
 * `writer_op/rs/lib.rs` (`WriterDiff`) and `note_op/rs/lib.rs` (`NoteDiff`) are the two W1
 * proof-of-mechanism types and are deliberately NOT in this list. A handful of entries are
 * permanently-test-fixture files (`protocol_command`/`protocol_causal`/`protocol_crdt`/
 * `protocol laws`/`plugin`/`db_document`/`db_engine`'s own `AddDiff`/`DummyDiff`/`TestDiff`/
 * `RegisterDiff`/`GraphDiff`/`CausalAddDiff`/`HashDiff`/`BenchDiff` law/bench fixtures, used to test
 * the trait machinery itself, never a real document type) — those are expected to stay allowlisted forever,
 * mirrored by `POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST`'s precedent for permanent entries.
 * Full coverage (all real diff types deriving/implementing `DiffCodec`, then `DiffCodec` promoted to
 * a hard supertrait bound) is wave 6's "Lane C (B5)" item. Remove an entry once that file's diff
 * type gets a `DiffCodec` impl.
 */
export const POLICY_DIFF_COMPLETENESS_ALLOWLIST = new Set<string>([
  // permanent: trait-machinery test fixtures, never real document types
  "os/plugin",
  "db/document",
  "db/engine",
  "protocol/crdt",
  "protocol/command",
  "protocol/causal",
  "store/sync",
  // deferred to W6: real diff types not yet covered by #[derive(dsl::DslDiff)]
  "flow/core",
  "directed/dag",
  "os/playbook",
  "products/os",
  "os/store",
  "trinity/rewriting/op",
  "trinity/ram",
  "remodel/op",
  "raster/op",
  "process/3d/op",
  "norm/core",
  "norm/artifact",
  "cad/op",
  "block/3d/op",
  "block/5d/op",
  "block/2d/op",
  "reasoning/wires/op",
  "sequence/op",
  "animate/presentation/op",
  "space/home/op",
  "procedural/3d/op",
  "procedural/2d/op",
  "vcs/op",
  "vcs/engine",
  "gis/3d/op",
  "gis/2d/op",
  "imperative/op",
  "sourcing/curate/op",
  "architect/artifact",
  "architect/spine",
  "shooting/op",
  "mathematical/equation/op",
  "layout/op",
  "puzzle/3d/op",
  "puzzle/5d/op",
  "puzzle/2d/op",
  "fem/3d/op",
  "fem/2d/op",
  "draw/op",
  "playbook/procedural",
  "lowpoly/op",
  // seeded by W1b scaffold, burn down as W2/W3 land real 🧰️triples-backed sparse diffs (see
  // w1b-scaffold-manifest.md §6 "Diff/Mutation shape" — a full-replace scaffold Diff today, all
  // 8 laws pass, but no real DiffCodec impl yet). Keys computed via policyNormalizeRelPath.
  // W3 closer (26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) removed
  // html/epw/mp3/tsv/wav — each now carries a real hand-rolled `impl protocol::DiffCodec for
  // <X>Diff` in the same file (verified by grep before removal, policy re-run green after). mp4/avi
  // stay: their `MutationDiff`/`DiffAlgebra` impls exist but neither file has a `DiffCodec` impl —
  // a real, still-open gap, not yet satisfied.
  "stdio/mp4/standards#isobmff-subsets-any-schema-diff-component",
  "stdio/avi/standards#1.0-subsets-any-schema-diff-component",
  // W2a's 6 subsets (brep/cad/drawing/mesh/model/object) removed by the W2a closer
  // (26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) — each verified to
  // now carry a real hand-rolled `impl protocol::DiffCodec for Semio<X>Diff` in the same file
  // (grepped before removal, `policyDiffCompletenessBreaches` re-run green after; see
  // `w2a-close-report.md`). W2b/W3 subsets are untouched here.
]);

/**
 * 🎫️ Handcrafted `.grammar.semio` / `.protocol.semio` specs per artifact facet (see ticket
 * `26/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`). Paths listed here are still on generic/stub
 * specs; remove each entry once the facet's normative spec + recognizer sweep is committed.
 */
export const POLICY_GRAMMAR_FILE_ALLOWLIST = new Set<string>([]);

/** 📡️ Stub `.protocol.semio` files not yet backed by a byte-level recognizer proof. */
export const POLICY_PROTOCOL_FILE_ALLOWLIST = new Set<string>([]);

/**
 * ⚖️ Constitutional artifact facets whose `🟦️.ts` may remain a WASM scaffold stub.
 * Stubs under these non-mutation facets are accepted
 * structurally — never tracked in a per-file allowlist (Wave 2b / OPERATIONS-TO-MUTATIONS).
 */
export const POLICY_TS_FACADE_CONSTITUTIONAL_FACETS = new Set<string>([
  "🗣️dsl",
  "🔧️op",
  "🔺️diff",
  "🎒️pack",
  "📡️spr",
  "⚙️engine",
]);
export const POLICY_ENGINE_FACET = "⚙️engine";
export const POLICY_OP_FACET = "🔧️op";
/**
 * ⚖️P3/M4: colliding .grammar.semio/.protocol.semio after name/start normalization. Remove a path once its normalized hash is unique.
 * Seeded 0 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
export const POLICY_SPEC_DISTINCTNESS_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: generic catch-all grammar specs under ✏️s/. Remove once handcrafted.
 * Seeded 0 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
export const POLICY_GENERIC_SPEC_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: grammars with use family-X that never reference a family production. Remove once wired.
 * Seeded 0 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
export const POLICY_DECLARED_USE_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: facet 🦀️.rs missing include_str! of sibling spec. Remove once wired.
 * Seeded 220 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
export const POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: artifacts with facet specs but no register_language under the artifact. Remove once engines register.
 * Seeded 44 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
export const POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: empty/stub `.semio` under `📚️examples/** /🖼️assets/` (size≤64). Kept empty — stubs are breaches.
 */
export const POLICY_EMPTY_EXAMPLE_EXEMPTIONS = new Set<string>([]);
/**
 * ⚖️P6: empty — DocumentDsl/OpText/DocumentPack/OpBinary emission deleted from DslDocument/DslOps.
 * Scanner now flags residual __rt/op_rt codec calls (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
const POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS = new Set<string>([]);

//#endregion 🔧️PolicyAllowlists
