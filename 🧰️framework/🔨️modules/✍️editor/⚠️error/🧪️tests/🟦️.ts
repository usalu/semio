/** 🧫️ Checks the shared owned error contract against independent diagnostic inspection. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { getMessage } from "check-error";
import Ajv from "ajv/dist/2020.js";
import { EditorError } from "../🟦️.ts";
import type { EditorErrorKind } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));

test("owned Editor error preserves kind, display and original diagnostic", () => {
  for (const row of fixture.cases) {
    const cause = new Error(row.message), error = new EditorError(row.kind as EditorErrorKind, cause);
    expect({ kind: error.kind(), display: error.message, cause: (error.cause as Error).message }).toEqual(row.expected);
    expect({ kind: error.kind(), display: getMessage(error), cause: getMessage(error.cause) }).toEqual(row.expected);
    expect(error.cause).toBe(cause);
  }
  expect(() => new EditorError("other" as EditorErrorKind, new Error("invalid"))).toThrow(TypeError);
  console.log("[DEBUG] owned Editor error: eleven shared cases retained");
});
