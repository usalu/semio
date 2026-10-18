/** 🧩️ Every form a schema export can take renders as a GraphQL declaration, a protobuf message and a TypeScript type, and the rendered text is accepted by the independent GraphQL, protobuf and TypeScript parsers. The cases and their expected text are the language-agnostic corpus next to this file.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧪️tests/🧩️rendered-forms/🔣️.json
 */
import { expect, test } from "bun:test";
import { parse } from "graphql";
import protobuf from "protobufjs";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import cases from "./🔣️.json";
import { renderGraphql, renderProto, renderTsType } from "../../📐️type-render/🟦️.ts";

type Row = { readonly name: string; readonly definition: Record<string, unknown>; readonly taken?: readonly string[]; readonly graphql: string | null; readonly protobuf: string | null; readonly typescript: string | null };
const rows = cases.cases as unknown as readonly Row[];

const PRELUDE = "interface Base { readonly id: string }\ninterface Pt { readonly x: number }\n";

const typeErrors = async (text: string): Promise<string[]> => {
  const ts = await import("typescript");
  const directory = mkdtempSync(join(tmpdir(), "semio-rendered-forms-"));
  try {
    const file = join(directory, "row.ts");
    writeFileSync(file, `${PRELUDE}${text}`);
    const program = ts.createProgram([file], { strict: true, noEmit: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler });
    return ts.getPreEmitDiagnostics(program).map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
};

for (const row of rows) {
  const taken = (name: string): boolean => (row.taken ?? []).includes(name);
  test(`GraphQL: ${row.name}`, () => {
    const text = renderGraphql("Row", row.definition, taken) ?? null;
    expect(text).toBe(row.graphql);
    if (text === null) return;
    const document = parse(text);
    const names = document.definitions.map((definition) => ("name" in definition ? definition.name?.value : undefined));
    expect(new Set(names).size, "a declaration is made twice").toBe(names.length);
  });

  test(`protobuf: ${row.name}`, () => {
    const text = renderProto("Row", row.definition, taken) ?? null;
    expect(text).toBe(row.protobuf);
    if (text === null) return;
    const root = protobuf.parse(`syntax = "proto3";\n${text}`).root;
    expect(root.nested?.Row, "the export is a message of its own name").toBeDefined();
  });

  test(`TypeScript: ${row.name}`, async () => {
    const text = renderTsType("Row", row.definition) ?? null;
    expect(text).toBe(row.typescript);
    if (text === null) return;
    expect(await typeErrors(text)).toEqual([]);
  });
}
