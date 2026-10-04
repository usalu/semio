/** 🎨️ Materializes the canonical print paints for the native LaTeX renderer. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { renderPrintLatexTokenStylesheet } from "./🧮️resolution/🟦️.ts";
export * from "./🧮️resolution/🟦️.ts";
const workspaceRoot = getWorkspaceRoot();
const productRoot = join(workspaceRoot, "🧰️framework", "🛍️products", "📓️print");
const latexDirectory = join(productRoot, "🖋️latex");
const latexTokensPath = join(latexDirectory, "semio-tokens.sty");

/** 🎨️ Writes the generated LaTeX design-token stylesheet. */
export function writePrintLatexTokenStylesheet(): void {
  mkdirSync(latexDirectory, { recursive: true });
  writeFileSync(latexTokensPath, renderPrintLatexTokenStylesheet(), "utf8");
}

