import { expect, test } from "bun:test";
import Ajv from "ajv";
import { BufferGeometry, DoubleSide, Float32BufferAttribute, Mesh, MeshBasicMaterial, Raycaster, Vector3 } from "three";
import terrain from "../../../../../../../../../🔨️modules/🗺️surface/🏔️terrain/🧪️tests/🧫️fixtures/📦️elevation-grid/🔣️.json" with { type: "json" };
import fixture from "../../../../../../../../../🔨️modules/🗺️surface/🏔️terrain/🗺️overlay/🧪️tests/🧫️fixtures/🖱️move-position.json" with { type: "json" };
import schema from "../../../../../../../../../🔨️modules/🗺️surface/🏔️terrain/🗺️overlay/📍️move-position/🔣️.json" with { type: "json" };
import { parseTerrainMovePosition, TerrainPositionDrag } from "../📍️move-position.ts";

test("only a valid editor descriptor opts into terrain marker dragging", () => {
  const validate = new Ajv().compile(schema);
  for (const descriptor of [fixture.descriptor, undefined, { action: "" }, { action: "other:action" }, { ...fixture.descriptor, extra: true }]) {
    expect(Boolean(parseTerrainMovePosition(descriptor))).toBe(validate(descriptor));
  }
});

function oracleCoordinates(rayIndex: number, missing?: readonly number[]): readonly [number, number] | undefined {
  const { bounds: [west, south, east, north], origin: [longitude, latitude], samples, exaggeration, pick } = terrain;
  const { size, southwest, northwardStep, eastwardStep } = samples;
  const xScale = 111_320 * Math.cos(latitude! * Math.PI / 180);
  const yScale = 111_320;
  const vertex = (column: number, row: number, elevation: number) => new Vector3(
    (west! - longitude! + (east! - west!) * column / (size - 1)) * xScale,
    (south! - latitude! + (north! - south!) * row / (size - 1)) * yScale,
    elevation,
  );
  const { raisedSample } = pick;
  const positions = Array.from({ length: size * size }, (_, index) => {
    const [row, column] = [Math.floor(index / size), index % size];
    const raised = raisedSample.row === row && raisedSample.column === column ? raisedSample.meters : 0;
    return vertex(column, row, (southwest + row * northwardStep + column * eastwardStep + raised) * exaggeration).toArray();
  }).flat();
  const missingIndex = missing ? missing[1]! * size + missing[0]! : -1;
  const indices: number[] = [];
  for (let row = 0; row < size - 1; row++) {
    for (let column = 0; column < size - 1; column++) {
      const sw = row * size + column;
      const nw = sw + size;
      for (const triangle of [[sw, sw + 1, nw + 1], [sw, nw + 1, nw]]) {
        if (!triangle.includes(missingIndex)) indices.push(...triangle);
      }
    }
  }
  const geometry = new BufferGeometry();
  const material = new MeshBasicMaterial({ side: DoubleSide });
  try {
    geometry.setAttribute("position", new Float32BufferAttribute(positions, 3));
    geometry.setIndex(indices);
    const { originSample, direction } = pick.rays[rayIndex]!;
    const ray = new Raycaster(vertex(originSample[0]!, originSample[1]!, originSample[2]!), new Vector3().fromArray(direction).normalize());
    const point = ray.intersectObject(new Mesh(geometry, material))[0]?.point;
    return point && [point.x / xScale + longitude!, point.y / yScale + latitude!];
  } finally {
    geometry.dispose();
    material.dispose();
  }
}

test("one captured marker emits at most one move on a valid release", () => {
  for (const row of fixture.cases) {
    const drag = new TerrainPositionDrag();
    let releases = 0;
    expect(drag.begin(7, fixture.marker.id, 0, 0, () => { ++releases; })).toBe(true);
    expect(drag.begin(8, "other", 0, 0, () => {})).toBe(false);
    if (row.cancel || row.stale) drag.cancel(7);
    const x = row.noTravel ? 0 : 20;
    const coordinates = oracleCoordinates(row.rayIndex, row.missingSample);
    expect(drag.finish(8, x, 0, coordinates)).toBeUndefined();
    const move = drag.finish(7, x, 0, coordinates);
    if (row.action) {
      expect(move?.id).toBe(row.action.id);
      expect(move?.longitude).toBeCloseTo(row.action.longitude, 12);
      expect(move?.latitude).toBeCloseTo(row.action.latitude, 12);
    } else {
      expect(move, row.name).toBeUndefined();
    }
    expect(drag.finish(7, x, 0, coordinates)).toBeUndefined();
    expect(releases).toBe(1);
  }
});
