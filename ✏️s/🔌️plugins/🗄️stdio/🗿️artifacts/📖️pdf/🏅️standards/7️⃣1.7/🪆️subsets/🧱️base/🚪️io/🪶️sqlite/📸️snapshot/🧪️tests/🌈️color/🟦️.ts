import { pdfFunctionFromNativeJson,pdfColorFromNativeJson } from "../../../../📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
/** 🌈️ All authored function and color variants, exact IEEE words and independent SQL queries. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import type { PdfFunction, PdfColorSpace, Binary64 } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { parsePdfFunction, parsePdfColorSpace } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { pdfFunctionToSqliteDatabase, pdfFunctionFromSqliteDatabase, pdfColorSpaceToSqliteDatabase, pdfColorSpaceFromSqliteDatabase } from "../../🌈️color/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

async function cases(): Promise<{ functions: PdfFunction[]; colors: PdfColorSpace[]; depth: number }> {
  const input = await Bun.file(new URL("../../🌈️color/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const values: Binary64[] = input.binary64.map((bits: string) => ({ bits: BigInt(`0x${bits}`) }));
  const white: [Binary64, Binary64, Binary64] = [values[0]!, values[1]!, values[2]!];
  const exponent: PdfFunction = { kind: "exponential", domain: values, range: [], c0: [], c1: values, n: values[5]! };
  const functions: PdfFunction[] = [
    { kind: "sampled", domain: values, range: [], size: [4294967295, 0], bitsPerSample: 32, order: null, encode: [], decode: null, samples: input.sampleBytes }, exponent,
    { kind: "stitching", domain: values, range: null, functions: [exponent], bounds: [], encode: values },
    { kind: "postScript", domain: values, range: [], code: input.postScript }, { kind: "array", functions: [] }
  ];
  const colors: PdfColorSpace[] = [
    { kind: "deviceGray" }, { kind: "deviceRgb" }, { kind: "deviceCmyk" },
    { kind: "calGray", whitePoint: white, blackPoint: null, gamma: values[5]! },
    { kind: "calRgb", whitePoint: white, blackPoint: white, gamma: white, matrix: [values[0]!, values[1]!, values[2]!, values[3]!, values[4]!, values[5]!, values[6]!, values[0]!, values[1]!] },
    { kind: "lab", whitePoint: white, blackPoint: null, range: [values[3]!, values[4]!, values[5]!, values[6]!] },
    { kind: "iccBased", components: 4, profile: input.profileBytes, alternate: null, range: [] },
    { kind: "indexed", base: { kind: "deviceRgb" }, hival: 255, lookup: input.sampleBytes },
    { kind: "separation", name: input.names[0], alternate: { kind: "deviceGray" }, tintTransform: exponent },
    { kind: "deviceN", names: input.names, alternate: { kind: "deviceCmyk" }, tintTransform: exponent, attributes: [{ key: "Owner", value: { kind: "int", value: 9223372036854775807n } }] },
    { kind: "pattern", base: null }, { kind: "named", name: input.names[1] }
  ];
  return { functions, colors, depth: input.depth };
}

test("PDF functions and colors retain every variant and optional empty sequence", async () => {
  const input = await cases();
  for (const value of input.functions) {
    const encoded = await pdfFunctionToSqliteDatabase(value);
    const bytes = await exportSqliteDatabase(encoded.database);
    expect(await pdfFunctionFromSqliteDatabase(await importSqliteDatabase(bytes), encoded.root)).toEqual(value);
    const sql = Database.deserialize(bytes); expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]); sql.close();
  }
  for (const value of input.colors) {
    const encoded = await pdfColorSpaceToSqliteDatabase(value);
    const bytes = await exportSqliteDatabase(encoded.database);
    expect(await pdfColorSpaceFromSqliteDatabase(await importSqliteDatabase(bytes), encoded.root)).toEqual(value);
    const sql = Database.deserialize(bytes); expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" }); expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]); sql.close();
  }
});

test("PDF color REAL companions preserve signaling NaN, signed zero and independent finite edits", async () => {
  const input = await cases();
  const encoded = await pdfFunctionToSqliteDatabase(input.functions[1]!);
  const sql = Database.deserialize(await exportSqliteDatabase(encoded.database));
  expect(sql.query("SELECT CAST(exponent_bits AS TEXT) AS bits,exponent_class AS class,exponent IS NULL AS nullQuery FROM pdf_function").get()).toEqual({ bits: BigInt("0x7ff0000000000042").toString(), class: "nan", nullQuery: 1 });
  expect(sql.query("SELECT CAST(value_bits AS TEXT) AS bits FROM pdf_function_real WHERE role='domain' AND ordinal=1").get()).toEqual({ bits: "-9223372036854775808" });
  sql.run("UPDATE pdf_function_real SET value=7,value_bits=4619567317775286272,value_class='finite' WHERE role='domain' AND ordinal=0");
  const edited = await pdfFunctionFromSqliteDatabase(await importSqliteDatabase(sql.serialize()), encoded.root);
  if (edited.kind !== "exponential") throw new Error("Function kind changed"); expect(edited.domain[0]).toEqual({ bits: 0x401c000000000000n }); sql.close();
});

test("PDF function and alternate color traversal is iterative, bounded and rejects extra ownership", async () => {
  const { depth } = await cases();
  let functionValue: PdfFunction = { kind: "array", functions: [] };
  let color: PdfColorSpace = { kind: "deviceRgb" };
  for (let index = 0; index < depth; index++) { functionValue = { kind: "array", functions: [functionValue] }; color = { kind: "pattern", base: color }; }
  const functionEncoded = await pdfFunctionToSqliteDatabase(functionValue);
  let restored = await pdfFunctionFromSqliteDatabase(functionEncoded.database, functionEncoded.root);
  for (let index = 0; index < depth; index++) { if (restored.kind !== "array") throw new Error("Function depth differs"); restored = restored.functions[0]!; }
  expect(restored).toEqual({ kind: "array", functions: [] });
  const encoded = await pdfColorSpaceToSqliteDatabase(color);
  let restoredColor = await pdfColorSpaceFromSqliteDatabase(encoded.database, encoded.root);
  for (let index = 0; index < depth; index++) { if (restoredColor.kind !== "pattern" || !restoredColor.base) throw new Error("Color depth differs"); restoredColor = restoredColor.base; }
  expect(restoredColor).toEqual({ kind: "deviceRgb" });
  await expect(pdfColorSpaceToSqliteDatabase(color, { maxRows: 4 })).rejects.toThrow();
  const cancelled = new AbortController(); cancelled.abort(); await expect(pdfFunctionToSqliteDatabase(functionValue, { signal: cancelled.signal })).rejects.toThrow();
  const cyclic: PdfColorSpace = { kind: "pattern", base: null }; cyclic.base = cyclic; await expect(pdfColorSpaceToSqliteDatabase(cyclic)).rejects.toThrow();
  await expect(pdfColorSpaceFromSqliteDatabase(encoded.database, 1n)).rejects.toThrow();
});

test("PDF function and color native JSON admission uses the same owned Binary64 model", () => {
  expect(parsePdfFunction(pdfFunctionFromNativeJson({ kind: "exponential", domain: [0, -0], range: null, c0: [1], c1: [], n: -0 }))).toEqual({ kind: "exponential", domain: [{ bits: 0n }, { bits: 0x8000000000000000n }], range: null, c0: [{ bits: 0x3ff0000000000000n }], c1: [], n: { bits: 0x8000000000000000n } });
  expect(parsePdfColorSpace(pdfColorFromNativeJson({ kind: "pattern", base: { kind: "calGray", whitePoint: [1, 0, -0], blackPoint: null, gamma: 0 } }))).toEqual({ kind: "pattern", base: { kind: "calGray", whitePoint: [{ bits: 0x3ff0000000000000n }, { bits: 0n }, { bits: 0x8000000000000000n }], blackPoint: null, gamma: { bits: 0n } } });
});
