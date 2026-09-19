// #region 🧲️Header
/** @emoji 🤏️ `🌐️World3dHost` multi-touch law: a SECOND contact hands the viewport to the orbit rig's own
 * two-finger gesture (`TOUCH.DOLLY_PAN`), so the host's single-pointer lane — marquee, pick, engagement —
 * must go quiet for the whole gesture instead of growing a marquee under the pinch, and must not replay
 * the release as a click. Driven with real `PointerEvent`s; only the WebGL seam is replaced, exactly as in
 * `🧪️tests/🖱️world3d-interaction`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { createElement, type ReactNode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
// #endregion 🔌️Adapters

// #region 🧪️SceneSeam
vi.mock("@react-three/fiber", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const three = await import("three");
  const camera = new three.PerspectiveCamera(45, 1, 0.1, 1000);
  camera.position.set(10, -10, 10);
  camera.lookAt(0, 0, 0);
  camera.updateMatrixWorld(true);
  const state = { camera, gl: { domElement: { clientWidth: 800, clientHeight: 800 } }, scene: new three.Scene(), size: { width: 800, height: 800 }, raycaster: new three.Raycaster(), invalidate: () => {}, clock: { elapsedTime: 0 } };
  return { ...actual, useFrame: () => {}, useThree: (selector?: (value: typeof state) => unknown) => (selector ? selector(state) : state), useLoader: () => null };
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

import { WindowInstanceIdContext, World3dHost } from "../../🟦️.tsx";

const dispatched: string[] = [];

function sceneNode() {
  return createElement(
    WindowInstanceIdContext.Provider,
    { value: "touch-window:1" },
    createElement(World3dHost as never, {
      node: {
        type: "componentScene",
        surfaceId: "touch-window",
        controllerId: "touch-app",
        componentKind: "world-3d",
        world3d: {
          cameraJson: JSON.stringify({ position: [10, -10, 10], target: [0, 0, 0], zoom: 1, fov: 45 }),
          meshesJson: "{}",
          instancesJson: "[]",
          selectionJson: "{}",
          vorticesJson: "[]",
          attractionsJson: "[]",
          interactionJson: '{"activeUtility":"select"}',
        },
      },
      onAction: (descriptor: { action: string }) => {
        dispatched.push(descriptor.action);
        return Promise.resolve();
      },
    }),
  );
}

function pointer(type: string, pointerId: number, clientX: number, clientY: number): PointerEvent {
  return new PointerEvent(type, { bubbles: true, cancelable: true, pointerId, clientX, clientY, pointerType: "touch", button: 0, buttons: 1 });
}

function mountWorld(): HTMLElement {
  const view = render(sceneNode());
  return view.container.querySelector(".semio-world-3d-host") as HTMLElement;
}
// #endregion 🧪️SceneSeam

// #region 🤏️MultiTouchLaws
describe("🤏️ world 3d host multi-touch", () => {
  afterEach(() => {
    cleanup();
    dispatched.length = 0;
  });

  it("declares touch-action: none so the browser cannot steal the second contact for a page scroll", () => {
    expect(mountWorld().className).toContain("touch-none");
  });

  it("a single contact still starts the marquee lane", () => {
    const host = mountWorld();
    act(() => void host.dispatchEvent(pointer("pointerdown", 1, 120, 140)));
    act(() => void host.dispatchEvent(pointer("pointermove", 1, 260, 300)));
    expect(host.querySelector("[data-world-marquee], svg, .semio-world-3d-marquee") ?? host.getAttribute("data-interaction-json")).toBeTruthy();
  });

  it("a second contact abandons the in-flight marquee instead of growing it under the pinch", () => {
    const host = mountWorld();
    act(() => void host.dispatchEvent(pointer("pointerdown", 1, 120, 140)));
    act(() => void host.dispatchEvent(pointer("pointermove", 1, 260, 300)));
    act(() => void host.dispatchEvent(pointer("pointerdown", 2, 520, 380)));
    const before = dispatched.length;
    act(() => {
      host.dispatchEvent(pointer("pointermove", 1, 60, 60));
      host.dispatchEvent(pointer("pointermove", 2, 740, 560));
    });
    // 🤏️ Nothing the host owns may dispatch while the orbit rig owns the gesture.
    expect(dispatched.slice(before).filter((action) => action !== "noteWorldNavigation")).toEqual([]);
  });

  it("lifting the second finger ends the pinch without replaying it as a selection click", () => {
    const host = mountWorld();
    act(() => {
      host.dispatchEvent(pointer("pointerdown", 1, 300, 300));
      host.dispatchEvent(pointer("pointerdown", 2, 500, 300));
    });
    const before = dispatched.length;
    act(() => void host.dispatchEvent(pointer("pointerup", 2, 500, 300)));
    expect(dispatched.slice(before)).not.toContain("worldSelect");
    expect(dispatched.slice(before)).not.toContain("interactionSelect");
    expect(dispatched.slice(before)).not.toContain("setSelection");
  });
});
// #endregion 🤏️MultiTouchLaws
