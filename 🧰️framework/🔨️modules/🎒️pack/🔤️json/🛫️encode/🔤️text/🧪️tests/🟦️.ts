/** 🔤️ JSON.stringify independently certifies fragmented native key and scalar text bytes. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const quoted = (chunks: string[]) => {
  const output = [34];
  for (const chunk of chunks) for (const byte of new TextEncoder().encode(chunk)) {
    if (byte === 34 || byte === 92) output.push(92, byte);
    else if (byte === 8) output.push(92, 98);
    else if (byte === 9) output.push(92, 116);
    else if (byte === 10) output.push(92, 110);
    else if (byte === 12) output.push(92, 102);
    else if (byte === 13) output.push(92, 114);
    else if (byte < 32) output.push(...new TextEncoder().encode(`\\u${byte.toString(16).padStart(4, "0")}`));
    else output.push(byte);
  }
  output.push(34);
  return output;
};

test("canonical native UTF8 scalar and key roles preserve exact JSON across chunk and grant boundaries", () => {
  const law = read("../🧫️fixtures/🔣️.json");
  for (const row of law.cases) {
    const bytes = [123, ...quoted(row.keyChunks), 58, ...quoted(row.valueChunks), 125];
    const oracle = new TextEncoder().encode(JSON.stringify({ [row.keyChunks.join("")]: row.valueChunks.join("") }));
    for (const grant of law.copyGrants) {
      const output: number[] = [];
      for (let offset = 0; offset < bytes.length; offset += grant) {
        const chunk = bytes.slice(offset, offset + grant);
        expect(chunk.length).toBeLessThanOrEqual(grant);
        output.push(...chunk);
      }
      expect(new Uint8Array(output)).toEqual(oracle);
    }
  }
  console.log("[DEBUG] JSON.stringify native text/key parity preserves Unicode/NUL/escaping across 1/2/7-byte grants and physical text chunks");
  const native = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(native.includes("pub enum ArtifactCanonicalJsonText")).toBe(true);
  expect(native.includes("text.text_chunk(")).toBe(true);
  expect(native.includes("to_string_owner")).toBe(false);
  expect(native.includes("pub struct ArtifactCanonicalJsonTextCursor")).toBe(true);
  expect(native.includes("RetainedCloneGrant")).toBe(true);
});
