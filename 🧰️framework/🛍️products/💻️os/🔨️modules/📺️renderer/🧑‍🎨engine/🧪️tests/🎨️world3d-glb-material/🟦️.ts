// @vitest-environment node
/** 🎨️ Actual installed Three GLTFLoader oracle for primitive-local GLB materials. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  ClampToEdgeWrapping,
  DoubleSide,
  FrontSide,
  LinearFilter,
  MirroredRepeatWrapping,
  NearestFilter,
  SRGBColorSpace,
  type Mesh,
  type MeshStandardMaterial,
  type Side,
  type TextureFilter,
  type Wrapping,
} from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { afterAll, beforeAll, describe, expect, it, vi } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixtureRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎨️world3d-glb-material");
const fixture = JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️.json"), "utf8"));
const sideNames = new Map<Side, string>([[FrontSide, "FrontSide"], [DoubleSide, "DoubleSide"]]);
const wrapNames = new Map<Wrapping, string>([[ClampToEdgeWrapping, "ClampToEdgeWrapping"], [MirroredRepeatWrapping, "MirroredRepeatWrapping"]]);
const filterNames = new Map<TextureFilter, string>([[LinearFilter, "LinearFilter"], [NearestFilter, "NearestFilter"]]);
const rounded = (values: readonly number[]) => values.map((value) => Number(value.toFixed(6)));

async function loadActualThreePrimitives(): Promise<readonly Mesh[]> {
  const source = readFileSync(resolve(fixtureRoot, fixture.asset));
  const bytes = source.buffer.slice(source.byteOffset, source.byteOffset + source.byteLength);
  const loaded = await new GLTFLoader().parseAsync(bytes, "");
  const meshes: Mesh[] = [];
  loaded.scene.traverse((object) => {
    if ((object as Mesh).isMesh) meshes.push(object as Mesh);
  });
  return meshes;
}

describe("🎨️ primitive-local GLB material carriage", () => {
  beforeAll(() => {
    vi.stubGlobal("self", globalThis);
    vi.stubGlobal("createImageBitmap", async () => ({ width: 1, height: 1, close() {} }));
  });

  afterAll(() => vi.unstubAllGlobals());

  it("validates the neutral bounded ownership and publication contract", () => {
    expect(fixture.publication).toEqual({
      workUnit: "onePrimitiveMaterialFieldPerStep",
      ownership: "primitiveLocal",
      commit: "geometryAndMaterialsTogether",
      cancellation: "retirePartialWithoutPublication",
      override: "replaceEveryPrimitiveMaterial",
    });
  });

  it("matches installed Three GLTFLoader material, vertex-color, texture, side, and alpha semantics", async () => {
    const meshes = await loadActualThreePrimitives();
    expect(meshes.map((mesh) => (mesh.material as MeshStandardMaterial).name)).toEqual(fixture.primitiveOrder);
    for (const [index, record] of fixture.cases.entries()) {
      const mesh = meshes[index];
      const material = mesh.material as MeshStandardMaterial;
      const color = mesh.geometry.getAttribute("color");
      expect({
        color: rounded(material.color.toArray()),
        opacity: material.opacity,
        metalness: material.metalness,
        roughness: material.roughness,
        emissive: rounded(material.emissive.toArray()),
        transparent: material.transparent,
        alphaTest: material.alphaTest,
        side: sideNames.get(material.side),
        depthWrite: material.depthWrite,
        vertexColors: material.vertexColors,
        mapColorSpace: material.map?.colorSpace ?? null,
        normalScale:material.normalScale.toArray(),occlusionStrength:material.aoMapIntensity,textureCoordinates:Object.fromEntries(["map","metalnessMap","roughnessMap","normalMap","aoMap","emissiveMap"].filter(role=>(material as any)[role]).map(role=>[role,(material as any)[role].channel])),
      }, record.id).toEqual(record.expectedThree);
      expect(color ? { itemSize: color.itemSize, normalized: color.normalized, bytes: [...color.array] } : null, record.id).toEqual(record.vertexColor);
      expect(material.map ? {
        mimeType: material.map.userData.mimeType,
        wrapS: wrapNames.get(material.map.wrapS),
        wrapT: wrapNames.get(material.map.wrapT),
        magFilter: filterNames.get(material.map.magFilter),
        minFilter: filterNames.get(material.map.minFilter),
      } : null, record.id).toEqual(record.texture);
      if (index === 1) {
        expect([...mesh.geometry.getAttribute("tangent").array]).toEqual([0,1,0,-1,0,1,0,-1,0,1,0,-1]);
        for (const [role,colorSpace] of Object.entries(fixture.textureRoles)) {
          const texture = material[role as keyof MeshStandardMaterial] as import("three").Texture;
          expect(texture, role).toBeTruthy();expect(texture.colorSpace,role).toBe(colorSpace);
        }
        expect(material.metalnessMap!.channel).toBe(material.roughnessMap!.channel);expect(material.metalnessMap!.image).toBe(material.roughnessMap!.image);
        expect(material.map).toBe(material.emissiveMap);
      }
      if (material.map) expect(material.map.colorSpace).toBe(SRGBColorSpace);
    }
  });
});
