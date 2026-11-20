/** 📐️ Actual Three SVGRenderer and Sharp oracle for first-party Icon SVG export. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import sharp from "sharp";
import { AmbientLight, BufferGeometry, Color, DirectionalLight, EdgesGeometry, Float32BufferAttribute, LineBasicMaterial, LineSegments, Mesh, MeshStandardMaterial, PerspectiveCamera, QuadraticBezierCurve, Scene, Vector2, Vector3 } from "three";
import { SVGRenderer } from "three/examples/jsm/renderers/SVGRenderer.js";
import { clipIconSvgMarkupToEllipse } from "@semio-tech/ui-react";
import { describe, expect, it } from "vitest";
import { paintIconDrawing } from "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/🖋️drawing/🟦️.ts";
import type { IconRenderRequest } from "@semio-tech/ui-styling";

const host = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const read = (path: string) => JSON.parse(readFileSync(resolve(host, path), "utf8"));
const fixture = read("🧪️tests/🧫️fixtures/📤️svg-export/🔣️.json");

function fixtureCamera(): PerspectiveCamera {
  const camera = new PerspectiveCamera(fixture.camera.fov, fixture.width / fixture.height, 0.1, 10_000);
  camera.up.fromArray(fixture.camera.up);
  camera.position.fromArray(fixture.camera.position);
  camera.lookAt(new Vector3().fromArray(fixture.camera.target));
  camera.updateMatrixWorld();
  return camera;
}

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
  const camera = fixtureCamera();
  const renderer = new SVGRenderer();
  renderer.setSize(fixture.width, fixture.height);
  renderer.render(scene, camera);
  const markup = new XMLSerializer().serializeToString(renderer.domElement);
  return fixture.shape === "ellipse" ? clipIconSvgMarkupToEllipse(markup, fixture.width, fixture.height) : markup;
}

describe("📐️ Icon SVG export", () => {
  

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

describe("📚️ SVG drawing layers", () => {
  it("matches the drawing fixture with Three.js projections and Sharp compositing", async () => {
    const camera = fixtureCamera();
    const points = fixture.geometry.positions.map((position: [number, number, number]) => {
      const projected = new Vector3(...position).project(camera);
      return new Vector2(projected.x * fixture.width / 2, -projected.y * fixture.height / 2);
    });
    const emit = (command: string, point: Vector2) => `${command}${point.x},${point.y}`;
    for (const sample of fixture.drawingCases) {
      const drawing = sample.drawing;
      const radius = drawing.lineEasing ?? 0;
      const cornerCurves = points.map((point: Vector2, index: number) => {
        const previous: Vector2 = points[(index + 2) % 3];
        const next: Vector2 = points[(index + 1) % 3];
        const trim = Math.min(radius, previous.distanceTo(point) / 2, next.distanceTo(point) / 2);
        return new QuadraticBezierCurve(
          point.clone().lerp(previous, trim / previous.distanceTo(point)),
          point,
          point.clone().lerp(next, trim / next.distanceTo(point)),
        );
      });
      const paths = points.map((point: Vector2, index: number) => {
        const next = (index + 1) % 3;
        if (!radius) return emit("M", point) + emit("L", points[next]);
        const from = cornerCurves[index];
        const to = cornerCurves[next];
        if (drawing.mergeLines) return (index === 0 ? emit("M", from.v2) : "") + emit("L", to.v0) + emit("Q", to.v1) + emit(" ", to.v2);
        return emit("M", from.getPoint(0.5)) + emit("Q", from.v1.clone().lerp(from.v2, 0.5))
          + emit(" ", from.v2) + emit("L", to.v0) + emit("Q", to.v0.clone().lerp(to.v1, 0.5)) + emit(" ", to.getPoint(0.5));
      });
      const document = new DOMParser().parseFromString(renderFixture(), "image/svg+xml");
      const svg = document.documentElement;
      const clip = svg.querySelector("g")!;
      const rect = clip.querySelector("rect")!;
      const fill = [...clip.querySelectorAll("path")].find((path) => path.getAttribute("style")?.includes("fill:rgb("))!;
      const line = document.createElementNS(svg.namespaceURI, "path");
      const stroke = new Color(drawing.visibleColor ?? fixture.material.stroke).getStyle();
      line.setAttribute("d", paths.join("") + (radius && drawing.mergeLines ? "Z" : ""));
      line.setAttribute("style", `fill:none;stroke:${stroke};stroke-width:${drawing.visibleWidth ?? 1};stroke-linecap:round`);
      clip.replaceChildren();
      const group = (id: string, child: Element) => {
        const layer = document.createElementNS(svg.namespaceURI, "g");
        layer.setAttribute("id", id);
        if (id === "fills" && drawing.fillOpacity !== undefined) layer.setAttribute("opacity", String(drawing.fillOpacity));
        layer.append(child);
        clip.append(layer);
      };
      if (sample.backgroundOpacity) rect.setAttribute("fill-opacity", sample.backgroundOpacity);
      if (drawing.background !== false) group("background", rect);
      if (drawing.fillColor) fill.setAttribute("style", `fill:${new Color(drawing.fillColor).getStyle()}`);
      if (drawing.fill !== false) group("fills", fill);
      if (drawing.visibleLines !== false) group("visible-lines", line);
      expect([...clip.children].map((layer) => layer.id), sample.name).toEqual(sample.groups);
      if (sample.stroke) {
        expect(stroke).toBe(sample.stroke);
        expect((line.getAttribute("d")?.match(/M/g) ?? []).length).toBe(sample.moveCount);
        expect((line.getAttribute("d")?.match(/Q/g) ?? []).length).toBe(sample.roundedCount);
      }
      const markup = new XMLSerializer().serializeToString(svg);
      const image = await sharp(Buffer.from(markup)).ensureAlpha().raw().toBuffer();
      const offset = (32 * fixture.width + 48) * 4;
      const expected = sample.center ?? (drawing.fill !== false ? fixture.expected.probes[0].rgba : drawing.background !== false ? fixture.expected.probes[1].rgba : [0, 0, 0, 0]);
      const actual = [...image.subarray(offset, offset + 4)];
      if (sample.center) actual.forEach((channel, axis) => expect(Math.abs(channel - expected[axis]), sample.name).toBeLessThanOrEqual(1));
      else expect(actual, sample.name).toEqual(expected);
    }
  });
});

describe("🖋️ React layered SVG output", () => {
  it("sends the same visible styles and layer controls as the reference", async () => {
    const geometry = new BufferGeometry();
    geometry.setAttribute("position", new Float32BufferAttribute(fixture.geometry.positions.flat(), 3));
    geometry.setAttribute("normal", new Float32BufferAttribute(fixture.geometry.normals.flat(), 3));
    geometry.setIndex(fixture.geometry.indices);
    const scene = new Scene();
    scene.background = new Color(fixture.background);
    const material = new MeshStandardMaterial({ color: fixture.material.color });
    scene.add(new Mesh(geometry, material));
    const edges = new EdgesGeometry(geometry);
    const camera = fixtureCamera();
    try {
      for (const sample of fixture.drawingCases) {
        const renderer = new SVGRenderer();
        renderer.setSize(fixture.width, fixture.height);
        renderer.render(scene, camera);
        const request: IconRenderRequest = {
          assetUrl: "fixture.glb", camera: { ...fixture.camera, projection: "perspective", zoom: 1 }, lights: fixture.lighting,
          width: fixture.width, height: fixture.height, format: "svg", background: sample.background ?? fixture.background, drawing: sample.drawing,
        };
        await paintIconDrawing(renderer.domElement as SVGSVGElement, scene, camera, request, () => edges, fixture.material.stroke);
        const svg = renderer.domElement;
        expect([...svg.children].map((layer) => layer.id), sample.name).toEqual(sample.groups);
        if (sample.backgroundOpacity) {
          const markup = new XMLSerializer().serializeToString(svg);
          const pixels = await sharp(Buffer.from(markup)).ensureAlpha().raw().toBuffer();
          const offset = (32 * fixture.width + 48) * 4;
          pixels.subarray(offset, offset + 4).forEach((channel, axis) => expect(Math.abs(channel - sample.center[axis]), sample.name).toBeLessThanOrEqual(1));
        }
        const lines = svg.querySelector("#visible-lines");
        if (sample.stroke) {
          expect(new Color(lines?.getAttribute("stroke") ?? "").getStyle()).toBe(sample.stroke);
          expect(Number(lines?.getAttribute("stroke-width"))).toBe(sample.drawing.visibleWidth ?? 1);
          const path = [...(lines?.querySelectorAll("path") ?? [])].map((path) => path.getAttribute("d")).join("");
          expect((path.match(/M/g) ?? []).length, sample.name).toBe(sample.moveCount);
          expect((path.match(/Q/g) ?? []).length, sample.name).toBe(sample.roundedCount);
        }
        if (sample.fill) {
          const fill = svg.querySelector("#fills");
          expect(Number(fill?.getAttribute("opacity"))).toBe(sample.drawing.fillOpacity);
          expect(new Color(fill?.querySelector("path")?.style.fill ?? "").getStyle()).toBe(sample.fill);
        }
      }
    } finally {
      edges.dispose();
      geometry.dispose();
      material.dispose();
    }
  });
});

it("preserves preview aspect ratio and straight alpha within the upload budget with Sharp", async () => {
  const raster = fixture.previewRaster;
  for (const sample of raster.cases) {
    const [width, height] = sample.sourceSize;
    const bound = Math.max(...sample.expectedSize);
    const markup = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}"><rect width="${width}" height="${height}" fill="${raster.fill}" fill-opacity="${raster.opacity}"/></svg>`;
    const { data, info } = await sharp(Buffer.from(markup)).resize({ width: bound, height: bound, fit: "inside", withoutEnlargement: true }).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
    expect([info.width, info.height]).toEqual(sample.expectedSize);
    expect(data.length).toBeLessThanOrEqual(raster.byteBudget);
    data.subarray(0, 4).forEach((channel, axis) => expect(Math.abs(channel - raster.rgba[axis])).toBeLessThanOrEqual(1));
  }
});
