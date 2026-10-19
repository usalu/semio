//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { AjvJsonSchemaValidator } from "@modelcontextprotocol/sdk/validation/ajv-provider.js";
import { CallToolResultSchema, ListToolsResultSchema } from "@modelcontextprotocol/sdk/types.js";
//#endregion 🔌️Adapters

//#region 🧭️Oracle

type Vector = Readonly<{ name: string; outputSchema?: boolean; expected: Record<string, unknown> }>;
type Fixture = Readonly<{ schema: string; outputSchema: Record<string, unknown>; vectors: readonly Vector[] }>;

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧱️tool-result-shape.json"), "utf8")) as Fixture;

/** 🧱️ The official MCP SDK (Zod `CallToolResultSchema`, AJV output validation) is the third-party oracle for the shared tool-result shape fixture that the Go and Rust twins replay. */
describe("tool result shape fixture against the MCP SDK", () => {
  test("the fixture identity is the wrapped-result revision", () => {
    expect(fixture.schema).toBe("semio.mcp.tool-result-shape/2");
  });

  for (const vector of fixture.vectors) {
    test(`${vector.name}: the expected wire result is a valid CallToolResult`, () => {
      const parsed = CallToolResultSchema.safeParse(vector.expected);
      expect(parsed.success).toBe(true);
      const structured = vector.expected.structuredContent;
      if (structured !== undefined) expect(typeof structured === "object" && structured !== null && !Array.isArray(structured)).toBe(true);
    });
  }

  test("an array or a scalar as structuredContent is refused by the SDK, so it is wrapped", () => {
    for (const raw of [[1, 2], "ok", 0, false, null]) expect(CallToolResultSchema.safeParse({ content: [], structuredContent: raw }).success).toBe(false);
  });

  test("every declared structuredContent validates against the declared outputSchema", () => {
    const validate = new AjvJsonSchemaValidator().getValidator(fixture.outputSchema as never);
    const declared = fixture.vectors.filter((vector) => vector.outputSchema === true && vector.expected.structuredContent !== undefined);
    expect(declared.length).toBeGreaterThan(0);
    for (const vector of declared) expect(validate(vector.expected.structuredContent).valid, vector.name).toBe(true);
  });

  test("a tool listing carrying the declared outputSchema is a valid ListToolsResult", () => {
    const listing = { tools: [{ name: "ticket_open", inputSchema: { type: "object" }, outputSchema: fixture.outputSchema }] };
    expect(ListToolsResultSchema.safeParse(listing).success).toBe(true);
  });
});

//#endregion 🧭️Oracle
