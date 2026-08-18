// 🚪️ IoEntryDescriptor[] mirror for `s.draw.draw@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️component.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to. svg export and json both directions are real conversions; svg import and every
// pdf/png/dwg/dxf hop are honest not-yet-implemented stubs (pdf/png/dwg/dxf export previously
// mislabeled this artifact's own DSL text as the target format — fixed to refuse honestly instead).
// Shaped inline (no generated `IoEntryDescriptor` type checked in anywhere yet in this repo) so this
// file has no fragile forward reference; swap for a real import once ts-rs generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const DRAW = "s.draw.draw@1/*";
const SVG = "s.stdio.svg@1.1/*";
const PDF = "s.stdio.pdf@1.4/*";
const PNG = "s.stdio.png@1.2/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const DWG = "s.stdio.dwg@ac1018/*";
const DXF = "s.stdio.dxf@r12/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: DRAW, into: SVG, fidelity: "Lossy", sniffs: false },
  { from: SVG, into: DRAW, fidelity: "Lossy", sniffs: false },
  { from: DRAW, into: PDF, fidelity: "Lossy", sniffs: false },
  { from: PDF, into: DRAW, fidelity: "Lossy", sniffs: false },
  { from: DRAW, into: PNG, fidelity: "Lossy", sniffs: false },
  { from: PNG, into: DRAW, fidelity: "Lossy", sniffs: false },
  { from: DRAW, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: DRAW, fidelity: "Exact", sniffs: false },
  { from: DRAW, into: DWG, fidelity: "Lossy", sniffs: false },
  { from: DWG, into: DRAW, fidelity: "Lossy", sniffs: false },
  { from: DRAW, into: DXF, fidelity: "Lossy", sniffs: false },
  { from: DXF, into: DRAW, fidelity: "Lossy", sniffs: false },
];
