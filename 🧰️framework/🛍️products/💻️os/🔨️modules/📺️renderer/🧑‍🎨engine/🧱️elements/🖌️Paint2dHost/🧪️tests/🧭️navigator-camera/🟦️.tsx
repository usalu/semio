// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { uiI18n } from "@semio-tech/ui-react";
import { Matrix3, Vector3 } from "three";
import { afterEach, expect, it, vi } from "vitest";
import { Paint2dHost } from "../../🟦️.tsx";
import schema from "../../🧬️schema/🧭️navigator-camera/🔣️.json";
import fixture from "../../🧫️fixtures/🧭️navigator-camera/🔣️.json";

vi.mock("../../../🪪️WasmSessionLoader/🟦️.tsx", async (load) => ({ ...await load<Record<string, unknown>>(), createRasterSession: () => new Promise(() => {}) }));

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

it("validates the neutral Navigator camera contract", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

it("the mounted React Navigator publishes the authored content camera on wheel", async () => {
  await uiI18n.changeLanguage("en");
  for (const sample of fixture.cases) {
    const actions: any[] = [];
    const scene = { documentSyncJson: '{"schema":"raster.document","layers":[]}', assetsJson: "{}", cameraJson: JSON.stringify(sample.camera), selectionJson: "[]", activeUtility: "brush", brushSize: 12, brushOpacity: 1, brushColor: "#000000", brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24, viewMode: "navigator" as const, ...(sample.viewport ? { compositeViewportJson: JSON.stringify(sample.viewport) } : {}) };
    const view = render(<Paint2dHost node={{ type: "componentScene", surfaceId: "navigator-camera", controllerId: "raster", componentKind: "paint2d", paint2d: scene }} onAction={action => { actions.push(action); }} />);
    const host = view.container.querySelector<HTMLElement>('[data-surface-id="navigator-camera"]')!;
    const [x, y, width, height] = sample.surface;
    vi.spyOn(host, "getBoundingClientRect").mockReturnValue({ x, y, width, height, left: x, top: y, right: x + width, bottom: y + height, toJSON: () => ({}) });
    await act(async () => { host.querySelector(".z-30")!.dispatchEvent(new WheelEvent("wheel", { bubbles: true, cancelable: true, clientX: sample.point[0], clientY: sample.point[1], deltaY: sample.deltaY })); });
    expect(actions, sample.id).toHaveLength(1);
    expect(actions[0].action).toBe("setCamera");
    expect(actions[0].controllerId).toBe("raster");
    expect(actions[0].args.surfaceId).toBe("navigator-camera");
    for (const key of ["x", "y", "zoom"] as const) expect(actions[0].args.camera[key], sample.id).toBeCloseTo(sample.expected[key], 10);
    cleanup();
  }
});

it("the resulting camera preserves the wheel anchor through Three matrix inversion", () => {
  for (const sample of fixture.cases) {
    const viewport = sample.viewport ?? { width: 800, height: 600 };
    const screen = new Vector3(sample.point[0] - sample.surface[0], sample.point[1] - sample.surface[1], 1);
    const unproject = (camera: typeof sample.camera) => screen.clone().applyMatrix3(new Matrix3().set(camera.zoom, 0, viewport.width / 2 - camera.x * camera.zoom, 0, camera.zoom, viewport.height / 2 - camera.y * camera.zoom, 0, 0, 1).invert());
    expect(unproject(sample.camera).distanceTo(unproject(sample.expected)), sample.id).toBeLessThan(1e-9);
  }
});
