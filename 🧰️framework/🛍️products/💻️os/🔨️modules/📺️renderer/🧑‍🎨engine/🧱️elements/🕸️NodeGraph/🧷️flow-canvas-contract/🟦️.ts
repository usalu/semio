/** 🧷️ The flow canvas rules the host and its hooks share: surface render admission, the camera gesture rule, the unpresentable presentation verdict and the canvas key.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost
export function flowSurfaceRenderAllowed(surfaceReady: boolean): boolean {
  return surfaceReady;
}

/** ⏲️ How long after the last wheel tick a zoom gesture counts as settled. Long enough that one
 * continuous scroll is ONE gesture on a trackpad's own inter-tick spacing, short enough that the
 * camera the next open honours is published while the user still thinks of it as this gesture. */
export const FLOW_CAMERA_GESTURE_SETTLE_MS = 140;

/** 🔌️ What a camera gesture needs from its host, injected so a law drives the real rule over a
 * virtual clock instead of a browser. */
export type FlowCameraGesturePorts = Readonly<{
  readonly begin: (reason: string) => void;
  readonly end: (reason: string) => void;
  readonly invalidate: () => void;
  readonly publish: () => void;
  readonly schedule: (run: () => void, delayMs: number) => unknown;
  readonly cancel: (handle: unknown) => void;
}>;

/** 🎥️ The camera-gesture rule, as one unit: N ticks open ONE gesture and repaint through the
 * scheduler, and the plugin hears about the camera exactly once, when the ticks stop.
 *
 * A wheel gesture has no release event, so its end is the absence of the next tick — every tick
 * restarts the settle. Publishing per tick instead cost a `performInvocation` → `refreshUi` → React
 * commit for every notch of the wheel; publishing never would lose the camera the next open honours. */
export function createFlowCameraGesture(ports: FlowCameraGesturePorts, settleMs: number = FLOW_CAMERA_GESTURE_SETTLE_MS) {
  let settle: unknown = null;
  return {
    tick(): void {
      if (settle === null) ports.begin("wheel");
      else ports.cancel(settle);
      settle = ports.schedule(() => {
        settle = null;
        ports.end("wheel");
        ports.publish();
      }, settleMs);
      ports.invalidate();
    },
    active(): boolean {
      return settle !== null;
    },
    dispose(): void {
      if (settle !== null) ports.cancel(settle);
      settle = null;
    },
  };
}

/** 🎥️ Whether a press starts a camera PAN rather than a content gesture — the middle button, or a
 * pointer already held on it. A pan moves the board's own camera and changes neither selection nor
 * fixture, so its release owes the plugin one viewport publication and nothing else. */
export function flowGestureIsCameraPan(button: number, buttons: number): boolean {
  return button === 1 || buttons === 4;
}

/** 🪧️ The one frame verdict this host has to act on — `FlowPresentation.unpresentable` in
 * `🖥️flow-host.js`, spelled here the way every other flow surface word (`"created"`,
 * `"cancelled"`, `"device-lost"`) is spelled on both sides of the byte ABI.
 *
 * It means the frame wants the 2D draw-list replay and the canvas element cannot give a 2D context:
 * a WebGPU context is bound to it and its device is gone (or a bring-up bound one and then failed).
 * A canvas admits exactly one context kind for its whole life, so that element is finished — the
 * presentation can only be decided again for a NEW one, which is what {@link flowSurfaceCanvasKey}
 * mints. */
export const flowUnpresentablePresentation = "unpresentable";

/** 🔑️ React key for the flow scene canvas. A new generation is a new DOM element, which is the only
 * way a surface that lost its device can be presented again. */
export function flowSurfaceCanvasKey(surfaceId: string, generation: number): string {
  return `${surfaceId}#${generation}`;
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
