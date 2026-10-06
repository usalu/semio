/** 🪪️ Explicit finite native JSON admission into PDF's canonical owned scalar model. */
import type { PdfObject, PdfDictEntry, PdfStreamFilter, PdfFunction, PdfColorSpace, Binary64 } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

function record(value: unknown): Record<string, unknown> { if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("PDF native JSON requires an object"); return value as Record<string, unknown>; }
function array(value: unknown): unknown[] { if (!Array.isArray(value)) throw new Error("PDF native JSON requires an array"); return value; }
function text(value: unknown): string { if (typeof value !== "string") throw new Error("PDF native JSON requires text"); return value; }
function boolean(value: unknown): boolean { if (typeof value !== "boolean") throw new Error("PDF native JSON requires a boolean"); return value; }
function integer(value: unknown): number { if (typeof value !== "number" || !Number.isSafeInteger(value)) throw new Error("PDF native JSON integer cannot be represented exactly"); return value; }
function integers(value: unknown): number[] { return array(value).map(integer); }
function real(value: unknown): Binary64 { if (typeof value !== "number" || !Number.isFinite(value)) throw new Error("PDF native JSON REAL must be finite"); return binary64(value); }
function reals(value: unknown): Binary64[] { return array(value).map(real); }
function optionalReals(value: unknown): Binary64[] | null { return value == null ? null : reals(value); }
function three(value: unknown): [Binary64, Binary64, Binary64] { const values = reals(value); if (values.length !== 3) throw new Error("PDF native JSON requires three components"); return [values[0]!, values[1]!, values[2]!]; }
function four(value: unknown): [Binary64, Binary64, Binary64, Binary64] { const values = reals(value); if (values.length !== 4) throw new Error("PDF native JSON requires four components"); return [values[0]!, values[1]!, values[2]!, values[3]!]; }
function nine(value: unknown): [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] { const values = reals(value); if (values.length !== 9) throw new Error("PDF native JSON requires nine components"); return [values[0]!, values[1]!, values[2]!, values[3]!, values[4]!, values[5]!, values[6]!, values[7]!, values[8]!]; }

function filters(value: unknown): PdfStreamFilter[] {
  return array(value).map(item => {
    const row = record(item); const kind = text(row.kind);
    const predictor = (): { predictor: number; colors: number; bitsPerComponent: number; columns: number } | null => { if (row.predictor == null) return null; const value = record(row.predictor); return { predictor: integer(value.predictor), colors: integer(value.colors), bitsPerComponent: integer(value.bitsPerComponent), columns: integer(value.columns) }; };
    switch (kind) {
      case "flate": return { kind, predictor: predictor() };
      case "lzw": return { kind, predictor: predictor(), earlyChange: boolean(row.earlyChange) };
      case "asciiHex": case "ascii85": case "runLength": case "jpx": return { kind };
      case "dct": return { kind, colorTransform: row.colorTransform == null ? null : integer(row.colorTransform) };
      case "jbig2": return { kind, globals: row.globals == null ? null : integers(row.globals) };
      case "crypt": return { kind, name: row.name == null ? null : text(row.name) };
      case "ccitt": { const value = record(row.parameters); return { kind, parameters: { k: integer(value.k ?? 0), columns: integer(value.columns ?? 1728), rows: integer(value.rows ?? 0), blackIs1: boolean(value.blackIs1 ?? false), encodedByteAlign: boolean(value.encodedByteAlign ?? false), endOfLine: boolean(value.endOfLine ?? false), endOfBlock: boolean(value.endOfBlock ?? true), damagedRowsBeforeError: integer(value.damagedRowsBeforeError ?? 0) } }; }
      default: throw new Error("Unknown PDF native JSON filter");
    }
  });
}

function cosShallow(input: unknown): PdfObject {
  const row = record(input); const kind = text(row.kind);
  switch (kind) {
    case "null": return { kind };
    case "bool": return { kind, value: boolean(row.value) };
    case "int": return { kind, value: BigInt(integer(row.value)) };
    case "real": return { kind, negative: boolean(row.negative), coefficient: text(row.coefficient), scale: integer(row.scale) };
    case "str": return { kind, value: integers(row.value) };
    case "name": return { kind, value: text(row.value) };
    case "ref": return { kind, num: integer(row.num), gen: integer(row.gen) };
    case "array": return { kind, value: [] };
    case "dict": return { kind, value: [] };
    case "stream": return { kind, dict: [], data: integers(row.data), filters: filters(row.filters) };
    default: throw new Error("Unknown PDF native JSON COS value");
  }
}
function cosChildren(input: unknown, value: PdfObject): readonly unknown[] { const row = record(input); switch (value.kind) { case "array": case "dict": return array(row.value); case "stream": return array(row.dict); default: return []; } }

/** 🌳️ Admit native JSON COS trees iteratively; native numbers become exact owned bigint. */
export function pdfCosFromNativeJson(input: unknown): PdfObject {
  const root = cosShallow(input);
  const pending = [{ value: root, children: cosChildren(input, root), index: 0 }];
  while (pending.length) {
    const frame = pending[pending.length - 1]!;
    if (frame.index === frame.children.length) { pending.pop(); continue; }
    const member = frame.children[frame.index++];
    const dictionary = frame.value.kind !== "array";
    const source = dictionary ? record(member).value : member;
    const value = cosShallow(source);
    if (frame.value.kind === "array") frame.value.value.push(value);
    else if (frame.value.kind === "dict" || frame.value.kind === "stream") { const entries = frame.value.kind === "dict" ? frame.value.value : frame.value.dict; entries.push({ key: text(record(member).key), value }); }
    else throw new Error("Native PDF scalar cannot own COS children");
    pending.push({ value, children: cosChildren(source, value), index: 0 });
  }
  return root;
}
/** 📚️ Admit ordered native dictionary members into the canonical COS owner. */
export function pdfDictionaryFromNativeJson(input: unknown): PdfDictEntry[] { const value = pdfCosFromNativeJson({ kind: "dict", value: array(input) }); if (value.kind !== "dict") throw new Error("PDF native dictionary kind changed"); return value.value; }

function functionShallow(input: unknown): PdfFunction {
  const row = record(input); const kind = text(row.kind);
  switch (kind) {
    case "sampled": return { kind, domain: reals(row.domain), range: reals(row.range), size: integers(row.size), bitsPerSample: integer(row.bitsPerSample), order: row.order == null ? null : integer(row.order), encode: optionalReals(row.encode), decode: optionalReals(row.decode), samples: integers(row.samples) };
    case "exponential": return { kind, domain: reals(row.domain), range: optionalReals(row.range), c0: reals(row.c0), c1: reals(row.c1), n: real(row.n) };
    case "stitching": return { kind, domain: reals(row.domain), range: optionalReals(row.range), functions: [], bounds: reals(row.bounds), encode: reals(row.encode) };
    case "postScript": return { kind, domain: reals(row.domain), range: reals(row.range), code: text(row.code) };
    case "array": return { kind, functions: [] };
    default: throw new Error("Unknown PDF native JSON function");
  }
}
function functionChildren(input: unknown, value: PdfFunction): readonly unknown[] { return value.kind === "array" || value.kind === "stitching" ? array(record(input).functions) : []; }

/** 🎚️ Admit function nesting while wrapping finite native scalars into Binary64 words. */
export function pdfFunctionFromNativeJson(input: unknown): PdfFunction {
  const root = functionShallow(input);
  const pending = [{ value: root, children: functionChildren(input, root), index: 0 }];
  while (pending.length) {
    const frame = pending[pending.length - 1]!;
    if (frame.index === frame.children.length) { pending.pop(); continue; }
    const source = frame.children[frame.index++]; const value = functionShallow(source);
    if (frame.value.kind !== "array" && frame.value.kind !== "stitching") throw new Error("PDF scalar function cannot own children");
    frame.value.functions.push(value); pending.push({ value, children: functionChildren(source, value), index: 0 });
  }
  return root;
}

function colorAlternate(input: unknown): unknown { const row = record(input); switch (row.kind) { case "iccBased": case "separation": case "deviceN": return row.alternate; case "indexed": case "pattern": return row.base; default: return null; } }
function colorShallow(input: unknown, child: PdfColorSpace | null): PdfColorSpace {
  const row = record(input); const kind = text(row.kind);
  switch (kind) {
    case "deviceGray": case "deviceRgb": case "deviceCmyk": return { kind };
    case "calGray": return { kind, whitePoint: three(row.whitePoint), blackPoint: row.blackPoint == null ? null : three(row.blackPoint), gamma: row.gamma == null ? null : real(row.gamma) };
    case "calRgb": return { kind, whitePoint: three(row.whitePoint), blackPoint: row.blackPoint == null ? null : three(row.blackPoint), gamma: row.gamma == null ? null : three(row.gamma), matrix: row.matrix == null ? null : nine(row.matrix) };
    case "lab": return { kind, whitePoint: three(row.whitePoint), blackPoint: row.blackPoint == null ? null : three(row.blackPoint), range: row.range == null ? null : four(row.range) };
    case "iccBased": return { kind, components: integer(row.components), profile: integers(row.profile), alternate: child, range: optionalReals(row.range) };
    case "indexed": if (!child) throw new Error("Native PDF indexed color has no base"); return { kind, base: child, hival: integer(row.hival), lookup: integers(row.lookup) };
    case "separation": if (!child) throw new Error("Native PDF color has no alternate"); return { kind, name: text(row.name), alternate: child, tintTransform: pdfFunctionFromNativeJson(row.tintTransform) };
    case "deviceN": if (!child) throw new Error("Native PDF color has no alternate"); return { kind, names: array(row.names).map(text), alternate: child, tintTransform: pdfFunctionFromNativeJson(row.tintTransform), attributes: row.attributes == null ? null : pdfDictionaryFromNativeJson(row.attributes) };
    case "pattern": return { kind, base: child };
    case "named": return { kind, name: text(row.name) };
    default: throw new Error("Unknown PDF native JSON color");
  }
}

/** 🌈️ Admit explicitly owned color alternate chains without reflective snapshot traversal. */
export function pdfColorFromNativeJson(input: unknown): PdfColorSpace {
  const pending: unknown[] = []; let source: unknown = input;
  while (source != null) { pending.push(source); source = colorAlternate(source); }
  let child: PdfColorSpace | null = null; while (pending.length) child = colorShallow(pending.pop(), child);
  if (!child) throw new Error("Native PDF color is empty"); return child;
}

export { record, array, text, boolean, integer, integers, real, reals, three, four, filters };
