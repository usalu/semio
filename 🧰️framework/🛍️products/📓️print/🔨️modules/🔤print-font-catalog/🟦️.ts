import { copyFileSync, existsSync, mkdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

//#region 🔤️PrintFontCatalog
export type PrintFontDescriptor = {
  readonly family: string;
  readonly directory: string;
  readonly filename: string;
  readonly texFilename: string;
  readonly sourceUrl: string;
};

export type PrintFontProvisioning = {
  readonly downloaded: number;
  readonly total: number;
};

const printProductRoot = join(getWorkspaceRoot(), "🧰️framework", "🛍️products", "📓️print");
const fontRoot = join(printProductRoot, "🖼️assets", "🔤️font");
const texFontRoot = join(getWorkspaceRoot(), ".🧬semio", "🦑️repo", "⚡️cache", "print-fonts");
const PRINT_FONT_DESCRIPTORS: readonly PrintFontDescriptor[] = [
  { family: "Anta", directory: "🅰️anta", filename: "🅰️Anta-Regular.ttf", texFilename: "Anta-Regular.ttf", sourceUrl: "https://raw.githubusercontent.com/google/fonts/main/ofl/anta/Anta-Regular.ttf" },
  { family: "Share Tech Mono", directory: "🖥️share-tech-mono", filename: "💻️ShareTechMono-Regular.ttf", texFilename: "ShareTechMono-Regular.ttf", sourceUrl: "https://raw.githubusercontent.com/google/fonts/main/ofl/sharetechmono/ShareTechMono-Regular.ttf" },
  { family: "Noto Emoji", directory: "😀️noto-emoji", filename: "😀️NotoEmoji-Regular.ttf", texFilename: "NotoEmoji-Regular.ttf", sourceUrl: "https://raw.githubusercontent.com/google/fonts/main/ofl/notoemoji/NotoEmoji%5Bwght%5D.ttf" },
];

/** 🔤️ Lists the canonical print-font descriptors. */
export function printFontDescriptors(): readonly PrintFontDescriptor[] {
  return PRINT_FONT_DESCRIPTORS;
}

/** 🔤️ Resolves deterministic local TTF search paths for the print compiler. */
export function printFontSearchPaths(): readonly string[] {
  return [texFontRoot];
}

/** 🔤️ Ensures every canonical print font is available in the local catalog. */
export async function provisionPrintFonts(): Promise<PrintFontProvisioning> {
  let downloaded = 0;
  mkdirSync(texFontRoot, { recursive: true });
  for (const font of PRINT_FONT_DESCRIPTORS) {
    const directory = join(fontRoot, font.directory);
    const destination = join(directory, font.filename);
    mkdirSync(directory, { recursive: true });
    if (!existsSync(destination)) {
      const response = await fetch(font.sourceUrl);
      if (!response.ok) throw new Error(`Font download failed for ${font.family}: ${response.status}`);
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (bytes.length < 4 || bytes[0] !== 0x00 || bytes[1] !== 0x01 || bytes[2] !== 0x00 || bytes[3] !== 0x00) throw new Error(`Downloaded bytes for ${font.family} are not TTF (got ${bytes.length} bytes)`);
      writeFileSync(destination, bytes);
      downloaded += 1;
    }
    const texDestination = join(texFontRoot, font.texFilename);
    if (!existsSync(texDestination) || statSync(texDestination).size !== statSync(destination).size) copyFileSync(destination, texDestination);
  }
  return { downloaded, total: PRINT_FONT_DESCRIPTORS.length };
}
//#endregion 🔤️PrintFontCatalog
