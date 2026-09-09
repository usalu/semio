import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
const here = dirname(fileURLToPath(import.meta.url));
describe("bestest-950", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../../../../🖼️assets/🏛️bestest-950/🗣️.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
