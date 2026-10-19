/** 🔎️ The typed parsers of the stream payloads that the stream role validation reads. */

import schema from "../🔣️.json" with { type: "json" };
import type { PdfOp } from "../🖋️content/🟦️.ts";
import type { PdfImage } from "../🖼️resource/🟦️.ts";
import type { PdfEmbeddedCMap, PdfFontProgram, PdfToUnicode } from "../🔤️font/🟦️.ts";
import { validateAgainst } from "../🛂️validation/🟦️.ts";

export const parsePdfOp = (value: unknown): PdfOp => validateAgainst<PdfOp>(schema, "/$defs/PdfOp", value);

export const parsePdfImage = (value: unknown): PdfImage => validateAgainst<PdfImage>(schema, "/$defs/PdfImage", value);

export const parsePdfToUnicode = (value: unknown): PdfToUnicode => validateAgainst<PdfToUnicode>(schema, "/$defs/PdfToUnicode", value);

export const parsePdfFontProgram = (value: unknown): PdfFontProgram => validateAgainst<PdfFontProgram>(schema, "/$defs/PdfFontProgram", value);

export const parsePdfEmbeddedCMap = (value:unknown):PdfEmbeddedCMap=>validateAgainst<PdfEmbeddedCMap>(schema,"/$defs/PdfEmbeddedCMap",value);
