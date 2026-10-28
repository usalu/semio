import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {parseIfc4x3Step} from "../step.ts";
import {ifc4x3ToGltf,writeIfcGltfGlb} from "../gltf.ts";
import {ifc4x3ToFem3d} from "../fem.ts";
import {ifc4x3ToBimSpatial} from "../bim.ts";
import {ifcNumber} from "../step.ts";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import {Box3,Vector3} from "three";
import type {IfcSnapshot} from "../../🧬️schema/📸️snapshot/🟦️.ts";
import {project as projectIfcGeometry, projectWith, DEFAULT_GEOMETRY_LIMITS, geometryPayloads} from "../../../../../4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🟦️.ts";

const sorted=(snapshot:IfcSnapshot)=>({...snapshot,entities:snapshot.entities.toSorted((a,b)=>a.id<b.id?-1:a.id>b.id?1:0)});
const oracle=process.env.SEMIO_IFC_ORACLE_OUTPUT;
test.skipIf(!oracle)("IfcOpenShell independently roundtrips complete official fixture graphs",async()=>{
 for(const name of ["triangulated","alignment","structural"]) {
  const source=await parseIfc4x3Step(await readFile(new URL(name+".ifc",import.meta.url),"utf8")),independent=await parseIfc4x3Step(await readFile(`${oracle}/${name}.ifc`,"utf8"));
  expect(sorted(independent)).toEqual(sorted(source));
  const metadata=JSON.parse(await readFile(`${oracle}/${name}.json`,"utf8"));
  expect(metadata.schema).toBe("IFC4X3_ADD2");expect(metadata.entities.length).toBe(source.entities.length);
 }
 const geometry=JSON.parse(await readFile(`${oracle}/triangulated.json`,"utf8")).geometry.find((row:{id:number})=>row.id===1000);
 expect(geometry.vertices).toBe(8);expect(geometry.triangles).toBe(12);expect(geometry.bounds).toEqual([[.5,-.5,0],[1.5,.5,2]]);
});
test.skipIf(!oracle)("IfcOpenShell reconstructed BIM and FEM graphs retain supported projections",async()=>{
 const fem=await ifc4x3ToFem3d(await parseIfc4x3Step(await readFile(`${oracle}/fem-export.ifc`,"utf8")));
 expect(fem.artifact).toBeDefined();expect(fem.artifact!.elements.length).toBe(1);expect(fem.artifact!.loadCases[0]!.loads.length).toBe(2);
 expect(ifcNumber({kind:"real",value:fem.artifact!.materials[0]!.e})).toBe(2e11);
 const bim=await ifc4x3ToBimSpatial(await parseIfc4x3Step(await readFile(`${oracle}/bim-export.ifc`,"utf8")));
 expect(bim.artifact).toBeDefined();expect(Object.values(bim.artifact!.buildings!)[0]!.origin).toEqual({x:1,y:2});expect(Object.values(bim.artifact!.storeys!).map(row=>row.level)).toEqual([0,1]);
});
test("three.js independently reads exported GLB geometry and world bounds",async()=>{
 const snapshot=await parseIfc4x3Step(await readFile(new URL("triangulated.ifc",import.meta.url),"utf8")),outcome=await ifc4x3ToGltf(snapshot),bytes=await writeIfcGltfGlb(outcome.artifact!);
 const result=await new Promise<{scene:import("three").Group}>( (resolve,reject)=>new GLTFLoader().parse(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer,"",resolve,reject));
 result.scene.updateMatrixWorld(true);const bounds=new Box3().setFromObject(result.scene,true),min=bounds.min,max=bounds.max;
 expect([min.x,-max.z,min.y]).toEqual([.5,-.5,0]);expect([max.x,-min.z,max.y]).toEqual([1.5,.5,2]);
 let vertices=0,triangles=0;
 result.scene.traverse(child=>{const mesh=child as import("three").Mesh;if(mesh.isMesh){vertices+=mesh.geometry.getAttribute("position").count;triangles+=mesh.geometry.getIndex()!.count/3;}});
 expect(vertices).toBe(8);expect(triangles).toBe(12);
});

test("shared editor projection reads official ADD2 geometry and suppresses cancelled meshes",async()=>{
 const snapshot=await parseIfc4x3Step(await readFile(new URL("triangulated.ifc",import.meta.url),"utf8")),scene=projectIfcGeometry(snapshot);
 expect(scene.status).toBe("complete");expect(scene.issues).toEqual([]);expect(scene.meshes).toHaveLength(1);
 expect(scene.meshes[0]!.vertices).toHaveLength(8);expect(scene.meshes[0]!.triangles).toHaveLength(12);
 const vertices=scene.meshes[0]!.vertices;
 expect([0,1,2].map(i=>Math.min(...vertices.map(vertex=>vertex[i]!)))) .toEqual([500,-500,0]);
 expect([0,1,2].map(i=>Math.max(...vertices.map(vertex=>vertex[i]!)))) .toEqual([1500,500,2000]);
 const cancelled=projectWith(snapshot,DEFAULT_GEOMETRY_LIMITS,()=>false),payload=geometryPayloads(cancelled);
 expect(cancelled.status).toBe("cancelled");expect(JSON.parse(payload.meshesJson)).toEqual([]);expect(JSON.parse(payload.instancesJson)).toEqual([]);
});
