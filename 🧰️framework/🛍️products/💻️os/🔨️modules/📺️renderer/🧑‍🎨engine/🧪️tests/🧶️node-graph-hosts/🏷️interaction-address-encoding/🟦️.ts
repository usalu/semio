/** 🏷️ The scene-owned interaction address, world3d target and pack byte encoding tests. */
import { expect, it } from "vitest";
import { nodeGraphHoverActionArgs, nodeGraphSelectionActionArgs, world3dHoverActionArgs, world3dSelectionActionArgs, interactionTargetsForInstances, WORLD3D_DEFAULT_INTERACTION_GRANULARITY, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dMarkerInteractionTarget, world3dInstanceInteractionTarget, world3dInstanceInteractionTargets, world3dSelectionTargetsActionArgs, sceneToSyncPack } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 📮 Registers the node graph and world3d interaction addresses, scene record targets, vortex marker args, instance collapse and pack byte tests. */
export function registerInteractionAddressEncodingTests(): void {
  it("encodes node graph selection and hover with its scene-owned framework interaction address", () => {
    const domain = { id: "graph", nodeTargetPrefix: "document.node.", edgeTargetPrefix: "document.edge.", handleTargetPrefix: "document.handle." };
    expect(nodeGraphSelectionActionArgs(domain, { nodeIds: ["node-a"], edgeIds: ["edge-a"], handleIds: ["handle-a"] })).toEqual({
      domainId: "graph",
      targets: JSON.stringify([
        { granularity: "node", id: "document.node.node-a" },
        { granularity: "edge", id: "document.edge.edge-a" },
        { granularity: "handle", id: "document.handle.handle-a" },
      ]),
      merge: "replace",
      method: "pick",
    });
    expect(nodeGraphHoverActionArgs(domain, "node-a")).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "node", id: "document.node.node-a" }]),
    });
    expect(nodeGraphSelectionActionArgs(undefined, { nodeIds: ["node-a"] })).toBeUndefined();
    expect(nodeGraphHoverActionArgs(undefined, "node-a")).toBeUndefined();
  });
  it("encodes world3d interaction dispatch args the same way the node graph does", () => {
    expect(WORLD3D_DEFAULT_INTERACTION_GRANULARITY).toBe("handle");
    expect(world3dHoverActionArgs("graph", "handle", "extrude@solid")).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "handle", id: "extrude@solid" }]),
    });
    expect(world3dHoverActionArgs("graph", "handle", null)).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([]),
    });
    expect(world3dSelectionActionArgs("graph", "handle", ["extrude@solid"], "replace")).toEqual({
      domainId: "graph",
      targets: JSON.stringify([{ granularity: "handle", id: "extrude@solid" }]),
      merge: "replace",
      method: "pick",
    });
  });
  it("resolves world3d marker interaction targets from the scene record, not from host knowledge", () => {
    expect(WORLD3D_DEFAULT_MARKER_GRANULARITY).toEqual({ vortex: "vortex", attraction: "attraction", targetVolume: "targetVolume", reference: "reference" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0")).toEqual({ granularity: "vortex", id: "seed-left-001:v0" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0", {})).toEqual({ granularity: "vortex", id: "seed-left-001:v0" });
    expect(world3dMarkerInteractionTarget("targetVolume", "volume-1")).toEqual({ granularity: "targetVolume", id: "volume-1" });
    expect(world3dMarkerInteractionTarget("reference", "ref-1")).toEqual({ granularity: "reference", id: "ref-1" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0", { interactionGranularityId: "pin", interactionId: "pin-7" })).toEqual({ granularity: "pin", id: "pin-7" });
  });
  it("resolves world3d instance interaction targets per record, falling back to the scene granularity", () => {
    const instances = [{ id: "n1" }, { id: "e1", interactionGranularityId: "element" }, { id: "l2:head", interactionId: "l2", interactionGranularityId: "load" }, { id: "l2:shaft", interactionId: "l2", interactionGranularityId: "load" }];
    expect(world3dInstanceInteractionTarget(instances, "n1", "node")).toEqual({ granularity: "node", id: "n1" });
    expect(world3dInstanceInteractionTarget(instances, "e1", "node")).toEqual({ granularity: "element", id: "e1" });
    expect(world3dInstanceInteractionTarget(instances, "l2:shaft", "node")).toEqual({ granularity: "load", id: "l2" });
    expect(world3dInstanceInteractionTarget(instances, "ghost", "node")).toEqual({ granularity: "node", id: "ghost" });
    expect(world3dInstanceInteractionTargets(instances, ["l2:head", "e1", "l2:shaft", "n1"], "node")).toEqual([
      { granularity: "load", id: "l2" },
      { granularity: "element", id: "e1" },
      { granularity: "node", id: "n1" },
    ]);
    expect(world3dSelectionTargetsActionArgs("fem3d", world3dInstanceInteractionTargets(instances, ["e1", "n1"], "node"), "replace", "rectangle")).toEqual({
      domainId: "fem3d",
      targets: JSON.stringify([
        { granularity: "element", id: "e1" },
        { granularity: "node", id: "n1" },
      ]),
      merge: "replace",
      method: "rectangle",
    });
    expect(world3dSelectionTargetsActionArgs("fem3d", [{ granularity: "node", id: "n1" }], "additive")).toEqual({
      domainId: "fem3d",
      targets: JSON.stringify([{ granularity: "node", id: "n1" }]),
      merge: "additive",
      method: "pick",
    });
  });
  it("encodes a vortex marker pick and hover as generic domain interaction args", () => {
    const target = world3dMarkerInteractionTarget("vortex", "seed-left-001:v0");
    expect(world3dSelectionActionArgs("vortex", target.granularity, [target.id], "invertive")).toEqual({
      domainId: "vortex",
      targets: JSON.stringify([{ granularity: "vortex", id: "seed-left-001:v0" }]),
      merge: "invertive",
      method: "pick",
    });
    expect(world3dHoverActionArgs("vortex", target.granularity, target.id)).toEqual({
      domainId: "vortex",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "vortex", id: "seed-left-001:v0" }]),
    });
    expect(world3dHoverActionArgs("vortex", WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, undefined)).toEqual({
      domainId: "vortex",
      channel: "pointer",
      targets: JSON.stringify([]),
    });
  });
  it("collapses world instance ids onto the interaction targets they stand for", () => {
    const instances = [{ id: "profile@wire#0", interactionId: "profile@wire" }, { id: "profile@wire#1", interactionId: "profile@wire" }, { id: "extrude@solid#0", interactionId: "extrude@solid" }, { id: "plain-instance" }];
    expect(interactionTargetsForInstances(instances, ["profile@wire#0", "profile@wire#1", "extrude@solid#0"])).toEqual(["profile@wire", "extrude@solid"]);
    expect(interactionTargetsForInstances(instances, ["plain-instance"])).toEqual(["plain-instance"]);
    expect(interactionTargetsForInstances(instances, ["unknown-id"])).toEqual(["unknown-id"]);
  });
  it("encodes node graph scenes as pack bytes for wasm sync", async () => {
    const scene = {
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
    };
    const bytes = sceneToSyncPack(scene);
    expect(bytes.length).toBeGreaterThan(8);
  });
}
