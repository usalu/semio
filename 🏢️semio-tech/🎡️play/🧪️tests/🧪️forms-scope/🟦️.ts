/** 📋️ The Forms browser owner activates exactly its authored component closure. */
import { describe, expect, it } from "vitest";
import project from "../../📋️project.json";
import fixture from "./🧫️fixtures/🔣️.json";
import { PLAY_RUNTIME_TARGETS, playRuntimeComponentIds, playRuntimeModuleLayout } from "../../🔨️modules/🧩️runtime/🟦️.ts";
import { playE2eServiceV1 } from "../../🔨️modules/🧩️runtime/🧪️e2e/🟦️.ts";

describe("Forms acceptance scope", () => {
  it("selects the three Forms runtime components", () => {
    const targets = PLAY_RUNTIME_TARGETS.filter(target => target.variant === fixture.pane);
    expect(targets).toHaveLength(1);
    expect(playRuntimeComponentIds(targets)).toEqual(fixture.components);
    const layout = playRuntimeModuleLayout(targets);
    expect(layout.pluginModuleDirNames.length + layout.extensionModuleDirNames.length - 2).toBe(fixture.components.length);
  });

  it("owns preparation and an isolated continuous service", () => {
    const targets = project.targets as Record<string, { dependsOn?: string[]; continuous?: boolean; cache?: boolean }>;
    expect(targets[fixture.test]?.dependsOn).toEqual([fixture.serve]);
    expect(targets[fixture.serve]?.dependsOn).toEqual([fixture.prepare]);
    expect(targets[fixture.serve]?.continuous).toBe(true);
    expect(targets[fixture.prepare]?.dependsOn).toEqual([fixture.activation, "workspace:deps-browsers"]);
    for (const target of [fixture.test, fixture.prepare, fixture.serve]) expect(targets[target]?.cache).toBe(false);
    const full = playE2eServiceV1("workspace"), scoped = playE2eServiceV1("workspace", fixture.pane);
    expect(scoped.root).not.toBe(full.root);
    expect(scoped.owner).not.toBe(full.owner);
    expect(() => playE2eServiceV1("workspace", "unknown")).toThrow();
  });
});
