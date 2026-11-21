import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {pathToFileURL as fileUrl} from "node:url";
import ts from "typescript";
import { PNG } from "pngjs";
import { BufferGeometry, DoubleSide, Float32BufferAttribute, Mesh, MeshBasicMaterial, Raycaster, Vector3 } from "three";
import Ajv2020 from "ajv/dist/2020.js";
import corpus from "../🧫️fixtures/🔌️ownership/🔣️.json";
import overlayFixture from "../../🏔️terrain/🗺️overlay/🧪️tests/🧫️fixtures/🔣️.json";
import overlaySchema from "../../🏔️terrain/🗺️overlay/🧬️schema/🔣️.json";
import elevationGridFixture from "../../🏔️terrain/🧪️tests/🧫️fixtures/📦️elevation-grid/🔣️.json";
import terrainPickSchema from "../../🏔️terrain/🎯️pick/🧬️schema/🔣️.json";

const files = new Map<string, string>(JSON.parse(process.env.SEMIO_SURFACE_WASM_PACKAGE ?? "[]"));
function file(name: string): string {
 const path = files.get(name);
 if (!path) throw Error(`Produced surface package lacks ${name}`);
 return path;
}

test("General Surface generated browser owners initialize without Specific graph exports",async()=>{
 const source=ts.createSourceFile("framework_surface.d.ts",readFileSync(file("framework_surface.d.ts"),"utf8"),ts.ScriptTarget.Latest,true);
 const classes=source.statements.filter(ts.isClassDeclaration).filter(statement=>statement.modifiers?.some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword)).map(statement=>statement.name!.text).sort();
 expect(classes).toEqual([...corpus.browserSessions].sort());
 const module=await import(fileUrl(file("framework_surface.js")).href);
 const bytes=readFileSync(file("framework_surface_bg.wasm"));
 const compiled=await WebAssembly.compile(bytes);
 const reflected=WebAssembly.Module.exports(compiled).map(entry=>entry.name);
 const initialized=await module.default({module_or_path:compiled});
 for(const name of corpus.browserSessions){expect(typeof module[name]).toBe("function");expect(reflected.some(symbol=>symbol.startsWith(name.toLowerCase()+"_"))).toBe(true);}
 for(const name of corpus.specificBrowserSessions){expect(name in module).toBe(false);expect(reflected.some(symbol=>symbol.toLowerCase().includes(name.toLowerCase()))).toBe(false);}
 expect(Object.keys(initialized).sort()).toEqual(reflected.sort());
});

test("Terrain overlays follow the actual browser mesh triangles", async () => {
 const validate = new Ajv2020({ strict: true }).compile(overlaySchema);
 expect(validate(overlayFixture.overlays)).toBe(true);
 const module = await import(fileUrl(file("framework_surface.js")).href);
 await module.default({ module_or_path: await WebAssembly.compile(readFileSync(file("framework_surface_bg.wasm"))) });
 const session = new module.TerrainSession();
 const [z, x, y] = overlayFixture.tile;
 const [lon, lat] = overlayFixture.origin;
 session.set_project_origin(lon, lat);
 session.set_exaggeration(overlayFixture.exaggeration);
 const png = new PNG({ width: 256, height: 256 });
 for (let row = 0; row < 256; row += 1) {
  for (let col = 0; col < 256; col += 1) {
   const high = (Math.floor(row / 32) + Math.floor(col / 32)) % 2 !== 0;
   png.data.set([128, high ? 125 : 40, 0, 255], (row * 256 + col) * 4);
  }
 }
 expect(session.upload_elevation_tile(z, x, y, PNG.sync.write(png))).toBe(true);
 const data = JSON.parse(session.terrain_tile_mesh_json(z, x, y));
 const overlay = JSON.parse(session.terrain_overlay_tile_json(z, x, y, JSON.stringify(overlayFixture.overlays))) as {
  positions: { id: string; point: [number, number, number] }[];
  routes: { id: string; segments: [[number, number, number], [number, number, number]][] }[];
  regions: { id: string; segments: [[number, number, number], [number, number, number]][] }[];
 };
 session.free();
 const expected = overlayFixture.expected;
 expect(overlay.positions.map(feature => feature.id)).toEqual(expected.visiblePositions);
 expect(overlay.routes.map(feature => feature.id)).toEqual(expected.visibleRoutes);
 expect(overlay.regions.map(feature => feature.id)).toEqual(expected.visibleRegions);
 expect(overlay.positions[1]!.point[2]).toBeCloseTo(expected.raisedWorldHeight, 3);
 const geometry = new BufferGeometry();
 geometry.setAttribute("position", new Float32BufferAttribute(data.positions, 3));
 geometry.setIndex(data.indices);
 const mesh = new Mesh(geometry, new MeshBasicMaterial());
 const terrainHeight = (x: number, y: number): number => {
  const ray = new Raycaster(new Vector3(x, y, 1_000), new Vector3(0, 0, -1));
  return ray.intersectObject(mesh)[0]!.point.z;
 };
 const [groundX, groundY, height] = overlay.positions[0]!.point;
 expect(height).toBeCloseTo(terrainHeight(groundX, groundY), 2);
 for (const path of [...overlay.routes, ...overlay.regions]) {
  for (const [a, b] of path.segments) {
   for (const fraction of [0.25, 0.5, 0.75]) {
    const [x, y, z] = a.map((coordinate, axis) => coordinate + (b[axis]! - coordinate) * fraction);
    expect(z!).toBeCloseTo(terrainHeight(x!, y!), 2);
   }
  }
 }
 mesh.material.dispose();
 geometry.dispose();
});

test("An embedded artifact elevation grid draws and drapes without tile uploads", async () => {
 const { bounds, origin, exaggeration, samples, expected, overlays } = elevationGridFixture;
 const module = await import(fileUrl(file("framework_surface.js")).href);
 await module.default({ module_or_path: await WebAssembly.compile(readFileSync(file("framework_surface_bg.wasm"))) });
 const session = new module.TerrainSession();
 try {
  session.set_project_origin(origin[0], origin[1]);
  session.set_exaggeration(exaggeration);
  expect(session.set_elevation_grid_json(JSON.stringify({ bounds, rows: [[40]] }))).toBe(false);
  const rows = Array.from({ length: samples.size }, (_, row) => Array.from({ length: samples.size }, (_, column) =>
   samples.southwest + row * samples.northwardStep + column * samples.eastwardStep));
  expect(session.set_elevation_grid_json(JSON.stringify({ bounds, rows }))).toBe(true);
  const [visible] = JSON.parse(session.visible_terrain_tiles_json("{}"));
  expect(JSON.parse(session.visible_terrain_tiles_json("{}"))).toHaveLength(1);
  const { z, x, y } = visible;
  const data = JSON.parse(session.terrain_tile_mesh_json(z, x, y));
  expect(data.positions[2]).toBe(expected.northwestHeight);
  expect(data.positions.at(-1)).toBe(expected.southeastHeight);
  expect(data.indices).toHaveLength(expected.triangleCount * 3);
  expect(session.terrain_origin_elevation(z, x, y)).toBeCloseTo(expected.originHeight, 3);
  const overlay = JSON.parse(session.terrain_overlay_tile_json(z, x, y, JSON.stringify(overlays)));
  expect(overlay.positions[0].point[2]).toBeCloseTo(expected.originHeight, 3);
  expect(overlay.routes[0].segments.length).toBeGreaterThan(32);
  expect(overlay.regions[0].segments.length).toBeGreaterThan(32);
  const geometry = new BufferGeometry();
  const material = new MeshBasicMaterial();
  try {
   geometry.setAttribute("position", new Float32BufferAttribute(data.positions, 3));
   geometry.setIndex(data.indices);
   const mesh = new Mesh(geometry, material);
   for (const path of [...overlay.routes, ...overlay.regions]) {
    for (const [a, b] of path.segments) {
     const [x, y, height] = a.map((value: number, axis: number) => (value + b[axis]) / 2);
     const ray = new Raycaster(new Vector3(x, y, 1_000), new Vector3(0, 0, -1));
     expect(ray.intersectObject(mesh)[0]!.point.z).toBeCloseTo(height, 2);
    }
   }
  } finally {
   material.dispose();
   geometry.dispose();
  }
 } finally {
  session.free();
 }
});

test("An embedded terrain pick agrees with Three.js on the rendered triangles", async () => {
 const { bounds: [west, south, east, north], origin, exaggeration, samples, pick } = elevationGridFixture;
 const { $defs, $schema } = terrainPickSchema;
 const ajv = new Ajv2020({ strict: true });
 const validateRay = ajv.compile({ $schema, $defs, $ref: "#/$defs/ray" });
 const validateHit = ajv.compile({ $schema, $defs, $ref: "#/$defs/hit" });
 const module = await import(fileUrl(file("framework_surface.js")).href);
 await module.default({ module_or_path: await WebAssembly.compile(readFileSync(file("framework_surface_bg.wasm"))) });
 const session = new module.TerrainSession();
 const geometry = new BufferGeometry();
 const material = new MeshBasicMaterial({ side: DoubleSide });
 try {
  session.set_project_origin(origin[0], origin[1]);
  session.set_exaggeration(exaggeration);
  const rows = Array.from({ length: samples.size }, (_, row) => Array.from({ length: samples.size }, (_, column) =>
   samples.southwest + row * samples.northwardStep + column * samples.eastwardStep));
  const raised = pick.raisedSample;
  rows[raised.row]![raised.column]! += raised.meters;
  expect(session.set_elevation_grid_json(JSON.stringify({ bounds: [west, south, east, north], rows }))).toBe(true);
  const data = JSON.parse(session.terrain_tile_mesh_json(0, 0, 0));
  geometry.setAttribute("position", new Float32BufferAttribute(data.positions, 3));
  geometry.setIndex(data.indices);
  const mesh = new Mesh(geometry, material);
  const cells = samples.size - 1;
  for (const vector of pick.rays) {
   const [column, row, z] = vector.originSample;
   const lon = west + (east - west) * column / cells;
   const lat = south + (north - south) * row / cells;
   const ray = { origin: [(lon - origin[0]) * 111_320 * Math.cos(origin[1] * Math.PI / 180), (lat - origin[1]) * 111_320, z], direction: vector.direction };
   expect(validateRay(ray)).toBe(true);
   const hit = JSON.parse(session.pickElevationGridJson(JSON.stringify(ray)));
   const oracle = new Raycaster(new Vector3().fromArray(ray.origin), new Vector3().fromArray(ray.direction).normalize()).intersectObject(mesh)[0];
   if (vector.miss) {
    expect(hit).toBeNull();
    expect(oracle).toBeUndefined();
    continue;
   }
   expect(validateHit(hit)).toBe(true);
   expect(oracle).toBeDefined();
   for (const [axis, value] of oracle!.point.toArray().entries()) expect(hit.position[axis]).toBeCloseTo(value, 3);
   if (vector.sample) expect(hit.sample).toEqual(vector.sample);
  }
 } finally {
  material.dispose();
  geometry.dispose();
  session.free();
 }
});
