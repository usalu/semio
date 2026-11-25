/** 🧱️ The PDF image body admission of native JSON shared by the image resources and the inline images of content streams. */

import { parseArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import type { PdfImageBody } from "../../../../../🧬️schema/📸️snapshot/🖼️resource/🟦️.ts";
import { record, text, integers } from "../🧰️scalars/🟦️.ts";

/** 🖼️ Admit the native packed rows of ISO 32000-1 §8.9.3 or an explicit foreign artifact reference. */
export function pdfImageBodyFromNativeJson(input:unknown):PdfImageBody{const row=record(input);const kind=text(row.kind);switch(kind){case "samples":{const packed=integers(row.packed);if(packed.some(byte=>byte<0||byte>255))throw new Error("PDF packed image byte is outside 0..255");return {kind,packed};}case "artifact":return {kind,reference:parseArtifactRef(row.reference)};default:throw new Error("Unknown PDF image body");}}
