import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));

describe("example 🏗️nakagin-capsule-tower", () => {
  it("ships a non-empty dsl asset", () => {
    const text = readFileSync(join(here, "../🖼️assets/🗣️tower.dsl.semio"), "utf8");
    expect(text.length).toBeGreaterThan(64);
    expect(text.startsWith("semio ")).toBe(true);
  });

  it("ships nonempty op/pack/spr assets", () => {
    expect(readFileSync(join(here, "../🖼️assets/🔧️tower.op.semio"), "utf8").length).toBeGreaterThan(64);
    expect(readFileSync(join(here, "../🖼️assets/🎒️tower.pack.semio")).byteLength).toBeGreaterThan(64);
    expect(readFileSync(join(here, "../🖼️assets/📡️tower.spr.semio")).byteLength).toBeGreaterThan(64);
  });
});
