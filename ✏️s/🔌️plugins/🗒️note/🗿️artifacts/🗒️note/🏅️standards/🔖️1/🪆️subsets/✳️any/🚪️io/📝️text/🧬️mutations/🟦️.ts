/** 🔧️ note.mutation op-text facade. Real codec: `impl protocol::OpText for NoteMutation` in the
 * sibling `🦀️.rs` (`parse_op`/`print_op`, callable natively from Rust today). Not wired
 * here: `world actor`'s WIT surface
 * (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`) exports only
 * `poll`/`jobs`/`checkpoint`/`describe` — the per-verb
 * `apply-mutations[-text]` export that could have carried this was deleted in the B1 world-collapse
 * in favor of one turn-loop entry point. Needs a new stateless codec-call WIT export plus a TS host
 * loader (jco-generated component bindings, see 🧫️fixtures/🔌️jcoprobe) before this can call real
 * Rust. See 📓️wasm-facade-wiring.md. */
export function parseDsl(text: string): unknown {
  throw new Error("note.mutation parseDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
export function printDsl(value: unknown): string {
  throw new Error("note.mutation printDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
