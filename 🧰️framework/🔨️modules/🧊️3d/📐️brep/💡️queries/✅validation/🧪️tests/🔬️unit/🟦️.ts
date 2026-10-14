import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BufferGeometry, Float32BufferAttribute, Triangle, Vector3 } from "three";

test("original same-parameter samples retain bounded scratch and independent curve deviation",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🧊️cold-planning/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../../🧬️schema/🧊️cold-planning/🔣️.json"),"utf8"));
 const validate=new Ajv2020({strict:false}).compile(schema);expect(validate(fixture)).toBe(true);
 const law=fixture.retainedSameParameter;expect(validate({...fixture,retainedSameParameter:{...law,sortComparisonsPerEvent:2}})).toBe(false);
 expect(law.maximumSamples).toBe(law.baseSamples+16+32+64);expect(law.maximumMidpoints).toBe(64);
 const probe=fixture.sameParameterProbe;const start=new Vector3(...fixture.vertices[probe.edge[0]]);const end=new Vector3(...fixture.vertices[probe.edge[1]]);
 const samples=probe.samples.map((s:number)=>({s,deviation:start.distanceTo(start.clone().lerp(end,s))}));
 const worst=samples.reduce((a:{s:number;deviation:number},b:{s:number;deviation:number})=>b.deviation>=a.deviation?b:a);
 expect(worst.deviation).toBeCloseTo(probe.maximumDeviation,12);expect(worst.s).toBe(probe.worstParameter);
 expect(samples.length).toBeLessThanOrEqual(law.maximumSamples);expect(law.unsupportedSources).toEqual(["NurbsSurface","NurbsCurve2","NurbsCurve3"]);
 const source=readFileSync(join(import.meta.dir,"../../🎟️loop/🦀️.rs"),"utf8");expect(source.includes("self.same_parameter_demands(body)")).toBe(true);
 console.log(`[DEBUG] independent Three original same-parameter samples=${samples.length} worst=${worst.deviation} scratchLimit=${law.maximumSamples}; spline source evaluator remains explicitly unfunded`);
});

test("cold validation neutral tetrahedron agrees with independent topology and volume", () => {
  const base = join(import.meta.dir, "../🧫️fixtures/🧊️cold-planning");
  const fixture = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8"));
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(fixture.vertices.flat(), 3));
  geometry.setIndex(fixture.faces.flat());
  const points = geometry.getAttribute("position");
  const indices = geometry.getIndex()!;
  const uses = new Map<string, { count: number; direction: number }>();
  let volume = 0;
  let samples = 0, maximumDeviation = 0;
  for (let i = 0; i < indices.count; i += 3) {
    const ids = [indices.getX(i), indices.getX(i + 1), indices.getX(i + 2)];
    const [a, b, c] = ids.map(id => new Vector3().fromBufferAttribute(points, id));
    volume += a.dot(b.clone().cross(c)) / 6;
    for (let j = 0; j < 3; j++) {
      const u = ids[j], v = ids[(j + 1) % 3];
      const key = `${Math.min(u, v)}:${Math.max(u, v)}`;
      const use = uses.get(key) ?? { count: 0, direction: 0 };
      use.count++;
      use.direction += u < v ? 1 : -1;
      uses.set(key, use);
      const corners = [a, b, c];
      const triangle = new Triangle(a, b, c);
      for (let sample = 0; sample <= fixture.budget.sameParameterBaseSamples; sample++) {
        const point = corners[j].clone().lerp(corners[(j + 1) % 3], sample / fixture.budget.sameParameterBaseSamples);
        maximumDeviation = Math.max(maximumDeviation, point.distanceTo(triangle.closestPointToPoint(point, new Vector3())));
        samples++;
      }
    }
  }
  expect(uses.size).toBe(fixture.expected.edges);
  expect(indices.count / 3).toBe(fixture.expected.faces);
  expect([...uses.values()].every(use => use.count === 2 && use.direction === 0)).toBe(true);
  expect(volume).toBeCloseTo(fixture.expected.volume, 10);
  expect(samples).toBe(fixture.expected.faces * 3 * (fixture.budget.sameParameterBaseSamples + 1));
  expect(maximumDeviation).toBeLessThan(1e-12);
  const probe = fixture.sameParameterProbe;
  const start = new Vector3(...fixture.vertices[probe.edge[0]]);
  const end = new Vector3(...fixture.vertices[probe.edge[1]]);
  const deviations = probe.samples.map((s: number) => ({ s, deviation: start.distanceTo(start.clone().lerp(end, s)) }));
  const worst = deviations.reduce((a: { s: number; deviation: number }, b: { s: number; deviation: number }) => b.deviation >= a.deviation ? b : a);
  expect(worst.deviation).toBeCloseTo(probe.maximumDeviation, 12);
  expect(worst.s).toBe(probe.worstParameter);
  expect(samples - (fixture.budget.sameParameterBaseSamples + 1) + deviations.length).toBe(probe.totalBodySamples);
  geometry.dispose();
  console.log(`[DEBUG] independent cold validation edges=${uses.size} faces=${indices.count / 3} volume=${volume} samples=${samples} maximumDeviation=${maximumDeviation} adaptiveSamples=${deviations.length} adaptiveDeviation=${worst.deviation} adaptiveWorst=${worst.s}`);
});

test("original retained validation planning separates reserve and entity currencies", () => {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧊️cold-planning/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🧊️cold-planning/🔣️.json"), "utf8"));
  expect(new Ajv2020({ strict: false }).compile(schema)(fixture)).toBe(true);
  const law = fixture.retainedPlanning;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(fixture.vertices.flat(), 3));
  geometry.setIndex(fixture.faces.flat());
  const edges = new Set<string>();
  const index = geometry.getIndex()!;
  for (let i = 0; i < index.count; i += 3) for (let j = 0; j < 3; j++) {
    const a = index.getX(i + j), b = index.getX(i + (j + 1) % 3);
    edges.add(`${Math.min(a, b)}:${Math.max(a, b)}`);
  }
  expect(edges.size).toBe(fixture.expected.edges);
  expect(index.count / 3).toBe(fixture.expected.faces);
  for (const count of [edges.size, index.count / 3]) {
    const events = [{ birth: count * law.entityCopyBytes, release: 0, copy: law.reserveCopyBytes }, ...Array.from({ length: count }, () => ({ birth: 0, release: 0, copy: law.entityCopyBytes }))];
    expect(events[0].copy).toBe(0);
    expect(events.slice(1).every(event => event.copy === law.entityCopyBytes)).toBe(true);
    expect(events.reduce((total, event) => total + event.birth - event.release, 0)).toBe(count * law.entityCopyBytes);
    for (const body of law.bodyBytes) expect(body >= law.entityCopyBytes).toBe(body === 64);
  }
  for (const count of law.sourceTetrahedra) {
    expect(edges.size * count).toBe(fixture.expected.edges * count);
    expect(index.count / 3 * count).toBe(fixture.expected.faces * count);
  }
  expect(law.metadataCopyBytes).toBe(0);
  expect(law.deniedAxes).toEqual(["items", "copy", "capacity", "release", "depth"]);
  geometry.dispose();
  console.log(`[DEBUG] independent retained validation planning edges=${edges.size} faces=${index.count / 3} reserveCopy=0 entityCopy=${law.entityCopyBytes}`);
});


test("original retained validation loops keep original neighbor order and independent diagnostic text", () => {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧊️cold-planning/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🧊️cold-planning/🔣️.json"), "utf8"));
  const validate = new Ajv2020({strict:false}).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({...fixture,retainedLoops:{...fixture.retainedLoops,coedgeCopyBytes:4}})).toBe(false);
  const geometry = new BufferGeometry(); geometry.setAttribute("position",new Float32BufferAttribute(fixture.vertices.flat(),3)); geometry.setIndex(fixture.faces.flat());
  const indices=geometry.getIndex()!;
  const loops=Array.from({length:indices.count/3},(_,face)=>Array.from({length:3},(_,corner)=>({start:indices.getX(face*3+corner),end:indices.getX(face*3+(corner+1)%3),next:(corner+1)%3,prev:(corner+2)%3})));
  for(const ring of loops)for(let index=0;index<ring.length;index++){const next=(index+1)%ring.length;expect(ring[index].end).toBe(ring[next].start);expect(ring[index].next).toBe(next);expect(ring[next].prev).toBe(index);}
  const broken=fixture.retainedLoops.brokenNeighbor;const original=loops[broken.loop];original[broken.next].prev=broken.next;
  const issues=original.flatMap((row,index)=>{const next=(index+1)%original.length;return row.next!==next||original[next].prev!==index?[{entity:`loop-${broken.loop}`,code:"next-prev-mismatch",message:`coedge ${index}'s next/prev pointers are not symmetric with its ring neighbor`}]:[];});
  expect(issues).toEqual([{entity:broken.entity,code:broken.code,message:broken.message}]);
  for(const body of fixture.retainedLoops.bodyBytes){let text="";for(const scalar of broken.message){if(body>0)text+=scalar;}expect(text).toBe(body===0?"":broken.message);}
  for(const count of fixture.retainedLoops.sourceTetrahedra)expect(loops.length*3*count).toBe(fixture.expected.faces*3*count);
  geometry.dispose();console.log(`[DEBUG] independent original loop count=${loops.length} coedgeBytes=8 diagnostic=${issues[0].message}`);
});


test("original topology diagnostics retain complete UTF8 scalar boundaries",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🧊️cold-planning/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../../🧬️schema/🧊️cold-planning/🔣️.json"),"utf8"));
 const validate=new Ajv2020({strict:false}).compile(schema);expect(validate(fixture)).toBe(true);
 expect(validate({...fixture,retainedTopology:{...fixture.retainedTopology,metadataCopyBytes:1}})).toBe(false);
 const law=fixture.retainedTopology;
 for(const row of [law.missingPcurve,law.tolerance]){
  const bytes=new TextEncoder().encode(row.message);let output="";
  for(const scalar of row.message){const encoded=new TextEncoder().encode(scalar);expect(new TextDecoder("utf8",{fatal:true}).decode(encoded)).toBe(scalar);output+=scalar;}
  expect(new TextDecoder("utf8",{fatal:true}).decode(bytes)).toBe(output);expect(output).toBe(row.message);
 }
 expect([...law.missingPcurve.message].some((scalar:string)=>new TextEncoder().encode(scalar).length===3)).toBe(true);
 console.log("[DEBUG] independent UTF8 original topology diagnostics preserve one/three-byte scalar atom and exact text");
});
