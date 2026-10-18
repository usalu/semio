import * as THREE from "three";
import type { GumballHandleKind, GumballVec3 } from "../🧿️gumball-model/🟦️.ts";

/** 📡️ World-space camera ray through an NDC point — orthographic uses parallel near→far unproject rays;
 * perspective uses the pinhole from `camera.position`. Duck-types `isOrthographicCamera` (not `instanceof`) so
 * R3F-swapped cameras from a duplicate `three` copy stay correct (otherwise ortho drags fly away from the cursor). */
export function gumballRayFromNdc(ndcX: number, ndcY: number, camera: THREE.Camera): { origin: GumballVec3; dir: GumballVec3 } {
  (camera as THREE.Camera & { updateMatrixWorld?: (force?: boolean) => void }).updateMatrixWorld?.(true);
  const ortho = camera as THREE.OrthographicCamera & { readonly isOrthographicCamera?: boolean };
  if (ortho.isOrthographicCamera) {
    const origin = new THREE.Vector3(ndcX, ndcY, -1).unproject(camera);
    const far = new THREE.Vector3(ndcX, ndcY, 1).unproject(camera);
    const dir = far.sub(origin);
    if (dir.lengthSq() < 1e-12) return { origin: [origin.x, origin.y, origin.z], dir: [0, 0, -1] };
    dir.normalize();
    return { origin: [origin.x, origin.y, origin.z], dir: [dir.x, dir.y, dir.z] };
  }
  const origin = camera.position.clone();
  const point = new THREE.Vector3(ndcX, ndcY, 0.5).unproject(camera);
  const dir = point.sub(origin);
  if (dir.lengthSq() < 1e-12) return { origin: [origin.x, origin.y, origin.z], dir: [0, 0, -1] };
  dir.normalize();
  return { origin: [origin.x, origin.y, origin.z], dir: [dir.x, dir.y, dir.z] };
}

export function gumballNdcFromPointer(clientX: number, clientY: number, rect: DOMRect): { x: number; y: number } {
  return {
    x: ((clientX - rect.left) / rect.width) * 2 - 1,
    y: -(((clientY - rect.top) / rect.height) * 2 - 1),
  };
}

const _gumballPickRaycaster = new THREE.Raycaster();
const _gumballPickNdc = new THREE.Vector2();

/** 🎯️ World-space depth bias so gumball handles win picking over occluding scene geometry (see puzzle vortex pick priority). */
const GUMBALL_PICK_DEPTH_BIAS = 1.5;

/** 🎯️ Mesh raycast biasing gumball hits closer so occluding meshes do not swallow handle interaction. */
export function gumballHandleRaycast(this: THREE.Mesh, raycaster: THREE.Raycaster, intersects: THREE.Intersection[]): void {
  const local: THREE.Intersection[] = [];
  THREE.Mesh.prototype.raycast.call(this, raycaster, local);
  for (const hit of local) {
    hit.distance = Math.max(hit.distance - GUMBALL_PICK_DEPTH_BIAS, hit.distance * 0.01);
    intersects.push(hit);
  }
}

export function gumballBindHandleRaycast(root: THREE.Object3D): void {
  root.traverse((node) => {
    if (node instanceof THREE.Mesh) {
      node.raycast = gumballHandleRaycast;
    }
  });
}

/** 🎛️ Raycasts a gumball root subtree for a handle at a client-space pointer. */
export function gumballRaycastOwnedAtClientPoint(camera: THREE.Camera, canvas: HTMLElement, clientX: number, clientY: number, root: THREE.Object3D): { readonly kind: GumballHandleKind; readonly object: THREE.Object3D } | null {
  const rect = canvas.getBoundingClientRect();
  if (rect.width <= 0 || rect.height <= 0) return null;
  root.updateWorldMatrix(true, false);
  const ndc = gumballNdcFromPointer(clientX, clientY, rect);
  _gumballPickNdc.set(ndc.x, ndc.y);
  _gumballPickRaycaster.setFromCamera(_gumballPickNdc, camera);
  const hits = _gumballPickRaycaster.intersectObjects([root], true);
  for (const hit of hits) {
    const kind = gumballKindFromRaycastObject(hit.object);
    if (kind) return { kind, object: hit.object };
  }
  return null;
}

/** 🎛️ True while a gumball handle owns the active canvas pointer (blocks marquee / background pick). */
export const gumballPointerConsumesCanvasEventRef = { current: false };

/** 🎛️ Reads a gumball handle kind from a raycast hit object or its parents. */
export function gumballKindFromRaycastObject(object: THREE.Object3D | null): GumballHandleKind | null {
  let node: THREE.Object3D | null = object;
  while (node) {
    const kind = node.userData?.gumballHandleKind;
    if (typeof kind === "string") return kind as GumballHandleKind;
    node = node.parent;
  }
  return null;
}

/** 🎛️ Raycasts the scene for a gumball handle at a client-space pointer (any depth; not limited to the closest scene hit). */
export function gumballRaycastAtClientPoint(scene: THREE.Scene, camera: THREE.Camera, canvas: HTMLElement, clientX: number, clientY: number): { readonly kind: GumballHandleKind; readonly object: THREE.Object3D } | null {
  const rect = canvas.getBoundingClientRect();
  if (rect.width <= 0 || rect.height <= 0) return null;
  const ndc = gumballNdcFromPointer(clientX, clientY, rect);
  _gumballPickNdc.set(ndc.x, ndc.y);
  _gumballPickRaycaster.setFromCamera(_gumballPickNdc, camera);
  const hits = _gumballPickRaycaster.intersectObjects(scene.children, true);
  for (const hit of hits) {
    const kind = gumballKindFromRaycastObject(hit.object);
    if (kind) return { kind, object: hit.object };
  }
  return null;
}

/** 🎛️ Raycasts the scene for a gumball handle kind at a client-space pointer. */
export function gumballRaycastKindAtClientPoint(scene: THREE.Scene, camera: THREE.Camera, canvas: HTMLElement, clientX: number, clientY: number): GumballHandleKind | null {
  return gumballRaycastAtClientPoint(scene, camera, canvas, clientX, clientY)?.kind ?? null;
}
