import { describe, expect, test } from "vitest";
import vector from "./🔣️.json" with { type: "json" };

/** 🧭️ Independent oracle: JavaScript strings are UTF-16, so the LSP character of a byte offset is the length of the decoded prefix on its line. */
function lspPosition(text: string, byteOffset: number): { line: number; character: number } {
  const prefix = new TextDecoder().decode(new TextEncoder().encode(text).subarray(0, byteOffset));
  const rows = prefix.split(/\r\n|\r|\n/);
  return { line: rows.length - 1, character: rows[rows.length - 1]!.length };
}

describe("jack lsp positions against the JavaScript UTF-16 oracle", () => {
  for (const row of vector.cases.filter(row => row.oracle)) {
    test(row.name, () => {
      expect(lspPosition(row.text, row.offset)).toEqual({ line: row.line, character: row.character });
    });
  }
});
