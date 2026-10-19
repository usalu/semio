/** 🧱️ Collision bodies from three objects, containment and intersection tests, and solid overlap volume.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { MeshBVH, type HitPointInfo } from "@semio-tech/ui-react";
import {
  Box3,
  BufferGeometry,
  Group,
  Matrix4,
  Mesh,
  Object3D,
  Ray,
  Vector3
} from "../🔌️adapters/🟦️.ts";
import { GLB_MESH_FRAME_ROTATION_X } from "../🧷️r3f-foundation/🟦️.ts";

// #region Collision
/** 🧊️ One mesh BVH part in pose-local coordinates (GLB frame rotation baked in). */
export interface CollisionMeshPart {
  readonly geometry: BufferGeometry;
  readonly bvh: MeshBVH;
  readonly localMatrix: Matrix4;
}

/** 🧊️ Mesh-backed collision volume built from a GLB root. */
export interface CollisionBody {
  readonly parts: readonly CollisionMeshPart[];
  readonly localBounds: Box3;
}

const _collisionHit: HitPointInfo = { point: new Vector3(), distance: 0, faceIndex: 0 };
const _collisionBoxA = new Box3();
const _collisionBoxB = new Box3();
const _collisionInterBox = new Box3();
const _collisionSize = new Vector3();
const _collisionPoint = new Vector3();
const _collisionLocal = new Vector3();
const _collisionPartLocal = new Vector3();
const _collisionMatA = new Matrix4();
const _collisionMatB = new Matrix4();
const _collisionAtoB = new Matrix4();
const _collisionInvA = new Matrix4();
const _collisionInvB = new Matrix4();
const _collisionRay = new Ray();
const _collisionVa = new Vector3();
const _collisionVb = new Vector3();
const _collisionVc = new Vector3();
const _collisionNormal = new Vector3();
const _collisionToPoint = new Vector3();

/** 🧊️ Builds a {@link CollisionBody} from a GLB root in pose-local space (includes {@link GLB_MESH_FRAME_ROTATION_X}). */
export function collisionBodyFromObject(root: Object3D): CollisionBody | null {
  const poseLocal = new Group();
  const frame = new Group();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.add(root.clone(true));
  poseLocal.add(frame);
  poseLocal.updateMatrixWorld(true);
  const poseInverse = poseLocal.matrixWorld.clone().invert();
  const parts: CollisionMeshPart[] = [];
  poseLocal.traverse((obj) => {
    const mesh = obj as Mesh;
    if (!mesh.isMesh || !mesh.geometry) {
      return;
    }
    const position = mesh.geometry.getAttribute("position");
    if (!position || position.count === 0) {
      return;
    }
    const geometry = mesh.geometry.clone();
    const meshToPose = new Matrix4().multiplyMatrices(poseInverse, mesh.matrixWorld);
    geometry.applyMatrix4(meshToPose);
    geometry.computeBoundingBox();
    const bvh = new MeshBVH(geometry);
    parts.push({ geometry, bvh, localMatrix: new Matrix4().identity() });
  });
  if (parts.length === 0) {
    return null;
  }
  const localBounds = new Box3();
  let hasBounds = false;
  for (const part of parts) {
    const partBox = new Box3();
    part.bvh.getBoundingBox(partBox);
    if (!Number.isFinite(partBox.min.x) || partBox.isEmpty()) {
      continue;
    }
    if (!hasBounds) {
      localBounds.copy(partBox);
      hasBounds = true;
    } else {
      localBounds.union(partBox);
    }
  }
  if (!hasBounds) {
    return null;
  }
  return { parts, localBounds };
}

/** 📦️ World-space AABB for a posed {@link CollisionBody}. */
export function collisionBodyWorldBounds(body: CollisionBody, worldMatrix: Matrix4, target = new Box3()): Box3 {
  target.copy(body.localBounds).applyMatrix4(worldMatrix);
  return target;
}

function collisionPointInsidePart(part: CollisionMeshPart, pointPoseLocal: Vector3): boolean {
  _collisionPartLocal.copy(pointPoseLocal).applyMatrix4(part.localMatrix.clone().invert());
  const hit = part.bvh.closestPointToPoint(_collisionPartLocal, _collisionHit, 0, Infinity);
  if (!hit) {
    return false;
  }
  if (hit.distance < 1e-4) {
    return true;
  }
  const geometry = part.geometry;
  const index = geometry.index;
  const position = geometry.getAttribute("position");
  if (!index || !position) {
    return false;
  }
  const faceVertex = hit.faceIndex * 3;
  const ia = index.getX(faceVertex);
  const ib = index.getX(faceVertex + 1);
  const ic = index.getX(faceVertex + 2);
  _collisionVa.fromBufferAttribute(position, ia);
  _collisionVb.fromBufferAttribute(position, ib);
  _collisionVc.fromBufferAttribute(position, ic);
  _collisionNormal.subVectors(_collisionVb, _collisionVa).cross(_collisionToPoint.subVectors(_collisionVc, _collisionVa)).normalize();
  _collisionToPoint.subVectors(_collisionPartLocal, hit.point);
  return _collisionToPoint.dot(_collisionNormal) < 0;
}

function collisionPointInsideBody(body: CollisionBody, worldToPose: Matrix4, worldPoint: Vector3): boolean {
  _collisionLocal.copy(worldPoint).applyMatrix4(worldToPose);
  for (const part of body.parts) {
    if (collisionPointInsidePart(part, _collisionLocal)) {
      return true;
    }
  }
  return false;
}

function collisionBodiesContainmentOverlap(a: CollisionBody, worldA: Matrix4, b: CollisionBody, worldB: Matrix4): boolean {
  collisionBodyWorldBounds(a, worldA, _collisionBoxA);
  collisionBodyWorldBounds(b, worldB, _collisionBoxB);
  _collisionInterBox.copy(_collisionBoxA).intersect(_collisionBoxB);
  if (_collisionInterBox.isEmpty() || !Number.isFinite(_collisionInterBox.min.x)) {
    return false;
  }
  _collisionInterBox.getCenter(_collisionPoint);
  _collisionInvA.copy(worldA).invert();
  _collisionInvB.copy(worldB).invert();
  return collisionPointInsideBody(a, _collisionInvA, _collisionPoint) && collisionPointInsideBody(b, _collisionInvB, _collisionPoint);
}

/** 💥️ True when two posed {@link CollisionBody} instances have intersecting mesh geometry. */
export function bodiesIntersect(a: CollisionBody, worldA: Matrix4, b: CollisionBody, worldB: Matrix4): boolean {
  collisionBodyWorldBounds(a, worldA, _collisionBoxA);
  collisionBodyWorldBounds(b, worldB, _collisionBoxB);
  if (!_collisionBoxA.intersectsBox(_collisionBoxB)) {
    return false;
  }
  for (const partA of a.parts) {
    _collisionMatA.multiplyMatrices(worldA, partA.localMatrix);
    for (const partB of b.parts) {
      _collisionMatB.multiplyMatrices(worldB, partB.localMatrix);
      _collisionAtoB.copy(_collisionMatB).invert().multiply(_collisionMatA);
      if (partB.bvh.intersectsGeometry(partA.geometry, _collisionAtoB)) {
        return true;
      }
    }
  }
  return collisionBodiesContainmentOverlap(a, worldA, b, worldB);
}

export interface SolidOverlapVolumeOptions {
  readonly sampleCount?: number;
}

/** 📐️ Estimates solid overlap volume (m3) between two posed {@link CollisionBody} instances. */
export function solidOverlapVolume(a: CollisionBody, worldA: Matrix4, b: CollisionBody, worldB: Matrix4, opts?: SolidOverlapVolumeOptions): number {
  collisionBodyWorldBounds(a, worldA, _collisionBoxA);
  collisionBodyWorldBounds(b, worldB, _collisionBoxB);
  _collisionInterBox.copy(_collisionBoxA).intersect(_collisionBoxB);
  if (_collisionInterBox.isEmpty() || !Number.isFinite(_collisionInterBox.min.x)) {
    return 0;
  }
  if (!bodiesIntersect(a, worldA, b, worldB) && !collisionBodiesContainmentOverlap(a, worldA, b, worldB)) {
    return 0;
  }
  _collisionInterBox.getSize(_collisionSize);
  const boxVol = _collisionSize.x * _collisionSize.y * _collisionSize.z;
  if (boxVol <= 0) {
    return 0;
  }
  const sampleCount = opts?.sampleCount ?? Math.min(4096, Math.max(256, Math.round(boxVol * 200)));
  _collisionInvA.copy(worldA).invert();
  _collisionInvB.copy(worldB).invert();
  let insideBoth = 0;
  for (let i = 0; i < sampleCount; i += 1) {
    _collisionPoint.set(_collisionInterBox.min.x + Math.random() * _collisionSize.x, _collisionInterBox.min.y + Math.random() * _collisionSize.y, _collisionInterBox.min.z + Math.random() * _collisionSize.z);
    if (collisionPointInsideBody(a, _collisionInvA, _collisionPoint) && collisionPointInsideBody(b, _collisionInvB, _collisionPoint)) {
      insideBoth += 1;
    }
  }
  return (insideBoth / sampleCount) * boxVol;
}
// #endregion Collision
