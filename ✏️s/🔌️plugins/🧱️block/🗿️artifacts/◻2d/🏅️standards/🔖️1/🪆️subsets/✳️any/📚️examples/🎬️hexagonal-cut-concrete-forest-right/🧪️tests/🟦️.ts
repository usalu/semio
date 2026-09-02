import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "../../../../../../../../../../../../🧪️artifact.ts";
const here = dirname(fileURLToPath(import.meta.url));
describe("hexagonal-cut-concrete-forest-right", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../🖼️assets/🗣️hexagonal-cut-concrete-forest-right.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
