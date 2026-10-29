/** 🔭️ Level of detail: shapes above the scale's level and textures at 1:500 are omitted.
 * @see ./🥒️.feature */
import { describe, expect, it } from "vitest";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { rolesState } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const ids = (denominator: number): string[] =>
  depict(rolesState(), defaultDepictionOptions("light-solid", "en", denominator)).layers.flatMap((l) => l.items.flatMap((i) => (i.kind === "path" && i.id.startsWith("shape:") ? [i.id] : [])));

describe("level of detail", () => {
  it("omits the texture at 1:500", () => expect(ids(500)).not.toContain("shape:texture"));
  it("keeps the texture at 1:200", () => expect(ids(200)).toContain("shape:texture"));
  it("adds lod 1 details from 1:200 and lod 3 details only at 1:50", () => {
    expect(ids(500)).not.toContain("shape:detail");
    expect(ids(200)).toContain("shape:detail");
    expect(ids(100)).not.toContain("shape:fine-detail");
    expect(ids(50)).toContain("shape:fine-detail");
  });
});
