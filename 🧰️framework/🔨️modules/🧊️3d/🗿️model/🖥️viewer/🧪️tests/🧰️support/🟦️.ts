/** 🧰️ Viewer test helpers: fixture loading and three.js as the independent oracle (camera matrices, skinning, per-pixel ray casting). Never imported by the viewer itself. */
import { Bone, BufferAttribute, BufferGeometry, DoubleSide, FrontSide, Matrix4, Mesh, MeshBasicMaterial, Object3D, OrthographicCamera, PerspectiveCamera, Raycaster, Skeleton, SkinnedMesh, Vector2, Vector3 } from "three";
import { poseModel, type PoseSelector } from "../../../🟦️.ts";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { basisOf, clipPlanes, type CameraState } from "../../📷️camera/🟦️.ts";
import { directoryNear, readFixture } from "../../../🧪️tests/🧰️support/🧫️reader/🟦️.ts";

export const fixture = <T = any>(name: string): T => readFixture<T>(directoryNear(import.meta.url, "../../🧫️fixtures"), name);

export const withoutLines = (doc: ModelDocument): ModelDocument => ({ ...doc, meshes: doc.meshes.map((m) => ({ ...m, primitives: m.primitives.filter((p) => p.mode === "triangles") })) });

export function threeCamera(camera: CameraState, aspect: number): PerspectiveCamera | OrthographicCamera {
  const { up, eye } = basisOf(camera);
  const { near, far } = clipPlanes(camera);
  const c = camera.projection === "perspective"
    ? new PerspectiveCamera((camera.fovY * 180) / Math.PI, aspect, near, far)
    : new OrthographicCamera((-camera.orthoHeight * aspect) / 2, (camera.orthoHeight * aspect) / 2, camera.orthoHeight / 2, -camera.orthoHeight / 2, near, far);
  c.up.set(up[0], up[1], up[2]);
  c.position.set(eye[0], eye[1], eye[2]);
  c.lookAt(camera.target[0], camera.target[1], camera.target[2]);
  c.updateMatrixWorld(true);
  c.updateProjectionMatrix();
  return c;
}

export type ThreeWorld = { root: Object3D; meshes: Mesh[]; skinned: SkinnedMesh[]; objects: Map<string, Object3D>; skeletons: Skeleton[] };

export function threeWorld(doc: ModelDocument, selector?: PoseSelector): ThreeWorld {
  const pose = poseModel(doc, selector);
  const root = new Object3D();
  const objects = new Map<string, Object3D>();
  for (const n of doc.nodes) objects.set(n.id, new Bone());
  for (const n of doc.nodes) {
    const o = objects.get(n.id)!, t = pose.local[n.id]!;
    o.position.fromArray(t.translation);
    o.quaternion.fromArray(t.rotation);
    o.scale.fromArray(t.scale);
    (n.parent ? objects.get(n.parent)! : root).add(o);
  }
  root.updateMatrixWorld(true);
  const skeletons = doc.skins.map((s) => {
    const bones = s.joints.map((j) => objects.get(j) as Bone);
    return new Skeleton(bones, s.inverseBind.map((m) => new Matrix4().fromArray(m as unknown as number[])));
  });
  const meshes: Mesh[] = [], skinned: SkinnedMesh[] = [];
  for (const n of doc.nodes) {
    if (!n.mesh) continue;
    for (const p of doc.meshes.find((m) => m.id === n.mesh)!.primitives) {
      if (p.mode !== "triangles") continue;
      const material = doc.materials.find((m) => m.id === p.material)!;
      const g = new BufferGeometry();
      g.setAttribute("position", new BufferAttribute(Float32Array.from(p.positions), 3));
      g.setIndex([...p.indices]);
      const basic = new MeshBasicMaterial({ side: material.doubleSided ? DoubleSide : FrontSide });
      if (n.skin && p.joints && p.weights) {
        g.setAttribute("skinIndex", new BufferAttribute(Uint16Array.from(p.joints), 4));
        g.setAttribute("skinWeight", new BufferAttribute(Float32Array.from(p.weights), 4));
        const mesh = new SkinnedMesh(g, basic);
        mesh.userData.node = n.id;
        root.add(mesh);
        mesh.bind(skeletons[doc.skins.findIndex((s) => s.id === n.skin)]!, new Matrix4());
        skinned.push(mesh);
        meshes.push(mesh);
      } else {
        const mesh = new Mesh(g, basic);
        mesh.userData.node = n.id;
        objects.get(n.id)!.add(mesh);
        meshes.push(mesh);
      }
    }
  }
  root.updateMatrixWorld(true);
  for (const s of skeletons) s.update();
  return { root, meshes, skinned, objects, skeletons };
}

export type PixelOracle = { width: number; height: number; hit: Uint8Array; distance: Float64Array };

export function castPixels(world: ThreeWorld, camera: PerspectiveCamera | OrthographicCamera, width: number, height: number): PixelOracle {
  const ray = new Raycaster();
  const hit = new Uint8Array(width * height), distance = new Float64Array(width * height).fill(Infinity);
  const forward = camera.getWorldDirection(new Vector3());
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    ray.setFromCamera(new Vector2(((x + 0.5) / width) * 2 - 1, 1 - ((y + 0.5) / height) * 2), camera);
    const h = ray.intersectObjects(world.meshes, false)[0];
    if (!h) continue;
    hit[y * width + x] = 1;
    distance[y * width + x] = h.point.clone().sub(camera.position).dot(forward);
  }
  return { width, height, hit, distance };
}
