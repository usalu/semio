import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { createSurfaceSchemaOracleV1 } from "./🔮️schema/🟦️.ts";
import { Earcut } from "three/src/extras/Earcut.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { validateJsonSchemaSubset } from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

const root=resolve(import.meta.dir,"../../../../../../.."),mesh=join(root,"✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh"),artifact=join(root,"✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d");
type Point=readonly[number,number,number];
type Triangle=readonly[number,number,number];
interface Surface {readonly regionId:string;readonly points:readonly Point[];readonly triangles:readonly Triangle[];readonly tetrahedra:readonly(readonly[number,number,number,number])[]}
interface ParsedSurface {readonly triangles:readonly(readonly Point[])[];readonly names:readonly string[]}
const oriented=(triangles:readonly(readonly Point[])[])=>triangles.map(points=>{const rows=points.map(point=>point.join(","));return rows.map((_,index)=>[...rows.slice(index),...rows.slice(0,index)].join("|")).sort()[0]!}).sort();
const surfaceTriangles=(surface:Surface)=>surface.triangles.map(triangle=>triangle.map(index=>surface.points[index]!));
const volume=(triangles:readonly(readonly Point[])[])=>triangles.reduce((sum,[a,b,c])=>sum+(a![0]*(b![1]*c![2]-b![2]*c![1])+a![1]*(b![2]*c![0]-b![0]*c![2])+a![2]*(b![0]*c![1]-b![1]*c![0]))/6,0);
const tetVolume=(surface:Surface)=>surface.tetrahedra.reduce((sum,tet)=>{const [a,b,c,d]=tet.map(index=>surface.points[index]!);const edges=[b!,c!,d!].map(point=>point.map((value,axis)=>value-a![axis]!));return sum+Math.abs(edges[0]![0]!*(edges[1]![1]!*edges[2]![2]!-edges[1]![2]!*edges[2]![1]!)-edges[0]![1]!*(edges[1]![0]!*edges[2]![2]!-edges[1]![2]!*edges[2]![0]!)+edges[0]![2]!*(edges[1]![0]!*edges[2]![1]!-edges[1]![1]!*edges[2]![0]!))/6;},0);
const topology=(surface:Surface)=>{const edges=new Map<string,number>(),vertices=new Set<number>();for(const triangle of surface.triangles){for(const index of triangle){expect(index).toBeLessThan(surface.points.length);vertices.add(index);}for(const edge of[[triangle[0],triangle[1]],[triangle[1],triangle[2]],[triangle[2],triangle[0]]]){const key=edge.sort((a,b)=>a-b).join(",");edges.set(key,(edges.get(key)??0)+1);}}expect([...edges.values()].every(count=>count===2)).toBe(true);return vertices.size-edges.size+surface.triangles.length;};
const bounds=(triangles:readonly(readonly Point[])[])=>{const points=triangles.flat();return[Array.from({length:3},(_,axis)=>Math.min(...points.map(point=>point[axis]!))),Array.from({length:3},(_,axis)=>Math.max(...points.map(point=>point[axis]!)))];};
/** 🔮️ Independent OBJ/STL parsing exposes only first-party test data and explicit f32 projection. */
function thirdPartySurface(text:string,format:"obj"|"stl"):ParsedSurface{
 const triangles:Point[][]=[],names:string[]=[];
 const add=(geometry:{getAttribute(name:string):{count:number;getX(index:number):number;getY(index:number):number;getZ(index:number):number}})=>{const positions=geometry.getAttribute("position");for(let index=0;index<positions.count;index+=3)triangles.push([0,1,2].map(corner=>[positions.getX(index+corner),positions.getY(index+corner),positions.getZ(index+corner)] as Point));};
 if(format==="obj")new OBJLoader().parse(text).traverse(object=>{if("geometry"in object){names.push(object.name);add(object.geometry as Parameters<typeof add>[0]);}});else add(new STLLoader().parse(text));return{triangles,names};
}
const referenceObj=(surface:Surface)=>surface.points.map(point=>"v "+point.join(" ")).join("\n")+"\no "+surface.regionId+"\n"+surface.triangles.map(triangle=>"f "+triangle.map(index=>index+1).join(" ")).join("\n")+"\n";
const referenceStl=(surface:Surface)=>"solid "+surface.regionId+"\n"+surfaceTriangles(surface).map(points=>"facet normal 0 0 0\nouter loop\n"+points.map(point=>"vertex "+point.join(" ")).join("\n")+"\nendloop\nendfacet").join("\n")+"\nendsolid "+surface.regionId+"\n";
const planarArea=(region:typeof fixture.vectors[number]["regions"][number])=>{const contours=[region.domain.outer,...region.domain.holes],points=contours.flat(),holeIndices:number[]=[];let offset=region.domain.outer.length;for(const hole of region.domain.holes){holeIndices.push(offset);offset+=hole.length;}const indices=Earcut.triangulate(points.flat(),holeIndices,2);let area=0;for(let index=0;index<indices.length;index+=3){const [a,b,c]=[0,1,2].map(corner=>points[indices[index+corner]!]!);area+=Math.abs((b![0]!-a![0]!)*(c![1]!-a![1]!)-(b![1]!-a![1]!)*(c![0]!-a![0]!))/2;}return area;};

test("portable complete binary64 inputs and surface schema agree with independent Ajv",()=>{
 const {input,surface}=createSurfaceSchemaOracleV1(schema);
 for(const vector of fixture.vectors)for(const region of vector.regions){expect(input(region)).toBe(true);expect(validateJsonSchemaSubset(schema.$defs.RegionSurfaceInputV1,region,schema)).toEqual([]);}
 expect(surface(fixture.referenceSurface)).toBe(true);expect(validateJsonSchemaSubset(schema.$defs.RegionVolumeSurfaceV1,fixture.referenceSurface,schema)).toEqual([]);
 for(const refusal of fixture.schemaRefusals){const changed=structuredClone(fixture.referenceSurface) as any;if(refusal.mutation==="foreign")changed.foreignSnapshot={};if(refusal.mutation==="point-dimension")changed.points[0]=[0,0];if(refusal.mutation==="negative-index")changed.triangles[0][0]=-1;expect(surface(changed)).toBe(false);expect(validateJsonSchemaSubset(schema.$defs.RegionVolumeSurfaceV1,changed,schema).length).toBeGreaterThan(0);}
 console.log("[DEBUG] Portable complete region/surface JSON agreed with independent Ajv");
});
test("independent binary64 Earcut preserves hole area and analytic extrusion volume",()=>{
 for(const vector of fixture.vectors)for(const expected of vector.expected){const region=vector.regions.find(region=>region.regionId===expected.regionId)!;const area=planarArea(region);expect(Math.abs(area-expected.area)).toBeLessThan(1e-12);expect(Math.abs(area*Math.abs(region.thickness)-expected.volume)).toBeLessThan(1e-12);}
 expect(fixture.vectors.find(vector=>vector.id==="bars-beams-empty")!.elements.map(element=>element.kind)).toEqual(["bar","beam"]);
 console.log("[DEBUG] Independent Earcut binary64 area/volume vectors retained winding and holes");
});
test("independent OBJ and ASCII STL outputs retain reference topology and declared f32 projection",()=>{
 const surface=fixture.referenceSurface as Surface,expected=surfaceTriangles(surface).map(points=>points.map(point=>point.map(Math.fround) as unknown as Point));
 for(const [format,text]of[["obj",referenceObj(surface)],["stl",referenceStl(surface)]] as const){const parsed=thirdPartySurface(text,format);expect(oriented(parsed.triangles)).toEqual(oriented(expected));expect(parsed.triangles.length).toBe(12);expect(Math.abs(volume(parsed.triangles)-1)).toBeLessThan(1e-12);expect(bounds(parsed.triangles)).toEqual([[0,0,0],[2,1,.5]]);expect(Math.abs(tetVolume(surface)-1)).toBeLessThan(1e-12);expect(topology(surface)).toBe(2);if(format==="obj")expect(parsed.names).toContain("rect");}
 console.log("[DEBUG] Independent Three OBJ/STL complete output oracle retained oriented reference faces");
});
test("actual neutral package and FEM native compile closure contain no removable StdIO owner",()=>{
 const manifest=join(mesh,"📦️packages/🦀️rust/Cargo.toml"),types=join(mesh,"📐️surface/🧬️schema/🦀️.rs"),breaches:string[]=[];if(!existsSync(manifest))breaches.push("neutral mesh owning Cargo manifest absent");if(!existsSync(types))breaches.push("neutral f64 region volume/surface types absent");const native=Bun.TOML.parse(readFileSync(join(artifact,"📦️packages/🦀️rust/Cargo.toml"),"utf8")) as {dependencies:Record<string,unknown>};for(const name of Object.keys(native.dependencies).filter(name=>name.startsWith("semio-s-artifact-stdio")))breaches.push("native core retains removable provider: "+name);if(existsSync(join(root,"✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🕸️meshing/🦀️.rs")))breaches.push("document/foreign snapshot bridge remains under neutral module");expect(breaches).toEqual([]);
});
test("actual received neutral and both installed exports preserve complete portable corpus",()=>{
 const path=process.env.SEMIO_FEM_SURFACE_RESULTS;expect(typeof path).toBe("string");const rows=JSON.parse(readFileSync(path!,"utf8")) as {id:string;neutral:{regions:Surface[]};obj:string;stl:string}[];expect(rows.map(row=>row.id)).toEqual(fixture.vectors.map(vector=>vector.id));
 for(const vector of fixture.vectors){const row=rows.find(row=>row.id===vector.id)!;expect(validateJsonSchemaSubset(schema,row.neutral)).toEqual([]);expect(row.neutral.regions.map(region=>region.regionId)).toEqual(vector.expected.map(expected=>expected.regionId));for(const expected of vector.expected){const region=row.neutral.regions.find(region=>region.regionId===expected.regionId)!;expect(bounds(surfaceTriangles(region))).toEqual(expected.bounds);expect(Math.abs(volume(surfaceTriangles(region))-expected.volume)).toBeLessThan(1e-10);expect(region.triangles.length).toBeGreaterThanOrEqual(expected.minimumTriangles);const input=vector.regions.find(input=>input.regionId===expected.regionId)!;expect(Math.abs(tetVolume(region)-expected.volume)).toBeLessThan(1e-10);expect(topology(region)).toBe(2-2*input.domain.holes.length);for(const point of input.domain.outer)expect(region.points.some(position=>position[0]===point[0]&&position[1]===point[1])).toBe(true);}
  const expected=row.neutral.regions.flatMap(region=>surfaceTriangles(region)).map(points=>points.map(point=>point.map(Math.fround) as unknown as Point));for(const [format,text]of[["obj",row.obj],["stl",row.stl]] as const){const parsed=thirdPartySurface(text,format);expect(oriented(parsed.triangles)).toEqual(oriented(expected));expect(Math.abs(volume(parsed.triangles)-volume(expected))).toBeLessThan(1e-5);}
 }
 console.log("[DEBUG] Actual neutral receiver and both installed leaves retained all portable vectors");
});
