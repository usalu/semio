/** @emoji 👕️ In-canvas remote presence overlays — peer cursors, viewport rects, selection chips.
 * Domain-neutral; Board2d / World3d / TextEditor hosts mount this over their surface.
 */
import { useMemo, useSyncExternalStore, type CSSProperties, type ReactElement } from "react";
import {
  canvasPeerViewportRect,
  canvasPointToScreen,
  orbitPointToScreen,
  peerOverlayLabels,
  peerOverlayPath,
  peersForWindow,
  presenceColorCss,
  type PresencePeerInput,
} from "@semio-tech/framework-replication";
import {
  artifactPresenceRosterV1,
  localPresenceActorV1,
  subscribeArtifactPresenceRosterV1,
  subscribeLocalPresenceActorV1,
} from "./🟦️.ts";

export type CanvasPresenceSurfaceV1 = {
  readonly runtimeKey: string | null | undefined;
  readonly windowId: string;
  readonly space: "canvas" | "world" | "geo" | string;
  readonly myActor: string;
  readonly mySurface?: string;
  readonly localColor?: number;
  readonly locale?: string;
  readonly scenePath?: string;
  readonly localCanvas?: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly localOrbit?: {
    readonly position: readonly [number, number, number];
    readonly target: readonly [number, number, number];
    readonly up: readonly [number, number, number];
    readonly fov: number;
  };
  readonly localSizePx?: readonly [number, number];
  readonly objectWorldPositions?: Readonly<Record<string, readonly [number, number, number]>>;
  readonly selectedIds?: readonly string[];
  readonly domain?: string;
};

function useArtifactRoster(runtimeKey: string | null | undefined): readonly PresencePeerInput[] {
  return useSyncExternalStore(
    subscribeArtifactPresenceRosterV1,
    () => artifactPresenceRosterV1(runtimeKey) as readonly PresencePeerInput[],
    () => [],
  );
}

function admittedActorId(candidate: string | null | undefined): string | null {
  if (!candidate || candidate === "__local__") return null;
  return candidate;
}

function useLocalPresenceActor(runtimeKey: string | null | undefined, fallback: string): string | null {
  return useSyncExternalStore(
    subscribeLocalPresenceActorV1,
    () => admittedActorId(localPresenceActorV1(runtimeKey)) ?? admittedActorId(fallback),
    () => admittedActorId(fallback),
  );
}

/** Hub-admitted local actor id for host self-exclusion (null until socket-actor). */
export function useLocalPresenceActorIdV1(runtimeKey: string | null | undefined = "local"): string | null {
  return useSyncExternalStore(subscribeLocalPresenceActorV1, () => admittedActorId(localPresenceActorV1(runtimeKey)), () => null);
}

/** 👕️ Renders foreign cursors / viewport rects / selection marks for one host surface. */
export function CanvasPresenceOverlayV1(props: CanvasPresenceSurfaceV1): ReactElement | null {
  const roster = useArtifactRoster(props.runtimeKey);
  const myActor = useLocalPresenceActor(props.runtimeKey, props.myActor);
  const labels = peerOverlayLabels(props.locale);
  const spec = useMemo(
    () => (myActor ? peersForWindow(roster, props.windowId, props.space, props.mySurface, myActor, props.localColor ?? 0) : { windowId: props.windowId, localColor: props.localColor ?? 0, artifactPeers: [], marks: {} }),
    [roster, props.windowId, props.space, props.mySurface, myActor, props.localColor],
  );
  const scenePath = props.scenePath ?? "scene";
  if (!myActor) return null;
  if (spec.artifactPeers.length === 0 && Object.keys(spec.marks).length === 0) return null;

  const localCanvas = props.localCanvas;
  const localSize = props.localSizePx;

  return (
    <div className="pointer-events-none absolute inset-0 z-[15] overflow-hidden" data-slot="canvas-presence-overlay" data-testid="canvas-presence-overlay" aria-hidden={false}>
      {spec.artifactPeers.map((peer, index) => {
        const color = presenceColorCss(peer.color);
        const nodes: ReactElement[] = [];
        if (peer.view.kind === "canvas" && localCanvas && localSize && peer.pointer) {
          const screen = canvasPointToScreen(localCanvas, localSize, [peer.pointer[0], peer.pointer[1]]);
          const rect = canvasPeerViewportRect(peer.view, peer.size, localCanvas, localSize);
          nodes.push(
            <div
              key={`vp-${peer.actor}-${peer.windowId}`}
              data-ui-path={peerOverlayPath(scenePath, "Camera", index, peer.actor)}
              data-peer-actor={peer.actor}
              data-peer-color={peer.color ?? 0}
              data-peer-viewport=""
              aria-label={labels.viewport(peer.label)}
              style={{
                position: "absolute",
                left: rect[0],
                top: rect[1],
                width: rect[2],
                height: rect[3],
                border: `1.5px solid ${color}`,
                borderRadius: 2,
                opacity: 0.55,
                boxSizing: "border-box",
              }}
            />,
          );
          nodes.push(
            <div
              key={`cur-${peer.actor}-${peer.windowId}`}
              data-ui-path={peerOverlayPath(scenePath, "Cursor", index, peer.actor)}
              data-peer-actor={peer.actor}
              data-peer-color={peer.color ?? 0}
              data-peer-cursor="" data-testid="peer-cursor"
              aria-label={labels.cursor(peer.label)}
              style={{
                position: "absolute",
                left: screen[0],
                top: screen[1],
                transform: "translate(-2px, -2px)",
                width: 10,
                height: 10,
                borderRadius: 999,
                background: color,
                boxShadow: `0 0 0 2px color-mix(in srgb, ${color} 35%, transparent)`,
              } as CSSProperties}
              title={peer.activeTool ? labels.tool(peer.label, peer.activeTool) : peer.label}
              data-peer-active-tool={peer.activeTool ?? ""}
            >
              <span
                style={{
                  position: "absolute",
                  left: 12,
                  top: -2,
                  fontSize: 10,
                  lineHeight: 1.2,
                  padding: "1px 4px",
                  borderRadius: 3,
                  background: color,
                  color: "white",
                  whiteSpace: "nowrap",
                }}
              >
                {peer.activeTool ? labels.tool(peer.label, peer.activeTool) : peer.label}
              </span>
            </div>,
          );
        } else if (peer.pointer && localSize && props.localOrbit && peer.view.kind === "orbit") {
          const screen = orbitPointToScreen(props.localOrbit, localSize, peer.pointer);
          if (screen) {
            nodes.push(
              <div
                key={`cur3-${peer.actor}-${peer.windowId}`}
                data-ui-path={peerOverlayPath(scenePath, "Cursor", index, peer.actor)}
                data-peer-actor={peer.actor}
                data-peer-color={peer.color ?? 0}
                data-peer-cursor="" data-testid="peer-cursor"
                data-peer-cursor-world=""
                data-peer-hit={`${peer.pointer[0]},${peer.pointer[1]},${peer.pointer[2]}`}
                data-peer-ray-origin={peer.rayOrigin ? `${peer.rayOrigin[0]},${peer.rayOrigin[1]},${peer.rayOrigin[2]}` : ""}
                aria-label={labels.cursor(peer.label)}
                style={{
                  position: "absolute",
                  left: screen[0],
                  top: screen[1],
                  transform: "translate(-50%, -50%)",
                  width: 12,
                  height: 12,
                  borderRadius: 999,
                  background: color,
                  boxShadow: `0 0 0 2px color-mix(in srgb, ${color} 35%, transparent)`,
                } as CSSProperties}
                title={peer.activeTool ? labels.tool(peer.label, peer.activeTool) : peer.label}
                data-peer-active-tool={peer.activeTool ?? ""}
              >
                <span
                  style={{
                    position: "absolute",
                    left: 14,
                    top: -2,
                    fontSize: 10,
                    padding: "1px 4px",
                    borderRadius: 3,
                    background: color,
                    color: "white",
                    whiteSpace: "nowrap",
                  }}
                >
                  {peer.activeTool ? labels.tool(peer.label, peer.activeTool) : peer.label}
                </span>
              </div>,
            );
          }
        }
        return nodes;
      })}
      {props.domain &&
        Object.entries(spec.marks[props.domain] ?? {}).flatMap(([id, marks], index) =>
          marks.map((mark) => {
            const world = props.objectWorldPositions?.[id];
            const projected =
              world && props.localOrbit && localSize ? orbitPointToScreen(props.localOrbit, localSize, world) : null;
            // Orbit hosts paint projected selection rings only — never a normalized HUD stack.
            if (props.localOrbit && !projected) return null;
            return (
              <div
                key={`mark-${mark.actor}-${id}`}
                data-ui-path={peerOverlayPath(scenePath, "Marks", index, `${props.domain}:${id}`)}
                data-peer-actor={mark.actor}
                data-peer-color={mark.color ?? 0}
                data-peer-marks=""
                data-peer-mark={mark.selected ? "selection" : "hover"}
                data-testid={mark.selected ? "peer-selection" : "peer-hover"}
                data-peer-selection-id={mark.selected ? id : undefined}
                aria-label={mark.selected ? labels.selection(mark.label) : labels.hover(mark.label)}
                style={
                  projected
                    ? {
                        position: "absolute",
                        left: projected[0],
                        top: projected[1],
                        transform: "translate(-50%, -50%)",
                        width: 18,
                        height: 18,
                        borderRadius: 4,
                        border: `2px solid ${presenceColorCss(mark.color)}`,
                        boxSizing: "border-box",
                        background: "transparent",
                      }
                    : {
                        position: "absolute",
                        right: 8,
                        top: 8 + index * 18,
                        fontSize: 10,
                        padding: "1px 6px",
                        borderRadius: 999,
                        background: presenceColorCss(mark.color),
                        color: "white",
                      }
                }
              >
                {projected ? null : mark.label.slice(0, 2).toUpperCase()}
              </div>
            );
          }),
        )}
    </div>
  );
}
