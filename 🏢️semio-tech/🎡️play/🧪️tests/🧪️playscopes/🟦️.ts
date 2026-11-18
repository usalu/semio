/** 🔭️ A scoped play browser owner (a pane variant or a group) activates exactly its authored panes' component closure. */
import { describe, expect, it } from "vitest";
import project from "../../📋️project.json";
import fixtures from "./🧫️fixtures/🔣️.json";
import { playRuntimeComponentIds, playRuntimeModuleLayout, playScopeTargets } from "../../🔨️modules/🧩️runtime/🟦️.ts";
import { playE2eServiceV1 } from "../../🔨️modules/🧩️runtime/🧪️e2e/🟦️.ts";

type Target = { dependsOn?: string[]; continuous?: boolean; cache?: boolean; options?: { env?: Record<string, string> } };

describe.each(fixtures)("play scope $scope", fixture => {
  it("selects exactly its panes and their runtime components", () => {
    const targets = playScopeTargets(fixture.scope);
    expect(targets.map(target => target.variant)).toEqual(fixture.variants);
    if ("components" in fixture) expect(playRuntimeComponentIds(targets)).toEqual(fixture.components);
    const layout = playRuntimeModuleLayout(targets);
    expect(layout.pluginModuleDirNames.length + layout.extensionModuleDirNames.length - 2).toBe(playRuntimeComponentIds(targets).length);
  });

  it("owns preparation and an isolated continuous service over its scope", () => {
    const targets = project.targets as Record<string, Target>;
    expect(targets[fixture.test]?.dependsOn).toEqual([fixture.serve]);
    expect(targets[fixture.serve]?.dependsOn).toEqual([fixture.prepare]);
    expect(targets[fixture.serve]?.continuous).toBe(true);
    expect(targets[fixture.prepare]?.dependsOn).toEqual([...fixture.activations, "workspace:deps-browsers"]);
    for (const target of [fixture.test, fixture.prepare, fixture.serve]) {
      expect(targets[target]?.cache).toBe(false);
      expect(targets[target]?.options?.env?.SEMIO_TECH_PLAY_SCOPE).toBe(fixture.scope);
    }
    if ("dev" in fixture) expect(targets[fixture.dev]?.dependsOn).toEqual(fixture.activations);
    const full = playE2eServiceV1("workspace"), scoped = playE2eServiceV1("workspace", fixture.scope);
    expect(scoped.root).not.toBe(full.root);
    expect(scoped.owner).not.toBe(full.owner);
  });
});

describe("play scope", () => {
  it("refuses an unknown scope", () => {
    expect(() => playScopeTargets("unknown")).toThrow();
    expect(() => playE2eServiceV1("workspace", "unknown")).toThrow();
  });
});
