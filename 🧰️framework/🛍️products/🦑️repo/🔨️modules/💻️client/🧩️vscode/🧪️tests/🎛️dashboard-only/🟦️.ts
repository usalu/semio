import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { runInNewContext } from "node:vm";
import Ajv from "ajv/dist/2020.js";
import { parse } from "@babel/parser";
import ts from "typescript";

const root = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🎛️dashboard-only/🔣️.json"), "utf8"));

/** 🎛️ Collects executable command identifiers through the TypeScript syntax tree. */
function typescriptEvidence(source: string): { commands: string[]; symbols: string[] } {
  const commands = new Set<string>(), symbols = new Set<string>();
  const tree = ts.createSourceFile("extension.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const visit = (node: ts.Node): void => {
    if (ts.isStringLiteral(node) && fixture.forbiddenCommands.includes(node.text)) commands.add(node.text);
    if (ts.isIdentifier(node) && fixture.forbiddenSymbols.includes(node.text)) symbols.add(node.text);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  return { commands: [...commands].sort(), symbols: [...symbols].sort() };
}

/** 🔬️ Uses an independent Babel parser to verify the same portable action evidence. */
function babelEvidence(source: string): { commands: string[]; symbols: string[] } {
  const commands = new Set<string>(), symbols = new Set<string>();
  const visit = (value: unknown): void => {
    if (!value || typeof value !== "object") return;
    const node = value as Record<string, unknown>;
    if (node.type === "StringLiteral" && fixture.forbiddenCommands.includes(node.value)) commands.add(String(node.value));
    if (node.type === "Identifier" && fixture.forbiddenSymbols.includes(node.name)) symbols.add(String(node.name));
    for (const item of Object.values(node)) if (Array.isArray(item)) item.forEach(visit); else if (item && typeof item === "object") visit(item);
  };
  visit(parse(source, { sourceType: "module", plugins: ["typescript"] }));
  return { commands: [...commands].sort(), symbols: [...symbols].sort() };
}

describe("IDE dashboard ownership", () => {
  test("validates portable cases with TypeScript and an independent Babel oracle", () => {
    const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🎛️dashboard-only/🔣️.json"), "utf8"));
    const validate = new Ajv().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of fixture.cases) {
      const expected = { commands: row.forbiddenCommands, symbols: row.forbiddenSymbols };
      expect(typescriptEvidence(row.source), row.id).toEqual(expected);
      expect(babelEvidence(row.source), row.id).toEqual(expected);
    }
  });

  test("dissolves every IDE operation handler and action lens", () => {
    const owners = ["🟦️.ts", "⌨️commands/🟦️.ts", "🔎️code-lens/🟦️.ts", "🎞️constants/🟦️.ts", "⚙️data-models/🟦️.ts", "🎩️globals/🟦️.ts", "🎼️utilities/🟦️.ts", "🏬️uri-resolution/🟦️.ts", "🗺️file-analysis/🟦️.ts", "🌳️tree-providers/🟦️.ts", "⌛️queries/🟦️.ts"];
    for (const owner of owners) {
      const source = readFileSync(resolve(root, owner), "utf8");
      expect(typescriptEvidence(source), owner).toEqual({ commands: [], symbols: [] });
      expect(babelEvidence(source), owner).toEqual({ commands: [], symbols: [] });
    }
    const commands = readFileSync(resolve(root, "⌨️commands/🟦️.ts"), "utf8");
    for (const command of fixture.retainedCommands) expect(commands).toContain(`register("${command}"`);
  });

  test("exposes navigation commands without operational menus or keybindings", () => {
    const manifest = JSON.parse(readFileSync(resolve(root, "📦️packages/🟦️typescript/package.json"), "utf8"));
    const commands = manifest.contributes.commands.map((row: { command: string }) => row.command);
    const bindings = Object.values(manifest.contributes.menus).flat().concat(manifest.contributes.keybindings);
    for (const command of fixture.forbiddenCommands) {
      expect(commands).not.toContain(command);
      expect(bindings.some((row: any) => row.command === command)).toBe(false);
    }
    for (const command of fixture.retainedCommands) expect(commands).toContain(command);
  });

  test("registers only passive commands at runtime and preserves source navigation", () => {
    const source = readFileSync(resolve(root, "⌨️commands/🟦️.ts"), "utf8"), tree = ts.createSourceFile("extension.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const registration = tree.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "registerCommands")!;
    const handlers = new Map<string, (...args: any[]) => unknown>(), opened: unknown[][] = [];
    const environment = {
      vscode: { commands: { registerCommand: (name: string, handler: (...args: any[]) => unknown) => { handlers.set(name, handler); return {}; } } },
      loadAvailableFilterValues: () => undefined,
      openFileAtLine: (...args: unknown[]) => opened.push(args),
    };
    const code = ts.transpileModule(`${registration.getText(tree).replace(/^export\s+/u, "")}\nregisterCommands({ subscriptions: [] });`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    runInNewContext(code, environment, { timeout: 5000 });
    for (const command of fixture.forbiddenCommands) expect(handlers.has(command), command).toBe(false);
    for (const command of fixture.retainedCommands) expect(handlers.has(command), command).toBe(true);
    handlers.get("compose.navigateToDefinition")!({ filePath: "source.ts", definition: { range: { start: 4, end: 6 } } });
    expect(opened).toEqual([["source.ts", 4, 6]]);
    console.log("[DEBUG] IDE registration runtime: repository operations absent; source navigation retained.");
  });
});
