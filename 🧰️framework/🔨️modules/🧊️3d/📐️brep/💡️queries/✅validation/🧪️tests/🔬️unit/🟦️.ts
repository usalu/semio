import { expect, test } from "bun:test";
import Ajv2020 from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BufferGeometry, Float32BufferAttribute, Line3, Plane, ShapeUtils, Triangle, Vector2, Vector3 } from "three";

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
 expect(samples.length).toBeLessThanOrEqual(law.maximumSamples);expect(law.splineMaximumDegree).toBe(32);
 const source=readFileSync(join(import.meta.dir,"../../🎟️loop/🦀️.rs"),"utf8");expect(source.includes("self.same_parameter_demands(body)")).toBe(true);
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
  geometry.dispose();
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
});

test("closed, open and reversed tetrahedral shells report their edge incidence", () => {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧊️cold-planning/🔣️.json"), "utf8"));
  const faces: number[][] = fixture.faces;
  for (const [name, triangles] of Object.entries({
    closedBoundaryEdges: faces,
    missingFaceBoundaryEdges: faces.slice(1),
    reversedFaceOrientationEdges: [[...faces[0]].reverse(), ...faces.slice(1)],
  })) {
    const geometry = new BufferGeometry().setIndex(triangles.flat());
    const index = geometry.getIndex()!;
    const counts = new Map<string, number>();
    const incidence = new Map<string, number>();
    for (let face = 0; face < index.count; face += 3) {
      for (let corner = 0; corner < 3; corner++) {
        const a = index.getX(face + corner), b = index.getX(face + (corner + 1) % 3);
        const key = [Math.min(a, b), Math.max(a, b)].join(",");
        counts.set(key, (counts.get(key) ?? 0) + Math.sign(b - a));
        incidence.set(key,(incidence.get(key)??0)+1);
      }
    }
    expect([...counts.values()].filter(n => n !== 0).length).toEqual(fixture.shellClosure[name]);
    if(name==="missingFaceBoundaryEdges"){
      expect([...incidence].every(([key,n])=>n===1||n===2&&counts.get(key)===0)).toBe(fixture.shellClosure.openShellPasses);
      expect([...incidence.values()].every(n=>n===2)).toBe(fixture.shellClosure.openSolidPasses);
    }
    geometry.dispose();
  }
});

test("Three.js distinguishes the selected closed solid from unrelated reversed topology", () => {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🧊️cold-planning/🔣️.json"),"utf8"));
  const row = fixture.scopeSelection;
  const points = fixture.vertices.map(([x,y,z]: [number,number,number]) => new Vector3(x,y,z));
  const signed = (faces: Array<[number,number,number]>) => faces.reduce((sum,[i,j,k]) =>
    sum + points[i].dot(points[j].clone().cross(points[k])) / 6,0);
  const selected = signed(fixture.faces);
  const unrelated = signed(fixture.faces.map(([a,b,c]: [number,number,number]) => [a,c,b]));
  expect(selected).toBeCloseTo(row.selectedSignedVolume, 12);
  expect(selected > 0).toBe(row.selectedPasses);
  expect(unrelated > 0).toBe(row.unrelatedPasses);
});

test("original triangular face crossings and coplanar area agree with Three",()=>{
  type Coordinates=[number,number,number][];
  const f=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/triangular-intersection.json"),"utf8"));
  const points=(rows:Coordinates)=>rows.map(row=>new Vector3(...row));
  const a=points(f.triangle);
  const plane=new Triangle(a[0],a[1],a[2]).getPlane(new Plane());
  const crosses=(a:Vector3[],b:Vector3[])=>{
    const planeA=new Triangle(a[0],a[1],a[2]).getPlane(new Plane());
    const planeB=new Triangle(b[0],b[1],b[2]).getPlane(new Plane());
    const axis=planeA.normal.clone().cross(planeB.normal).normalize();
    const interval=(points:Vector3[],plane:Plane)=>{
      const offsets=points.map(p=>plane.distanceToPoint(p));
      if(Math.min(...offsets)>=0||Math.max(...offsets)<=0)return null;
      const endpoints=points.flatMap((p,i)=>{
        if(offsets[i]===0)return [p.dot(axis)];
        const next=(i+1)%3;
        if(offsets[i]*offsets[next]>=0)return [];
        const crossing=plane.intersectLine(new Line3(p,points[next]),new Vector3())!;
        return [crossing.dot(axis)];
      });
      return [Math.min(...endpoints),Math.max(...endpoints)];
    };
    const left=interval(a,planeB),right=interval(b,planeA);
    return left!==null&&right!==null&&Math.min(left[1],right[1])-Math.max(left[0],right[0])>1e-12;
  };
  const overlap=(a:Vector3[],b:Vector3[])=>{
    let polygon=b.map(p=>new Vector2(p.x,p.y));
    for(let i=0;i<3;i++){
      const start=new Vector2(a[i].x,a[i].y),end=new Vector2(a[(i+1)%3].x,a[(i+1)%3].y);
      const edge=end.sub(start);
      const side=(p:Vector2)=>edge.cross(p.clone().sub(start));
      polygon=polygon.flatMap((p,j)=>{
        const q=polygon[(j+1)%polygon.length],u=side(p),v=side(q);
        if((u>=0)===(v>=0))return u>=0?[p]:[];
        const hit=p.clone().lerp(q,u/(u-v));
        return u>=0?[p,hit]:[hit];
      });
    }
    return Math.abs(ShapeUtils.area(polygon))>1e-12;
  };
  for(const row of f.cases){
    const b=points(row.triangle);
    const actual=b.every(p=>plane.distanceToPoint(p)===0)?overlap(a,b):crosses(a,b);
    expect(actual).toBe(row.intersects);
  }
});
