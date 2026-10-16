/** 🌍️ The TypeScript side of the compiler's font port (`🦀️.rs`). The Rust crate embeds no font: the five files in
 * {@link COMPILER_FONT_FILES} live in {@link COMPILER_FONTS_DIRECTORY}, and a host delivers them through Canvas's runtime asset
 * port (`🖼️canvas/🔌️runtime-assets/🟦️.ts`), which declares the route they are served at. */

/** 📂️ The font assets' directory, relative to the repository root. */
export const COMPILER_FONTS_DIRECTORY = "🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts";

/** 🏷️ `FontRole::ALL` order (math, serif, serif italic, mono, emoji) with the Rust `FontRole::asset_file_name` of each. */
export const COMPILER_FONT_FILES = ["🧮️LibertinusMath-Regular.otf", "📖️LibertinusSerif-Regular.otf", "🖋️LibertinusSerif-Italic.otf", "⌨️LibertinusMono-Regular.otf", "🔤️NotoColorEmoji-subset.ttf"] as const;
