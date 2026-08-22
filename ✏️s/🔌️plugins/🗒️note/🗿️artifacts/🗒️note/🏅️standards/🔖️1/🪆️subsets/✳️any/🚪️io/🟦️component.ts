// 🚪️ IoEntryDescriptor[] mirror for `s.note.note@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️component.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to. json is a lossless serde round trip (Exact); every other dialect drops real
// structure (approximated geometry, text-only extraction, or a blank canvas), so those are Lossy.
// Shaped inline (no generated `IoEntryDescriptor` type checked in anywhere yet in this repo) so
// this file has no fragile forward reference; swap for a real import once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const NOTE = "s.note.note@1/*";
const SVG = "s.stdio.svg@1.1/*";
const PDF = "s.stdio.pdf@1.4/*";
const PNG = "s.stdio.png@1.2/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const DWG = "s.stdio.dwg@ac1018/*";
const DXF = "s.stdio.dxf@r12/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: NOTE, into: SVG, fidelity: "Lossy", sniffs: false },
  { from: SVG, into: NOTE, fidelity: "Lossy", sniffs: false },
  { from: NOTE, into: PDF, fidelity: "Lossy", sniffs: false },
  { from: PDF, into: NOTE, fidelity: "Lossy", sniffs: false },
  { from: NOTE, into: PNG, fidelity: "Lossy", sniffs: false },
  { from: PNG, into: NOTE, fidelity: "Lossy", sniffs: false },
  { from: NOTE, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: NOTE, fidelity: "Exact", sniffs: false },
  { from: NOTE, into: DWG, fidelity: "Lossy", sniffs: false },
  { from: DWG, into: NOTE, fidelity: "Lossy", sniffs: false },
  { from: NOTE, into: DXF, fidelity: "Lossy", sniffs: false },
  { from: DXF, into: NOTE, fidelity: "Lossy", sniffs: false },
];
