import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { createSurfaceSchemaOracleV1 } from "../🔮️schema/🟦️.ts";
import { Earcut } from "three/src/extras/Earcut.js";
import { validateJsonSchemaSubset } from "../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🔣️.json";
type Point=readonly[number,number,number];
interface Surface {regionId:string;points:Point[];tetrahedra:[number,number,number,number][];triangles:[number,number,number][]}
/** 🔮️ Independent Earcut output crosses a first-party numeric-only oracle interface. */
function independentArea(outer:readonly(readonly number[])[],holes:readonly(readonly(readonly number[])[])[]):number {
 const points=[...outer,...holes.flat()],offsets:number[]=[];let offset=outer.length;for(const hole of holes){offsets.push(offset);offset+=hole.length;}const indices=Earcut.triangulate(points.flat(),offsets,2);let area=0;for(let index=0;index<indices.length;index+=3){const [a,b,c]=[0,1,2].map(corner=>points[indices[index+corner]!]!);area+=Math.abs((b![0]!-a![0]!)*(c![1]!-a![1]!)-(b![1]!-a![1]!)*(c![0]!-a![0]!))/2;}return area;
}
const binary64Bits=(value:number)=>{const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,value,false);return bytes.getBigUint64(0,false).toString(16).padStart(16,"0");};
const determinant=(a:Point,b:Point,c:Point)=>a[0]*(b[1]*c[2]-b[2]*c[1])-a[1]*(b[0]*c[2]-b[2]*c[0])+a[2]*(b[0]*c[1]-b[1]*c[0]);
test("all nine real neutral region surfaces retain binary64 topology volume and independent Earcut footprint",()=>{
 const path=process.env.SEMIO_FEM_NEUTRAL_RESULTS;expect(typeof path).toBe("string");const rows=JSON.parse(readFileSync(path!,"utf8")) as {id:string;neutral:{regions:Surface[]}}[];expect(rows.map(row=>row.id)).toEqual(fixture.vectors.map(vector=>vector.id));
 const {batch:validate}=createSurfaceSchemaOracleV1(schema);
 for(const [index,vector]of fixture.vectors.entries()){
  const row=rows[index]!;expect(validate(row.neutral)).toBe(true);expect(validateJsonSchemaSubset(schema,row.neutral)).toEqual([]);expect(row.neutral.regions.map(region=>region.regionId)).toEqual(vector.expected.map(expected=>expected.regionId));
  for(const expected of vector.expected){const region=row.neutral.regions.find(region=>region.regionId===expected.regionId)!,input=vector.regions.find(region=>region.regionId===expected.regionId)!;expect(region.points.every(point=>point.every(Number.isFinite))).toBe(true);expect([0,1].map(side=>[0,1,2].map(axis=>(side===0?Math.min:Math.max)(...region.points.map(point=>point[axis]!))))).toEqual(expected.bounds);expect([0,1].map(side=>[0,1,2].map(axis=>binary64Bits((side===0?Math.min:Math.max)(...region.points.map(point=>point[axis]!)))))).toEqual(expected.boundsBits);for(const point of input.domain.outer)expect(region.points.some(position=>position[0]===point[0]&&position[1]===point[1])).toBe(true);
   const edges=new Map<string,number>(),vertices=new Set<number>();let signedVolume=0;
   for(const triangle of region.triangles){const [a,b,c]=triangle.map(index=>{expect(index).toBeLessThan(region.points.length);vertices.add(index);return region.points[index]!});signedVolume+=determinant(a!,b!,c!)/6;for(const [a,b]of[[triangle[0],triangle[1]],[triangle[1],triangle[2]],[triangle[2],triangle[0]]]){const key=[a,b].sort((a,b)=>a!-b!).join(",");edges.set(key,(edges.get(key)??0)+1);}}
   const tetVolume=region.tetrahedra.reduce((volume,tet)=>{const [a,b,c,d]=tet.map(index=>region.points[index]!);return volume+Math.abs(determinant([b![0]-a![0],b![1]-a![1],b![2]-a![2]],[c![0]-a![0],c![1]-a![1],c![2]-a![2]],[d![0]-a![0],d![1]-a![1],d![2]-a![2]]))/6},0);
   expect([...edges.values()].every(count=>count===2)).toBe(true);expect(vertices.size-edges.size+region.triangles.length).toBe(2-2*input.domain.holes.length);expect(region.triangles.length).toBeGreaterThanOrEqual(expected.minimumTriangles);expect(Math.abs(signedVolume-expected.volume)).toBeLessThan(1e-10);expect(Math.abs(tetVolume-expected.volume)).toBeLessThan(1e-10);const area=independentArea(input.domain.outer,input.domain.holes);expect(Math.abs(area-expected.area)).toBeLessThan(1e-12);expect(Math.abs(area*Math.abs(input.thickness)-tetVolume)).toBeLessThan(1e-10);
  }
 }
 console.log("[DEBUG] All nine actual neutral vectors agreed with portable binary64 goldens and independent Earcut area");
});

test("strict independent schema oracle refuses undeclared and malformed annotation vocabulary",()=>{
 const unknown=structuredClone(schema) as any;unknown["x-unregistered-executable-policy"]={};expect(()=>createSurfaceSchemaOracleV1(unknown)).toThrow("unknown keyword");
 const narrowed=structuredClone(schema);narrowed["x-semio-numeric-policy"].positions="binary32";expect(()=>createSurfaceSchemaOracleV1(narrowed)).toThrow();
 const implicit=structuredClone(schema);implicit["x-semio-operation-contract"].refusal="default-grant";expect(()=>createSurfaceSchemaOracleV1(implicit)).toThrow();
 expect(createSurfaceSchemaOracleV1(schema).surface(fixture.referenceSurface)).toBe(true);
 console.log("[DEBUG] Strict Ajv accepted only the two declared exact first-party annotation vocabularies");
});
