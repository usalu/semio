import { parseArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import { pdfDateFromNativeJson } from "./🗓️date/🟦️.ts";
/** 🪪️ Explicit finite native JSON admission into PDF's canonical owned scalar model. */
import type { PdfObject, PdfDictEntry } from "../../../../🧬️schema/📸️snapshot/🧩️cos/🟦️.ts";
import type { PdfFunction, PdfColorSpace } from "../../../../🧬️schema/📸️snapshot/🌈️color/🟦️.ts";
import { record, array, text, boolean, integer, integers, real, reals, optionalReals, three, four, nine, filters } from "./🧰️scalars/🟦️.ts";

function cosShallow(input: unknown): PdfObject {
  const row = record(input); const kind = text(row.kind);
  switch (kind) {
    case "null": return { kind };
    case "bool": return { kind, value: boolean(row.value) };
    case "int": return { kind, value: BigInt(integer(row.value)) };
    case "real": return { kind, negative: boolean(row.negative), coefficient: text(row.coefficient), scale: integer(row.scale) };
    case "text": return {kind,value:text(row.value)};
    case "date": return {kind,value:pdfDateFromNativeJson(row.value)};
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
    case "iccBased": return { kind, components: integer(row.components), profile: parseArtifactRef(row.profile), alternate: child, range: optionalReals(row.range) };
    case "indexed": if (!child) throw new Error("Native PDF indexed color has no base"); return { kind, base: child, hival: integer(row.hival), palette: integers(row.palette) };
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

