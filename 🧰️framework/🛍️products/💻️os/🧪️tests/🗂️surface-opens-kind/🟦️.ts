/** 🗂️ The browser's surface ↔ artifact-kind pairing rule (`surfaceOpensArtifactKindV1`), replayed from the hub's own
 * fixture `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🗂️surface-opens-kind/🔣️.json` — the same cases
 * the hub's `app_opens_kind` answers, so the two implementations cannot drift again (ticket 26/09/23 S15). */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { surfaceOpensArtifactKindV1, type SurfaceArtifactKindV1 } from "../../🔨️modules/📇️directory/🧬️schema/🟦️.ts";

type SurfaceOpensKindCase = {
  readonly name: string;
  readonly pluginArtifactKinds: readonly SurfaceArtifactKindV1[];
  readonly app: { readonly artifactKinds: readonly SurfaceArtifactKindV1[]; readonly dialectArtifactKind: string };
  readonly artifact: { readonly kind: string; readonly schema: string };
  readonly opens: boolean;
};
const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🗂️surface-opens-kind/🔣️.json", import.meta.url)), "utf8")) as { readonly cases: readonly SurfaceOpensKindCase[] };

describe("🗂️ surface opens artifact kind", () => {
  it("replays at least the declaration-tree, plugin-level and sibling-dialect cases", () => {
    expect(fixture.cases.length).toBeGreaterThanOrEqual(6);
  });
  for (const testCase of fixture.cases) {
    it(testCase.name, () => {
      expect(surfaceOpensArtifactKindV1(testCase.pluginArtifactKinds, testCase.app, testCase.artifact)).toBe(testCase.opens);
    });
  }
});
