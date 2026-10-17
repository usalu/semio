/** 🎊️ Instance celebration: the transient celebrating-instance store with its prune timer and subscription, the conic spin angle, the conic shaders and the celebrating conic material factory. */
import { useMemo, useSyncExternalStore } from "react";
import { clearColorResolveCache, resolveColorHex, tokenVar } from "@semio-tech/ui-styling";
import { CELEBRATE_STAMP_DURATION_MS, Color, DoubleSide, ShaderMaterial } from "@semio-tech/ui-react";

//#region 🔖️World3dHost
//#region WorldSceneParsing
//#region WorldMeshPaint
//#region 🎉️WorldInstanceCelebrate
/** 🎉️ Expiry timestamps keyed by world instance id — transient completion paint after catalogue drop (mirrors DOM `celebrateElements`). */
const celebratingWorldInstanceUntil = new Map<string, number>();
const celebratingWorldInstanceListeners = new Set<() => void>();
let celebratingWorldInstanceVersion = 0;
let celebratingWorldInstanceTimer: number | null = null;

function notifyCelebratingWorldInstances(): void {
  celebratingWorldInstanceVersion += 1;
  celebratingWorldInstanceListeners.forEach((listener) => listener());
}

function pruneCelebratingWorldInstances(now = performance.now()): void {
  let changed = false;
  for (const [id, until] of celebratingWorldInstanceUntil) {
    if (until <= now) {
      celebratingWorldInstanceUntil.delete(id);
      changed = true;
    }
  }
  if (changed) notifyCelebratingWorldInstances();
}

function scheduleCelebratingWorldInstancePrune(): void {
  if (celebratingWorldInstanceTimer != null) window.clearTimeout(celebratingWorldInstanceTimer);
  let next = Infinity;
  const now = performance.now();
  for (const until of celebratingWorldInstanceUntil.values()) {
    if (until > now && until < next) next = until;
  }
  if (!Number.isFinite(next)) {
    celebratingWorldInstanceTimer = null;
    return;
  }
  celebratingWorldInstanceTimer = window.setTimeout(() => {
    celebratingWorldInstanceTimer = null;
    pruneCelebratingWorldInstances();
    scheduleCelebratingWorldInstancePrune();
  }, Math.max(0, next - now + 1));
}

/** 🎉️ Stamps world instance ids as celebrating for `durationMs` so their mesh paint uses the spinning conic brand gradient instead of selected. */
export function celebrateWorldInstances(ids: readonly string[], durationMs = CELEBRATE_STAMP_DURATION_MS): () => void {
  const until = performance.now() + durationMs;
  for (const id of ids) celebratingWorldInstanceUntil.set(id, until);
  notifyCelebratingWorldInstances();
  scheduleCelebratingWorldInstancePrune();
  return () => {
    let changed = false;
    for (const id of ids) {
      if (celebratingWorldInstanceUntil.delete(id)) changed = true;
    }
    if (changed) notifyCelebratingWorldInstances();
  };
}

/** 🎉️ Whether `id` is still inside its transient celebration window. */
export function isWorldInstanceCelebrating(id: string, now = performance.now()): boolean {
  const until = celebratingWorldInstanceUntil.get(id);
  return until != null && until > now;
}

function subscribeCelebratingWorldInstances(listener: () => void): () => void {
  celebratingWorldInstanceListeners.add(listener);
  return () => {
    celebratingWorldInstanceListeners.delete(listener);
  };
}

function celebratingWorldInstanceSnapshot(): number {
  return celebratingWorldInstanceVersion;
}

/** 🎉️ React subscription to the transient celebrating-instance set — re-renders when stamps are added or expire. */
export function useCelebratingWorldInstanceIds(): ReadonlySet<string> {
  const version = useSyncExternalStore(subscribeCelebratingWorldInstances, celebratingWorldInstanceSnapshot, celebratingWorldInstanceSnapshot);
  return useMemo(() => {
    const now = performance.now();
    const ids = new Set<string>();
    for (const [id, until] of celebratingWorldInstanceUntil) {
      if (until > now) ids.add(id);
    }
    return ids;
  }, [version]);
}

/** 🎉️ One spin period of the celebrate conic — matches `--celebrate-border-duration` (1.2s). */
const CELEBRATE_CONIC_SPIN_SECONDS = 1.2;

/** 🎉️ Document-timeline celebrate spin angle — phase-locked to the CSS `:root` celebrate-border-spin clock. */
export function celebrateConicAngleRadians(): number {
  const timelineTime = typeof document !== "undefined" ? document.timeline?.currentTime : null;
  const timeMs = typeof timelineTime === "number" ? timelineTime : performance.now();
  return (((timeMs / 1000) % CELEBRATE_CONIC_SPIN_SECONDS) / CELEBRATE_CONIC_SPIN_SECONDS) * Math.PI * 2;
}

export const CELEBRATE_CONIC_VERTEX_SHADER = /* glsl */ `
varying vec3 vObjectPosition;
void main() {
  vObjectPosition = position;
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`;

export const CELEBRATE_CONIC_FRAGMENT_SHADER = /* glsl */ `
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform vec3 uColorC;
uniform float uAngle;
uniform float uOpacity;
varying vec3 vObjectPosition;
void main() {
  float a = atan(vObjectPosition.y, vObjectPosition.x) + uAngle;
  float t = fract(a / 6.28318530718);
  vec3 color;
  if (t < 0.333333) {
    color = mix(uColorA, uColorB, t / 0.333333);
  } else if (t < 0.666667) {
    color = mix(uColorB, uColorC, (t - 0.333333) / 0.333333);
  } else {
    color = mix(uColorC, uColorA, (t - 0.666667) / 0.333333);
  }
  gl_FragColor = vec4(color, uOpacity);
}
`;

export function hexToRgb01(hex: string): [number, number, number] {
  const color = new Color(hex);
  return [color.r, color.g, color.b];
}

export function createCelebratingConicMaterial(opacity = 1): ShaderMaterial {
  clearColorResolveCache();
  const a = hexToRgb01(resolveColorHex(tokenVar("primary")));
  const b = hexToRgb01(resolveColorHex(tokenVar("secondary")));
  const c = hexToRgb01(resolveColorHex(tokenVar("tertiary")));
  return new ShaderMaterial({
    vertexShader: CELEBRATE_CONIC_VERTEX_SHADER,
    fragmentShader: CELEBRATE_CONIC_FRAGMENT_SHADER,
    uniforms: {
      uColorA: { value: a },
      uColorB: { value: b },
      uColorC: { value: c },
      uAngle: { value: 0 },
      uOpacity: { value: opacity },
    },
    transparent: opacity < 1,
    side: DoubleSide,
    depthWrite: opacity >= 1,
  });
}
//#endregion 🎉️WorldInstanceCelebrate
//#endregion WorldMeshPaint
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
