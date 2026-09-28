/** @emoji 📐️ Actual Three SVGRenderer and Sharp oracle for first-party Icon SVG export. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import sharp from "sharp";
import { AmbientLight, BufferGeometry, Color, DirectionalLight, EdgesGeometry, Float32BufferAttribute, LineBasicMaterial, LineSegments, Mesh, MeshStandardMaterial, PerspectiveCamera, Scene, Vector3 } from "three";
import { SVGRenderer } from "three/examples/jsm/renderers/SVGRenderer.js";
import { clipIconSvgMarkupToEllipse } from "@semio-tech/ui-react";
import { describe, expect, it } from "vitest";

const host = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const read = (path: string) => JSON.parse(readFileSync(resolve(host, path), "utf8"));
const fixture = read("🧫️fixtures/📤️svg-export/🔣️.json");

function renderFixture(): string {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(fixture.geometry.positions.flat(), 3));
  geometry.setAttribute("normal", new Float32BufferAttribute(fixture.geometry.normals.flat(), 3));
  geometry.setIndex(fixture.geometry.indices);
  const material = new MeshStandardMaterial({
    color: fixture.material.color,
    emissive: fixture.material.emissive,
    metalness: fixture.material.metalness,
    roughness: fixture.material.roughness,
  });
  const mesh = new Mesh(geometry, material);
  const outline = new LineSegments(new EdgesGeometry(geometry), new LineBasicMaterial({ color: fixture.material.stroke }));
  outline.scale.setScalar(1.001);
  mesh.add(outline);
  const scene = new Scene();
  scene.background = new Color(fixture.background);
  scene.add(mesh, new AmbientLight(fixture.lighting.ambientColor, fixture.lighting.ambientIntensity));
  const sun = new DirectionalLight(fixture.lighting.sunColor, fixture.lighting.sunIntensity);
  sun.position.copy(new Vector3(...fixture.lighting.sunDirection));
  scene.add(sun);
  const camera = new PerspectiveCamera(fixture.camera.fov, fixture.width / fixture.height, 0.1, 10_000);
  camera.up.fromArray(fixture.camera.up);
  camera.position.fromArray(fixture.camera.position);
  camera.lookAt(new Vector3().fromArray(fixture.camera.target));
  const renderer = new SVGRenderer();
  renderer.setSize(fixture.width, fixture.height);
  renderer.render(scene, camera);
  const markup = new XMLSerializer().serializeToString(renderer.domElement);
  return fixture.shape === "ellipse" ? clipIconSvgMarkupToEllipse(markup, fixture.width, fixture.height) : markup;
}

describe("📐️ Icon SVG export", () => {
  it("validates the neutral scene and output contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(read("🧬️schema/📤️svg-export/🔣️.json"));
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("matches installed Three SVGRenderer geometry, flat lighting, outlines, ellipse, and Sharp pixels", async () => {
    const markup = renderFixture();
    const document = new DOMParser().parseFromString(markup, "image/svg+xml");
    const paths = [...document.querySelectorAll("path")];
    expect(paths).toHaveLength(fixture.expected.pathCount);
    expect(paths.some((path) => path.getAttribute("style")?.includes(`fill:${fixture.expected.fill}`))).toBe(true);
    expect(paths.some((path) => path.getAttribute("style")?.includes(`stroke:${fixture.expected.stroke}`))).toBe(true);
    expect(document.querySelector('clipPath[id="semio-icon-ellipse-clip"] ellipse')).not.toBeNull();
    const image = await sharp(Buffer.from(markup)).ensureAlpha().raw().toBuffer();
    for (const probe of fixture.expected.probes) {
      const offset = (probe.position[1] * fixture.width + probe.position[0]) * 4;
      expect([...image.subarray(offset, offset + 4)], probe.position.join(",")).toEqual(probe.rgba);
    }
  });
});
