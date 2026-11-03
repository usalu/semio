/** 🏷️ The browser-safe content revision agrees with `node:crypto` and with the shared vectors.
 * @see ../../🔨️modules/🏷️revision/🟦️.ts
 * @see ../../🧫️fixtures/🏷️revision/🔣️.json */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { canonicalJson, revisionOfText, sha256Hex } from "../../🔨️modules/🏷️revision/🟦️.ts";

const vectors = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/🏷️revision/🔣️.json"), "utf8")) as {
  sha256: { text: string; hex: string }[];
  revision: { text: string; same: string }[];
  canonical: { value: unknown; text: string }[];
};
const reference = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");

describe("🏷️ content revision", () => {
  it("matches the published vectors", () => {
    for (const vector of vectors.sha256) expect(sha256Hex(new TextEncoder().encode(vector.text))).toBe(vector.hex);
  });

  it("matches node:crypto for every length around the block and padding borders", () => {
    for (const length of [0, 1, 54, 55, 56, 57, 63, 64, 65, 119, 120, 127, 128, 129, 1000, 65537]) {
      const bytes = Uint8Array.from({ length }, (_, index) => (index * 131 + length) & 255);
      expect(sha256Hex(bytes)).toBe(reference(bytes));
    }
  });

  it("agrees with node:crypto on a whole catalog file", () => {
    const file = resolve(import.meta.dirname, "../../🗂️catalog/🌤️environments/🔣️.json");
    const bytes = readFileSync(file);
    expect(sha256Hex(bytes)).toBe(reference(bytes));
  });

  it("is the same for LF and CRLF checkouts", () => {
    for (const { text, same } of vectors.revision) {
      expect(revisionOfText(text)).toBe(revisionOfText(same));
      expect(revisionOfText(text)).toBe(`sha256:${reference(new TextEncoder().encode(same))}`);
    }
  });

  it("writes canonical JSON with sorted keys and kept non-ASCII text", () => {
    for (const { value, text } of vectors.canonical) expect(canonicalJson(value)).toBe(text);
  });
});
