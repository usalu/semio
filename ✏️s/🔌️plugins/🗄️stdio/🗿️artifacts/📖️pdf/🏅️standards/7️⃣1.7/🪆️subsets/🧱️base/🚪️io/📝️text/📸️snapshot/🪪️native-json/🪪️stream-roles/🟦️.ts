/** 🪪️ Native transport admission constructs explicit semantic roles. */
import { record, array } from "../🟦️.ts";
import { pdfOperationFromNativeJson } from "../🖋️content/🟦️.ts";
import { pdfUnicodeFromNativeJson, pdfCMapFromNativeJson, pdfProgramFromNativeJson } from "../🔤️font/🟦️.ts";
import { pdfImageFromNativeJson } from "../🖼️resource/🟦️.ts";
import { parsePdfAdmittedStreamRole, type PdfAdmittedStreamRole } from "../../../../../🧬️schema/🪪️stream-roles/🟦️.ts";

/** 🛂️ Interprets numeric native transport once before pure role projection. */
export function pdfStreamRoleFromNativeJson(input: unknown): PdfAdmittedStreamRole {
  const role = record(input), value = record(role.value);
  let semantic: unknown = value;
  if (value.kind === "operators") semantic = { kind: value.kind, content: array(value.content).map(pdfOperationFromNativeJson) };
  else if (value.kind === "unicodeMap") semantic = { kind: value.kind, mapping: pdfUnicodeFromNativeJson(value.mapping) };
  else if (value.kind === "characterMap") { const cmap = pdfCMapFromNativeJson({ kind: "embedded", cmap: value.cmap }); if (cmap.kind !== "embedded") throw new Error("Native embedded CMap changed kind"); semantic = { kind: value.kind, cmap: cmap.cmap }; }
  else if (value.kind === "fontProgram") semantic = { kind: value.kind, program: pdfProgramFromNativeJson(value.program) };
  else if (value.kind === "image") semantic = { kind: value.kind, image: pdfImageFromNativeJson(value.image) };
  return parsePdfAdmittedStreamRole({ identity: role.identity, dependencies: role.dependencies, value: semantic });
}
