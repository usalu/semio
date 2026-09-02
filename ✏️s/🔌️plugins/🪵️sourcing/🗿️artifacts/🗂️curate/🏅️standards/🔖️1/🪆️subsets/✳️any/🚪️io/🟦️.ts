// 🚪️ IoEntryDescriptor[] mirror for `s.sourcing.curate@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to. `json` exercises a real conversion (`Exact`, a genuine `serde_json` structural
// round trip); `zip`/`png`/`stl`/`obj` are pre-existing non-functional stubs (format mismatch,
// confirmed by inspection, not fixed this pass) and `txt` is an honest not-yet-implemented stub —
// all four labeled `Lossy` for honesty. Shaped inline (no generated `IoEntryDescriptor` type
// checked in anywhere yet in this repo) so this file has no fragile forward reference; swap for a
// real import once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const CURATE = "s.sourcing.curate@1/*";
const ZIP = "s.stdio.zip@2.0/*";
const PNG = "s.stdio.png@1.2/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const STL = "s.stdio.stl@ascii/*";
const OBJ = "s.stdio.obj@3.0/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: CURATE, into: ZIP, fidelity: "Lossy", sniffs: false },
  { from: ZIP, into: CURATE, fidelity: "Lossy", sniffs: false },
  { from: CURATE, into: PNG, fidelity: "Lossy", sniffs: false },
  { from: PNG, into: CURATE, fidelity: "Lossy", sniffs: false },
  { from: CURATE, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: CURATE, fidelity: "Exact", sniffs: false },
  { from: CURATE, into: STL, fidelity: "Lossy", sniffs: false },
  { from: STL, into: CURATE, fidelity: "Lossy", sniffs: false },
  { from: CURATE, into: OBJ, fidelity: "Lossy", sniffs: false },
  { from: OBJ, into: CURATE, fidelity: "Lossy", sniffs: false },
  { from: CURATE, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: CURATE, fidelity: "Lossy", sniffs: false },
];
