import type { NoteSnapshot } from "../../../🧬️schema/📸️snapshot/🟦️.ts";

/** 🗣️ note.note snapshot text facade. Real codec: `parse_note_dsl`/`print_note_dsl` (the
 * outside-crate bridge) and `parse_dsl`/`print_dsl` in the sibling `🦀️.rs`, wrapping
 * `store::ArtifactDsl for NoteSnapshot` (callable natively from Rust today). Not wired here:
 * `world actor`'s WIT surface
 * (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`) exports only
 * `poll`/`jobs`/`checkpoint`/`describe` — the
 * `read/load-app-document-text` export that could have carried this was deleted in the B1
 * world-collapse in favor of one turn-loop entry point. Needs a new stateless codec-call WIT export
 * plus a TS host loader (jco-generated component bindings, see 🧫️fixtures/🔌️jcoprobe). See
 * 📓️wasm-facade-wiring.md. */
export function parseDsl(text: string): NoteSnapshot {
  throw new Error("note.note parseDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
export function printDsl(value: NoteSnapshot): string {
  throw new Error("note.note printDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
