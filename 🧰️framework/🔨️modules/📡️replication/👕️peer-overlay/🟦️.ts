/** @emoji 👕️ Pure peer-overlay derivation from presence roster + local window (contract-freeze §C7.8).
 * Ephemeral shared only — never persisted. Twin of `👕️peer-overlay/🦀️.rs`.
 */

export type PeerOverlayKind = "Camera" | "Cursor" | "Marks" | "Caret" | "Playhead";

export type PresenceViewKindInput =
  | { readonly kind: "canvas"; readonly x: number; readonly y: number; readonly zoom: number }
  | { readonly kind: "orbit"; readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly up: readonly [number, number, number]; readonly fov: number }
  | { readonly kind: "geo"; readonly lng: number; readonly lat: number; readonly zoom: number; readonly bearing: number; readonly pitch: number };

export type PresenceWindowViewInput = {
  readonly windowId: string;
  readonly space: string;
  readonly kind: PresenceViewKindInput;
  readonly size: readonly [number, number];
  readonly pointer?: readonly [number, number, number];
  readonly rayOrigin?: readonly [number, number, number];
};

export type PresenceDomainInput = {
  readonly domain: string;
  readonly granularity: string;
  readonly selected: readonly string[];
  readonly hovered: readonly string[];
};

export type PresencePeerInput = {
  readonly actor: string;
  readonly label?: string;
  readonly color?: number;
  readonly surface?: string;
  readonly views: readonly PresenceWindowViewInput[];
  readonly interaction?: { readonly app_id: string; readonly domains: readonly PresenceDomainInput[] };
  readonly activeTool?: string;
};

export type UiPeerMark = {
  readonly actor: string;
  readonly color: number | undefined;
  readonly hovered: boolean;
  readonly selected: boolean;
  readonly label: string;
};

export type PeerView = {
  readonly actor: string;
  readonly label: string;
  readonly color: number | undefined;
  readonly view: PresenceViewKindInput;
  readonly pointer: readonly [number, number, number] | undefined;
  readonly rayOrigin: readonly [number, number, number] | undefined;
  readonly size: readonly [number, number];
  readonly windowId: string;
  readonly space: string;
  readonly activeTool: string | undefined;
};

export type PeerOverlaySpec = {
  readonly windowId: string;
  readonly localColor: number;
  readonly artifactPeers: readonly PeerView[];
  readonly marks: Readonly<Record<string, Readonly<Record<string, readonly UiPeerMark[]>>>>;
};

export function peerOverlayPath(scenePath: string, kind: PeerOverlayKind, index: number, key: string): string {
  const segment =
    kind === "Camera" ? "peerCamera" : kind === "Cursor" ? "peerCursor" : kind === "Marks" ? "peerMarks" : kind === "Caret" ? "peerCaret" : "peerPlayhead";
  return `${scenePath}/${segment}[${index}]#${key}`;
}

/** 👕️ Views filtered to matching `windowId`+`space`; own actor excluded. Marks fold only matching peers. */
export function peersForWindow(
  roster: readonly PresencePeerInput[],
  windowId: string,
  space: string,
  _mySurface: string | undefined,
  myActor: string,
  localColor: number,
): PeerOverlaySpec {
  const artifactPeers: PeerView[] = [];
  const marks: Record<string, Record<string, UiPeerMark[]>> = {};
  for (const peer of roster) {
    if (peer.actor === myActor) continue;
    let matched = false;
    for (const view of peer.views) {
      if (view.windowId !== windowId || view.space !== space) continue;
      matched = true;
      artifactPeers.push({
        actor: peer.actor,
        label: peer.label ?? peer.actor,
        color: peer.color,
        view: view.kind,
        pointer: view.pointer,
        rayOrigin: view.rayOrigin,
        size: view.size,
        windowId: view.windowId,
        space: view.space,
        activeTool: peer.activeTool,
      });
    }
    if (!matched) continue;
    const interaction = peer.interaction;
    if (!interaction) continue;
    for (const domain of interaction.domains) {
      const byId = (marks[domain.domain] ??= {});
      for (const id of domain.selected) {
        const list = (byId[id] ??= []);
        const existing = list.find((mark) => mark.actor === peer.actor);
        if (existing) Object.assign(existing, { selected: true });
        else list.push({ actor: peer.actor, color: peer.color, hovered: false, selected: true, label: peer.label ?? peer.actor });
      }
      for (const id of domain.hovered) {
        const list = (byId[id] ??= []);
        const existing = list.find((mark) => mark.actor === peer.actor);
        if (existing) Object.assign(existing, { hovered: true });
        else list.push({ actor: peer.actor, color: peer.color, hovered: true, selected: false, label: peer.label ?? peer.actor });
      }
    }
  }
  artifactPeers.sort((a, b) => (a.actor !== b.actor ? (a.actor < b.actor ? -1 : 1) : a.windowId < b.windowId ? -1 : a.windowId > b.windowId ? 1 : 0));
  return { windowId, localColor, artifactPeers, marks };
}

export function peerMarksFor(spec: PeerOverlaySpec, domain: string, id: string): readonly UiPeerMark[] {
  return spec.marks[domain]?.[id] ?? [];
}

export function canvasPointToScreen(
  localView: { readonly x: number; readonly y: number; readonly zoom: number },
  localSizePx: readonly [number, number],
  world: readonly [number, number],
): readonly [number, number] {
  const zoom = localView.zoom || 1;
  return [(world[0] - localView.x) * zoom + localSizePx[0] / 2, (world[1] - localView.y) * zoom + localSizePx[1] / 2];
}

export function canvasPeerViewportRect(
  peerView: { readonly x: number; readonly y: number; readonly zoom: number },
  peerSize: readonly [number, number],
  localView: { readonly x: number; readonly y: number; readonly zoom: number },
  localSizePx: readonly [number, number],
): readonly [number, number, number, number] {
  const peerZoom = peerView.zoom || 1;
  const halfW = peerSize[0] / (2 * peerZoom);
  const halfH = peerSize[1] / (2 * peerZoom);
  const corners: Array<readonly [number, number]> = [
    [peerView.x - halfW, peerView.y - halfH],
    [peerView.x + halfW, peerView.y - halfH],
    [peerView.x + halfW, peerView.y + halfH],
    [peerView.x - halfW, peerView.y + halfH],
  ];
  const screens = corners.map((corner) => canvasPointToScreen(localView, localSizePx, corner));
  const xs = screens.map((s) => s[0]);
  const ys = screens.map((s) => s[1]);
  const minX = Math.min(...xs);
  const minY = Math.min(...ys);
  return [minX, minY, Math.max(...xs) - minX, Math.max(...ys) - minY];
}


/** Project a world-space point through a local orbit camera into screen pixels. Returns null when behind the camera. */
export function orbitPointToScreen(
  localView: {
    readonly position: readonly [number, number, number];
    readonly target: readonly [number, number, number];
    readonly up: readonly [number, number, number];
    readonly fov: number;
  },
  localSizePx: readonly [number, number],
  world: readonly [number, number, number],
): readonly [number, number] | null {
  const forward = normalize([localView.target[0] - localView.position[0], localView.target[1] - localView.position[1], localView.target[2] - localView.position[2]]);
  const right = normalize(cross(forward, localView.up));
  const trueUp = cross(right, forward);
  const rel: readonly [number, number, number] = [world[0] - localView.position[0], world[1] - localView.position[1], world[2] - localView.position[2]];
  const camZ = rel[0] * forward[0] + rel[1] * forward[1] + rel[2] * forward[2];
  if (camZ <= 1e-9) return null;
  const camX = rel[0] * right[0] + rel[1] * right[1] + rel[2] * right[2];
  const camY = rel[0] * trueUp[0] + rel[1] * trueUp[1] + rel[2] * trueUp[2];
  const aspect = localSizePx[0] / (localSizePx[1] || 1);
  const halfV = Math.tan((localView.fov * Math.PI) / 360);
  const halfH = halfV * aspect;
  const ndcX = camX / (camZ * halfH);
  const ndcY = camY / (camZ * halfV);
  return [(ndcX + 1) * 0.5 * localSizePx[0], (1 - ndcY) * 0.5 * localSizePx[1]];
}

export function orbitFrustumCorners(
  position: readonly [number, number, number],
  target: readonly [number, number, number],
  up: readonly [number, number, number],
  fovDeg: number,
  aspect: number,
  depth: number,
): readonly (readonly [number, number, number])[] {
  const forward = normalize([target[0] - position[0], target[1] - position[1], target[2] - position[2]]);
  const right = normalize(cross(forward, up));
  const trueUp = cross(right, forward);
  const halfV = Math.tan((fovDeg * Math.PI) / 360) * depth;
  const halfH = halfV * aspect;
  const far: readonly [number, number, number] = [position[0] + forward[0] * depth, position[1] + forward[1] * depth, position[2] + forward[2] * depth];
  return [
    position,
    add(far, scale(trueUp, halfV), scale(right, -halfH)),
    add(far, scale(trueUp, halfV), scale(right, halfH)),
    add(far, scale(trueUp, -halfV), scale(right, halfH)),
    add(far, scale(trueUp, -halfV), scale(right, -halfH)),
  ];
}

export function orbitFrustumSegments(
  corners: readonly (readonly [number, number, number])[],
): readonly (readonly [readonly [number, number, number], readonly [number, number, number]])[] {
  const [apex, a, b, c, d] = corners;
  return [[apex, a], [apex, b], [apex, c], [apex, d], [a, b], [b, c], [c, d], [d, a]];
}

function cross(a: readonly [number, number, number], b: readonly [number, number, number]): readonly [number, number, number] {
  return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
}
function normalize(v: readonly [number, number, number]): readonly [number, number, number] {
  const len = Math.hypot(v[0], v[1], v[2]) || 1;
  return [v[0] / len, v[1] / len, v[2] / len];
}
function scale(v: readonly [number, number, number], s: number): readonly [number, number, number] {
  return [v[0] * s, v[1] * s, v[2] * s];
}
function add(...vs: readonly (readonly [number, number, number])[]): readonly [number, number, number] {
  let x = 0, y = 0, z = 0;
  for (const v of vs) { x += v[0]; y += v[1]; z += v[2]; }
  return [x, y, z];
}

export function presenceColorCss(index: number | undefined): string {
  const i = typeof index === "number" && Number.isFinite(index) ? Math.abs(Math.trunc(index)) % 12 : 0;
  return `var(--presence-${i})`;
}

export const PEER_OVERLAY_LABELS = {
  en: {
    cursor: (name: string) => `${name}'s cursor`,
    viewport: (name: string) => `${name}'s viewport`,
    selection: (name: string) => `${name}'s selection`,
    hover: (name: string) => `${name}'s hover`,
    caret: (name: string) => `${name}'s caret`,
    tool: (name: string, tool: string) => `${name}: ${tool}`,
  },
  de: {
    cursor: (name: string) => `Cursor von ${name}`,
    viewport: (name: string) => `Ansicht von ${name}`,
    selection: (name: string) => `Auswahl von ${name}`,
    hover: (name: string) => `Hover von ${name}`,
    caret: (name: string) => `Caret von ${name}`,
    tool: (name: string, tool: string) => `${name}: ${tool}`,
  },
} as const;

export type PeerOverlayLocale = keyof typeof PEER_OVERLAY_LABELS;

export function peerOverlayLabels(locale: string | undefined): (typeof PEER_OVERLAY_LABELS)["en"] {
  return locale === "de" ? PEER_OVERLAY_LABELS.de : PEER_OVERLAY_LABELS.en;
}
