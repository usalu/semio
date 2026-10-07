import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolveEmbeddedWgpuBoot } from "../../🎯️targets/🧊️wgpu/🎬️renderer-boot/🧩️selection/🟦️.ts";
import { resolveWgpuBootDescriptor, type WgpuBootOverrides } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";

interface SelectionCase { readonly id: string; readonly input: { readonly modules: readonly { readonly pluginId: string }[]; readonly baseUrl: string; readonly hash?: string; readonly overrides?: WgpuBootOverrides }; readonly expected: { readonly variant: string; readonly selection: "all" | "variant" } }

const fixtures: readonly SelectionCase[] = JSON.parse(readFileSync(new URL("../../🧬️schema/🧪️tests/🧩️embedded-selection/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));

describe("Explicit embedded contribution selection", () => {
  it("preserves contribution and override inputs during selection", () => {
    const before = JSON.parse(JSON.stringify(fixtures));
    for (const row of fixtures) resolveEmbeddedWgpuBoot(row.input);
    expect(fixtures).toEqual(before);
  });
  for (const row of fixtures) it(row.id, () => {
    const result = resolveEmbeddedWgpuBoot(row.input);
    expect(result.descriptor.pluginVariant).toBe(row.expected.variant);
    expect(result.pluginRegistrySelection).toBe(row.expected.selection);
    expect(result.plugins.map(module => module.pluginId)).toEqual(row.input.modules.map(module => module.pluginId));
    expect(result.descriptor).toEqual(resolveWgpuBootDescriptor({ hash: row.input.hash, defaultVariant: row.expected.variant, overrides: row.input.overrides }));
  });
  it("withdraws the final contribution without retaining a prior variant", () => {
    const populated = fixtures.find(row => row.id === "one");
    const withdrawal = fixtures.find(row => row.id === "withdrawal");
    if (!populated || !withdrawal) throw new Error("Closed transition cases are required");
    expect(resolveEmbeddedWgpuBoot(populated.input).descriptor.pluginVariant).toBe(populated.expected.variant);
    const result = resolveEmbeddedWgpuBoot(withdrawal.input);
    expect(result.plugins).toEqual([]);
    expect(result.descriptor.pluginVariant).toBe("");
    expect(result.pluginRegistrySelection).toBe("all");
  });
});
