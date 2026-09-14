//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📚️Reference
/**
 * 📚️ A third implementation of the ranking function, written from `🧬️schema/🔣️.json` and the feature
 * file using nothing but the Node standard library. Registered as `cross-semio-implementation`: it
 * raises the cost of a shared misreading, it is never independent evidence.
 */

/** 📐️ Levenshtein distance over UTF-8 bytes, abandoned as soon as it is provably above the budget. */
function boundedDistance(left: string, right: string, maximum: number): number {
  if (left === right) return 0;
  const leftBytes = new TextEncoder().encode(left);
  const rightBytes = new TextEncoder().encode(right);
  if (Math.abs(leftBytes.length - rightBytes.length) > maximum) return maximum + 1;
  let previous = Array.from({ length: rightBytes.length + 1 }, (_unused, index) => index);
  for (let row = 1; row <= leftBytes.length; row += 1) {
    const current = new Array<number>(rightBytes.length + 1).fill(0);
    current[0] = row;
    let minimum = current[0];
    for (let column = 1; column <= rightBytes.length; column += 1) {
      const cost = leftBytes[row - 1] === rightBytes[column - 1] ? 0 : 1;
      current[column] = Math.min(previous[column] + 1, current[column - 1] + 1, previous[column - 1] + cost);
      minimum = Math.min(minimum, current[column]);
    }
    if (minimum > maximum) return maximum + 1;
    previous = current;
  }
  return previous[rightBytes.length];
}

/** ✂️ Words are the runs of ASCII letters, ASCII digits and non-ASCII characters. */
function words(text: string): string[] {
  const lowered = text.toLowerCase();
  const parts: string[] = [];
  let current = "";
  for (const character of lowered) {
    const code = character.codePointAt(0) ?? 0;
    const kept = (character >= "a" && character <= "z") || (character >= "0" && character <= "9") || code >= 0x80;
    if (kept) current += character;
    else if (current !== "") {
      parts.push(current);
      current = "";
    }
  }
  if (current !== "") parts.push(current);
  return parts;
}

/** 🏅️ The conjunction score, or null when one term does not match. */
function score(text: string, terms: { term: string; fuzziness: number }[]): number | null {
  const lowered = text.toLowerCase();
  const tokens = words(text);
  let total = 0;
  for (const query of terms) {
    const term = query.term.toLowerCase();
    let best = query.fuzziness + 1;
    if (lowered.includes(term)) best = 0;
    for (const token of tokens) {
      const distance = boundedDistance(term, token, query.fuzziness);
      if (distance < best) best = distance;
    }
    if (best > query.fuzziness) return null;
    total += query.fuzziness - best + 1;
  }
  return total;
}

type Vector = {
  name: string;
  documents: Record<string, string>;
  deleted?: string[];
  reindexed?: Record<string, string>;
  terms: { term: string; fuzziness: number }[];
  size: number;
};

/** 🔎️ Indexes, mutates and ranks one vector, and renders it the way every implementation reports it. */
function rank(vector: Vector): string {
  const documents = new Map<string, string>();
  for (const id of Object.keys(vector.documents).sort()) documents.set(id, vector.documents[id].trim());
  for (const id of vector.deleted ?? []) documents.delete(id);
  for (const id of Object.keys(vector.reindexed ?? {}).sort()) documents.set(id, (vector.reindexed ?? {})[id].trim());
  const hits: { id: string; score: number }[] = [];
  for (const id of [...documents.keys()].sort()) {
    const value = score(documents.get(id) ?? "", vector.terms);
    if (value !== null) hits.push({ id, score: value });
  }
  hits.sort((left, right) => (left.score === right.score ? (left.id < right.id ? -1 : left.id > right.id ? 1 : 0) : right.score - left.score));
  const total = hits.length;
  const page = vector.size >= 0 ? hits.slice(0, vector.size) : hits;
  return `${vector.name}=${total}:${page.map((hit) => `${hit.id}@${hit.score}`).join(",")}`;
}
//#endregion 📚️Reference

//#region 🧭️Adapter
/** 🟦️ The TypeScript reference the Go and Rust subjects are compared against. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "vectors-rank-the-same-way": {
      oracle: (ctx) => {
        const file = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://📡️ranking-vectors.json"))) as { vectors: Vector[] };
        return { projection: { rankings: file.vectors.map(rank) } };
      },
    },
  },
});
//#endregion 🧭️Adapter
