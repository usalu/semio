// 🚪️ IoEntryDescriptor[] mirror for `s.sourcing.curation@1/*`: the real Rust `io()` entries (`🦀️.rs`), one `Serializer`/`Deserializer` pair per foreign dialect — zip 2.0 (document archive: DSL member plus rfc8259 rendition), json rfc8259 and text utf-8 — each a lossless `Exact` hop. The language-agnostic expectations live in `🧪️tests/🧫️fixtures/🚪️io-curation-1/🔣️.json`.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const CURATION = "s.sourcing.curation@1/*";
const ZIP = "s.stdio.zip@2.0/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const TXT = "framework.io.text@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: CURATION, into: ZIP, fidelity: "Exact", sniffs: false },
  { from: ZIP, into: CURATION, fidelity: "Exact", sniffs: false },
  { from: CURATION, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: CURATION, fidelity: "Exact", sniffs: false },
  { from: CURATION, into: TXT, fidelity: "Exact", sniffs: false },
  { from: TXT, into: CURATION, fidelity: "Exact", sniffs: false },
];
