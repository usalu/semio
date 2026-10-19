/** 🧱️ The PDF image body admission of native JSON shared by the image resources and the inline images of content streams. */

import { parseArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import type { PdfImageBody } from "../../../../../🧬️schema/📸️snapshot/🖼️resource/🟦️.ts";
import { record, text, integers } from "../🧰️scalars/🟦️.ts";

/** 🖼️ Admit logical sample words or an explicit foreign artifact reference. */
export function pdfImageBodyFromNativeJson(input:unknown):PdfImageBody{const row=record(input);const kind=text(row.kind);switch(kind){case "samples":{const values=integers(row.values);if(values.some(value=>value<0||value>65535))throw new Error("PDF sample is outside its logical word width");return {kind,values};}case "artifact":return {kind,reference:parseArtifactRef(row.reference)};default:throw new Error("Unknown PDF image body");}}
