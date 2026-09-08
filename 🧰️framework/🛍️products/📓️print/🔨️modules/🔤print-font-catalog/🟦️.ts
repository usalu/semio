import { readFileSync } from "node:fs";
import { join, basename } from "node:path";
import { getWorkspaceRoot } from "../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { stageArtifacts } from "../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";

export type PrintFontDescriptor = { readonly family: string; readonly directory: string; readonly filename: string; readonly texFilename: string };
const product = "🧰️framework/🛍️products/📓️print";
const fonts: readonly PrintFontDescriptor[] = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));

/** 🔤️ Lists authored print fonts and their compiler filenames. */
export function printFontDescriptors(): readonly PrintFontDescriptor[] { return fonts; }

/** 📦️ Resolves the font producer's owned compiler inputs. */
export function printFontSearchPaths(workspace = getWorkspaceRoot()): readonly string[] { return [join(workspace, product, "📦️packages/🟦️typescript/dist/fonts")]; }

/** 🔤️ Stages tracked TTF sources without acquiring or mutating source assets. */
export function stagePrintFonts(workspace = getWorkspaceRoot()): { readonly total: number } {
  const files = new Map<string, string>();
  for (const font of fonts) {
    for (const value of [font.directory, font.filename, font.texFilename]) if (basename(value) !== value || /[\\/]/.test(value) || value === "." || value === "..") throw new Error(`Invalid print font path: ${value}`);
    const source = join(workspace, product, "🖼️assets/🔤️font", font.directory, font.filename);
    const bytes = readFileSync(source);
    if (bytes.length < 12 || bytes.readUInt32BE(0) !== 0x00010000) throw new Error(`Print font is not TTF: ${source}`);
    if (files.has(font.texFilename)) throw new Error(`Duplicate print font: ${font.texFilename}`);
    files.set(font.texFilename, source);
  }
  stageArtifacts(printFontSearchPaths(workspace)[0]!, "@semio-tech/print:fonts", files);
  return { total: files.size };
}
