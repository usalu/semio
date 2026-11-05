/** 🎟️ Exact contributor leases route child controls while keeping parent parameters in their owner. */
import { describe, expect, it } from "vitest";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/🧩️external-slots/🔣️.json";
import { bindContributorActionsV1, resolveExternalSlots, retireContributorInstances, type ContributorInstance, type ExternalSlotResolverContext } from "../../🧩️extensions/🟦️.ts";
import type { BuiltNode } from "../../../🛂️manifest/🟦️.ts";

const node = (key: string, component: unknown, children: BuiltNode[] = [], bindings: unknown[] = []): BuiltNode => ({ key, component, children, bindings }) as BuiltNode;
const binding = (scope: string, name: string) => ({ trigger: "activate", action: { scope, name, version: 1 }, args: null, capability: null });

describe("contributor action leases", () => {
  it("matches independent JSON Patch and preserves parent parameter bindings", () => {
    const token = `contributor:${JSON.stringify([fixture.pluginId, fixture.actions.instanceId])}`;
    const body = node("body", { type: "text", value: "Run" }, [node("parameter", { type: "text", value: "Height" }, [], [binding(fixture.actions.parentController, fixture.actions.parent)])], [binding(fixture.controllerId, fixture.actions.child)]);
    const expected = applyPatch(structuredClone(body), [{ op: "replace", path: "/bindings/0/action/scope", value: token }]).newDocument;
    expect(bindContributorActionsV1(body, fixture.controllerId, token)).toEqual(expected);
    expect(body.bindings[0]!.action.scope).toBe(fixture.controllerId);
    expect(bindContributorActionsV1(body.children[0]!, fixture.controllerId, token)).toBe(body.children[0]);
  });

  it("isolates duplicate child controllers, forwards effects and retires only the removed slot", async () => {
    let created = 0;
    const retired: number[] = [], effects: unknown[] = [];
    const handle = {
      manifest: { apps: [{ id: fixture.appId, controllerId: fixture.controllerId, defaultModeId: fixture.modeId, modes: [{ id: fixture.modeId }], windowKinds: [{ id: fixture.windowKindId, bodyKey: fixture.bodyKey }] }] },
      createApp: async () => ++created,
      destroyApp: async (id: number) => { retired.push(id); },
      refreshUi: async () => ({ windows: [{ key: "extension", hash: "body", value: node("run", { type: "text", value: "Run" }, [], [binding(fixture.controllerId, fixture.actions.child)]) }], requestedEffects: [{ notify: { message: "Ready" } }] }),
    };
    const instances = new Map<string, ContributorInstance>();
    const context = { plugins: new Map([[fixture.pluginId, handle]]), contributorInstances: instances, contributorPasses: new Map(), viewState: fixture.viewState, ownerId: fixture.ownerId, onEffects: (...args: unknown[]) => { effects.push(args); } } as ExternalSlotResolverContext;
    const slots = fixture.slots.map(slot => node(slot.key, { type: "extension", extension: `${fixture.pluginId}/${fixture.appId}`, props: slot.props }));
    const output = await resolveExternalSlots(node("root", { type: "text", value: "Parent" }, slots), context);
    const leases = [...instances.values()];
    expect(leases).toHaveLength(2);
    expect(leases.map(lease => lease.instanceId)).toEqual([1, 2]);
    expect(leases.map(lease => lease.viewState?.extensionInputJson)).toEqual(fixture.slots.map(slot => JSON.stringify(slot.props)));
    expect(output.children.map(slot => slot.children[0]!.bindings[0]!.action.scope)).toEqual(leases.map(lease => lease.dispatchControllerId));
    expect(new Set(leases.map(lease => lease.dispatchControllerId)).size).toBe(2);
    expect(effects).toHaveLength(2);
    await resolveExternalSlots(node("root", { type: "text", value: "Parent" }, [slots[1]!]), context);
    expect(retired).toEqual([1]);
    expect([...instances.values()][0]).toBe(leases[1]);
    await retireContributorInstances(instances);
    expect(retired).toEqual([1, 2]);
  });
});
