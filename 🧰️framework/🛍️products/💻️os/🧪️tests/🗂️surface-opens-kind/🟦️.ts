/** 🗂️ The browser's surface ↔ artifact-kind pairing rule (`surfaceOpensArtifactKindV1`), replayed from the hub's own
 * fixtures: `🗂️surface-opens-kind` (the rule's cases, viewers of an editor's dialect among them) and
 * `🎯️descriptor-open-targets` (the open targets the hub's `app_opens_kind` publishes for one descriptor, which the hub's
 * unit laws replay too), so the two implementations cannot drift again (ticket 26/09/23 S15, C13).
 * @see ../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { surfaceOpensArtifactKindV1, type SurfaceArtifactKindV1, type SurfaceKindAppV1 } from "../../🔨️modules/📇️directory/🧬️schema/🟦️.ts";

type SurfaceOpensKindCase = {
  readonly name: string;
  readonly pluginArtifactKinds: readonly SurfaceArtifactKindV1[];
  readonly hostedArtifactKinds: readonly SurfaceArtifactKindV1[];
  readonly apps: readonly SurfaceKindAppV1[];
  readonly app: number;
  readonly artifact: { readonly kind: string; readonly schema: string };
  readonly opens: boolean;
};
type DescriptorOpenTargetsCase = {
  readonly name: string;
  readonly kindId: string;
  readonly declaredOn: "plugin" | "editor" | "plugin-and-editor" | "hosted" | "hosted-presented";
  readonly execution: "isolated" | "linked";
  readonly targets: readonly { readonly role: "editor" | "viewer"; readonly surfaceId: string; readonly write: boolean }[];
};
const fixturesDirectory = "../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures";
const read = <T,>(name: string): T => JSON.parse(readFileSync(fileURLToPath(new URL(`${fixturesDirectory}${name}/🔣️.json`, import.meta.url)), "utf8")) as T;
const pairing = read<{ readonly cases: readonly SurfaceOpensKindCase[] }>("🗂️surface-opens-kind");
const openTargets = read<{ readonly cases: readonly DescriptorOpenTargetsCase[] }>("🎯️descriptor-open-targets");

describe("🗂️ surface opens artifact kind", () => {
  it("replays the declaration-tree, plugin-level, sibling-dialect, viewer and hosted cases", () => {
    expect(pairing.cases.length).toBeGreaterThanOrEqual(19);
    expect(pairing.cases.some((testCase) => testCase.hostedArtifactKinds.length > 0 && testCase.opens)).toBe(true);
    expect(pairing.cases.some((testCase) => testCase.apps[testCase.app]!.role === "viewer" && testCase.opens)).toBe(true);
  });
  for (const testCase of pairing.cases) {
    it(testCase.name, () => {
      expect(surfaceOpensArtifactKindV1(testCase.pluginArtifactKinds, testCase.hostedArtifactKinds, testCase.apps, testCase.apps[testCase.app]!, testCase.artifact)).toBe(testCase.opens);
    });
  }
});

describe("🎯️ the hub's descriptor open targets", () => {
  const dialect = { artifactKind: "s.fixture.document", standard: "1", subset: "*" };
  const schema = "fixture.document@1";
  for (const testCase of openTargets.cases) {
    it(testCase.name, () => {
      const kind: SurfaceArtifactKindV1 = { id: testCase.kindId, schema };
      const hosted = testCase.declaredOn === "hosted" || testCase.declaredOn === "hosted-presented";
      const pluginArtifactKinds = testCase.declaredOn === "plugin" || testCase.declaredOn === "plugin-and-editor" ? [kind] : [];
      const hostedArtifactKinds = hosted ? [kind] : [];
      const apps: SurfaceKindAppV1[] = [
        { role: "editor", dialect, artifactKinds: testCase.declaredOn === "editor" || testCase.declaredOn === "plugin-and-editor" ? [kind] : [], presents: testCase.declaredOn === "hosted-presented" ? kind : null },
        { role: "viewer", dialect, artifactKinds: [], presents: null },
      ];
      const opened = testCase.execution !== "isolated" ? [] : apps.filter((app) => surfaceOpensArtifactKindV1(pluginArtifactKinds, hostedArtifactKinds, apps, app, { kind: kind.id, schema })).map((app) => `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}#${app.role}`);
      expect(opened).toEqual(testCase.targets.map((target) => target.surfaceId));
    });
  }
});
