//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📏️Segmentation
/** 📏️ UAX #29 extended grapheme clusters, from the ICU data the runtime ships. */
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });

/** 😀️ A grapheme counts as an emoji when it carries a pictographic base, a regional indicator or a keycap. */
function isEmojiGrapheme(grapheme: string): boolean {
  return /\p{Extended_Pictographic}/u.test(grapheme) || /\p{Regional_Indicator}/u.test(grapheme) || grapheme.includes("⃣");
}

/** 🧲️ Splits the leading emoji grapheme off, or reports none. */
function splitLeadingEmoji(value: string): [string, string] {
  if (value === "") return ["", ""];
  const first = segmenter.segment(value)[Symbol.iterator]().next().value as { segment: string } | undefined;
  if (first === undefined) return ["", value];
  if (!isEmojiGrapheme(first.segment)) return ["", value];
  return [first.segment, value.slice(first.segment.length)];
}
//#endregion 📏️Segmentation

//#region 🧭️Adapter
/**
 * 🟦️ Unicode decides where the leading emoji grapheme ends; the owned rune scanner is judged against
 * it. Everything else this module does is a repository convention with no reference, so no handler is
 * registered for the second scenario.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "the-leading-emoji-grapheme-is-unicodes": {
      oracle: (ctx) => {
        const file = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://📡️emoji-vectors.json"))) as { graphemes: { name: string; input: string }[] };
        return {
          projection: {
            graphemes: file.graphemes.map((vector) => {
              const [emoji, remaining] = splitLeadingEmoji(vector.input);
              return `${vector.name}=${emoji}|${remaining}`;
            }),
          },
        };
      },
    },
  },
});
//#endregion 🧭️Adapter
