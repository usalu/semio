import Ajv2020 from "ajv/dist/2020.js";
import { expect, test } from "bun:test";
import { Color, ColorManagement, Plane, Vector3 } from "three";

import fixture from "../../🧫️fixtures/📏️world3d-modelling/🔣️.json";
import schema from "../../🧬️schema/📏️world3d-modelling/🔣️.json";
import { WORLD3D_SCENE_LANES, world3dSceneFromLanes, type World3dScene } from "../../🟦️.ts";
import {
  parseWorld3dAnnotationLayer,
  parseWorld3dModellingOptions,
  parseWorld3dScalarField,
  resolveWorld3dText,
  WORLD3D_COLOR_RAMPS,
  world3dPickTargets,
  world3dRampHex,
  world3dScalarColorHex,
  world3dScalarFieldColorBytes,
  world3dScalarLegend,
  world3dSectionClipPlane,
  type World3dColorRamp,
  type World3dPickGranularity,
  type World3dScalarField,
  type World3dSection,
} from "../../📏️world3d-modelling/🟦️.ts";

ColorManagement.enabled = false;

const ajv = new Ajv2020({ strict: true, allowUnionTypes: true });
ajv.addSchema(schema);
const validator = (def: string) => ajv.compile({ $ref: `${schema.$id}#/$defs/${def}` });
const parsers: Record<string, (value: unknown) => unknown> = {
  annotationLayer: parseWorld3dAnnotationLayer,
  scalarField: parseWorld3dScalarField,
  modellingOptions: parseWorld3dModellingOptions,
};

test("the schema document compiles and accepts every valid fixture through the reference validator", () => {
  for (const entry of fixture.valid) {
    const validate = validator(entry.def);
    expect(validate(entry.value), `${entry.name}: ${ajv.errorsText(validate.errors)}`).toBe(true);
  }
});

test("the schema document rejects every schema-level invalid fixture and accepts the semantic ones", () => {
  for (const entry of fixture.invalid) {
    const validate = validator(entry.def);
    expect(validate(entry.value), `${entry.name} (${entry.reason})`).toBe(entry.ajvAccepts);
  }
});

test("the typed parsers return the normalized value of every valid fixture", () => {
  for (const entry of fixture.valid) expect(parsers[entry.def]!(entry.value), entry.name).toEqual(entry.normalized);
});

test("the typed parsers reject every invalid fixture, schema-level and semantic", () => {
  for (const entry of fixture.invalid) expect(parsers[entry.def]!(entry.value), `${entry.name} (${entry.reason})`).toBeNull();
  const item = fixture.valid[0]!.normalized.items![2]!;
  expect(parseWorld3dAnnotationLayer({ items: Array.from({ length: 512 }, (_, index) => ({ ...item, id: `m${index}` })) })).not.toBeNull();
  expect(parseWorld3dAnnotationLayer({ items: Array.from({ length: 513 }, (_, index) => ({ ...item, id: `m${index}` })) })).toBeNull();
  for (const hostile of [null, undefined, 4, "x", [], { items: "no" }, { items: [null] }]) expect(parseWorld3dAnnotationLayer(hostile)).toBeNull();
});

test("colour ramps interpolate exactly as three.js does", () => {
  for (const [name, ramp] of Object.entries(fixture.ramps)) {
    expect(WORLD3D_COLOR_RAMPS[name as World3dColorRamp]).toEqual(ramp.stops);
    for (const sample of ramp.samples) {
      expect(world3dRampHex(name as World3dColorRamp, sample.t), `${name} @ ${sample.t}`).toBe(sample.hex);
      const hexes = ramp.stops;
      const scaled = sample.t * (hexes.length - 1);
      const index = Math.min(Math.floor(scaled), hexes.length - 2);
      const oracle = new Color(hexes[index]).lerp(new Color(hexes[index + 1]), scaled - index);
      expect(world3dRampHex(name as World3dColorRamp, sample.t)).toBe(`#${oracle.getHexString()}`);
    }
    expect(world3dRampHex(name as World3dColorRamp, -3)).toBe(ramp.samples[0]!.hex);
    expect(world3dRampHex(name as World3dColorRamp, 7)).toBe(ramp.samples.at(-1)!.hex);
  }
});

test("scalar fields colour values through range clamping and mark missing data", () => {
  for (const entry of fixture.scalarColors) {
    const field = parseWorld3dScalarField(entry.field)!;
    expect(field.values.map((value) => world3dScalarColorHex(field, value))).toEqual(entry.colors);
    const bytes = world3dScalarFieldColorBytes(field);
    expect(bytes.length).toBe(field.values.length * 3);
    entry.colors.forEach((hex, index) => expect([...bytes.subarray(index * 3, index * 3 + 3)]).toEqual([1, 3, 5].map((offset) => Number.parseInt(hex.slice(offset, offset + 2), 16))));
  }
  expect(world3dScalarColorHex(parseWorld3dScalarField(fixture.scalarColors[0]!.field)!, null)).toBe(fixture.noDataHex);
});

test("scalar field legends sample the ramp at evenly spaced ticks", () => {
  for (const entry of fixture.legends) {
    const ticks = world3dScalarLegend(parseWorld3dScalarField(entry.field) as World3dScalarField);
    expect(ticks.length).toBe(entry.ticks.length);
    ticks.forEach((tick, index) => {
      expect(tick.value).toBeCloseTo(entry.ticks[index]!.value, 12);
      expect(tick.hex).toBe(entry.ticks[index]!.hex);
    });
  }
});

test("the pick filter admits exactly one granularity", () => {
  for (const row of fixture.pickTargets) expect(world3dPickTargets(row.filter as World3dPickGranularity), row.filter).toEqual(row.targets);
  expect(world3dPickTargets(undefined)).toEqual({ mesh: true, face: true, edge: true, vertex: true });
});

test("the section plane keeps the side opposite its normal, as three.js clips it", () => {
  for (const row of fixture.sectionPlanes) {
    const section = parseWorld3dModellingOptions({ section: row.section })!.section as World3dSection;
    const plane = world3dSectionClipPlane(section);
    plane.forEach((component, index) => expect(component).toBeCloseTo(row.plane[index]!, 12));
    const oracle = new Plane(new Vector3(plane[0], plane[1], plane[2]), plane[3]);
    const origin = new Vector3(...section.origin);
    const normal = new Vector3(...section.normal).normalize();
    expect(oracle.distanceToPoint(origin)).toBeCloseTo(0, 12);
    expect(oracle.distanceToPoint(origin.clone().addScaledVector(normal, 1))).toBeLessThan(0);
    expect(oracle.distanceToPoint(origin.clone().addScaledVector(normal, -1))).toBeGreaterThan(0);
  }
});

test("text resolves for the active locale and never invents a language", () => {
  const text = { en: "Width", de: "Breite" };
  expect(resolveWorld3dText(text, "de")).toBe("Breite");
  expect(resolveWorld3dText(text, "de-CH")).toBe("Breite");
  expect(resolveWorld3dText(text, "en-GB")).toBe("Width");
  expect(resolveWorld3dText(text, "fr")).toBe("Width");
});

test("the three modelling lanes ride the world scene as typed json carriers", () => {
  const lanes = fixture.laneRoundTrip.laneKeys.map((key) => WORLD3D_SCENE_LANES.find((lane) => lane.bodyKey === key));
  expect(lanes.map((lane) => lane?.field)).toEqual(["annotations", "scalarField", "modellingOptions"]);
  for (const lane of lanes) {
    expect(lane?.encoding).toBe("json");
    expect(lane?.optional).toBe(true);
  }
  const scene = fixture.laneRoundTrip.scene;
  const texts = new Map(fixture.laneRoundTrip.laneKeys.map((key, index) => [key, JSON.stringify(Object.values(scene)[index])]));
  const spine = { cameraJson: "{}", meshesJson: "[]", instancesJson: "[]", selectionJson: "{}" } as World3dScene;
  const assembled = world3dSceneFromLanes(spine, texts);
  expect(assembled.annotations).toEqual(scene.annotations as never);
  expect(assembled.scalarField).toEqual(scene.scalarField as never);
  expect(assembled.modellingOptions).toEqual(scene.modellingOptions as never);
  expect(parseWorld3dAnnotationLayer(assembled.annotations)).toEqual(scene.annotations as never);
});
