/** 🎨️ Materializes the canonical print paints for the native LaTeX renderer. */
import { join } from "node:path";
import { writeGeneratedFileIfChanged } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { renderPrintLatexTokenStylesheet } from "./🧮️resolution/🟦️.ts";
export * from "./🧮️resolution/🟦️.ts";
const workspaceRoot = findWorkspaceRoot();
const productRoot = join(workspaceRoot, "🧰️framework", "🛍️products", "📓️print");
const latexDirectory = join(productRoot, "🖋️latex");
const latexTokensPath = join(latexDirectory, "semio-tokens.sty");

/** 🎨️ Writes the generated LaTeX design-token stylesheet. */
export function writePrintLatexTokenStylesheet(): void {
  writeGeneratedFileIfChanged(latexTokensPath,renderPrintLatexTokenStylesheet());
}

