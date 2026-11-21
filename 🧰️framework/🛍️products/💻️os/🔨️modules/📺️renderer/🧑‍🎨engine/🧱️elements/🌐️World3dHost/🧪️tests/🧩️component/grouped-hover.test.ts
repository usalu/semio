import { describe, expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🎯️grouped-hover.json";
import { world3dInstanceHoverTargetsJson, world3dInstanceInteractionTarget } from "../../🔗️instance-selection/🟦️.ts";
import { world3dHoverActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";

describe("world instance hover targets", () => {
  for (const { name, defaultGranularity, instance, expected } of fixture.cases) {
    test(name, () => {
      const [x = 0, y = 0, z = 0] = instance.position ?? [];
      const record = { ...instance, position: [x, y, z] as const };
      const primary = world3dInstanceInteractionTarget([record], record.id, defaultGranularity);
      const authored = world3dInstanceHoverTargetsJson(record);
      const args = world3dHoverActionArgs("procedural3d", primary.granularity, primary.id, authored);
      expect(JSON.parse(args.targets)).toEqual(expected);
      expect(JSON.parse(world3dHoverActionArgs("procedural3d", primary.granularity, null, authored).targets)).toEqual([]);
    });
  }

  test("bounds the UTF-8 identifier and escaped payload", () => {
    const { idBytes, payloadBytes, oversizedUniqueTargets, oversizedTargetText } = fixture.limits;
    const maxId = "ä".repeat(idBytes / 2);
    const valid = { id: "object", hoverTargets: [{ granularity: "handle", id: maxId }] };
    expect(JSON.parse(world3dInstanceHoverTargetsJson(valid) ?? "null")).toEqual(valid.hoverTargets);
    expect(world3dInstanceHoverTargetsJson({ ...valid, hoverTargets: [{ granularity: "handle", id: maxId + "x" }] })).toBeUndefined();
    const targets = Array.from({ length: oversizedUniqueTargets }, (_, index) => ({ granularity: "handle", id: oversizedTargetText + index }));
    expect(new TextEncoder().encode(JSON.stringify(targets)).length).toBeGreaterThan(payloadBytes);
    expect(world3dInstanceHoverTargetsJson({ id: "object", hoverTargets: targets })).toBeUndefined();
  });
});
