// @vitest-environment node
import { describe, expect, it } from "vitest";
import { menagerieVitePlugin, MENAGERIE_MODULE } from "../../🎯️targets/⚛️react/🏗️builder/🌐️vite/🟦️.ts";
import { parse } from "@babel/parser";

async function documentOf(path?: string): Promise<string> {
  const plugin = menagerieVitePlugin(path);
  const resolve = plugin.resolveId as Function;
  const load = plugin.load as Function;
  return await load(await resolve(MENAGERIE_MODULE));
}

describe("gallery document input", () => {
  it("starts empty and does not import testing examples", async () => {
    const source = await documentOf();
    const module = parse(source, { sourceType: "module" });
    expect(module.program.body.every(row => row.type === "ExportNamedDeclaration" && row.source === null)).toBe(true);
    expect(source).toContain("export const source = null;");
    expect(source).toContain('export const origin = "";');
    const rows = module.program.body.map(row => row.type === "ExportNamedDeclaration" && row.declaration?.type === "VariableDeclaration" ? row.declaration.declarations[0] : null);
    expect(rows[0]?.init?.type).toBe("NullLiteral");
    expect(rows[1]?.init?.type).toBe("StringLiteral");
  });
  it("imports only the explicit caller document", async () => {
    const path = "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";
    const source = await documentOf(path);
    const module = parse(source, { sourceType: "module" });
    expect(module.program.body[0].type).toBe("ExportNamedDeclaration");
    expect((module.program.body[0] as { source: { value: string } }).source.value.endsWith(path)).toBe(true);
    expect(source).toContain(`export const origin = ${JSON.stringify(path)};`);
  });
});
