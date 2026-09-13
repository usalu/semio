/** 🧪️ @vitest-environment node */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { vitestRunArguments } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

describe("test command routing", () => {
  it("consumes the Nx test level before forwarding Vitest arguments", async () => {
    const ts = await import("typescript");
    const path = resolve(process.cwd(), "📜️script.ts");
    const syntax = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const scriptClass = syntax.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
    expect(scriptClass && ts.isClassDeclaration(scriptClass)).toBe(true);
    const run = scriptClass && ts.isClassDeclaration(scriptClass) ? scriptClass.members.find((node) => ts.isMethodDeclaration(node) && node.name.getText(syntax) === "run") : undefined;
    const source = run?.getText(syntax) ?? "";
    expect(source).toContain("resolveTestLevel(segments)");
    expect(source).toContain("runVitest(this.root, rest,");
  });

  it("leaves an empty-selection failure to the owning Vitest config", () => {
    const args = vitestRunArguments(process.cwd(), ["../../🧪️tests/🛡️admin/🟦️.tsx"], "../../🧪️tests/🎚️config/🟦️.ts", false);
    expect(args).not.toContain("--passWithNoTests");
    expect(args.at(-1)).toBe("../../🧪️tests/🛡️admin/🟦️.tsx");
  });
});
