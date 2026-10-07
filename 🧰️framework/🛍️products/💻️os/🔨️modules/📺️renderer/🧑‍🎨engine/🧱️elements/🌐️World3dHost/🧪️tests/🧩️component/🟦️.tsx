/** 🎬️ A World3d window mounts its scene's `toolRunTrace` lane for real in jsdom: the lane pages of
 * `🧫️fixtures/⏯️tool-run-trace-mount.json` travel as base64url lane text into `World3dHost`, which must mount one
 * live `THREE.InstancedMesh` per resident `(mesh, verdict)` carrying exactly the fixture's instance counts,
 * draw each through `meshesJson[mesh]`, publish the `data-tool-run-*` counters and paint provisional instances
 * with the dashed provisional outline. three.js is the oracle for instance matrices (`Matrix4.compose`).
 * WebGL is replaced as in `🧪️tests/🖱️world3d-interaction`; component gesture laws feed the original mounted
 * SceneGumball callbacks at the UnifiedGumball hardware seam while retaining host publication and guest completion. */
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import Ajv2020 from "ajv/dist/2020.js";
import type { UnifiedGumballProps } from "@semio-tech/ui-react";
import { createElement, type ReactNode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { BufferGeometry, Float32BufferAttribute, Matrix4, Quaternion, Vector3, type InstancedMesh, type PerspectiveCamera } from "three";
import fixture from "../../🧫️fixtures/⏯️tool-run-trace-mount.json" with { type: "json" };
import { base64UrlEncode } from "../../../../../../../../../🔨️modules/🚪️io/🔤️base64/🟦️.ts";
import { encodeToolRunTraceDelta, toolRunIdentityFromJson, toolRunTraceOpFromJson, ToolRunTraceStore, type ToolRunTraceCursor, type ToolRunTraceSubject } from "../../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";

const recorded = vi.hoisted(() => ({ instanced: [] as { mesh: InstancedMesh; live: boolean }[], geometries: [] as BufferGeometry[], dashed: 0, frames: [] as ((state: unknown, delta: number) => void)[], camera: null as PerspectiveCamera | null, gumball: null as UnifiedGumballProps | null }));

vi.mock("@semio-tech/ui-react", async (importOriginal) => {
  const actual = (await importOriginal()) as typeof import("@semio-tech/ui-react");
  return { ...actual, UnifiedGumball: (props: UnifiedGumballProps) => { recorded.gumball = props; return null; } };
});

vi.mock("three", async (importOriginal) => {
  const actual = (await importOriginal()) as typeof import("three");
  class RecordingBufferGeometry extends actual.BufferGeometry {
    constructor() {
      super();
      recorded.geometries.push(this);
    }
    toString() { return this.uuid; }
  }
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
  return { ...actual, BufferGeometry: RecordingBufferGeometry, InstancedMesh: RecordingInstancedMesh, LineDashedMaterial: RecordingLineDashedMaterial };
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
import { WindowInstanceIdContext, World3dHost, world3dComponentInteractionTarget, buildMeshVisuals, disposeMeshVisuals, world3dGumballSelectionArgsV1, getWorldGumballTransformPreview } from "../../🟦️.tsx";

type Batch = { readonly mesh: number; readonly verdict: string; readonly count: number };

function sceneNode(toolRunTrace: string | null, world: Record<string, unknown> = {}, onAction: (action: { action: string; args?: Record<string, unknown> }) => Promise<void> = () => Promise.resolve()) {
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
          ...world,
        },
      },
      onAction,
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
      expect(mesh.geometry.getAttribute("position").count, "[TRACE] a batch must draw through meshesJson[mesh]").toBe(fixture.meshes[subject.mesh]!.data.positions.length / 3);
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
    const prefix = "[TRACE] react-world-frame ";
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


describe("exact analytic component interaction targets", () => {
  afterEach(() => {
    cleanup();
    for (const geometry of recorded.geometries) geometry.dispose();
    recorded.geometries.length = 0;
    recorded.frames.length = 0;
  });
  it("pins live component gumball targets and waits for guest completion before the next stream", async () => {
    const fixture = (await import("../../🧫️fixtures/🛠️gumball-component-events.json")).default;
    const source = (await import("../../🧫️fixtures/🎯️analytic-component-target.json")).default;
    const three = await vi.importActual<typeof import("three")>("three");
    const pose = (position: number[]) => ({ position: position as [number, number, number], quaternion: [0, 0, 0, 1] as const, scale: [1, 1, 1] as const });
    for (const target of fixture.targets) {
      recorded.gumball = null;
      const actions: { action: string; args?: Record<string, unknown> }[] = [];
      let completeFirst!: () => void;
      const first = new Promise<void>(resolve => { completeFirst = resolve; });
      const onAction = (action: typeof actions[number]) => {
        if (action.action !== "translateSelection") return Promise.resolve();
        actions.push(action);
        return actions.length === 1 ? first : Promise.resolve();
      };
      const node = (captured: boolean) => sceneNode(null, {
        meshesJson: JSON.stringify(source.meshes), instancesJson: JSON.stringify(source.instances), referencesJson: "[]",
        selectionJson: JSON.stringify({ ids: captured ? [fixture.owner] : [], selectionMode: captured ? target.mode : "object", gumballSelectionIds: captured ? target.ids : [], gumballActive: true, gumballTarget: fixture.positions[0], gumballLiveDispatch: true, transformMode: "move" }),
      }, onAction);
      const view = render(node(true));
      try {
        expect(recorded.gumball).not.toBeNull();
        await act(async () => {
          recorded.gumball!.onDraggingChanged?.(true);
          recorded.gumball!.onDragStart?.("moveX", pose(fixture.positions[0]!));
          recorded.gumball!.onDrag?.("moveX", pose(fixture.positions[1]!));
        });
        expect(actions).toHaveLength(1);
        await act(async () => {
          recorded.gumball!.onDrag?.("moveX", pose(fixture.positions[2]!));
          recorded.gumball!.onDrag?.("moveX", pose(fixture.positions[3]!));
        });
        expect(actions, `${target.mode}: the guest has not completed the first stream`).toHaveLength(1);
        view.rerender(node(false));
        await act(async () => { completeFirst(); });
        expect(actions).toHaveLength(2);
        await act(async () => {
          recorded.gumball!.onDragEnd?.("moveX", pose(fixture.positions[0]!), pose(fixture.positions[4]!));
          recorded.gumball!.onDraggingChanged?.(false);
        });
        expect(actions.map(action => action.args)).toEqual(fixture.dispatches.map(delta => ({ surfaceId: "trace-window", windowId: "trace-window:1", mode: target.mode, ids: target.ids, ...delta })));
        const oracle = new three.Vector3(...fixture.positions[0] as [number, number, number]);
        for (const action of actions) oracle.applyMatrix4(new three.Matrix4().makeTranslation(Number(action.args!.dx), Number(action.args!.dy), Number(action.args!.dz)));
        expect(oracle.toArray()).toEqual(fixture.offset);
        expect(getWorldGumballTransformPreview("trace-app")).toBeNull();
        console.info(`[DEBUG] React mounted live ${target.mode} gumball: pinned=${JSON.stringify(target.ids)} stream/stream/commit=${actions.length} oracle=${JSON.stringify(oracle.toArray())}`);
      } finally {
        completeFirst();
        view.unmount();
      }
    }
  });
  it("pins live component gumball across a guest mesh and source refresh", async () => {
    const fixture = (await import("../../../../../../♾️infinite/🌍️world/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).default;
    const row = fixture.gumball;
    const refresh = row.liveRefresh;
    const object = fixture.objects[row.object]!;
    const targets = row.ids.map(index => fixture.targets[index]!);
    const three = await vi.importActual<typeof import("three")>("three");
    const camera = new three.PerspectiveCamera(refresh.camera.fov, refresh.viewport[0]! / refresh.viewport[1]!, 0.1, 1000);
    camera.position.fromArray(refresh.camera.position);
    camera.up.fromArray(refresh.camera.up);
    camera.lookAt(new three.Vector3().fromArray(refresh.camera.target));
    camera.updateMatrixWorld(true);
    const raycaster = new three.Raycaster();
    const plane = new three.Plane(new three.Vector3(0, 0, 1), 0);
    const projected = refresh.pointers.map(pointer => {
      raycaster.setFromCamera(new three.Vector2(pointer[0]! * 2 / refresh.viewport[0]! - 1, 1 - pointer[1]! * 2 / refresh.viewport[1]!), camera);
      return raycaster.ray.intersectPlane(plane, new three.Vector3())!;
    });
    for (let index = 1; index < projected.length; index += 1) {
      expect(projected[index]!.x - projected[index - 1]!.x).toBeCloseTo(refresh.dispatches[index - 1]!.dx, 10);
    }
    const actions: { action: string; args?: Record<string, unknown> }[] = [];
    const onAction = (action: typeof actions[number]) => { if (action.action === "translateSelection") actions.push(action); return Promise.resolve(); };
    const node = (fresh: boolean) => sceneNode(null, {
      meshesJson: JSON.stringify([{ id: "analytic", data: { ...fixture.mesh, componentReferences: { face: [fixture.source.label] }, positions: fixture.mesh.positions.map((value, index) => value + (fresh ? refresh.positions[refresh.refreshAfter]![index % 3]! : 0)) } }]),
      instancesJson: JSON.stringify([{ id: object, meshId: "analytic", componentSource: { handle: fixture.source.handle, revision: fresh ? refresh.revision : fixture.source.revision } }]),
      cameraJson: JSON.stringify(refresh.camera), referencesJson: "[]",
      selectionJson: JSON.stringify({ ids: [object], selectionMode: row.mode, gumballSelectionIds: targets, gumballActive: true, gumballTarget: row.pivot, gumballLiveDispatch: true, transformMode: "move" }),
    }, onAction);
    const pose = (position: number[]) => ({ position: position as [number, number, number], quaternion: [0, 0, 0, 1] as const, scale: [1, 1, 1] as const });
    const view = render(node(false));
    try {
      expect(recorded.gumball).not.toBeNull();
      await act(async () => {
        recorded.gumball!.onDraggingChanged?.(true);
        recorded.gumball!.onDragStart?.("moveX", pose(refresh.positions[0]!));
        recorded.gumball!.onDrag?.("moveX", pose(refresh.positions[1]!));
      });
      expect(actions).toHaveLength(1);
      recorded.gumball = null;
      view.rerender(node(true));
      expect(recorded.gumball, "the actual refreshed scene still mounts its active gesture").not.toBeNull();
      await act(async () => { recorded.gumball!.onDrag?.("moveX", pose(refresh.positions[2]!)); });
      await act(async () => {
        recorded.gumball!.onDragEnd?.("moveX", pose(refresh.positions[0]!), pose(refresh.positions[3]!));
        recorded.gumball!.onDraggingChanged?.(false);
      });
      expect(actions.map(action => action.args)).toEqual(refresh.dispatches.map(delta => ({ surfaceId: "trace-window", windowId: "trace-window:1", mode: row.mode, ids: targets, ...delta })));
      const oracle = new three.Vector3();
      for (const action of actions) oracle.applyMatrix4(new three.Matrix4().makeTranslation(Number(action.args!.dx), Number(action.args!.dy), Number(action.args!.dz)));
      expect(oracle.toArray()).toEqual(refresh.offset);
      expect(getWorldGumballTransformPreview("trace-app")).toBeNull();
      console.info(`[DEBUG] React mounted live source refresh: revision=${refresh.revision} pinned=${JSON.stringify(targets)} phases=${JSON.stringify(actions.map(action => action.args!.phase))} independentThree=${JSON.stringify(oracle.toArray())}`);
    } finally { view.unmount(); }
  });
  it("paints neutral pure points in default object mode without fabricating surfaces or edges", async () => {
    const fixture = (await import("../../../../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).default;
    const point = fixture.cases.find(row => row.kind === "point")!;
    const three = await vi.importActual<typeof import("three")>("three");
    const oracle = new three.Points(new three.BufferGeometry().setAttribute("position", new three.Float32BufferAttribute(point.points.flat(), 3)), new three.PointsMaterial({ sizeAttenuation: false }));
    const data = { positions: point.points.flat(), normals: [], indices: [], vertexIds: [0], edgeIds: [], edgePositions: [] };
    const view = render(sceneNode(null, { meshesJson: JSON.stringify([{ id: "neutral-point", data }]), instancesJson: JSON.stringify([{ id: "point@geometry#0", meshId: "neutral-point" }]), selectionJson: JSON.stringify({ ids: [], selectionMode: "object", targets: { mesh: true, vertex: false, edge: false, face: false } }), referencesJson: "[]" }));
    try {
      const painted = [...view.container.querySelectorAll("points")].map(node => recorded.geometries.find(geometry => geometry.uuid === node.getAttribute("geometry"))).filter((geometry): geometry is BufferGeometry => Boolean(geometry));
      expect(painted, "an unselected object-mode pure point must have a visible original mesh instance").toHaveLength(1);
      expect(Array.from(painted[0]!.getAttribute("position").array)).toEqual(Array.from(oracle.geometry.getAttribute("position").array));
      expect(painted[0]!.getIndex()).toBeNull();
      expect(view.container.querySelectorAll("lineSegments")).toHaveLength(0);
      expect(oracle.geometry.getAttribute("position").count).toBe(1);
      console.info(`[DEBUG] React mounted default Point3d: points=${painted.length} position=${JSON.stringify(point.points[0])} selected=0 triangles=0 edges=0 Three=${oracle.type}`);
    } finally {
      view.unmount();
      oracle.geometry.dispose();
      oracle.material.dispose();
    }
  });
  it("paints primary point and wire objects with the existing selection and hover palette", async () => {
    const fixture = (await import("../../../../../../♾️infinite/🌍️world/🧫️fixtures/🎨️primary-geometry-style/🔣️.json")).default;
    const geometry = (await import("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🎯️component-source/🔣️.json")).default;
    const styling = await import("@semio-tech/ui-styling");
    const three = await vi.importActual<typeof import("three")>("three");
    styling.clearColorResolveCache();
    for (const name of fixture.geometry) {
      const row = geometry.zeroIndex.accepted.find(row => row.kind === name)!;
      const object = `${name}@geometry#0`;
      const positions = row.positions.flat();
      const data = { positions, normals: positions.map(() => 0), indices: [], vertexIds: row.vertexIds, edgePositions: row.edges.flat(2), edgeIds: row.edgeIds };
      const node = (state: typeof fixture.states[number]) => sceneNode(null, {
        meshesJson: JSON.stringify([{ id: name, data }]), instancesJson: JSON.stringify([{ id: object, meshId: name }]), referencesJson: "[]",
        selectionJson: JSON.stringify({ ids: state.selected ? [object] : [], hoveredId: state.hovered ? object : null, selectionMode: "object", targets: { mesh: true, vertex: false, edge: false, face: false } }),
      });
      const view = render(node(fixture.states[0]!));
      try {
        for (const state of fixture.states) {
          view.rerender(node(state));
          runFrames();
          const ref = styling.resolveColorHex(state.line);
          const oracle = name === "point" ? new three.PointsMaterial({ color: ref }) : new three.LineBasicMaterial({ color: ref });
          try {
            const material = view.container.querySelector(name === "point" ? "points > pointsmaterial" : "linesegments > linebasicmaterial");
            expect(material, `${name}/${state.name}: the primary physical visual remains mounted ${JSON.stringify([...view.container.querySelectorAll("linesegments,linebasicmaterial,points,pointsmaterial")].map(node => ({ tag: node.tagName, parent: node.parentElement?.tagName, color: node.getAttribute("color") })))}`).not.toBeNull();
            const painted = new three.Color(material!.getAttribute("color")!);
            expect(painted.getHexString(), `${name}/${state.name}: the original semantic line paint follows object chrome`).toBe(oracle.color.getHexString());
            expect(view.container.querySelectorAll("mesh")).toHaveLength(0);
            console.info(`[DEBUG] React primary ${name}/${state.name}: token=${state.line} color=#${painted.getHexString()} independentThree=#${oracle.color.getHexString()}`);
          } finally { oracle.dispose(); }
        }
      } finally { view.unmount(); }
    }
  });
  it("projects exact analytic component selections across instances", async () => {
    const fixture = (await import("../../../../../../♾️infinite/🌍️world/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).default;
    const three = await vi.importActual<typeof import("three")>("three");
    const indexed = new three.BufferGeometry().setAttribute("position", new three.Float32BufferAttribute(fixture.mesh.positions, 3)).setIndex(fixture.mesh.indices);
    const oracle = indexed.toNonIndexed();
    const meshes = [{ id: "selection-mesh", data: { ...fixture.mesh, componentReferences: { face: [fixture.source.label] } } }];
    const instances = fixture.objects.map((id, index) => ({ id, meshId: meshes[0]!.id, position: [index * 3, 0, 0], componentSource: { handle: fixture.source.handle, revision: fixture.source.revision } }));
    const node = (step: typeof fixture.steps[number]) => {
      const selection = { ids: step.selected.map(index => fixture.objects[index]), activeObjectId: step.active === null ? null : fixture.objects[step.active], componentIds: step.groups.map(Number), gumballSelectionIds: step.selected.map(index => fixture.targets[index]), granularity: "face", selectionMode: "face", targets: { mesh: false, vertex: false, edge: false, face: true }, hoveredComponent: null };
      expect(world3dGumballSelectionArgsV1(selection).ids).toEqual(step.selected.map(index => fixture.targets[index]));
      return sceneNode(null, { meshesJson: JSON.stringify(meshes), instancesJson: JSON.stringify(instances), selectionJson: JSON.stringify(selection), referencesJson: "[]" });
    };
    const view = render(node(fixture.steps[0]!));
    try {
      for (const step of fixture.steps) {
        view.rerender(node(step));
        runFrames();
        const overlays = [...view.container.querySelectorAll('[opacity="0.62"]')].map(material => {
          const mesh = material.parentElement!;
          const geometry = recorded.geometries.find(value => value.uuid === mesh.getAttribute("geometry"));
          expect(geometry, `${step.merge}: the mounted overlay retains its real Three geometry`).toBeDefined();
          expect(Array.from(geometry!.getAttribute("position").array), step.merge).toEqual(Array.from(oracle.getAttribute("position").array));
          const position = mesh.closest("group")!.getAttribute("position")!.split(",").map(Number);
          return fixture.objects[position[0]! / 3];
        }).sort();
        expect(overlays, `${step.merge}: every selected instance paints its selected component`).toEqual(step.selected.map(index => fixture.objects[index]).sort());
        console.info(`[DEBUG] React component overlay ${step.merge}: ${JSON.stringify(overlays)}`);
      }
      const refusalFixture = (await import("../../🧫️fixtures/🎯️analytic-component-target.json")).default;
      for (const row of refusalFixture.overlayRefusals) {
        const selection = { ids: [fixture.objects[0]], activeObjectId: fixture.objects[0], componentIds: [0], gumballSelectionIds: row.targets, granularity: "face", selectionMode: "face", targets: { mesh: false, vertex: false, edge: false, face: true } };
        const currentMeshes = [{ ...meshes[0], data: { ...meshes[0]!.data, componentReferences: { face: row.labels } } }];
        view.rerender(sceneNode(null, { meshesJson: JSON.stringify(currentMeshes), instancesJson: JSON.stringify(instances), selectionJson: JSON.stringify(selection), referencesJson: "[]" }));
        runFrames();
        expect(view.container.querySelectorAll('[opacity="0.62"]'), row.name).toHaveLength(0);
        console.info(`[DEBUG] React component overlay refuses ${row.name}`);
      }
    } finally {
      view.unmount();
      indexed.dispose();
      oracle.dispose();
    }
  });

  it("excludes native surface sample sentinels from vertex picking", async () => {
    const fixture = (await import("../../🧫️fixtures/🎯️analytic-component-target.json")).default;
    for (const row of fixture.vertexPickCases) {
      const visuals = buildMeshVisuals({ id: row.name, data: row.data });
      const oracle = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(row.expectedPositions, 3));
      try {
        expect(visuals.vertexPick?.vertexIds ?? [], row.name).toEqual(row.expectedIds);
        expect(Array.from(visuals.vertexPick?.geometry.getAttribute("position").array ?? []), row.name).toEqual(Array.from(oracle.getAttribute("position").array));
        for (const group of row.expectedIds) {
          const target = world3dComponentInteractionTarget([{ ...fixture.instances[0], meshId: row.name }], [{ id: row.name, data: row.data }], fixture.instances[0].id, "vertex", group);
          expect(target?.id.split("~")[2], row.name).toBe(row.data.componentReferences.vertex[group]);
        }
        expect(world3dComponentInteractionTarget([{ ...fixture.instances[0], meshId: row.name }], [{ id: row.name, data: row.data }], fixture.instances[0].id, "vertex", 4294967295)).toBeUndefined();
      } finally {
        disposeMeshVisuals(visuals);
        oracle.dispose();
      }
    }
  });
  it("retains zero-index analytic wire and point buffers and exact source admission", async () => {
    const fixture = (await import("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🎯️component-source/🔣️.json")).default;
    const three = await vi.importActual<typeof import("three")>("three");
    for (const row of fixture.zeroIndex.accepted) {
      const positions = row.positions.flat();
      const edges = row.edges.flat(2);
      const data = { positions, normals: positions.map(() => 0), indices: [], vertexIds: row.vertexIds, edgePositions: edges, edgeIds: row.edgeIds };
      const visuals = buildMeshVisuals({ id: row.kind, data });
      const oracle = new three.BufferGeometry().setAttribute("position", new three.Float32BufferAttribute(positions, 3)).setIndex([]);
      const edgeOracle = new three.BufferGeometry().setAttribute("position", new three.Float32BufferAttribute(edges, 3));
      try {
        oracle.computeBoundingBox();
        visuals.geometry!.computeBoundingBox();
        expect(visuals.geometry!.getIndex()?.count ?? 0, row.kind).toBe(oracle.getIndex()!.count);
        expect(Array.from(visuals.vertexPick?.geometry.getAttribute("position").array ?? []), row.kind).toEqual(row.vertexIds.length ? Array.from(oracle.getAttribute("position").array) : []);
        expect(visuals.vertexPick?.vertexIds ?? [], row.kind).toEqual(row.vertexIds);
        expect(Array.from(visuals.edge?.getAttribute("position").array ?? []), row.kind).toEqual(Array.from(edgeOracle.getAttribute("position").array));
        expect([visuals.geometry!.boundingBox!.min.toArray(), visuals.geometry!.boundingBox!.max.toArray()], row.kind).toEqual(row.bounds);
        expect([oracle.boundingBox!.min.toArray(), oracle.boundingBox!.max.toArray()], row.kind).toEqual(row.bounds);
        console.info(`[DEBUG] React zero-index ${row.kind}: vertices=${row.vertexIds.length} edges=${row.edgeIds.length} indices=0 bounds=${JSON.stringify(row.bounds)}`);
      } finally {
        disposeMeshVisuals(visuals);
        oracle.dispose();
        edgeOracle.dispose();
      }
    }
    const objectId = "source@solid#0";
    const instances = [{ id: objectId, meshId: "source", componentSource: fixture.source }];
    const meshes = [{ id: "source", data: { positions: [], normals: [], indices: [], componentReferences: fixture.admission.references } }];
    for (const row of fixture.admission.cases) {
      const target = world3dComponentInteractionTarget(instances, meshes, objectId, row.kind, row.group);
      expect(target, JSON.stringify(row)).toEqual(row.eligible ? { granularity: row.kind, id: `${objectId}.${row.kind}.${row.group}~${fixture.source.handle}~${row.label}~${fixture.source.revision}` } : undefined);
      console.info(`[DEBUG] React exact component source ${row.kind}/${row.group}: eligible=${row.eligible}`);
    }
  });

  it("reads an already mapped group exactly once and preserves uint64 labels", async () => {
    const fixture = (await import("../../🧫️fixtures/🎯️analytic-component-target.json")).default;
    const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(fixture.meshes[0].data.positions, 3)).setIndex(fixture.meshes[0].data.indices);
    geometry.addGroup(0, 3, 0);
    expect(geometry.groups[0].count / 3).toBe(1);
    for (const row of fixture.cases) expect(world3dComponentInteractionTarget(fixture.instances, fixture.meshes, fixture.instances[0].id, row.mode, row.group)).toEqual({ granularity: row.mode, id: row.target });
    for (const row of fixture.invalid) expect(world3dComponentInteractionTarget(fixture.instances, fixture.meshes, fixture.instances[0].id, row.mode, row.group)).toBeUndefined();
    expect(world3dComponentInteractionTarget(fixture.instances, [], fixture.instances[0].id, "edge", 1)).toBeUndefined();
    const duplicate = [{ ...fixture.meshes[0], data: { ...fixture.meshes[0].data, componentReferences: { edge: ["9007199254740993", "9007199254740993"] } } }];
    expect(world3dComponentInteractionTarget(fixture.instances, duplicate, fixture.instances[0].id, "edge", 1)).toBeUndefined();
    for (const source of fixture.invalidSources) expect(world3dComponentInteractionTarget([{ ...fixture.instances[0], componentSource: source }], fixture.meshes, fixture.instances[0].id, "edge", 1)).toBeUndefined();
    for (const label of fixture.invalidLabels) expect(world3dComponentInteractionTarget(fixture.instances, [{ ...fixture.meshes[0], data: { ...fixture.meshes[0].data, componentReferences: { edge: ["1", label] } } }], fixture.instances[0].id, "edge", 1)).toBeUndefined();
    geometry.dispose();
  });
});
