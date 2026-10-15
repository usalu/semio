/** 🌍️ The TypeScript side of the compiler's font port (`🦀️.rs`). The Rust crate embeds no font; a browser host serves the
 * five files in {@link COMPILER_FONT_FILES} at {@link COMPILER_FONTS_ROUTE} and the frame Worker hands their bytes to the renderer
 * through {@link installCompilerFonts} before the renderer boots. */

/** 📂️ The font assets' directory, relative to the repository root. */
export const COMPILER_FONTS_DIRECTORY = "🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts";

/** 🌐️ The route a host serves {@link COMPILER_FONTS_DIRECTORY} at. */
export const COMPILER_FONTS_ROUTE = "/renderer-modules/compiler-fonts";

/** 🏷️ `FontRole::ALL` order (math, serif, serif italic, mono, emoji) with the Rust `FontRole::asset_file_name` of each. */
export const COMPILER_FONT_FILES = ["🧮️LibertinusMath-Regular.otf", "📖️LibertinusSerif-Regular.otf", "🖋️LibertinusSerif-Italic.otf", "⌨️LibertinusMono-Regular.otf", "🔤️NotoColorEmoji-subset.ttf"] as const;

/** 🔌️ The renderer's `semioWgpuSetCompilerFonts` door, absent on renderer modules that paint no icons. */
export type CompilerFontBindings = {
  readonly semioWgpuSetCompilerFonts?: (math: Uint8Array, serif: Uint8Array, serifItalic: Uint8Array, mono: Uint8Array, emoji: Uint8Array) => void;
};

/** 📥️ Fetches the five font files relative to `base` and installs them in one call, so the renderer never resolves a math, emoji or
 * text icon without them. Answers `false` without fetching when the bindings carry no door; any failed fetch rejects. */
export async function installCompilerFonts(bindings: CompilerFontBindings, base: string, fetchFont: (url: string) => Promise<Response>): Promise<boolean> {
  const install = bindings.semioWgpuSetCompilerFonts;
  if (!install) return false;
  const [math, serif, serifItalic, mono, emoji] = await Promise.all(COMPILER_FONT_FILES.map(async file => {
    const url = new URL(`${COMPILER_FONTS_ROUTE}/${encodeURIComponent(file)}`, base).href;
    const response = await fetchFont(url);
    if (!response.ok) throw new Error(`compiler-fonts: ${response.status} for ${url}`);
    return new Uint8Array(await response.arrayBuffer());
  }));
  install(math!, serif!, serifItalic!, mono!, emoji!);
  return true;
}
