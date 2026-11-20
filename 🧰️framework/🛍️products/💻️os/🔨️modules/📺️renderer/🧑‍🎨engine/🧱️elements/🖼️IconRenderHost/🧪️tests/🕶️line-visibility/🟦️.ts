/** 🧪️ Checks authored camera projection and depth-clipped linework against an independent Three ray caster. */
import { readFileSync } from "node:fs";
import { buildIconCamera } from "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/🟦️.tsx";
import { JSDOM } from "jsdom";
import { paintIconDrawing, prepareIconSvgMeshes } from "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/🖋️drawing/🟦️.ts";
import { expect, test } from "bun:test";
import { BufferGeometry, Color, DoubleSide, EdgesGeometry, Scene, Float32BufferAttribute, InstancedMesh, Matrix4, Mesh, MeshBasicMaterial, OrthographicCamera, PerspectiveCamera, Raycaster, Vector3, Vector4 } from "three";
import { Occluders, clipEdge, projectPoint, type ClipPoint, type Edge, type Point } from "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/📐️line-visibility/🟦️.ts";

interface Fixture {
  width: number;
  height: number;
  projections: {
    kind: "perspective" | "orthographic";
    position: [number, number, number];
    target: [number, number, number];
    up: [number, number, number];
    referenceUp?: [number, number, number];
    zoom: number;
    fov: number;
    points: Point[];
    screen: [number, number][];
  }[];
  frustum: { id: string; input: [ClipPoint, ClipPoint]; clipped: [ClipPoint, ClipPoint] | null }[];
  meshScene: {
    camera: { projection: "orthographic"; position: [number, number, number]; up: [number, number, number]; target: [number, number, number]; zoom: number };
    positions: Point[];
    indices: number[];
    expected: { visible: string[]; hidden: string };
  };
  vectorMeshes: {
    positions: Point[];
    moved: Point[];
    translations: [number, number, number][];
    expected: [number, number, number][];
  };
  cases: {
    id: string;
    line: Edge;
    triangles: [Point, Point, Point][];
    visible: [number, number][];
    hidden: [number, number][];
  }[];
}

const fixture: Fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🕶️line-visibility/🔣️.json", import.meta.url), "utf8"));

test("perspective and orthographic screen positions match Three cameras", () => {
  for (const entry of fixture.projections) {
    const { width, height } = fixture;
    const camera = entry.kind === "orthographic"
      ? new OrthographicCamera(-width / 2, width / 2, height / 2, -height / 2, 0.1, 10_000)
      : new PerspectiveCamera(entry.fov, width / height, 0.1, 10_000);
    camera.position.set(...entry.position);
    camera.up.set(...(entry.referenceUp ?? entry.up));
    camera.lookAt(...entry.target);
    camera.zoom = entry.zoom;
    camera.updateProjectionMatrix();
    const rendered = buildIconCamera({
      assetUrl: "fixture.glb", format: "svg", width, height,
      camera: { position: entry.position, target: entry.target, up: entry.up, projection: entry.kind, zoom: entry.zoom, fov: entry.fov },
      lights: { ambientIntensity: 1, ambientColor: "#fff", sunIntensity: 0, sunColor: "#fff", sunAzimuth: 0, sunElevation: 0 },
    });
    for (const view of [camera, rendered]) {
      view.updateMatrixWorld();
      for (const [index, [x, y, z]] of entry.points.entries()) {
        const vector = new Vector4(x, y, z, 1).applyMatrix4(view.matrixWorldInverse).applyMatrix4(view.projectionMatrix);
        const screen = projectPoint([vector.x, vector.y, vector.z, vector.w], width, height);
        const [expectedX, expectedY] = entry.screen[index]!;
        expect(screen[0]).toBeCloseTo(expectedX, 6);
        expect(screen[1]).toBeCloseTo(expectedY, 6);
      }
    }
  }
});

test("homogeneous edge clipping uses the near, far, and viewport planes", () => {
  for (const entry of fixture.frustum) {
    expect(clipEdge(entry.input) ?? null, entry.id).toEqual(entry.clipped);
  }
});

for (const entry of fixture.cases) test(entry.id, () => {
  const occluders = new Occluders(fixture.width, fixture.height);
  for (const face of entry.triangles) occluders.add(face);
  const scan = occluders.classify(entry.line);
  let step = scan.next();
  while (!step.done) step = scan.next();
  const spans = step.value;
  expect(spans.filter((span) => !span.hidden).map((span) => span.range)).toEqual(entry.visible);
  expect(spans.filter((span) => span.hidden).map((span) => span.range)).toEqual(entry.hidden);

  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(entry.triangles.flat(2), 3));
  const material = new MeshBasicMaterial({ side: DoubleSide });
  const mesh = new Mesh(geometry, material);
  const cast = new Raycaster(new Vector3(), new Vector3(0, 0, 1), 0);
  const [a, b] = entry.line;
  for (let sample = 0; sample < 1024; sample++) {
    const t = (sample + 0.5) / 1024;
    const classified = entry.hidden.some(([from, to]) => from < t && t < to);
    cast.ray.origin.fromArray(a).lerp(new Vector3(...b), t);
    cast.far = 3 + cast.ray.origin.z - 1e-7;
    cast.ray.origin.z = -3;
    expect(cast.intersectObject(mesh).length > 0).toBe(classified);
  }
  geometry.dispose();
  material.dispose();
});

test("SVG line layers split a rear mesh edge independently of face paint", async () => {
  const dom = new JSDOM("");
  const previousDocument = Object.getOwnPropertyDescriptor(globalThis, "document");
  Object.defineProperty(globalThis, "document", { configurable: true, value: dom.window.document });
  const { width, height, meshScene } = fixture;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(meshScene.positions.flat(), 3));
  geometry.setIndex(meshScene.indices);
  const edges = new EdgesGeometry(geometry);
  const material = new MeshBasicMaterial();
  try {
    const camera = new OrthographicCamera(-width / 2, width / 2, height / 2, -height / 2, 0.1, 10_000);
    camera.position.set(...meshScene.camera.position);
    camera.up.set(...meshScene.camera.up);
    camera.lookAt(...meshScene.camera.target);
    const scene = new Scene();
    scene.background = new Color("#112233");
    scene.add(new Mesh(geometry, material));
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    const req = {
      assetUrl: "fixture.glb", format: "svg", width, height, camera: meshScene.camera,
      lights: { ambientIntensity: 1, ambientColor: "#ffffff", sunAzimuth: 0, sunElevation: 90, sunIntensity: 0, sunColor: "#ffffff" },
      drawing: { background: false, fill: false, visibleLines: true, hiddenLines: true, visibleColor: "#123456", hiddenColor: "#805050", hiddenWidth: 2 },
    } as const;
    await paintIconDrawing(svg, scene, camera, req, () => edges, "#000000");
    expect([...svg.children].map((node) => node.id)).toEqual(["hidden-lines", "visible-lines"]);
    expect(svg.querySelector("#hidden-lines")?.getAttribute("stroke-width")).toBe("2");
    expect(svg.querySelector("#visible-lines")?.getAttribute("stroke")).toBe("#123456");
    const directions = (d: string) => {
      const [a, b] = d.slice(1).split("L");
      return [d, `M${b}L${a}`];
    };
    const data = (layer: string) => [...svg.querySelectorAll(`#${layer} path`)].flatMap((path) => path.getAttribute("d")!.split(/(?=M)/).flatMap(directions));
    expect(data("hidden-lines")).toContain(meshScene.expected.hidden);
    for (const visible of meshScene.expected.visible) expect(data("visible-lines")).toContain(visible);

    const controller = new AbortController();
    controller.abort();
    await expect(paintIconDrawing(svg, scene, camera, req, () => edges, "#000000", controller.signal)).rejects.toThrow();
  } finally {
    if (previousDocument) Object.defineProperty(globalThis, "document", previousDocument);
    else Reflect.deleteProperty(globalThis, "document");
    geometry.dispose();
    edges.dispose();
    material.dispose();
    dom.window.close();
  }
});

test("vector fills and silhouette edges share materialized instance and morph geometry", async () => {
  const geometry = new BufferGeometry();
  const { vectorMeshes } = fixture;
  geometry.setAttribute("position", new Float32BufferAttribute(vectorMeshes.positions.flat(), 3));
  geometry.morphAttributes.position = [new Float32BufferAttribute(vectorMeshes.moved.flat(), 3)];
  const material = new MeshBasicMaterial();
  const scene = new Scene();
  const instances = new InstancedMesh(geometry, material, vectorMeshes.translations.length);
  for (const [index, t] of vectorMeshes.translations.entries()) instances.setMatrixAt(index, new Matrix4().makeTranslation(...t));
  scene.add(instances);
  const source = new Mesh(geometry, material);
  source.morphTargetInfluences![0] = 1;
  scene.add(source);
  const owned = await prepareIconSvgMeshes(scene);
  try {
    const points: number[][] = [];
    scene.updateMatrixWorld(true);
    scene.traverse((object) => {
      if (object instanceof Mesh) {
        expect(object).not.toBeInstanceOf(InstancedMesh);
        points.push(object.getVertexPosition(0, new Vector3()).applyMatrix4(object.matrixWorld).toArray());
      }
    });
    expect(points).toEqual(vectorMeshes.expected);
    expect(owned).toHaveLength(1);
    expect(new Vector3().fromBufferAttribute(geometry.getAttribute("position"), 0).toArray()).toEqual(vectorMeshes.positions[0]!);
  } finally {
    for (const mesh of owned) mesh.dispose();
    instances.dispose();
    material.dispose();
    geometry.dispose();
  }
});
