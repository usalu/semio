import { describe, expect, test } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { checkStylingArtifacts, renderStylingArtifacts } from "../../📽️projection/🟦️.ts";

/** 🧪️ The ignored generated styling sources equal the byte plan the checked-in token and theme sources render today.
 * @see ../../📦️packages/🦀️rust/📋️project.json `generate` */
describe("generated styling sources", () => {
  test("every artifact on disk equals its fresh render", () => {
    expect(() => checkStylingArtifacts()).not.toThrow();
  });

  test("the Rust token source carries the fields of the theme component", () => {
    const rust = renderStylingArtifacts().find((artifact) => artifact.path.endsWith(join("🔤️tokens", "🦀️.rs")))!;
    expect(readFileSync(rust.path, "utf8")).toBe(rust.content);
    for (const field of ["chip_row_height", "error_ink", "warning_ink", "success_ink"]) expect(rust.content).toContain(field);
  });
});
