import Decimal from "decimal.js";
import {binary64, binary64Value} from "@semio-tech/framework/value/ieee754";
import {ShapeUtils, Vector2, Shape, EllipseCurve, ExtrudeGeometry, BufferGeometry, BufferAttribute} from "three";
import {test as register} from "node:test";
import assert from "node:assert/strict";
export const records: {name: string; passed: boolean; error?: string}[] = [];
export const checks: Promise<void>[] = [];
const test = (name: string, run: () => void): void => {checks.push(register(name, () => {try {run(); records.push({name, passed: true});} catch (error) {records.push({name, passed: false, error: String(error)}); throw error;}}));};
const describe = (_name: string, run: () => void): void => run();
const expect = (actual: unknown) => ({
  toBe: (expected: unknown): void => assert.equal(actual, expected),
  toEqual: (expected: unknown): void => assert.deepEqual(actual, expected),
  toBeCloseTo: (expected: number, places: number): void => assert.ok(typeof actual === "number" && Math.abs(actual - expected) < 10 ** -places, String(actual) + " != " + expected),
  toBeGreaterThan: (expected: number): void => assert.ok(typeof actual === "number" && actual > expected),
  toBeLessThanOrEqual: (expected: number): void => assert.ok(typeof actual === "number" && actual <= expected),
});
import {readFileSync} from "node:fs";
import {decodeIfc4} from "../../../../🚪️io/🟦️.ts";
import {DEFAULT_GEOMETRY_LIMITS, geometryBounds, geometryPayloads, project, projectControlled, projectWith} from "../../🟦️.ts";
import type {GeometryScene, Point} from "../../🟦️.ts";
import type {IfcSnapshot} from "../../../📸️snapshot/🟦️.ts";

interface GeometryCase {name: string; data: string; expected: {products: number[]; vertices: number; triangles: number; bounds?: [Point, Point]; volume?: number; area?: number; issue?: boolean}}
const fixtures = JSON.parse(readFileSync(new URL("../🔣️.json", import.meta.url), "utf8")) as {cases: GeometryCase[]};
const snapshot = (data: string): IfcSnapshot => decodeIfc4("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;" + data + "ENDSEC;END-ISO-10303-21;");
const dot = (a: Point, b: Point): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: Point, b: Point): Point => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const minus = (a: Point, b: Point): Point => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
function measures(scene: GeometryScene): {area: number; volume: number} {
  let volume = 0, area = 0;
  for (const mesh of scene.meshes) for (const triangle of mesh.triangles) {
    const [a, b, c] = triangle.map(index => mesh.vertices[index]), normal = cross(minus(b, a), minus(c, a));
    volume += dot(a, cross(b, c)) / 6; area += Math.sqrt(dot(normal, normal)) / 2;
  }
  return {area, volume: Math.abs(volume)};
}

describe("IFC4 representation geometry", () => {
  for (const fixture of fixtures.cases) test("neutral geometry: " + fixture.name, () => {
    const scene = project(snapshot(fixture.data)), expected = fixture.expected;
    expect(scene.status).toBe("complete");
    expect(scene.meshes.map(mesh => mesh.product)).toEqual(expected.products.map(BigInt));
    expect(scene.meshes.reduce((sum, mesh) => sum + mesh.vertices.length, 0)).toBe(expected.vertices);
    expect(scene.meshes.reduce((sum, mesh) => sum + mesh.triangles.length, 0)).toBe(expected.triangles);
    if (expected.bounds) {
      const bounds = geometryBounds(scene)!;
      for (let side = 0; side < 2; side++) for (let axis = 0; axis < 3; axis++) expect(bounds[side][axis]).toBeCloseTo(expected.bounds[side][axis], 9);
    }
    if (expected.issue) expect(scene.issues.length).toBeGreaterThan(0);
    else expect(scene.issues).toEqual([]);
    const measured = measures(scene);
    if (expected.volume !== undefined) expect(measured.volume).toBeCloseTo(expected.volume, 9);
    if (expected.area !== undefined) expect(measured.area).toBeCloseTo(expected.area, 9);
  });

  test("original cancellation and yield stop at every step without partial products", () => {
    const input = snapshot(fixtures.cases[0].data), full = project(input);
    for (let stop = 1; stop < full.work; stop++) {
      const cancelled = projectWith(input, DEFAULT_GEOMETRY_LIMITS, value => value.work < stop);
      expect(cancelled.status).toBe("cancelled"); expect(cancelled.meshes).toEqual([]);
      const limited = projectControlled(input, DEFAULT_GEOMETRY_LIMITS, value => value.work < stop ? "continue" : "limit");
      expect(limited.status).toBe("limited"); expect(limited.meshes).toEqual([]);
      expect(geometryPayloads(limited).meshesJson).toBe("[]");
    }
  });

  test("all caller work, vertex, triangle and depth budgets are enforced", () => {
    const input = snapshot(fixtures.cases[0].data);
    for (const limits of [{...DEFAULT_GEOMETRY_LIMITS, work: 2}, {...DEFAULT_GEOMETRY_LIMITS, vertices: 2}, {...DEFAULT_GEOMETRY_LIMITS, triangles: 2}]) {
      const scene = projectControlled(input, limits);
      expect(scene.status).toBe("limited"); expect(scene.meshes).toEqual([]); expect(scene.work).toBeLessThanOrEqual(limits.work);
    }
    for (const work of [Infinity, NaN, -1]) expect(projectControlled(input, {...DEFAULT_GEOMETRY_LIMITS, work}).status).toBe("limited");
    const scene = projectControlled(input, {...DEFAULT_GEOMETRY_LIMITS, depth: 1});
    expect(scene.meshes).toEqual([]); expect(scene.issues.length).toBeGreaterThan(0);
  });

  test("scene payloads retain real product identity and numeric topology", () => {
    const scene = project(snapshot(fixtures.cases[0].data)), payloads = geometryPayloads(scene);
    const meshes = JSON.parse(payloads.meshesJson), instances = JSON.parse(payloads.instancesJson);
    expect(meshes[0].id).toBe("ifc-16-14"); expect(meshes[0].data.positions.length).toBe(24); expect(meshes[0].data.indices.length).toBe(36);
    expect(instances[0].interactionId).toBe("ifc-16"); expect(instances[0].position).toEqual([0, 0, 0]);
    const empty = geometryPayloads(project(snapshot("")));
    expect(empty.meshesJson).toBe("[]"); expect(empty.instancesJson).toBe("[]");
  });

  test("product ids beyond binary64 exact integers retain their complete identity", () => {
    const large = 9007199254740993n, data = fixtures.cases[1].data.replace(/#5=/, "#" + large + "=");
    const scene = project(snapshot(data));
    expect(scene.meshes[0].product).toBe(large);
    expect(JSON.parse(geometryPayloads(scene).instancesJson)[0].interactionId).toBe("ifc-" + large);
  });

  test("shared vertices stay correct across repeated extrusions in one representation", () => {
    const data = fixtures.cases[0].data.replace("'SweptSolid',(#13)", "'SweptSolid',(#13,#13)");
    const scene = project(snapshot(data));
    expect(scene.issues).toEqual([]); expect(scene.meshes[0].vertices.length).toBe(8); expect(scene.meshes[0].triangles.length).toBe(24);
    expect(measures(scene).volume).toBeCloseTo(48, 9);
  });
});

test("broken product representation references are diagnosed while metadata stays quiet", () => {
  const contract = JSON.parse(readFileSync(new URL("../📜️references.json", import.meta.url), "utf8")) as {cases: {name: string; base?: string; type: string; target: number; expected?: string}[]};
  for (const row of contract.cases) {
    const input = snapshot(fixtures.cases[row.base === "placed" ? 0 : 1].data), id = row.base === "placed" ? 16n : 5n;
    const product = input.entities.find(entity => entity.id === id)!;
    product.name = row.type; product.args[6] = {kind: "reference", value: BigInt(row.target)};
    const scene = project(input);
    expect(scene.status).toBe("complete"); expect(scene.meshes).toEqual([]);
    expect(scene.issues).toEqual(row.expected ? [{entity: id, reason: row.expected}] : []);
  }
});

interface ProfileCase extends GeometryCase {oracle: {a: number; b: number; depth: number; origin: [number, number]; rotation: number}}
const profileFixtures = JSON.parse(readFileSync(new URL("../📜️profiles.json", import.meta.url), "utf8")) as {tessellation: {relativeChordTolerance: number; segments: number}; cases: ProfileCase[]; invalid: {name: string; definition: string; reason: string}[]};

test("parameterized curves follow neutral tolerance, placement and winding contracts", () => {
  for (const row of profileFixtures.cases) {
    const scene = project(snapshot(row.data)), mesh = scene.meshes[0];
    assert.equal(scene.status, "complete", row.name); assert.deepEqual(scene.issues, [], row.name);
    assert.ok(mesh, row.name); assert.equal(mesh.vertices.length, row.expected.vertices); assert.equal(mesh.triangles.length, row.expected.triangles);
    assert.deepEqual(scene.meshes.map(value => value.product), row.expected.products.map(BigInt));
    const bounds = geometryBounds(scene)!;
    for (let side = 0; side < 2; side++) for (let axis = 0; axis < 3; axis++) assert.ok(Math.abs(bounds[side][axis] - row.expected.bounds![side][axis]) < 1e-9, row.name);
    assert.ok(Math.abs(measures(scene).volume - row.expected.volume!) < 1e-9, row.name);
    const {a, b, origin, rotation} = row.oracle, n = profileFixtures.tessellation.segments;
    for (let i = 0; i < n; i++) {
      const theta = 2 * Math.PI * (i + 0.5) / n;
      const local = [a * Math.cos(theta), b * Math.sin(theta)], exact = [origin[0] + local[0] * Math.cos(rotation) - local[1] * Math.sin(rotation), origin[1] + local[0] * Math.sin(rotation) + local[1] * Math.cos(rotation)];
      const midpoint = [0, 1].map(axis => (mesh.vertices[i][axis] + mesh.vertices[(i + 1) % n][axis]) / 2);
      assert.ok(Math.hypot(exact[0] - midpoint[0], exact[1] - midpoint[1]) <= Math.max(a, b) * profileFixtures.tessellation.relativeChordTolerance * (1 + 1e-12), row.name);
    }
    let signedVolume = 0;
    for (const [ia, ib, ic] of mesh.triangles) signedVolume += dot(mesh.vertices[ia], cross(mesh.vertices[ib], mesh.vertices[ic])) / 6;
    assert.ok(signedVolume > 0, row.name);
  }
});

test("invalid parameterized profile dimensions and placements publish explicit issues", () => {
  for (const row of profileFixtures.invalid) {
    const source = profileFixtures.cases[0].data.replace("IFCCIRCLEPROFILEDEF(.AREA.,$,$,2.)", row.definition), scene = project(snapshot(source));
    assert.deepEqual(scene.meshes, [], row.name); assert.equal(scene.issues[0]?.reason, row.reason, row.name);
  }
});

test("parameterized curve sampling obeys caller work and cancellation budgets", () => {
  const input = snapshot(profileFixtures.cases[0].data), full = project(input);
  for (const stop of [50, 90, 130, Math.floor(full.work / 2), full.work - 1]) {
    const cancelled = projectControlled(input, DEFAULT_GEOMETRY_LIMITS, value => value.work < stop ? "continue" : "cancel");
    assert.equal(cancelled.status, "cancelled"); assert.deepEqual(cancelled.meshes, []);
    const limited = projectControlled(input, {...DEFAULT_GEOMETRY_LIMITS, work: stop});
    assert.equal(limited.status, "limited"); assert.deepEqual(limited.meshes, []); assert.ok(limited.work <= stop);
  }
});

function thirdPartyProfileOracle(row: ProfileCase): {points: Point[]; volume: number; area: number; triangles: number} {
  const {a, b, origin, rotation, depth} = row.oracle, curve = new EllipseCurve(origin[0], origin[1], a, b, 0, Math.PI * 2, false, rotation), points = curve.getPoints(profileFixtures.tessellation.segments).slice(0, -1);
  const geometry = new ExtrudeGeometry(new Shape(points), {depth, bevelEnabled: false, steps: 1}), positions = geometry.getAttribute("position");
  let volume = 0, area = 0;
  for (let i = 0; i < positions.count; i += 3) {
    const [a, b, c] = [i, i + 1, i + 2].map(index => [positions.getX(index), positions.getY(index), positions.getZ(index)] as Point), normal = cross(minus(b, a), minus(c, a));
    volume += dot(a, cross(b, c)) / 6; area += Math.sqrt(dot(normal, normal)) / 2;
  }
  const triangles = positions.count / 3;
  geometry.dispose();
  return {points: points.map(point => [point.x, point.y, 0]), volume: Math.abs(volume), area, triangles};
}

test("circle and ellipse projection matches independent Three.js curve and extrusion output", () => {
  for (const row of profileFixtures.cases) {
    const scene = project(snapshot(row.data)), oracle = thirdPartyProfileOracle(row), mesh = scene.meshes[0], measured = measures(scene);
    assert.equal(mesh.triangles.length, oracle.triangles, row.name);
    for (let i = 0; i < oracle.points.length; i++) for (let axis = 0; axis < 3; axis++) assert.ok(Math.abs(mesh.vertices[i][axis] - oracle.points[i][axis]) < 1e-12, row.name);
    assert.ok(Math.abs(measured.volume - oracle.volume) < Math.max(1, oracle.volume) * 2e-7, row.name);
    assert.ok(Math.abs(measured.area - oracle.area) < Math.max(1, oracle.area) * 2e-7, row.name);
  }
});

test("voided extrusions retain manifold boundary segmentation and signed volume", () => {
  const row = fixtures.cases.find(row => row.name === "arbitrary-extrusion-with-void")!;
  const data = row.data.replace("#2=IFCCARTESIANPOINT((10.,0.,0.));", "#2=IFCCARTESIANPOINT((10.,0.,0.));#20=IFCCARTESIANPOINT((5.,0.,0.));").replace("IFCPOLYLINE((#1,#2,#3,#4,#1))", "IFCPOLYLINE((#1,#20,#2,#3,#4,#1))");
  const scene = project(snapshot(data)); assert.deepEqual(scene.issues, []);
  for (const mesh of scene.meshes) {
    const edges = new Map<string, number>(); let volume = 0;
    for (const triangle of mesh.triangles) {
      const [a,b,c] = triangle.map(id => mesh.vertices[id]); volume += dot(a,cross(b,c))/6;
      for (let i=0;i<3;i++) {const a=triangle[i],b=triangle[(i+1)%3],key=a<b?a+":"+b:b+":"+a;edges.set(key,(edges.get(key)??0)+1);}
    }
    assert.ok(Math.abs(volume - 168) < 1e-9); assert.ok([...edges.values()].every(count => count === 2));
  }
});
test("hollow circles and circular profile voids preserve holes within finite work", () => {
  const source = "#1=IFCCARTESIANPOINT((0.,0.));#2=IFCAXIS2PLACEMENT2D(#1,$);#3=IFCCIRCLEHOLLOWPROFILEDEF(.AREA.,$,#2,2.,0.5);#4=IFCDIRECTION((0.,0.,1.));#5=IFCEXTRUDEDAREASOLID(#3,$,#4,2.);#6=IFCSHAPEREPRESENTATION($,'Body','SweptSolid',(#5));#7=IFCPRODUCTDEFINITIONSHAPE($,$,(#6));#8=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Pipe',$,$,$,#7,$,$);";
  for (const data of [source, source.replace("#3=IFCCIRCLEHOLLOWPROFILEDEF(.AREA.,$,#2,2.,0.5);", "#30=IFCCIRCLE(#2,2.);#31=IFCCIRCLE(#2,1.5);#3=IFCARBITRARYPROFILEDEFWITHVOIDS(.AREA.,$,#30,(#31));")]) {
    const input=snapshot(data),scene=project(input); assert.deepEqual(scene.issues,[]); assert.equal(scene.meshes.length,1);
    const expected=72/2*Math.sin(2*Math.PI/72)*(4-2.25)*2;assert.ok(Math.abs(measures(scene).volume-expected)<1e-9);
    assert.ok(scene.work<DEFAULT_GEOMETRY_LIMITS.work);
    for(const stop of [1, Math.floor(scene.work/3),scene.work-1]) {const stopped=projectControlled(input,DEFAULT_GEOMETRY_LIMITS,value=>value.work<stop?"continue":"cancel");assert.equal(stopped.status,"cancelled");assert.deepEqual(stopped.meshes,[]);}
  }
});
test("profile positions refuse present scalar values and 3D zero-z coordinates", () => {
  const data="#1=IFCCARTESIANPOINT((0.,0.));#2=IFCAXIS2PLACEMENT2D(#1,$);#3=IFCCIRCLEPROFILEDEF(.AREA.,$,#2,2.);#4=IFCDIRECTION((0.,0.,1.));#5=IFCEXTRUDEDAREASOLID(#3,$,#4,2.);#6=IFCSHAPEREPRESENTATION($,'Body','SweptSolid',(#5));#7=IFCPRODUCTDEFINITIONSHAPE($,$,(#6));#8=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Circle',$,$,$,#7,$,$);";
  for(const invalid of [data.replace("IFCCIRCLEPROFILEDEF(.AREA.,$,#2,", "IFCCIRCLEPROFILEDEF(.AREA.,$,4,"),data.replace("IFCCARTESIANPOINT((0.,0.))", "IFCCARTESIANPOINT((0.,0.,0.))")]) {
    const scene=project(snapshot(invalid));assert.equal(scene.meshes.length,0);assert.ok(scene.issues.length>0);
  }
});

test("unchanged official IFC4 polygonal hole sample preserves the closed solid", () => {
  const text=readFileSync(new URL("../../../../🧪️tests/🧫️fixtures/🏛️official/polygonal-face-tessellation.ifc",import.meta.url),"utf8");
  const scene=project(decodeIfc4(text));assert.deepEqual(scene.issues,[]);assert.equal(scene.meshes.length,1);
  const mesh=scene.meshes[0];assert.equal(mesh.product,30n);assert.equal(mesh.vertices.length,16);assert.equal(mesh.triangles.length,28);
  assert.ok(Math.abs(measures(scene).volume-6.5e12)<1);assert.ok(Math.abs(measures(scene).area-3e9)<0.001);
  const edges=new Map<string,number>();for(const t of mesh.triangles)for(let i=0;i<3;i++){const a=t[i],b=t[(i+1)%3],key=a<b?a+":"+b:b+":"+a;edges.set(key,(edges.get(key)??0)+1);}assert.ok([...edges.values()].every(count=>count===2));
});

test("official holed solid matches independent Three triangulation measures", () => {
  const text=readFileSync(new URL("../../../../🧪️tests/🧫️fixtures/🏛️official/polygonal-face-tessellation.ifc",import.meta.url),"utf8"),document=decodeIfc4(text);
  const entities=new Map(document.entities.map(entity=>[entity.id,entity]));
  const points=entities.get(15n)!.args[0];assert.equal(points.kind,"aggregate");if(points.kind!=="aggregate")return;
  const vertices=points.value.map(row=>{assert.equal(row.kind,"aggregate");if(row.kind!=="aggregate")throw Error("points");return row.value.map(value=>value.kind==="real"?binary64Value(value.value):value.kind==="integer"?Number(value.value):NaN) as Point;});
  const triangles:number[][]=[];
  for(const entity of document.entities.filter(entity=>["IFCINDEXEDPOLYGONALFACE","IFCINDEXEDPOLYGONALFACEWITHVOIDS"].includes(entity.name))){
    const values=[entity.args[0],...(entity.name.endsWith("WITHVOIDS")&&entity.args[1].kind==="aggregate"?entity.args[1].value:[])];
    const rings=values.map(value=>{if(value.kind!=="aggregate")throw Error("indices");return value.value.map(v=>{if(v.kind!=="integer")throw Error("index");return Number(v.value)-1;});});
    const [a,b,c]=rings[0].slice(0,3).map(id=>vertices[id]),normal=cross(minus(b,a),minus(c,a));let axis=0;for(let i=1;i<3;i++)if(Math.abs(normal[i])>Math.abs(normal[axis]))axis=i;
    const projected=rings.map(r=>r.map(id=>new Vector2(vertices[id][(axis+1)%3],vertices[id][(axis+2)%3]))),flat=rings.flat();
    for(const triangle of ShapeUtils.triangulateShape(projected[0],projected.slice(1))){const ids=triangle.map(i=>flat[i]);if(normal[axis]<0)ids.reverse();triangles.push(ids);}
  }
  const reference:GeometryScene={meshes:[{product:30n,representation:28n,label:"Reference",vertices,triangles:triangles as [number,number,number][]}],issues:[],status:"complete",work:0};
  assert.equal(triangles.length,28);const actual=measures(project(document)),expected=measures(reference);assert.ok(Math.abs(actual.volume-expected.volume)<1);assert.ok(Math.abs(actual.area-expected.area)<0.001);
});

test("finite tiny and huge geometric directions and normals stay scale invariant", () => {
  for(const scale of [1e-120,1e120]){
    const number=scale.toExponential(1).replace("e","E");
    const data="#1=IFCCARTESIANPOINTLIST3D(((0.,0.,0.),(0.,"+number+",0.),(0.,0.,"+number+")));#2=IFCTRIANGULATEDFACESET(#1,$,.F.,((1,2,3)),$);#3=IFCSHAPEREPRESENTATION($,'Body','Tessellation',(#2));#4=IFCPRODUCTDEFINITIONSHAPE($,$,(#3));#5=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Scale',$,$,$,#4,$,$);";
    const scene=project(snapshot(data));assert.equal(scene.meshes.length,1);const payload=JSON.parse(geometryPayloads(scene).meshesJson);
    for(let i=0;i<payload[0].data.normals.length;i+=3){assert.ok(Math.abs(payload[0].data.normals[i]-1)<1e-9);assert.equal(payload[0].data.normals[i+1],0);assert.equal(payload[0].data.normals[i+2],0);}
    const directed=fixtures.cases[0].data.replace("IFCDIRECTION((0.,0.,1.))", "IFCDIRECTION((0.,0.,"+number+"))");assert.deepEqual(project(snapshot(directed)).issues,[]);
  }
});

test("indexed capsule arcs match independent Three extrusion output",()=>{
  const fixture=fixtures.cases.find(row=>row.name==="indexed-capsule-arcs")!,scene=project(snapshot(fixture.data));
  const right=new EllipseCurve(2,1,1,1,-Math.PI/2,Math.PI/2,false,0).getPoints(36),left=new EllipseCurve(0,1,1,1,Math.PI/2,3*Math.PI/2,false,0).getPoints(36);
  const points=[new Vector2(0,0),new Vector2(2,0),...right.slice(1),new Vector2(0,2),...left.slice(1,-1)],geometry=new ExtrudeGeometry(new Shape(points),{depth:2,steps:1,bevelEnabled:false}),positions=geometry.getAttribute("position");let volume=0;
  for(let i=0;i<positions.count;i+=3){const p=[0,1,2].map(j=>[positions.getX(i+j),positions.getY(i+j),positions.getZ(i+j)] as Point);volume+=dot(p[0],cross(p[1],p[2]))/6;}
  assert.ok(Math.abs(measures(scene).volume-Math.abs(volume))<1e-6);geometry.dispose();
});
test("indexed profile connectivity closure and source bounds refuse malformed curves",()=>{
  const fixture=fixtures.cases.find(row=>row.name==="indexed-capsule-arcs")!;
  for(const data of [fixture.data.replace("IFCARCINDEX((2,3,4))","IFCARCINDEX((3,2,4))"),fixture.data.replace("IFCARCINDEX((5,6,1))","IFCARCINDEX((5,6,2))"),fixture.data.replace("IFCARCINDEX((2,3,4))","IFCARCINDEX((2,3,99))"),fixture.data.replace("IFCARCINDEX((2,3,4))","IFCARCINDEX((2,3))")]){const scene=project(snapshot(data));assert.equal(scene.meshes.length,0);assert.ok(scene.issues.length>0);}
  const input=snapshot(fixture.data),scene=project(input);for(const stop of [10,50,100,200,Math.floor(scene.work/2)]){const cancelled=projectControlled(input,DEFAULT_GEOMETRY_LIMITS,value=>value.work<stop?"continue":"cancel");assert.equal(cancelled.status,"cancelled");assert.equal(cancelled.meshes.length,0);}
});

test("large implicit indexed point lists stop by caller budget without spread overflow",()=>{
  const row=fixtures.cases.find(row=>row.name==="indexed-implicit-polyline")!,input=snapshot(row.data),points=input.entities.find(entity=>entity.id===1n)!;
  const size=140000,coordinates=Array.from({length:size+1},(_,i)=>{const n=i===size?0:i,edge=Math.floor(n/(size/4)),offset=(n%(size/4))/(size/4),xy=edge===0?[offset,0]:edge===1?[1,offset]:edge===2?[1-offset,1]:[0,1-offset];return {kind:"aggregate" as const,value:xy.map(value=>({kind:"real" as const,value:binary64(value)}))};});
  points.args[0]={kind:"aggregate",value:coordinates};
  const scene=projectControlled(input,{...DEFAULT_GEOMETRY_LIMITS,work:200000});assert.equal(scene.status,"limited");assert.equal(scene.meshes.length,0);assert.ok(scene.work<=200000);assert.ok(scene.issues.every(issue=>!issue.reason.includes("RangeError")));
});

const cameraContract = JSON.parse(readFileSync(new URL("../📷️camera.json", import.meta.url), "utf8")) as {cases: {name: string; min: Point; max: Point; status: string; reason?: string}[]; invalid: {name: string; triangle: [number, number, number]; reason: string}[]};
function cameraScene(min: Point, max: Point): GeometryScene {
  const vertices: Point[] = [[min[0],min[1],min[2]],[max[0],min[1],min[2]],[min[0],max[1],min[2]],[min[0],min[1],max[2]],[max[0],max[1],max[2]]];
  return {status:"complete",work:0,issues:[],meshes:[{product:1n,representation:2n,label:"Camera",vertices,triangles:[[0,1,2],[0,3,1]]}]};
}
for (const fixture of cameraContract.cases) test("finite camera contract: " + fixture.name, () => {
  const payload = geometryPayloads(cameraScene(fixture.min,fixture.max)), status = JSON.parse(payload.statusJson);
  assert.equal(status.status,fixture.status);
  assert.ok([...payload.cameraPosition,...payload.cameraTarget].every(Number.isFinite));
  if (fixture.reason) {assert.equal(payload.meshesJson,"[]");assert.equal(payload.instancesJson,"[]");assert.ok(status.issues.some((issue: {reason: string})=>issue.reason===fixture.reason));return;}
  const center = fixture.min.map((lo,axis)=>new Decimal(lo).plus(fixture.max[axis]).div(2));
  const extent = Decimal.max(1,...fixture.min.map((lo,axis)=>new Decimal(fixture.max[axis]).minus(lo)));
  const position = center.map((target,axis)=>{const offset=extent.times([1.5,-1.5,1.2][axis]),preferred=target.plus(offset);return Number.isFinite(preferred.toNumber())?preferred:target.minus(offset);});
  for(let axis=0;axis<3;axis++){assert.ok(Math.abs(payload.cameraTarget[axis]-center[axis].toNumber())<=Math.max(Number.MIN_VALUE,Math.abs(center[axis].toNumber())*2e-15));assert.ok(Math.abs(payload.cameraPosition[axis]-position[axis].toNumber())<=Math.max(Number.MIN_VALUE,Math.abs(position[axis].toNumber())*2e-15));}
});
for (const fixture of cameraContract.invalid) test("malformed payload refuses atomically: "+fixture.name,()=>{
  const scene=cameraScene([0,0,0],[1,1,1]);scene.meshes[0].triangles=[fixture.triangle];const payload=geometryPayloads(scene),status=JSON.parse(payload.statusJson);assert.equal(payload.meshesJson,"[]");assert.equal(payload.instancesJson,"[]");assert.equal(status.status,"limited");assert.ok(status.issues.some((issue:{reason:string})=>issue.reason===fixture.reason));
});
test("nonfinite public mesh coordinates are never encoded as JSON null",()=>{for(const number of [NaN,Infinity,-Infinity]){const scene=cameraScene([0,0,0],[1,1,1]);scene.meshes[0].vertices[0][0]=number;const payload=geometryPayloads(scene),status=JSON.parse(payload.statusJson);assert.equal(payload.meshesJson,"[]");assert.equal(payload.instancesJson,"[]");assert.equal(status.status,"limited");assert.ok(status.issues.some((issue:{reason:string})=>issue.reason==="nonfinite-mesh-coordinate"));}});

test("public payload normals match independent Three including unused zero normals",()=>{const scene=cameraScene([0,0,0],[2,4,6]),mesh=scene.meshes[0],reference=new BufferGeometry();reference.setAttribute("position",new BufferAttribute(new Float64Array(mesh.vertices.flat()),3));reference.setIndex(mesh.triangles.flat());reference.computeVertexNormals();const expected=reference.getAttribute("normal"),actual=JSON.parse(geometryPayloads(scene).meshesJson)[0].data.normals;for(let i=0;i<actual.length;i++)assert.ok(Math.abs(actual[i]-expected.array[i])<1e-6);reference.dispose();assert.deepEqual(actual.slice(-3),[0,0,0]);});
test("zero-area public triangles are refused without guessed normals",()=>{const scene=cameraScene([0,0,0],[1,1,1]);scene.meshes[0].vertices[2]=[0.5,0,0];const payload=geometryPayloads(scene),status=JSON.parse(payload.statusJson);assert.equal(payload.meshesJson,"[]");assert.equal(payload.instancesJson,"[]");assert.ok(status.issues.some((issue:{reason:string})=>issue.reason==="unrepresentable-mesh-normal"));});

test("sparse public coordinate and index tuples are refused before access",()=>{for(const field of ["vertices","triangles"]as const){const scene=cameraScene([0,0,0],[1,1,1]);if(field==="triangles"){const tuple=[0,1,2]as [number,number,number];delete (tuple as number[])[1];scene.meshes[0].triangles=[tuple];}else{const tuple=[0,0,0]as Point;delete (tuple as number[])[1];scene.meshes[0].vertices[0]=tuple;}const payload=geometryPayloads(scene),status=JSON.parse(payload.statusJson);assert.equal(payload.meshesJson,"[]");assert.equal(payload.instancesJson,"[]");assert.equal(status.status,"limited");assert.ok(status.issues.some((issue:{reason:string})=>issue.reason===(field==="vertices"?"nonfinite-mesh-coordinate":"invalid-mesh-index")));}});
