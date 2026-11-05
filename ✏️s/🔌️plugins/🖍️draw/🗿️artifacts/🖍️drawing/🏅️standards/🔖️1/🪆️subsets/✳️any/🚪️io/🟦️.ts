/** 🚪️ Registered native Drawing IO metadata using the shared wire contract. */
import { parseDialectCoordinate, type IoEntryDescriptor } from "@semio-tech/framework/io/schema";

const DRAWING = parseDialectCoordinate("s.draw.drawing@1/*");
const SVG = parseDialectCoordinate("s.stdio.svg@1.1/*");
const PDF = parseDialectCoordinate("s.stdio.pdf@1.4/*");
const PNG = parseDialectCoordinate("s.stdio.png@1.2/*");
const JSON_DIALECT = parseDialectCoordinate("s.stdio.json@rfc8259/*");
const DWG = parseDialectCoordinate("s.stdio.dwg@ac1018/*");
const DXF = parseDialectCoordinate("s.stdio.dxf@r12/*");

export const ioEntries: IoEntryDescriptor[] = [
  { from: DRAWING, into: SVG, fidelity: "Lossy", sniffs: false },
  { from: DRAWING, into: PDF, fidelity: "Lossy", sniffs: false },
  { from: DRAWING, into: PNG, fidelity: "Lossy", sniffs: false },
  { from: DRAWING, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: DRAWING, fidelity: "Exact", sniffs: true },
  { from: SVG, into: DRAWING, fidelity: "Semantic", sniffs: true },
  { from: DRAWING, into: DWG, fidelity: "Lossy", sniffs: false },
  { from: DRAWING, into: DXF, fidelity: "Lossy", sniffs: false },
];
