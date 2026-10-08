/** 📏️ The World3d modelling layer, mounted for real in jsdom: annotations drawn at screen-constant size with an accessible
 * list, scalar-field heatmaps with an accessible legend, the pick granularity filter, the section plane with its stencil cap
 * and the sub-element highlight tokens.
 *
 * 🧫️ Every expectation about ramps, legends, pick targets and section planes is read from
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/📏️world3d-modelling/🔣️.json`, whose values were produced with three.js, never with our
 * own code; the screen-space geometry is checked against hand-computed orthographic pixels.
 *
 * 🎭️ Only the WebGL seam is replaced, exactly as in `🧪️tests/🖱️world3d-interaction`: `@react-three/fiber` hands out a real
 * renderer-free camera and records the `useFrame` callbacks so a test can play a frame. */
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { createElement, type ReactNode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { BufferAttribute, BufferGeometry, Color, ColorManagement, Group, Mesh, MeshBasicMaterial, MeshStandardMaterial, OrthographicCamera, PerspectiveCamera, Plane, SRGBColorSpace, Vector3 } from "three";
import fixture from "../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/📏️world3d-modelling/🔣️.json" with { type: "json" };

const recorded = vi.hoisted(() => ({ frames: [] as ((state: unknown, delta: number) => void)[], camera: null as PerspectiveCamera | null, size: { width: 800, height: 600 } }));

vi.mock("@react-three/fiber", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const three = await import("three");
  const camera = new three.PerspectiveCamera(45, 800 / 600, 0.1, 1000);
  camera.up.set(0, 0, 1);
  camera.position.set(6, -6, 4);
  camera.lookAt(0, 0, 0);
  camera.updateMatrixWorld(true);
  camera.updateProjectionMatrix();
  recorded.camera = camera;
  const state = { camera, gl: { domElement: { clientWidth: 800, clientHeight: 600 }, localClippingEnabled: false }, scene: new three.Scene(), size: recorded.size, raycaster: new three.Raycaster(), invalidate: () => {}, clock: { elapsedTime: 0 }, get: () => ({ camera, controls: null }) };
  return {
    ...actual,
    useFrame: (callback: (frameState: unknown, delta: number) => void) => {
      recorded.frames.push(callback);
    },
    useThree: (selector?: (value: typeof state) => unknown) => (selector ? selector(state) : state),
    useLoader: () => null,
  };
});

vi.mock("@semio-tech/infinite-world-r3f", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return {
    ...actual,
    WorldCanvas: ({ children, overlay }: { children?: ReactNode; overlay?: ReactNode }) => createElement("div", { "data-testid": "world-canvas" }, overlay, children),
    WorldOrbitGated: () => null,
    WorldOrbitViewControls: () => null,
    WorldProjectionRig: () => null,
    WorldOrbitViewSnapGateProvider: ({ children }: { children?: ReactNode }) => createElement("div", null, children),
    WorldLodBridge: ({ children }: { children?: ReactNode }) => createElement("div", null, children),
    WorldVolumeLayer: () => null,
    WorldReferenceLayer: () => null,
  };
});

import { buildMeshVisuals, disposeMeshVisuals, World3dHost, type WorldMeshData } from "../../🟦️.tsx";
import {
  applyWorld3dSectionClipping,
  collectWorld3dSolidMeshes,
  createWorld3dAnnotationStore,
  formatWorld3dLegendValue,
  projectWorld3dAnnotations,
  resolveWorld3dHighlightPaint,
  resolveWorld3dToneHex,
  world3dMeshDataWithScalarField,
  world3dMeshRecordsWithScalarField,
  World3dAnnotationOverlay,
  World3dScalarLegendView,
  World3dSectionCapStencils,
  world3dGlbHitFaceId,
  world3dGlbSubElementIds,
  world3dGlbSubElementMeshData,
  world3dLegendLines,
  world3dModellingStrings,
  world3dRampGradient,
  world3dSectionPlane,
  world3dSelectionWithPickFilter,
  world3dSubElementPaint,
  world3dToneColorExpression,
  type World3dProjectedAnnotation,
} from "../../📏️modelling/🟦️.tsx";
import { parseWorld3dAnnotationLayer, parseWorld3dModellingOptions, parseWorld3dScalarField, type World3dPickGranularity, type World3dTone } from "@semio-tech/framework";

afterEach(() => {
  cleanup();
  recorded.frames.length = 0;
});

function playFrames(): void {
  const state = { camera: recorded.camera, size: recorded.size, clock: { elapsedTime: 0 }, invalidate: () => {} };
  act(() => {
    for (const frame of recorded.frames) {
      try {
        frame(state, 0.016);
      } catch {
        continue;
      }
    }
  });
}

const layerOf = (value: unknown) => parseWorld3dAnnotationLayer(value)!;
const fieldOf = (name: string) => parseWorld3dScalarField(fixture.valid.find((entry) => entry.name === name)!.value)!;
const mixed = layerOf(fixture.valid.find((entry) => entry.name === "annotations-mixed")!.value);
const NEAR_POINTS: Record<string, Record<string, readonly number[]>> = {
  "dim-width": { from: [0, 0, 0], to: [2, 0, 0], offset: [0, -1, 0] },
  "ang-corner": { vertex: [0, 0, 0], directionA: [1, 0, 0], directionB: [0, 1, 0] },
  "mk-centroid": { position: [1, 0.5, 0.2] },
  "ld-fillet": { anchor: [1, 0, 1] },
};
const TONE_HEX: Partial<Record<World3dTone, string>> = { neutral: "#000000", primary: "#ff0000", warning: "#ffff00" };
const nearMixed = layerOf({ ...mixed, items: mixed.items.map((item) => ({ ...item, ...NEAR_POINTS[item.id] })) });

const SIZE = { width: 1000, height: 1000 };
function orthographic(): OrthographicCamera {
  const camera = new OrthographicCamera(-5, 5, 5, -5, 0.1, 100);
  camera.position.set(0, 0, 10);
  camera.lookAt(0, 0, 0);
  camera.updateProjectionMatrix();
  camera.updateMatrixWorld(true);
  return camera;
}

describe("📏️ annotation projection", () => {
  it("places a marker at the pixel the orthographic camera maps its point to", () => {
    const layer = layerOf({ items: [{ kind: "marker", id: "m", position: [2.5, 2.5, 0], text: { en: "P", de: "P" } }] });
    const [marker] = projectWorld3dAnnotations(layer, orthographic(), SIZE);
    expect(marker!.visible).toBe(true);
    expect(marker!.marker).toEqual({ x: 750, y: 250, shape: "dot" });
  });

  it("projects a dimension to its two extension lines, its dimension line and two arrowheads pointing outward", () => {
    const layer = layerOf({ items: [{ kind: "dimension", id: "d", from: [-2.5, 0, 0], to: [2.5, 0, 0], offset: [0, -2.5, 0], text: { en: "5", de: "5" } }] });
    const [dimension] = projectWorld3dAnnotations(layer, orthographic(), SIZE);
    expect(dimension!.lines).toEqual([
      [250, 500, 250, 750],
      [750, 500, 750, 750],
      [250, 750, 750, 750],
    ]);
    expect(dimension!.arrows.map((arrow) => [arrow.x, arrow.y, Math.round(arrow.angle * 1000) / 1000])).toEqual([
      [250, 750, 3.142],
      [750, 750, 0],
    ]);
    expect(dimension!.label!.x).toBe(500);
    expect(dimension!.label!.y).toBeLessThan(750);
  });

  it("sweeps an angle along its shorter arc and labels it beyond the arc radius", () => {
    const layer = layerOf({ items: [{ kind: "angle", id: "a", vertex: [0, 0, 0], directionA: [1, 0, 0], directionB: [0, 1, 0], radiusPx: 40, text: { en: "90", de: "90" } }] });
    const [angle] = projectWorld3dAnnotations(layer, orthographic(), SIZE);
    expect(angle!.arc!.cx).toBe(500);
    expect(angle!.arc!.cy).toBe(500);
    expect(angle!.arc!.radius).toBe(40);
    expect(angle!.arc!.start).toBeCloseTo(0, 12);
    expect(angle!.arc!.sweep).toBeCloseTo(-Math.PI / 2, 12);
    expect(Math.hypot(angle!.label!.x - 500, angle!.label!.y - 500)).toBeCloseTo(40 + 14, 9);
  });

  it("offsets a leader label by screen pixels from its anchor", () => {
    const layer = layerOf({ items: [{ kind: "leader", id: "l", anchor: [0, 0, 0], labelOffsetPx: [30, -20], text: { en: "L", de: "L" } }] });
    const [leader] = projectWorld3dAnnotations(layer, orthographic(), SIZE);
    expect(leader!.lines).toEqual([[500, 500, 530, 480]]);
    expect(leader!.label).toEqual({ x: 530, y: 480, align: "start" });
  });

  it("keeps sizes in screen pixels no matter how far the camera zooms", () => {
    const layer = layerOf({ items: [{ kind: "angle", id: "a", vertex: [0, 0, 0], directionA: [1, 0, 0], directionB: [0, 1, 0], radiusPx: 64, text: { en: "90", de: "90" } }] });
    const near = orthographic();
    const far = orthographic();
    far.zoom = 0.25;
    far.updateProjectionMatrix();
    const [a] = projectWorld3dAnnotations(layer, near, SIZE);
    const [b] = projectWorld3dAnnotations(layer, far, SIZE);
    expect(a!.arc!.radius).toBe(64);
    expect(b!.arc!.radius).toBe(64);
  });

  it("hides an annotation whose anchor is behind the camera", () => {
    const layer = layerOf({ items: [{ kind: "marker", id: "m", position: [0, 0, 50], text: { en: "P", de: "P" } }] });
    const [marker] = projectWorld3dAnnotations(layer, orthographic(), SIZE);
    expect(marker!.visible).toBe(false);
    expect(marker!.marker).toBeUndefined();
  });
});

describe("📏️ annotation overlay", () => {
  function mount(locale: string) {
    const store = createWorld3dAnnotationStore();
    const view = render(createElement(World3dAnnotationOverlay, { layer: nearMixed, store, locale, toneHex: (tone: World3dTone) => TONE_HEX[tone] ?? "#123456" }));
    const project = () => act(() => store.set(projectWorld3dAnnotations(nearMixed, recorded.camera!, recorded.size)));
    return { view, project, store };
  }

  it("lists every annotation for screen readers in the active language, even before the first projected frame", () => {
    const { view } = mount("en");
    const region = view.container.querySelector("[data-slot=world-annotation-layer]")!;
    expect(region.getAttribute("role")).toBe("region");
    expect(region.getAttribute("aria-label")).toBe("Measurements");
    const items = [...view.container.querySelectorAll("[data-slot=world-annotation-list] li")].map((item) => item.textContent);
    expect(items).toEqual(["Dimension: Width 40 mm", "Angle: Angle 90 degrees", "Marker: Centroid", "Label: Fillet radius 2"]);
  });

  it("switches the whole accessible list to German with the German region name", () => {
    const { view } = mount("de-CH");
    expect(view.container.querySelector("[data-slot=world-annotation-layer]")!.getAttribute("aria-label")).toBe("Messungen");
    const items = [...view.container.querySelectorAll("[data-slot=world-annotation-list] li")].map((item) => item.textContent);
    expect(items).toEqual(["Bemaßung: Breite 40 mm", "Winkel: Winkel 90 Grad", "Markierung: Schwerpunkt", "Beschriftung: Verrundungsradius 2"]);
  });

  it("hides the drawn geometry from assistive technology and draws one group and one label per visible annotation", () => {
    const { view, project } = mount("en");
    expect(view.container.querySelectorAll("[data-slot=world-annotation-label]").length).toBe(0);
    project();
    const svg = view.container.querySelector("[data-slot=world-annotation-geometry]")!;
    expect(svg.getAttribute("aria-hidden")).toBe("true");
    const groups = [...svg.querySelectorAll("g")];
    expect(groups.map((group) => group.getAttribute("data-annotation-kind"))).toEqual(["dimension", "angle", "marker", "leader"]);
    expect(groups.map((group) => group.getAttribute("data-tone"))).toEqual(["primary", "neutral", "warning", "neutral"]);
    expect(groups.map((group) => (group as unknown as HTMLElement).style.color)).toEqual(["rgb(255, 0, 0)", "rgb(0, 0, 0)", "rgb(255, 255, 0)", "rgb(0, 0, 0)"]);
    const labels = [...view.container.querySelectorAll("[data-slot=world-annotation-label]")];
    expect(labels.every((label) => label.getAttribute("aria-hidden") === "true")).toBe(true);
    expect(labels.map((label) => label.textContent)).toEqual(["Width 40 mm", "Angle 90 degrees", "Fillet radius 2"]);
    expect(svg.querySelectorAll("polygon").length).toBe(2);
    expect(svg.querySelectorAll("path").length).toBeGreaterThan(0);
  });

  it("draws nothing for an annotation the camera cannot see while still listing it", () => {
    const { view, store } = mount("en");
    const hidden: World3dProjectedAnnotation[] = nearMixed.items.map((item) => ({ id: item.id, kind: item.kind, tone: item.tone, visible: false, lines: [], arrows: [] }));
    act(() => store.set(hidden));
    expect(view.container.querySelectorAll("[data-slot=world-annotation-geometry] g").length).toBe(0);
    expect(view.container.querySelectorAll("[data-slot=world-annotation-list] li").length).toBe(4);
  });
});

describe("🌡️ scalar field heatmap", () => {
  const tetra: WorldMeshData = { positions: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1], normals: [], indices: [0, 1, 2, 0, 1, 3, 0, 2, 3, 1, 2, 3], faceIds: [10, 11, 12, 13], vertexIds: [20, 21, 22, 23] };

  it("paints a vertex field as canonical linear RGBA that matches three.js sRGB conversion, leaving picking ids alone", () => {
    ColorManagement.enabled = true;
    const field = parseWorld3dScalarField({ ...fixture.valid.find((entry) => entry.name === "scalar-vertex-inferno")!.value, values: [0, 0.5, 1, null] })!;
    const painted = world3dMeshDataWithScalarField(tetra, field)!;
    expect(painted.heatmap).toBe(true);
    expect(painted.faceIds).toEqual(tetra.faceIds);
    expect(painted.vertexIds).toEqual(tetra.vertexIds);
    const hex = ["#000004", fixture.ramps.inferno.samples.find((sample) => sample.t === 0.5)!.hex, "#fcffa4", fixture.noDataHex];
    hex.forEach((value, vertex) => {
      const oracle = new Color().setStyle(value, SRGBColorSpace);
      expect(painted.colors![vertex * 4]).toBeCloseTo(oracle.r, 6);
      expect(painted.colors![vertex * 4 + 1]).toBeCloseTo(oracle.g, 6);
      expect(painted.colors![vertex * 4 + 2]).toBeCloseTo(oracle.b, 6);
      expect(painted.colors![vertex * 4 + 3]).toBe(1);
    });
  });

  it("paints a face field as one constant colour per triangle and builds a geometry the renderer can draw", () => {
    const field = parseWorld3dScalarField({ ...fixture.valid.find((entry) => entry.name === "scalar-face-coolwarm")!.value, values: [10, 12, 18, 20] })!;
    const painted = world3dMeshDataWithScalarField(tetra, field)!;
    const visuals = buildMeshVisuals({ id: "m", data: painted });
    try {
      expect(visuals.geometry!.userData.world3dHeatmap).toBe(true);
      const color = visuals.geometry!.getAttribute("color");
      expect(color.count).toBe(12);
      const first = new Color().setStyle(fixture.ramps.coolwarm.samples[0]!.hex, SRGBColorSpace);
      expect(color.getX(0)).toBeCloseTo(first.r, 6);
      expect(color.getX(2)).toBeCloseTo(first.r, 6);
      expect(color.getX(0)).not.toBeCloseTo(color.getX(9), 3);
    } finally {
      disposeMeshVisuals(visuals);
    }
  });

  it("refuses a field whose value count does not fit the mesh, and keeps every other record's identity when it applies", () => {
    const short = parseWorld3dScalarField({ ...fixture.valid.find((entry) => entry.name === "scalar-vertex-inferno")!.value, meshId: "m", values: [0, 1] })!;
    expect(world3dMeshDataWithScalarField(tetra, short)).toBeNull();
    const records = [{ id: "m", data: tetra }, { id: "other", data: tetra }];
    expect(world3dMeshRecordsWithScalarField(records, short)).toEqual({ records, status: "mismatch" });
    expect(world3dMeshRecordsWithScalarField(records, undefined)).toEqual({ records, status: "none" });
    expect(world3dMeshRecordsWithScalarField([{ id: "other", data: tetra }], short).status).toBe("missing");
    const fits = parseWorld3dScalarField({ ...fixture.valid.find((entry) => entry.name === "scalar-vertex-inferno")!.value, meshId: "m", values: [0, 0.5, 1, null] })!;
    const applied = world3dMeshRecordsWithScalarField(records, fits);
    expect(applied.status).toBe("applied");
    expect(applied.records[1]).toBe(records[1]);
    expect(applied.records[0]).not.toBe(records[0]);
  });

  it("shows a legend with title, unit, the evenly spaced tick values and a no-data entry, in the active language", () => {
    const field = fieldOf("scalar-vertex-inferno");
    const view = render(createElement(World3dScalarLegendView, { field, locale: "en" }));
    const legend = view.container.querySelector("[data-slot=world-scalar-legend]")!;
    expect(legend.getAttribute("role")).toBe("group");
    expect(legend.getAttribute("aria-label")).toBe("Legend: Wall thickness");
    expect(view.container.querySelector("[data-slot=world-scalar-legend-title]")!.textContent).toBe("Wall thickness (mm)");
    const ticks = [...view.container.querySelectorAll("[data-slot=world-scalar-legend-tick]")];
    expect(ticks.map((tick) => tick.textContent)).toEqual(["0 mm", "0.25 mm", "0.5 mm", "0.75 mm", "1 mm"]);
    expect(ticks.map((tick) => tick.getAttribute("data-hex"))).toEqual(fixture.legends[0]!.ticks.map((tick) => tick.hex));
    expect(view.container.querySelector("[data-slot=world-scalar-legend-ticks]")!.getAttribute("aria-label")).toBe("Scale values");
    expect(view.container.querySelector("[data-slot=world-scalar-legend-ramp]")!.getAttribute("aria-hidden")).toBe("true");
    expect(view.container.querySelector("[data-slot=world-scalar-legend-no-data]")!.textContent).toBe("No data");
  });

  it("localizes the legend to German with a German decimal separator", () => {
    const view = render(createElement(World3dScalarLegendView, { field: fieldOf("scalar-vertex-inferno"), locale: "de" }));
    expect(view.container.querySelector("[data-slot=world-scalar-legend]")!.getAttribute("aria-label")).toBe("Legende: Wandstaerke");
    const ticks = [...view.container.querySelectorAll("[data-slot=world-scalar-legend-tick]")].map((tick) => tick.textContent);
    expect(ticks[1]).toBe("0,25 mm");
    expect(view.container.querySelector("[data-slot=world-scalar-legend-no-data]")!.textContent).toBe("Keine Daten");
  });

  it("draws the ramp gradient from its stops, bottom to top", () => {
    expect(world3dRampGradient("grayscale")).toBe("linear-gradient(to top, #000000 0%, #ffffff 100%)");
    expect(world3dRampGradient("viridis")).toBe(`linear-gradient(to top, ${fixture.ramps.viridis.stops.map((hex, index) => `${hex} ${index * 25}%`).join(", ")})`);
  });
});

describe("🎯️ pick granularity filter", () => {
  const guest = { selectionMode: "face", componentIds: [3, 4], hoveredComponent: { objectId: "a", mode: "face", id: 3 }, targets: { mesh: true, face: true, edge: true, vertex: true } };

  it.each(fixture.pickTargets)("restricts hover and selection to $filter", (row) => {
    const filtered = world3dSelectionWithPickFilter(guest, row.filter as World3dPickGranularity);
    expect(filtered.targets).toEqual({ ...row.targets, exclusive: true });
    expect(filtered.selectionMode).toBe(row.filter === "shape" ? "mesh" : row.filter);
  });

  it("keeps components of the chosen granularity and drops those of any other", () => {
    expect(world3dSelectionWithPickFilter(guest, "face")).toMatchObject({ componentIds: [3, 4], hoveredComponent: guest.hoveredComponent });
    for (const other of ["edge", "vertex", "shape"] as const) expect(world3dSelectionWithPickFilter(guest, other)).toMatchObject({ componentIds: [], hoveredComponent: undefined });
  });

  it("leaves the selection untouched without a filter", () => {
    expect(world3dSelectionWithPickFilter(guest, undefined)).toBe(guest);
  });
});

describe("🖍️ sub-element highlight tokens", () => {
  const colors = { select: "#aa0000", hover: "#00aa00", edgeHover: "#0000aa" };

  it("falls back to the theme paint for every granularity without a token", () => {
    expect(world3dSubElementPaint(colors, undefined, { edgeWidth: 3, vertexMarkPx: 11 })).toEqual({ faceSelect: "#aa0000", faceHover: "#00aa00", edgeSelect: "#aa0000", edgeHover: "#0000aa", vertexSelect: "#aa0000", vertexHover: "#00aa00", edgeWidth: 3, vertexMarkPx: 11 });
  });

  it("resolves tokens per granularity through the theme and overrides only what is named", () => {
    const paint = resolveWorld3dHighlightPaint({ face: { hover: "info", selected: "danger", widthPx: 3 }, edge: { selected: "success", widthPx: 5 }, vertex: { widthPx: 7 } }, (tone) => `hex-${tone}`);
    expect(world3dSubElementPaint(colors, paint, { edgeWidth: 3, vertexMarkPx: 11 })).toEqual({ faceSelect: "hex-danger", faceHover: "hex-info", edgeSelect: "hex-success", edgeHover: "#0000aa", vertexSelect: "#aa0000", vertexHover: "#00aa00", edgeWidth: 5, vertexMarkPx: 7 });
  });

  it("names theme tokens, with the foreground for neutral", () => {
    expect(world3dToneColorExpression("danger")).toBe("var(--color-danger)");
    expect(world3dToneColorExpression("neutral")).toBe("var(--color-foreground)");
    expect(typeof resolveWorld3dToneHex("primary")).toBe("string");
  });

  it("speaks the strings of the layer in English and German", () => {
    expect(world3dModellingStrings("en").noData).toBe("No data");
    expect(world3dModellingStrings("de-AT").noData).toBe("Keine Daten");
    expect(world3dModellingStrings(undefined).noData).toBe("No data");
  });
});

describe("✂️ section plane", () => {
  const section = parseWorld3dModellingOptions({ section: { origin: [0, 0, 5], normal: [0, 0, 2], cap: { tone: "secondary" } } })!.section!;

  function scene() {
    const plane = world3dSectionPlane(section);
    const root = new Group();
    const instance = new Group();
    instance.userData.world3dInstanceRoot = true;
    const solid = new Mesh(new BufferGeometry(), new MeshStandardMaterial());
    solid.userData.world3dSolid = true;
    const overlay = new Mesh(new BufferGeometry(), new MeshBasicMaterial());
    const furniture = new Mesh(new BufferGeometry(), new MeshBasicMaterial());
    instance.add(solid, overlay);
    root.add(instance, furniture);
    return { plane, root, solid, overlay, furniture };
  }

  it("builds the three.js plane the fixture derived: the side the normal points to is removed", () => {
    const plane = world3dSectionPlane(section);
    const oracle = new Plane(new Vector3(0, 0, -1), 5);
    expect(plane.normal.toArray().map((component) => component + 0)).toEqual(oracle.normal.toArray().map((component) => component + 0));
    expect(plane.constant).toBe(oracle.constant);
    expect(plane.distanceToPoint(new Vector3(0, 0, 9))).toBeLessThan(0);
    expect(plane.distanceToPoint(new Vector3(0, 0, 1))).toBeGreaterThan(0);
  });

  it("clips the materials inside instance roots only, flags them for recompilation once, and is idempotent", () => {
    const { plane, root, solid, overlay, furniture } = scene();
    const solidMaterial = solid.material as MeshStandardMaterial;
    const version = solidMaterial.version;
    expect(applyWorld3dSectionClipping(root, [plane])).toBe(2);
    expect((solidMaterial.clippingPlanes ?? [])[0]).toBe(plane);
    expect(solidMaterial.clipShadows).toBe(true);
    expect(solidMaterial.version).toBeGreaterThan(version);
    expect((overlay.material as MeshBasicMaterial).clippingPlanes).toEqual([plane]);
    expect((furniture.material as MeshBasicMaterial).clippingPlanes).toBeNull();
    expect(applyWorld3dSectionClipping(root, [plane])).toBe(0);
    expect(applyWorld3dSectionClipping(root, null)).toBe(2);
    expect(solidMaterial.clippingPlanes).toBeNull();
  });

  it("skips exempt subtrees", () => {
    const { plane, root, overlay } = scene();
    overlay.userData.world3dSectionExempt = true;
    expect(applyWorld3dSectionClipping(root, [plane])).toBe(1);
    expect((overlay.material as MeshBasicMaterial).clippingPlanes).toBeNull();
  });

  it("stencils exactly the solid meshes with a back-face increment and a front-face decrement pass, and retires them cleanly", () => {
    const { plane, root, solid, overlay } = scene();
    expect(collectWorld3dSolidMeshes(root)).toEqual([solid]);
    const stencils = new World3dSectionCapStencils(plane);
    stencils.sync(root);
    expect(stencils.count).toBe(1);
    const passes = solid.children as Mesh[];
    expect(passes.length).toBe(2);
    const [back, front] = passes.map((pass) => pass.material as MeshBasicMaterial);
    expect(back!.stencilWrite && front!.stencilWrite).toBe(true);
    expect(back!.colorWrite || front!.colorWrite).toBe(false);
    expect(back!.clippingPlanes).toEqual([plane]);
    expect(back!.stencilZPass).not.toBe(front!.stencilZPass);
    expect(passes.every((pass) => pass.userData.world3dSectionExempt === true)).toBe(true);
    expect(overlay.children.length).toBe(0);
    stencils.sync(root);
    expect(solid.children.length).toBe(2);
    solid.userData.world3dSolid = false;
    stencils.sync(root);
    expect(solid.children.length).toBe(0);
    expect(stencils.count).toBe(0);
    stencils.dispose();
  });
});

describe("🧭️ the host mounts the modelling lanes", () => {
  const baseScene = {
    cameraJson: JSON.stringify({ position: [6, -6, 4], target: [0, 0, 0], fov: 45 }),
    meshesJson: JSON.stringify([{ id: "mesh:part", data: { positions: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1], normals: [], indices: [0, 1, 2, 0, 1, 3, 0, 2, 3, 1, 2, 3], faceIds: [10, 11, 12, 13], vertexIds: [20, 21, 22, 23] } }]),
    instancesJson: JSON.stringify([{ id: "i1", meshId: "mesh:part", position: [0, 0, 0] }]),
    selectionJson: JSON.stringify({ method: "rectangle", mode: "replace", ids: [], hoveredId: null, selectionMode: "face", targets: { mesh: true, face: true, edge: true, vertex: true } }),
    vorticesJson: "[]",
    attractionsJson: "[]",
  };

  function mount(extra: Record<string, unknown>) {
    return render(createElement(World3dHost as never, { node: { type: "componentScene", surfaceId: "surface", controllerId: "controller", componentKind: "world-3d", world3d: { ...baseScene, ...extra } }, onAction: () => Promise.resolve() }));
  }

  it("shows the legend, announces the heatmap and renders the annotation region", () => {
    const view = mount({
      annotations: mixed,
      scalarField: parseWorld3dScalarField({ ...fixture.valid.find((entry) => entry.name === "scalar-vertex-inferno")!.value, values: [0, 0.3, 0.6, 1] }),
    });
    const host = view.container.querySelector(".semio-world-3d-host")!;
    expect(host.getAttribute("data-world-scalar-field")).toBe("mesh:part:applied");
    expect(host.getAttribute("data-world-annotation-count")).toBe("4");
    expect(view.container.querySelector("[data-slot=world-scalar-legend]")).not.toBeNull();
    expect(view.container.querySelectorAll("[data-slot=world-annotation-list] li").length).toBe(4);
  });

  it("says so, instead of painting a wrong heatmap, when the field does not fit the mesh", () => {
    const view = mount({ scalarField: parseWorld3dScalarField({ ...fixture.valid.find((entry) => entry.name === "scalar-vertex-inferno")!.value, values: [0, 1] }) });
    expect(view.container.querySelector(".semio-world-3d-host")!.getAttribute("data-world-scalar-field")).toBe("mesh:part:mismatch");
    expect(view.container.querySelector("[data-slot=world-scalar-legend]")).toBeNull();
    expect(view.container.querySelector("[data-slot=world-scalar-field-mismatch]")!.textContent).toBe("The scalar field does not match the mesh");
  });

  it.each(fixture.pickTargets)("constrains the published selection to the $filter filter", (row) => {
    const view = mount({ modellingOptions: { pickFilter: row.filter } });
    const host = view.container.querySelector(".semio-world-3d-host")!;
    expect(host.getAttribute("data-world-pick-filter")).toBe(row.filter);
    const published = JSON.parse(host.getAttribute("data-selection-json")!);
    expect(published.targets).toEqual({ ...row.targets, exclusive: true });
    expect(published.selectionMode).toBe(row.filter === "shape" ? "mesh" : row.filter);
  });

  it("announces the section state and mounts the cap plane only when capped", () => {
    const open = mount({ modellingOptions: { section: { origin: [0, 0, 5], normal: [0, 0, 1] } } });
    expect(open.container.querySelector(".semio-world-3d-host")!.getAttribute("data-world-section")).toBe("open");
    expect(open.container.querySelector("mesh[renderorder='2']")).toBeNull();
    cleanup();
    const capped = mount({ modellingOptions: { section: { origin: [0, 0, 5], normal: [0, 0, 1], cap: { tone: "secondary" } } } });
    expect(capped.container.querySelector(".semio-world-3d-host")!.getAttribute("data-world-section")).toBe("capped");
    expect(capped.container.querySelector("mesh[renderorder='2']")).not.toBeNull();
  });

  it("projects the layer on a played frame and only again when the camera moves", () => {
    const view = mount({ annotations: mixed });
    expect(recorded.frames.length).toBeGreaterThan(0);
    playFrames();
    const first = view.container.querySelectorAll("[data-slot=world-annotation-geometry] g").length;
    expect(first).toBeGreaterThan(0);
    const label = view.container.querySelector("[data-slot=world-annotation-label]")!.getAttribute("style");
    playFrames();
    expect(view.container.querySelector("[data-slot=world-annotation-label]")!.getAttribute("style")).toBe(label);
    recorded.camera!.position.set(3, -9, 2);
    recorded.camera!.lookAt(0, 0, 0);
    recorded.camera!.updateMatrixWorld(true);
    playFrames();
    expect(view.container.querySelector("[data-slot=world-annotation-label]")!.getAttribute("style")).not.toBe(label);
  });
});

function expectClose(actual: unknown, expected: unknown, path = "$"): void {
  if (typeof expected === "number") {
    expect(typeof actual, path).toBe("number");
    expect(Math.abs((actual as number) - expected), `${path}: ${actual} vs ${expected}`).toBeLessThan(1e-6);
  } else if (Array.isArray(expected)) {
    expect(Array.isArray(actual), path).toBe(true);
    expect((actual as unknown[]).length, path).toBe(expected.length);
    expected.forEach((entry, index) => expectClose((actual as unknown[])[index], entry, `${path}[${index}]`));
  } else if (expected !== null && typeof expected === "object") {
    expect(Object.keys(actual as object).sort(), path).toEqual(Object.keys(expected).sort());
    for (const [key, entry] of Object.entries(expected)) expectClose((actual as Record<string, unknown>)[key], entry, `${path}.${key}`);
  } else {
    expect(actual, path).toEqual(expected);
  }
}

describe("🧫️ shared fixture rows that the wgpu renderer is pinned against", () => {
  it("projects the annotation layer through the fixture camera to the fixture screen geometry", () => {
    const spec = fixture.annotationProjection;
    const camera = new PerspectiveCamera(spec.camera.fovYDegrees, spec.size.width / spec.size.height, spec.camera.near, spec.camera.far);
    camera.up.set(spec.camera.up[0]!, spec.camera.up[1]!, spec.camera.up[2]!);
    camera.position.set(spec.camera.position[0]!, spec.camera.position[1]!, spec.camera.position[2]!);
    camera.lookAt(spec.camera.target[0]!, spec.camera.target[1]!, spec.camera.target[2]!);
    camera.updateMatrixWorld(true);
    camera.updateProjectionMatrix();
    const projected = projectWorld3dAnnotations(layerOf(spec.layer), camera, spec.size).map((entry) => JSON.parse(JSON.stringify(entry)));
    expectClose(projected, spec.expected);
    expect(projected.some((entry) => !entry.visible)).toBe(true);
  });

  it("formats legend values exactly as the Intl rows say", () => {
    for (const row of fixture.legendValues.rows) expect(formatWorld3dLegendValue(row.value, row.locale)).toBe(row.text);
  });

  it("writes the legend text of both languages as the fixture rows say", () => {
    for (const row of fixture.legendLines.rows) expect(world3dLegendLines(parseWorld3dScalarField(row.field)!, row.locale)).toEqual(row.lines);
  });

  it("resolves sub-element paint tokens over the theme defaults as the fixture rows say", () => {
    for (const row of fixture.subElementPaint.rows) {
      const highlight = row.highlight === null ? undefined : parseWorld3dModellingOptions({ highlight: row.highlight })?.highlight;
      const paint = resolveWorld3dHighlightPaint(highlight, (tone) => `tone:${tone}`);
      expect(world3dSubElementPaint({ select: row.defaults.select, hover: row.defaults.hover, edgeHover: row.defaults.edgeHover }, paint, { edgeWidth: row.defaults.edgeWidth, vertexMarkPx: row.defaults.vertexMarkPx }), row.name).toEqual(row.expected);
    }
  });
});

describe("🪪️ GLB sub-element ids", () => {
  const spec = fixture.glbSubElementIds;
  function primitive(attributeNames: { readonly face: string; readonly vertex: string } | null, offset = 0): Mesh {
    const geometry = new BufferGeometry();
    geometry.setAttribute("position", new BufferAttribute(new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0]), 3));
    geometry.setIndex(spec.indices);
    if (attributeNames) {
      geometry.setAttribute(attributeNames.face, new BufferAttribute(new Uint32Array(spec.faceIds.map((id: number) => id + offset)), 1));
      geometry.setAttribute(attributeNames.vertex, new BufferAttribute(new Uint32Array(spec.vertexIds.map((id: number) => id + offset)), 1));
    }
    return new Mesh(geometry, new MeshBasicMaterial());
  }

  it("reads the face id at each triangle's first corner and the vertex id per vertex", () => {
    const ids = world3dGlbSubElementIds(primitive({ face: spec.faceAttribute, vertex: spec.vertexAttribute }).geometry as never);
    expect(ids.faceIds).toEqual(spec.expectedTriangleFaceIds);
    expect(ids.vertexIds).toEqual(spec.vertexIds);
  });

  it("accepts the lower-cased names three's GLTFLoader gives custom attributes", () => {
    const ids = world3dGlbSubElementIds(primitive({ face: spec.faceAttribute.toLowerCase(), vertex: spec.vertexAttribute.toLowerCase() }).geometry as never);
    expect(ids.faceIds).toEqual(spec.expectedTriangleFaceIds);
  });

  it("answers no ids for a primitive without the attributes and resolves a pointer hit to the topology face", () => {
    expect(world3dGlbSubElementIds(primitive(null).geometry as never)).toEqual({ faceIds: null, vertexIds: null });
    const mesh = primitive({ face: spec.faceAttribute, vertex: spec.vertexAttribute });
    expect(world3dGlbHitFaceId(mesh, 0)).toBe(7);
    expect(world3dGlbHitFaceId(mesh, 1)).toBe(9);
    expect(world3dGlbHitFaceId(mesh, 2)).toBeUndefined();
    expect(world3dGlbHitFaceId(primitive(null), 0)).toBeUndefined();
    expect(world3dGlbHitFaceId(undefined, 0)).toBeUndefined();
  });

  it("merges the primitives of a scene into one picking mesh and keeps an id table only when every primitive carries it", () => {
    const names = { face: spec.faceAttribute, vertex: spec.vertexAttribute };
    const root = new Group();
    root.add(primitive(names), primitive(names, 1000));
    const merged = world3dGlbSubElementMeshData(root)!;
    expect(merged.indices).toEqual([...spec.indices, ...spec.indices.map((index: number) => index + 4)]);
    expect(merged.faceIds).toEqual([...spec.expectedTriangleFaceIds, ...spec.expectedTriangleFaceIds.map((id: number) => id + 1000)]);
    expect(merged.vertexIds).toEqual([...spec.vertexIds, ...spec.vertexIds.map((id: number) => id + 1000)]);
    const mixed = new Group();
    mixed.add(primitive(names), primitive(null));
    const partial = world3dGlbSubElementMeshData(mixed)!;
    expect(partial.faceIds).toBeUndefined();
    expect(partial.vertexIds).toBeUndefined();
    expect(partial.indices.length).toBe(12);
    expect(world3dGlbSubElementMeshData(new Group())).toBeNull();
  });
});
