// 🚪️ IoEntryDescriptor[] mirror for `s.dag.dag@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️component.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to (json exercises a real conversion, csv/png/svg are best-effort structural
// reinterpretations, md wraps the native `.dag` DSL text, txt is an honest not-yet-implemented
// stub). Shaped inline (no generated `IoEntryDescriptor` type checked in anywhere yet in this
// repo) so this file has no fragile forward reference; swap for a real import once owned schema exporter
// generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const DAG = "s.dag.dag@1/*";
const CSV = "s.stdio.csv@rfc4180/*";
const MD = "s.stdio.md@commonmark/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const PNG = "s.stdio.png@1.2/*";
const SVG = "s.stdio.svg@1.1/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: DAG, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: DAG, fidelity: "Exact", sniffs: false },
  { from: DAG, into: MD, fidelity: "Canonical", sniffs: false },
  { from: MD, into: DAG, fidelity: "Canonical", sniffs: false },
  { from: DAG, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: DAG, fidelity: "Lossy", sniffs: false },
  { from: DAG, into: PNG, fidelity: "Lossy", sniffs: false },
  { from: PNG, into: DAG, fidelity: "Lossy", sniffs: false },
  { from: DAG, into: SVG, fidelity: "Lossy", sniffs: false },
  { from: SVG, into: DAG, fidelity: "Lossy", sniffs: false },
  { from: DAG, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: DAG, fidelity: "Lossy", sniffs: false },
];
