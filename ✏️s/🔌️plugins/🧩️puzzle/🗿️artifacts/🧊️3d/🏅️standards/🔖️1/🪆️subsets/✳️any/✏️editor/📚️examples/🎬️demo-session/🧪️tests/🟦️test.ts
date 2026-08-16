import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));

describe("example 🎬️demo-session", () => {
  it("ships a non-empty cmd demo script", () => {
    const text = readFileSync(join(here, "../🖼️assets/🎮️demo.cmd.semio"), "utf8");
    expect(text.length).toBeGreaterThan(64);
    expect(text).toContain("setActiveExample");
  });
});
