/** 🪪️ The one rule of scene-scoped ids of a model document: `<scene>-<id>`, and when that exceeds the 64 characters of a `Slug` a readable truncation of it plus `-` and a stable 11-digit hash of the full text, so the id is deterministic and distinct ids stay distinct.
 *
 * @see ../../../🧬️schema/🔣️.json — `Slug`, `maxLength` 64
 * @see ../🟦️.ts — `modelOf`, which scopes every solid, material and node of a scene
 * @see ../../📤️export/🗿️model/🧩️scenes/🟦️.ts — `joinScenes`, which scopes the resolved nodes, meshes and skins
 */
import type { Slug } from "../../../🧬️schema/🟦️.ts";

/** 📏️ The longest id of a model document (the `maxLength` of `Slug`). */
export const ID_MAX = 64;

const DIGITS = 11;

function hashOf(text: string): string {
  let h1 = 0xdeadbeef;
  let h2 = 0x41c6ce57;
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i);
    h1 = Math.imul(h1 ^ c, 2654435761);
    h2 = Math.imul(h2 ^ c, 1597334677);
  }
  h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909);
  h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909);
  return (4294967296 * (2097151 & h2) + (h1 >>> 0)).toString(16).padStart(14, "0").slice(0, DIGITS);
}

/** 🪪️ The id `<scene>-<id>` of `id` in `scene`, at most {@link ID_MAX} characters: longer ones keep their first characters (no trailing `-`) and end in `-<hash>` of the full text. */
export function scopedId(scene: Slug, id: Slug): Slug {
  const full = `${scene}-${id}`;
  if (full.length <= ID_MAX) return full;
  const head = full.slice(0, ID_MAX - DIGITS - 1).replace(/-+$/u, "");
  return `${head}-${hashOf(full)}`;
}
