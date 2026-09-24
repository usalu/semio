/** @emoji 🧪️ Canvas presence registry + overlay reduction smoke tests. */
import { describe, expect, it } from "vitest";
import {
  clearLocalPresenceWindowViewV1,
  collectLocalPresenceWindowViewsV1,
  collectLocalActiveToolV1,
  publishArtifactPresenceRosterV1,
  publishLocalActiveToolV1,
  publishLocalPresenceWindowViewV1,
  publishLocalPresenceActorV1,
  localPresenceActorV1,
  artifactPresenceRosterV1,
} from "../../🟦️.ts";
import { peersForWindow, canvasPointToScreen, orbitPointToScreen } from "@semio-tech/framework-replication";

describe("👕️canvas-presence registry", () => {
  it("collects throttled local window views per runtime key", () => {
    publishLocalPresenceWindowViewV1("doc-a", "w1", {
      windowId: "w1",
      space: "canvas",
      kind: { kind: "canvas", x: 1, y: 2, zoom: 1 },
      size: [100, 100],
      pointer: [3, 4, 0],
    });
    publishLocalPresenceWindowViewV1("doc-b", "w2", {
      windowId: "w2",
      space: "world",
      kind: { kind: "orbit", position: [0, 0, 1], target: [0, 0, 0], up: [0, 1, 0], fov: 45 },
      size: [200, 200],
    });
    expect(collectLocalPresenceWindowViewsV1("doc-a")).toHaveLength(1);
    expect(collectLocalPresenceWindowViewsV1("doc-a")[0]?.windowId).toBe("w1");
    clearLocalPresenceWindowViewV1("w1");
    expect(collectLocalPresenceWindowViewsV1("doc-a")).toHaveLength(0);
    clearLocalPresenceWindowViewV1("w2");
  });

  it("stores artifact roster for host overlays", () => {
    publishArtifactPresenceRosterV1("doc-a", [
      {
        actor: "peer-a",
        connectedAtMs: 1,
        views: [{ windowId: "w1", space: "canvas", kind: { kind: "canvas", x: 0, y: 0, zoom: 1 }, size: [10, 10], pointer: [5, 5, 0] }],
        interaction: { app_id: "drawing", domains: [{ domain: "layer", granularity: "stroke", selected: ["s1"], hovered: [] }] },
        color: 2,
        label: "Ada",
        activeTool: "brush",
      },
    ]);
    const roster = artifactPresenceRosterV1("doc-a");
    expect(roster).toHaveLength(1);
    const spec = peersForWindow(roster, "w1", "canvas", undefined, "self", 0);
    expect(spec.artifactPeers[0]?.actor).toBe("peer-a");
    expect(spec.artifactPeers[0]?.activeTool).toBe("brush");
    const screen = canvasPointToScreen({ x: 0, y: 0, zoom: 1 }, [100, 100], [5, 5]);
    expect(screen[0]).toBe(55);
    expect(screen[1]).toBe(55);
  });

  it("publishes active tool for heartbeat collection (bit 12)", () => {
    publishLocalActiveToolV1("doc-a", "brush");
    expect(collectLocalActiveToolV1("doc-a")).toBe("brush");
    expect(collectLocalActiveToolV1("local")).toBeUndefined();
    publishLocalActiveToolV1("local", "select");
    expect(collectLocalActiveToolV1("missing")).toBe("select");
    publishLocalActiveToolV1("doc-a", null);
    publishLocalActiveToolV1("local", null);
    expect(collectLocalActiveToolV1("doc-a")).toBeUndefined();
  });

  it("publishes hub-admitted actor so overlays exclude self", () => {
    publishLocalPresenceActorV1("doc-a", "actor-self");
    expect(localPresenceActorV1("doc-a")).toBe("actor-self");
    expect(localPresenceActorV1("local")).toBe("actor-self");
    publishArtifactPresenceRosterV1("doc-a", [
      {
        actor: "actor-self",
        connectedAtMs: 1,
        views: [{ windowId: "w1", space: "canvas", kind: { kind: "canvas", x: 0, y: 0, zoom: 1 }, size: [10, 10], pointer: [1, 1, 0] }],
      },
      {
        actor: "peer-b",
        connectedAtMs: 2,
        views: [{ windowId: "w1", space: "canvas", kind: { kind: "canvas", x: 0, y: 0, zoom: 1 }, size: [10, 10], pointer: [2, 2, 0] }],
      },
    ]);
    const roster = artifactPresenceRosterV1("doc-a");
    const selfActor = localPresenceActorV1("doc-a");
    expect(selfActor).toBe("actor-self");
    if (selfActor === null) throw new Error("expected hub-admitted local actor");
    const spec = peersForWindow(roster, "w1", "canvas", undefined, selfActor, 0);
    expect(spec.artifactPeers.map((peer) => peer.actor)).toEqual(["peer-b"]);
    publishLocalPresenceActorV1("doc-a", null);
    expect(localPresenceActorV1("doc-a")).toBeNull();
  });


  it("rejects __local__ placeholder so self-exclusion waits for hub actor", () => {
    publishLocalPresenceActorV1("doc-a", "__local__");
    expect(localPresenceActorV1("doc-a")).toBeNull();
    publishLocalPresenceActorV1("doc-a", "actor-real");
    expect(localPresenceActorV1("doc-a")).toBe("actor-real");
    publishLocalPresenceActorV1("doc-a", null);
    expect(localPresenceActorV1("doc-a")).toBeNull();
  });

  it("projects orbit world hit through local camera for 3D peer cursors", () => {
    const localOrbit = {
      position: [0, 0, 5] as const,
      target: [0, 0, 0] as const,
      up: [0, 1, 0] as const,
      fov: 45,
    };
    const screen = orbitPointToScreen(localOrbit, [1024, 768], [0.5, 0.25, 0]);
    expect(screen).not.toBeNull();
    expect(screen![0]).toBeGreaterThan(0);
    expect(screen![1]).toBeGreaterThan(0);
    publishLocalPresenceWindowViewV1("doc-a", "world-main", {
      windowId: "world-main",
      space: "world",
      kind: { kind: "orbit", position: [2, 3, 4], target: [0, 0, 0], up: [0, 1, 0], fov: 50 },
      size: [800, 600],
      pointer: [0.5, 0.25, 0],
      rayOrigin: [2, 3, 4],
    });
    const views = collectLocalPresenceWindowViewsV1("doc-a");
    expect(views[0]?.rayOrigin).toEqual([2, 3, 4]);
    clearLocalPresenceWindowViewV1("world-main");
  });

});
