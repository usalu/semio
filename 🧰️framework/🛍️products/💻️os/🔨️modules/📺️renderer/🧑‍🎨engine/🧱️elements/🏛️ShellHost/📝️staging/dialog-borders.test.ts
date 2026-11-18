import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const source = readFileSync(resolve(import.meta.dirname, "🟦️.tsx"), "utf8");

describe("staged action dialog borders", () => {
  it("keeps the action row flush with the dialog edges instead of inset in a padded box", () => {
    expect(source).toContain("bg-menu p-0 text-foreground");
    expect(source).toContain('<SharedBorderRow edges={["top"]} className="w-full">');
    expect(source).not.toMatch(/<SharedBorderRow[^>]*border-t/u);
    expect(source).not.toContain("ml-auto w-fit");
  });
});
