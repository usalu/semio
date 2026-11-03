/** 🎭️ Browser stand-in for the Node cast exporter. The site never writes a cast folder; the real exporter stays on disk for the command line.
 * @see ../../../🔨️modules/📤️export/🎭️cast/🟦️.ts — the Node exporter this replaces in the dev server
 */
export async function exportCast(): Promise<never> {
  throw new Error("Cast export runs in Node, not in the site.");
}
