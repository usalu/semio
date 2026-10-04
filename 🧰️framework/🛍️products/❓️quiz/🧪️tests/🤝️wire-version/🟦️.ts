/** 🤝️ Subject adapter of the wire-version case: `WIRE_VERSION` of `@semio-tech/quiz` and the fingerprint of the schema
 * without its prose, written by the TypeScript runtime's own JSON writer.
 *
 * @see ./🥒️.feature
 * @see ../../🧬️schema/🟦️.ts
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { WIRE_VERSION } from "../../📦️packages/🟦️typescript/🟦️.ts";

const SCHEMA = ["🧰️framework", "🛍️products", "❓️quiz", "🧬️schema", "🔣️.json"] as const;
const PROSE = new Set(["description", "title", "$comment"]);

/** ✍️ `node` without its prose as JSON with sorted keys and no whitespace. */
function canonical(node: unknown): string {
  if (Array.isArray(node)) return `[${node.map(canonical).join(",")}]`;
  if (node === null || typeof node !== "object") return JSON.stringify(node);
  const members = Object.entries(node).filter(([key, value]) => !(PROSE.has(key) && typeof value === "string"));
  return `{${members
    .sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0))
    .map(([key, value]) => `${JSON.stringify(key)}:${canonical(value)}`)
    .join(",")}}`;
}

/** 🔢️ FNV-1a 64 of `bytes` as 16 lowercase hex digits. */
function fnv1a64(bytes: Uint8Array): string {
  let hashed = 0xcbf29ce484222325n;
  for (const byte of bytes) hashed = ((hashed ^ BigInt(byte)) * 0x100000001b3n) & 0xffffffffffffffffn;
  return hashed.toString(16).padStart(16, "0");
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    agreement: {
      subject: (ctx) => ({ projection: { wireVersion: WIRE_VERSION, fingerprint: fnv1a64(new TextEncoder().encode(canonical(JSON.parse(readFileSync(join(ctx.repoRoot, ...SCHEMA), "utf8"))))) } }),
    },
  },
});
