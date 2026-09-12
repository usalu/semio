/** 🖱️ The gesture→action law of a domain-bound `World3dHost`, mounted for real in jsdom.
 *
 * 🧫️ Every expectation is read from `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`, the same file the
 * language-neutral Node oracle (`world3dPointerGestureOracle`, the react target's `📜️script.ts`) re-derives
 * independently — so the law lives in the fixture and neither implementation can drift alone.
 *
 * 🧯️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-hover-selection-2026-09-12.md` §5.3 item 1: the
 * 6.4k-line host had zero interactive coverage — only pure-function unit tests of the arg builders and two
 * `renderToStaticMarkup` snapshots. Nothing proved a pick reaches `interactionSelect`, a move reaches
 * `interactionHover`, a background click clears, a completed orbit debounces into exactly one `setCamera`,
 * or a marquee release replaces with the deduplicated topology targets.
 *
 * 🎭️ Only the WebGL seam is replaced: `@react-three/fiber`'s three hooks hand out a real (renderer-free)
 * `PerspectiveCamera` so the host's own projection math is the real one, and `@semio-tech/infinite-world-r3f`'s
 * `WorldCanvas`/`WorldOrbitGated` become DOM stand-ins that expose the two callbacks a real canvas would
 * call (`onPointerMissed`, `onCamera`). Every handler under test is the host's own.
 */
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement, type ReactNode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixture from "../../🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json" with { type: "json" };
import mergeModes from "../../../../../../../🔨️modules/🕹️interaction/🧫️fixtures/🎯️merge-modes.json" with { type: "json" };
import type { MergeMode } from "../../../../../../../🔨️modules/🕹️interaction/🟦️.ts";
import identity from "../../🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json" with { type: "json" };

type Vector3Tuple = readonly [number, number, number];
type CameraPose = { readonly position: Vector3Tuple; readonly target: Vector3Tuple; readonly zoom: number };
type Dispatched = { readonly controllerId: string; readonly action: string; readonly args: Record<string, unknown> };
type Gesture = (typeof fixture)["gestures"][number];

/** 🎛️ The two canvas-owned callbacks a real `WorldCanvas`/`WorldOrbitGated` would invoke — captured on
 * render so a test can play a background click or a completed orbit without a WebGL context. */
const seams: { onPointerMissed: ((event: MouseEvent) => void) | null; onCamera: ((camera: CameraPose) => void) | null } = { onPointerMissed: null, onCamera: null };

vi.mock("@react-three/fiber", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const three = await import("three");
  const pose = fixture.scene.camera;
  const camera = new three.PerspectiveCamera(pose.fov, fixture.viewport.width / fixture.viewport.height, 0.1, 1000);
  camera.up.set(0, 0, 1);
  camera.position.set(pose.position[0]!, pose.position[1]!, pose.position[2]!);
  camera.lookAt(pose.target[0]!, pose.target[1]!, pose.target[2]!);
  camera.updateMatrixWorld(true);
  camera.updateProjectionMatrix();
  const state = { camera, gl: { domElement: { clientWidth: fixture.viewport.width, clientHeight: fixture.viewport.height } }, scene: new three.Scene(), size: { width: fixture.viewport.width, height: fixture.viewport.height }, raycaster: new three.Raycaster(), invalidate: () => {} };
  return { ...actual, useFrame: () => {}, useThree: (selector?: (value: typeof state) => unknown) => (selector ? selector(state) : state), useLoader: () => null };
});

vi.mock("@semio-tech/infinite-world-r3f", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return {
    ...actual,
    WorldCanvas: ({ children, overlay, onPointerMissed }: { children?: ReactNode; overlay?: ReactNode; onPointerMissed?: (event: MouseEvent) => void }) => {
      seams.onPointerMissed = onPointerMissed ?? null;
      return createElement("div", { "data-testid": "world-canvas" }, overlay, children);
    },
    WorldOrbitGated: ({ onCamera }: { onCamera?: (camera: CameraPose) => void }) => {
      seams.onCamera = onCamera ?? null;
      return null;
    },
    WorldOrbitViewControls: () => null,
    WorldProjectionRig: () => null,
    WorldOrbitViewSnapGateProvider: ({ children }: { children?: ReactNode }) => createElement("div", null, children),
    WorldLodBridge: ({ children }: { children?: ReactNode }) => createElement("div", null, children),
    WorldVolumeLayer: () => null,
    WorldReferenceLayer: () => null,
  };
});

import { componentPickMergeMode, WindowInstanceIdContext, World3dHost, world3dSelectionActionArgs } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";

const gesture = (id: string): Gesture => {
  const found = fixture.gestures.find((entry) => entry.id === id);
  if (!found) throw new Error(`[DEBUG] fixture gesture ${id} missing`);
  return found;
};

function mountHost(pane?: { readonly surfaceId: string; readonly windowInstanceId: string }): { readonly dispatched: readonly Dispatched[]; readonly host: HTMLElement; readonly meshes: readonly HTMLElement[] } {
  const dispatched: Dispatched[] = [];
  const scene = fixture.scene;
  const view = render(
    createElement(
      WindowInstanceIdContext.Provider,
      { value: pane?.windowInstanceId ?? scene.windowInstanceId },
      createElement(World3dHost as never, {
        node: {
          type: "componentScene",
          surfaceId: pane?.surfaceId ?? scene.surfaceId,
          controllerId: scene.controllerId,
          componentKind: "world-3d",
          world3d: {
            cameraJson: JSON.stringify(scene.camera),
            meshesJson: JSON.stringify(scene.meshes),
            instancesJson: JSON.stringify(scene.instances),
            selectionJson: scene.selectionJson,
            vorticesJson: "[]",
            attractionsJson: "[]",
            interactionJson: scene.interactionJson,
            domainId: scene.domainId,
            domainGranularityId: scene.domainGranularityId,
          },
        },
        onAction: (action: Dispatched) => {
          dispatched.push(action);
          return Promise.resolve();
        },
      }),
    ),
  );
  const host = view.container.querySelector(".semio-world-3d-host") as HTMLElement;
  const { left, top, width, height } = fixture.viewport;
  host.getBoundingClientRect = () => ({ x: left, y: top, left, top, width, height, right: left + width, bottom: top + height, toJSON: () => ({}) }) as DOMRect;
  return { dispatched, host, meshes: [...host.querySelectorAll("mesh")] as HTMLElement[] };
}

const only = (dispatched: readonly Dispatched[], action: string): readonly Dispatched[] => dispatched.filter((entry) => entry.action === action);

function expectSelection(entry: Dispatched | undefined, expected: Gesture["expect"]): void {
  expect(entry, `[DEBUG] no ${expected.action} dispatched`).toBeTruthy();
  expect(entry!.controllerId).toBe(fixture.scene.controllerId);
  expect(entry!.args.surfaceId).toBe(fixture.scene.surfaceId);
  expect(entry!.args.windowId).toBe(fixture.scene.windowInstanceId);
  expect(entry!.args.domainId).toBe(expected.domainId);
  expect(entry!.args.merge).toBe(expected.merge);
  expect(mergeModes.vocabulary, `[DEBUG] merge "${String(entry!.args.merge)}" is outside the ONE schema vocabulary`).toContain(entry!.args.merge);
  expect(entry!.args.method).toBe(expected.method);
  const targets = JSON.parse(String(entry!.args.targets)) as readonly { readonly id: string }[];
  expect(targets).toEqual(expected.targets);
  expect(new Set(targets.map((target) => target.id)).size, `[DEBUG] duplicate topology target in ${JSON.stringify(targets)}`).toBe(targets.length);
}

export function testWorld3dInteraction(): void {
  describe("🖱️ world 3d host gestures on a domain-bound scene", () => {
    afterEach(() => cleanup());

    it("renders exactly one pickable mesh per scene instance", () => {
      const { meshes } = mountHost();
      expect(meshes.length).toBe(fixture.scene.instances.length);
    });

    // 🪪️ `🪪️world-surface-identity.json` (peer ticket 26/09/02 wave B20): two panes of ONE document each
    // have to name the window they were mounted into. Before that wave both published the per-document
    // DFS record id, so a second pane's picks were indistinguishable from the first's.
    it("names its own window instance on every pane of one document surface", () => {
      for (const pane of identity.panes) {
        const { host } = mountHost({ surfaceId: pane.documentSurface, windowInstanceId: pane.window });
        expect(host.getAttribute("data-surface-id")).toBe(pane.identity.surfaceId);
        expect(host.getAttribute("data-window-instance-id")).toBe(pane.window);
        cleanup();
      }
    });

    // 🎯️ Every modifier chord of `🎯️merge-modes.json`'s `modifierPolicy` reaches the wire as the SCHEMA
    // word, never a host-private spelling — the live 2026-09-12 defect faulted every one of these but
    // the bare click (`interactionSelect: unknown merge 'add'`).
    for (const id of ["instance-pick-replaces", "instance-pick-additive", "instance-pick-subtractive", "instance-pick-subtractive-on-command", "instance-pick-invertive", "second-instance-of-one-topology-id-pick"] as const) {
      it(`dispatches interactionSelect for the topology target on ${id}`, () => {
        const spec = gesture(id);
        const { dispatched, meshes } = mountHost();
        const index = fixture.scene.instances.findIndex((instance) => instance.id === spec.instanceId);
        fireEvent.click(meshes[index]!, spec.modifiers);
        const selects = only(dispatched, "interactionSelect");
        expect(selects.length).toBe(1);
        expectSelection(selects[0], spec.expect);
      });
    }

    // 🧮️ The builder itself is the host's set boundary — the call sites are proved above, but only a
    // law on the builder makes a duplicate target unrepresentable for EVERY caller (the marker picks,
    // the marquee release, and any future one).
    it("collapses repeated topology ids into one wire target in the selection args builder", () => {
      const spec = gesture("repeated-ids-collapse-in-the-args-builder");
      const ids = spec.ids!;
      expect(new Set(ids).size, "[DEBUG] the fixture must hand the builder a repeated id or the law proves nothing").toBeLessThan(ids.length);
      const args = world3dSelectionActionArgs(fixture.scene.domainId, spec.granularity!, ids, spec.expect.merge! as MergeMode);
      expect(args.domainId).toBe(spec.expect.domainId);
      expect(args.merge).toBe(spec.expect.merge);
      expect(args.method).toBe(spec.expect.method);
      expect(JSON.parse(args.targets)).toEqual(spec.expect.targets);
    });

    // 🖱️ The fixture's modifier policy IS the host's: each chord's `pick` word must be the one an
    // actual click on the mounted host puts on the wire, and no chord may resolve to a word the
    // viewport cannot mean (`range` — no ordered topology).
    it("resolves every declared modifier chord to its schema merge word and never to range", () => {
      for (const row of mergeModes.modifierPolicy.rows) {
        const { dispatched, meshes } = mountHost();
        fireEvent.click(meshes[0]!, row.modifiers);
        const selects = only(dispatched, "interactionSelect");
        expect(selects.length, `[DEBUG] ${row.id} dispatched ${selects.length} selects`).toBe(1);
        expect(selects[0]!.args.merge, `[DEBUG] ${row.id} must resolve to ${row.pick}`).toBe(row.pick);
        expect(componentPickMergeMode(row.pick as MergeMode), `[DEBUG] ${row.id} component pick`).toBe(row.componentPick);
        for (const never of mergeModes.modifierPolicy.neverEmitted) expect(selects[0]!.args.merge).not.toBe(never);
        cleanup();
      }
    });

    it("dispatches interactionHover at the scene granularity on an instance pointer move", () => {
      const spec = gesture("instance-hover");
      const { dispatched, meshes } = mountHost();
      const index = fixture.scene.instances.findIndex((instance) => instance.id === spec.instanceId);
      fireEvent.pointerMove(meshes[index]!);
      const hovers = only(dispatched, "interactionHover");
      expect(hovers.length).toBe(1);
      expect(hovers[0]!.args.domainId).toBe(spec.expect.domainId);
      expect(hovers[0]!.args.channel).toBe(spec.expect.channel);
      expect(JSON.parse(String(hovers[0]!.args.targets))).toEqual(spec.expect.targets);
    });

    it("clears the domain selection when a click hits no instance", () => {
      const spec = gesture("background-click-clears");
      const { dispatched } = mountHost();
      expect(seams.onPointerMissed).toBeTruthy();
      seams.onPointerMissed!(new MouseEvent("click", spec.modifiers));
      const selects = only(dispatched, "interactionSelect");
      expect(selects.length).toBe(1);
      expectSelection(selects[0], spec.expect);
    });

    it("debounces a completed orbit gesture into exactly one setCamera carrying the final pose", async () => {
      const spec = gesture("orbit-completes-into-one-setcamera");
      const { dispatched } = mountHost();
      expect(seams.onCamera).toBeTruthy();
      for (const step of spec.steps!) seams.onCamera!({ position: [step.position[0]!, step.position[1]!, step.position[2]!], target: [step.target[0]!, step.target[1]!, step.target[2]!], zoom: step.zoom });
      expect(only(dispatched, "setCamera").length).toBe(0);
      await new Promise((resolve) => setTimeout(resolve, fixture.cameraDebounceMs * 3));
      const cameras = only(dispatched, "setCamera");
      expect(cameras.length).toBe(spec.expect.dispatches);
      expect(cameras[0]!.args.windowId).toBe(fixture.scene.windowInstanceId);
      expect(cameras[0]!.args.camera).toEqual(spec.expect.camera);
    });

    it("replaces the selection with the deduplicated topology targets on a marquee release", () => {
      const spec = gesture("marquee-release-replaces");
      const { dispatched, host } = mountHost();
      const path = spec.path!;
      fireEvent.pointerDown(host, { button: 0, pointerId: 1, clientX: path[0]!.x, clientY: path[0]!.y });
      for (const point of path.slice(1)) fireEvent.pointerMove(host, { pointerId: 1, clientX: point.x, clientY: point.y });
      fireEvent.pointerUp(host, { button: 0, pointerId: 1, clientX: path[path.length - 1]!.x, clientY: path[path.length - 1]!.y });
      const selects = only(dispatched, "interactionSelect");
      expect(selects.length).toBe(1);
      expectSelection(selects[0], spec.expect);
    });
  });
}

testWorld3dInteraction();
