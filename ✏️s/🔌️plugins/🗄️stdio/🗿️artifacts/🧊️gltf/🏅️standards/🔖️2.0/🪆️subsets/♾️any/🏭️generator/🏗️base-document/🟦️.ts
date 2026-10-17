/** 🏗️ glTF fixture generator: the FileReader shim, the untyped document model and the ONE real base document that three's own scene graph and GLTFExporter produce. */
import * as THREE from "three";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";

//#region 🩹️Shims
class ShimFileReader {
  result: unknown = null;
  onload: (() => void) | null = null;
  onloadend: (() => void) | null = null;
  onerror: ((e: unknown) => void) | null = null;
  private done(): void {
    this.onloadend?.();
    this.onload?.();
  }
  readAsArrayBuffer(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => {
      this.result = buffer;
      this.done();
    }, (error) => this.onerror?.(error));
  }
  readAsDataURL(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => {
      this.result = `data:${blob.type || "application/octet-stream"};base64,${Buffer.from(buffer).toString("base64")}`;
      this.done();
    }, (error) => this.onerror?.(error));
  }
}
(globalThis as { FileReader?: unknown }).FileReader ??= ShimFileReader as unknown as typeof FileReader;

//#endregion 🩹️Shims
//#region 📦️Document model (untyped — this generator edits the raw exported glTF JSON structurally)
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Doc = any;
/** 🧾️ three's `GLTFWriter` assembles the document in `json`, which its type declaration leaves out. */
const documentOf = (writer: object): Doc => {
  if (!("json" in writer)) throw new Error("GLTFWriter exposes its document as json");
  return writer.json;
};

//#endregion 📦️Document model
//#region 🏗️Base scene
/** 🌳 The ONE real glTF document every recipe derives from, built by three's own scene graph + GLTFExporter:
 *  two scenes, a skinned+morphed textured-free mesh, two materials, a camera, an animation clip. */
export async function buildBaseDoc(): Promise<Doc> {
  const matA = new THREE.MeshStandardMaterial({ name: "matA", roughness: 0.4, metalness: 0.1, color: 0x3366cc });
  const matB = new THREE.MeshStandardMaterial({ name: "matB", roughness: 0.9, metalness: 0.0, color: 0xcc6633 });

  const geometry = new THREE.BoxGeometry(1, 1, 1);
  geometry.morphAttributes.position = [new THREE.Float32BufferAttribute(new Float32Array(geometry.attributes.position!.array.length).map((_, i) => (geometry.attributes.position!.array[i] as number) * 0.1), 3)];
  const mesh = new THREE.Mesh(geometry, matA);
  mesh.name = "meshNode";
  mesh.updateMorphTargets();
  mesh.morphTargetInfluences![0] = 0.5;

  const secondGeometry = new THREE.PlaneGeometry(1, 1);
  const secondMesh = new THREE.Mesh(secondGeometry, matB);
  secondMesh.name = "planeNode";

  const camera = new THREE.PerspectiveCamera(50, 1.5, 0.1, 100);
  camera.name = "cameraNode";

  const bone = new THREE.Bone();
  bone.name = "jointNode";
  bone.position.set(0, 1, 0);
  const skinnedGeometry = new THREE.CylinderGeometry(0.5, 0.5, 1, 8);
  const count = skinnedGeometry.attributes.position!.count;
  const skinIndex = new Float32Array(count * 4);
  const skinWeight = new Float32Array(count * 4);
  for (let i = 0; i < count; i += 1) skinWeight[i * 4] = 1;
  skinnedGeometry.setAttribute("skinIndex", new THREE.Uint16BufferAttribute(skinIndex, 4));
  skinnedGeometry.setAttribute("skinWeight", new THREE.Float32BufferAttribute(skinWeight, 4));
  const skinnedMesh = new THREE.SkinnedMesh(skinnedGeometry, matB);
  skinnedMesh.name = "skinnedNode";
  const skeleton = new THREE.Skeleton([bone]);
  skinnedMesh.add(bone);
  skinnedMesh.bind(skeleton);

  const root = new THREE.Group();
  root.name = "root";
  root.add(mesh, secondMesh, camera, skinnedMesh);
  const emptyNode = new THREE.Object3D();
  emptyNode.name = "emptyNode";
  root.add(emptyNode);

  const sceneA = new THREE.Scene();
  sceneA.name = "sceneA";
  sceneA.add(root);

  const sceneB = new THREE.Scene();
  sceneB.name = "sceneB";
  const soloNode = new THREE.Object3D();
  soloNode.name = "soloNode";
  sceneB.add(soloNode);

  const track = new THREE.VectorKeyframeTrack(`${mesh.uuid}.position`, [0, 1], [0, 0, 0, 0, 1, 0]);
  const clip = new THREE.AnimationClip("clip0", 1, [track]);

  const exporter = new GLTFExporter();
  exporter.register((writer) => ({
    afterParse(): void {
      const json = documentOf(writer);
      json.asset.generator = "three.js GLTFExporter (semio oracle fixture base)";
      json.asset.copyright = "2026 Ueli Saluz — CC0 fixture data";
      json.asset.extras = { fixtureBase: "gltf-2-0-any-reader-oracle", revision: 1 };
      json.extras = { documentPurpose: "semio reader-oracle base document" };
      json.extensionsUsed = ["KHR_materials_unlit"];
      for (const scene of json.scenes ?? []) scene.extras = { sceneKind: "fixture" };
      for (const node of json.nodes ?? []) node.extras = { nodeKind: "fixture" };
      for (const m of json.meshes ?? []) m.extras = { meshKind: "fixture" };
    },
  }));

  const output = (await new Promise((resolve, reject) => {
    exporter.parse([sceneA, sceneB], resolve, reject, { binary: false, animations: [clip], includeCustomExtensions: true });
  })) as Doc;
  return output;
}
//#endregion 🏗️Base scene
