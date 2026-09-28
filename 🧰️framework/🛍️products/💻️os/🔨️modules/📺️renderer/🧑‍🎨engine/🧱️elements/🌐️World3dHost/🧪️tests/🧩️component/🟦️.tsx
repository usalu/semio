/** @emoji 🎬️ A World3d window mounts its scene's `toolRunTrace` lane for real in jsdom: the lane pages of
 * `🧫️fixtures/⏯️tool-run-trace-mount.json` travel as base64url lane text into `World3dHost`, which must mount one
 * live `THREE.InstancedMesh` per resident `(mesh, verdict)` carrying exactly the fixture's instance counts,
 * draw each through `meshesJson[mesh]`, publish the `data-tool-run-*` counters and paint provisional instances
 * with the dashed provisional outline. three.js is the oracle for instance matrices (`Matrix4.compose`).
 * Only the WebGL seam is replaced, exactly as in `🧪️tests/🖱️world3d-interaction`. */
import { cleanup, render } from "@semio-tech/ui-react/test";
import { createElement, type ReactNode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Matrix4, Quaternion, Vector3, type InstancedMesh, type PerspectiveCamera } from "three";
import fixture from "../../🧫️fixtures/⏯️tool-run-trace-mount.json" with { type: "json" };
import { base64UrlEncode } from "../../../../../../../../../🔨️modules/🚪️io/🔤️base64/🟦️.ts";
import { encodeToolRunTraceDelta, toolRunIdentityFromJson, toolRunTraceOpFromJson, ToolRunTraceStore, type ToolRunTraceCursor, type ToolRunTraceSubject } from "../../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";

const recorded = vi.hoisted(() => ({ instanced: [] as { mesh: InstancedMesh; live: boolean }[], dashed: 0, frames: [] as ((state: unknown, delta: number) => void)[], camera: null as PerspectiveCamera | null }));

vi.mock("three", async (importOriginal) => {
  const actual = (await importOriginal()) as typeof import("three");
  class RecordingInstancedMesh extends actual.InstancedMesh {
    constructor(...args: ConstructorParameters<typeof actual.InstancedMesh>) {
      super(...args);
      const entry = { mesh: this as InstancedMesh, live: true };
      recorded.instanced.push(entry);
      (this.material as import("three").Material).addEventListener("dispose", () => (entry.live = false));
    }
  }
  class RecordingLineDashedMaterial extends actual.LineDashedMaterial {
    constructor(...args: ConstructorParameters<typeof actual.LineDashedMaterial>) {
      super(...args);
      recorded.dashed += 1;
    }
  }
  return { ...actual, InstancedMesh: RecordingInstancedMesh, LineDashedMaterial: RecordingLineDashedMaterial };
});

vi.mock("@react-three/fiber", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const three = await import("three");
  const camera = new three.PerspectiveCamera(45, 1, 0.1, 1000);
  camera.position.set(10, -10, 10);
  camera.lookAt(0, 0, 0);
  camera.updateMatrixWorld(true);
  recorded.camera = camera;
  const state = { camera, gl: { domElement: { clientWidth: 800, clientHeight: 800 } }, scene: new three.Scene(), size: { width: 800, height: 800 }, raycaster: new three.Raycaster(), invalidate: () => {}, clock: { elapsedTime: 0 } };
  return {
    ...actual,
    useFrame: (callback: (value: typeof state, delta: number) => void) => void recorded.frames.push(callback as never),
    useThree: (selector?: (value: typeof state) => unknown) => (selector ? selector(state) : state),
    useLoader: () => null,
  };
});

vi.mock("@semio-tech/infinite-world-r3f", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const passthrough = ({ children }: { children?: ReactNode }) => createElement("div", null, children);
  return {
    ...actual,
    WorldCanvas: ({ children, overlay }: { children?: ReactNode; overlay?: ReactNode }) => createElement("div", { "data-testid": "world-canvas" }, overlay, children),
    WorldOrbitGated: () => null,
    WorldOrbitViewControls: () => null,
    WorldProjectionRig: () => null,
    WorldOrbitViewSnapGateProvider: passthrough,
    WorldLodBridge: passthrough,
    WorldVolumeLayer: () => null,
    WorldReferenceLayer: () => null,
  };
});

import { setRuntimeDiagnostics } from "../../../🏛️ShellHost/🟦️.tsx";
import { WindowInstanceIdContext, World3dHost } from "../../🟦️.tsx";

type Batch = { readonly mesh: number; readonly verdict: string; readonly count: number };

function sceneNode(toolRunTrace: string | null) {
  return createElement(
    WindowInstanceIdContext.Provider,
    { value: "trace-window:1" },
    createElement(World3dHost as never, {
      node: {
        type: "componentScene",
        surfaceId: "trace-window",
        controllerId: "trace-app",
        componentKind: "world-3d",
        world3d: {
          cameraJson: JSON.stringify({ position: [10, -10, 10], target: [0, 0, 0], zoom: 1, fov: 45 }),
          meshesJson: JSON.stringify(fixture.meshes),
          instancesJson: JSON.stringify(fixture.instances),
          selectionJson: "{}",
          referencesJson: JSON.stringify([{ id: "forest-plan", url: "/forest.png", origin: [7, 0, 0.01], widthWorld: 50 }]),
          vorticesJson: "[]",
          attractionsJson: "[]",
          lodJson: JSON.stringify({ automaticLod: true, gridFactor: 10 }),
          interactionJson: '{"activeUtility":"select"}',
          toolRunTrace,
        },
      },
      onAction: () => Promise.resolve(),
    }),
  );
}

function runFrames(): void {
  for (const frame of recorded.frames.splice(0)) frame({ clock: { elapsedTime: 0 } }, 0);
}

function mountedBatches(ledger: ToolRunTraceStore): Batch[] {
  const counts = new Map<string, Batch>();
  const matrix = new Matrix4();
  for (const { mesh } of recorded.instanced.filter((entry) => entry.live && entry.mesh.count > 0)) {
    const kinds = new Set<string>();
    for (let at = 0; at < mesh.count; at += 1) {
      mesh.getMatrixAt(at, matrix);
      const hit = [...ledger.records()].find(([, record]) => {
        if (record.subject.kind !== "instance3d") return false;
        const subject = record.subject as Extract<ToolRunTraceSubject, { kind: "instance3d" }>;
        const oracle = new Matrix4().compose(new Vector3(...subject.position), new Quaternion(...subject.rotation), new Vector3(subject.scale, subject.scale, subject.scale));
        return oracle.elements.every((value, index) => Math.abs(value - matrix.elements[index]!) < 1e-5);
      });
      expect(hit, `instance ${at} of a live batch matches no resident record`).toBeTruthy();
      const subject = hit![1].subject as Extract<ToolRunTraceSubject, { kind: "instance3d" }>;
      expect(mesh.geometry.getAttribute("position").count, "[DEBUG] a batch must draw through meshesJson[mesh]").toBe(fixture.meshes[subject.mesh]!.data.positions.length / 3);
      kinds.add(`${subject.mesh}:${hit![1].verdict}`);
    }
    expect(kinds.size, "one instanced mesh carries exactly one (mesh, verdict)").toBe(1);
    const [key] = [...kinds];
    const [mesh0, verdict] = key!.split(":");
    const prior = counts.get(key!)?.count ?? 0;
    expect(prior, `${key} is mounted twice`).toBe(0);
    counts.set(key!, { mesh: Number(mesh0), verdict: verdict!, count: mesh.count });
  }
  return [...counts.values()].sort((a, b) => a.mesh - b.mesh || a.verdict.localeCompare(b.verdict));
}

const sortBatches = (batches: readonly Batch[]) => [...batches].sort((a, b) => a.mesh - b.mesh || a.verdict.localeCompare(b.verdict));

describe("🎬️ world 3d host tool run trace mount", () => {
  afterEach(() => {
    cleanup();
    recorded.instanced.length = 0;
    recorded.frames.length = 0;
    recorded.dashed = 0;
    recorded.camera?.position.set(10, -10, 10);
    setRuntimeDiagnostics(undefined);
    vi.restoreAllMocks();
  });

  it("prints one gated accepted-frame receipt and only repeats when the live camera changes", () => {
    const info = vi.spyOn(console, "info").mockImplementation(() => undefined);
    setRuntimeDiagnostics(false);
    render(sceneNode(null));
    const runMountedFrames = () => {
      for (const frame of [...recorded.frames]) frame({ clock: { elapsedTime: 0 } }, 0);
    };
    runMountedFrames();
    expect(info).not.toHaveBeenCalled();

    setRuntimeDiagnostics(true);
    runMountedFrames();
    expect(info).toHaveBeenCalledTimes(1);
    const prefix = "[DEBUG] react-world-frame ";
    const line = info.mock.calls[0]![0] as string;
    expect(line.startsWith(prefix)).toBe(true);
    const receipt = JSON.parse(line.slice(prefix.length)) as Record<string, any>;
    expect(receipt).toMatchObject({
      surfaceId: "trace-window",
      windowInstanceId: "trace-window:1",
      viewport: { width: 800, height: 800, aspect: 1 },
      references: [{ id: "forest-plan", origin: [7, 0, 0.01], widthWorld: 50 }],
      grid: { automaticLod: true, distanceReference: 100, gridFactor: 10 },
    });
    expect(receipt.camera).toMatchObject({ kind: "PerspectiveCamera", fov: 45, position: [10, -10, 10], target: [0, 0, 0] });
    expect(receipt.contentBounds).not.toBeNull();

    runMountedFrames();
    expect(info).toHaveBeenCalledTimes(1);
    recorded.camera!.position.x += 1;
    runMountedFrames();
    expect(info).toHaveBeenCalledTimes(2);
  });

  it("mounts one instanced mesh per (mesh, verdict) from the scene's toolRunTrace lane and publishes its counters", () => {
    const ledger = new ToolRunTraceStore(toolRunIdentityFromJson(fixture.identity as never), fixture.capacity, fixture.compactFloor);
    const view = render(sceneNode(null));
    const host = () => view.container.querySelector(".semio-world-3d-host") as HTMLElement;
    runFrames();
    expect(recorded.instanced.filter((entry) => entry.live)).toHaveLength(0);
    expect(host().getAttribute("data-tool-run-records")).toBe("0");
    expect(recorded.dashed, "every provisional instance paints the dashed provisional outline").toBe(fixture.provisionalOutlines);
    let cursor: ToolRunTraceCursor | null = null;
    for (const step of fixture.steps) {
      ledger.applyOps(step.ops.map((op) => toolRunTraceOpFromJson(op as never)));
      const delta = ledger.deltaAfter(cursor, Number.MAX_SAFE_INTEGER);
      view.rerender(sceneNode(base64UrlEncode(encodeToolRunTraceDelta(delta))));
      runFrames();
      cursor = { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next };
      expect(mountedBatches(ledger)).toEqual(sortBatches(step.batches));
      for (const [counter, value] of Object.entries(step.counters)) expect(host().getAttribute(`data-tool-run-${counter}`), counter).toBe(String(value));
      expect(host().getAttribute("data-tool-run-page")).toBe(String(delta.next));
    }
  });
});
