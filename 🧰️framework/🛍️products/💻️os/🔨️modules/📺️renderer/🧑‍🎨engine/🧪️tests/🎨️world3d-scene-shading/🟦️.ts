/** @emoji 🎨️ Actual Three r182 oracle for the language-neutral World3d shading fixture. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { Color, Matrix3, Matrix4, MeshStandardMaterial, REVISION, ShaderChunk, Vector3 } from "three";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️scene-shading/🔣️.json"), "utf8")) as any;

const expectVector = (actual: readonly number[], expected: readonly number[], precision = 7) => actual.forEach((value, index) => expect(value).toBeCloseTo(expected[index]!, precision));

describe("🎨️ World3d current Three shading oracle", () => {
  it("resolves the live semantic panel aliases through Three color management", () => {
    expect(REVISION).toBe(fixture.reference.threeRevision);
    for (const appearance of Object.values(fixture.appearances) as any[]) {
      expectVector(new Color(appearance.panelHex).toArray(), appearance.panelLinear.slice(0, 3));
    }
    expectVector(new Color(fixture.scene.fallbackLights.hemisphere.groundHex).toArray(), fixture.scene.fallbackLights.hemisphere.ground);
    expect(fixture.scene.fallbackLights.hemisphere.position).toEqual([0, 0, 1]);
    for (const row of fixture.provenanceCases) {
      const expected = row.source === "semanticNeutral" ? fixture.appearances[row.appearance].panelLinear : row.sourceLinear;
      expect(row.expectedNeutralLinear).toEqual(expected);
    }
  });

  it("uses actual Matrix3 normal matrices and material state for the static rows", () => {
    const normal = fixture.math.normalTransforms.find((row: any) => row.id === "nonuniform");
    const actual = new Vector3().fromArray(normal.objectNormal).applyNormalMatrix(new Matrix3().getNormalMatrix(new Matrix4().makeScale(normal.modelScale[0], normal.modelScale[1], normal.modelScale[2])));
    expectVector(actual.toArray(), normal.expectedWorldNormal);
    for (const row of fixture.styleCases) {
      const material = new MeshStandardMaterial({
        color: new Color().setRGB(row.baseColor[0], row.baseColor[1], row.baseColor[2]),
        vertexColors: row.vertexColorWeight === 1,
        metalness: fixture.scene.material.metalness,
        roughness: fixture.scene.material.roughness,
        emissive: new Color().setRGB(row.baseColor[0], row.baseColor[1], row.baseColor[2]),
        emissiveIntensity: row.emissiveIntensity,
        transparent: row.opacity < 1,
        opacity: row.opacity,
      });
      expectVector(material.color.toArray(), row.baseColor.slice(0, 3));
      expect(material.vertexColors).toBe(row.vertexColorWeight === 1);
      expect(material.emissiveIntensity).toBe(row.emissiveIntensity);
      expect(material.opacity).toBe(row.opacity);
      expect(material.metalness).toBe(0);
      expect(material.roughness).toBe(1);
      material.dispose();
    }
  });

  it("pins the installed Three standard BRDF, DFG multiscatter, and ACES program", () => {
    const physical = ShaderChunk.lights_physical_pars_fragment;
    const common = ShaderChunk.common;
    const tone = ShaderChunk.tonemapping_pars_fragment;
    const colorspace = ShaderChunk.colorspace_pars_fragment;
    for (const symbol of ["BRDF_GGX_Multiscatter", "V_GGX_SmithCorrelated", "D_GGX", "dfgLUT", "FssEss_V", "FssEss_L", "RE_IndirectDiffuse_Physical"]) expect(physical).toContain(symbol);
    expect(common).toContain("BRDF_Lambert");
    expect(common).toContain("RECIPROCAL_PI * diffuseColor");
    for (const symbol of ["ACESFilmicToneMapping", "toneMappingExposure / 0.6", "ACESInputMat", "RRTAndODTFit", "ACESOutputMat"]) expect(tone).toContain(symbol);
    for (const symbol of ["sRGBTransferOETF", "0.41666", "1.055", "0.0031308", "12.92"]) expect(colorspace).toContain(symbol);
    expect(fixture.pixelOracle.status).toBe("recorded");
    expect(fixture.pixelOracle.rows).toHaveLength(20);
    const oracleIds = new Set(fixture.pixelOracle.rows.map((row: any) => row.id));
    const layerIds = new Set([...fixture.styleCases, ...fixture.provenanceCases].map((row: any) => row.id));
    for (const row of fixture.compositingCases) {
      expect(oracleIds.has(row.id)).toBe(true);
      expect(row.clearLinearRgba[3]).toBe(1);
      for (const layer of row.layers) expect(layerIds.has(layer)).toBe(true);
    }
    expect(fixture.pixelOracle.profile).toEqual({ samplePosition: [32, 32], clearRgba: [0, 0, 0, 0], antialias: false, premultipliedAlpha: false, maximumRgbError: 1 });
  });
});
