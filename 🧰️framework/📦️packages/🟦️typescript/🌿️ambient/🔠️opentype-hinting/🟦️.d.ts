/** 🔠️ Declares the `hinting` switch of opentype.js `Glyph.getPath` / `Font.getPath`.
 *
 * `opentype.js` reads `options.hinting` (glyph.js: `if (options.hinting && font && font.hinting)`), but `@types/opentype.js`
 * leaves it out of `RenderOptions`. Independent font oracles turn it off to compare unhinted outlines.
 * @see https://opentype.js.org/ */
import type {} from "opentype.js";

declare module "opentype.js" {
  interface RenderOptions {
    hinting?: boolean | undefined;
  }
}
