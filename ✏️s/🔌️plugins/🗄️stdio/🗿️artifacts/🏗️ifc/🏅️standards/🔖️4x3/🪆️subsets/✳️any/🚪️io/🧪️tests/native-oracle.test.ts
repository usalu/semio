import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import {Box3} from "three";
import {parseIfc4x3Step} from "../step.ts";
import {extractIfc4x3Triangles} from "../geometry.ts";
import {ifc4x3ToBimSpatial} from "../bim.ts";
import {ifc4x3ToFem3d} from "../fem.ts";
import {ifcNumber} from "../step.ts";

const folder=process.env.SEMIO_IFC_NATIVE_ORACLE_OUTPUT;

test.skipIf(!folder)("three.js independently reads four genuine original-owned native geometry outputs",async()=>{
 const marker=JSON.parse(await readFile(`${folder}/native-owned-exchange.json`,"utf8"));expect(marker.nativeExecuted).toBe(true);expect(marker.producer).toBe("original-native-gltf-owned-tests");
 const cases=JSON.parse(await readFile(new URL("gltf-samples/cases.json",import.meta.url),"utf8"));
 for(const row of cases.cases){const stem=row.file.split(".")[0],bytes=await readFile(`${folder}/native-owned-${stem}.glb`),result=await new Promise<{scene:import("three").Group}>((resolve,reject)=>new GLTFLoader().parse(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer,"",resolve,reject));result.scene.updateMatrixWorld(true);const box=new Box3().setFromObject(result.scene,true);let vertices=0,triangles=0;result.scene.traverse(child=>{const mesh=child as import("three").Mesh;if(mesh.isMesh){vertices+=mesh.geometry.getAttribute("position").count;triangles+=mesh.geometry.getIndex()!.count/3;}});expect(vertices).toBe(row.vertices);expect(triangles).toBe(row.triangles);const expected=[row.ifcBounds[0][0],row.ifcBounds[0][2],-row.ifcBounds[1][1],row.ifcBounds[1][0],row.ifcBounds[1][2],-row.ifcBounds[0][1]].map(Math.fround);for(const [i,n]of[...box.min.toArray(),...box.max.toArray()].entries())expect(Math.abs(n-expected[i]!)).toBeLessThanOrEqual(8*Number.EPSILON*Math.max(1,Math.abs(n),Math.abs(expected[i]!)));}
});

test.skipIf(!folder)("Khronos and IfcOpenShell independently validate genuine original-owned native exchange",async()=>{
 const report=JSON.parse(await readFile(`${folder}/oracle/native-owned-oracle.json`,"utf8"));expect(report.nativeExecuted).toBe(true);expect(report.nativeOutputProducer).toBe("original-native-gltf-owned-tests");expect(report.validator.implementation).toBe("Khronos glTF Validator");expect(report.ifc.implementation).toBe("IfcOpenShell");expect(report.cases).toHaveLength(4);for(const row of report.cases){expect(row.khronosErrors).toBe(0);expect(row.ifcValidation).toEqual([]);expect(row.ifcTriangleCount).toBe(row.expectedTriangles);expect(row.ifcBoundsWithinEightEpsilons).toBe(true);expect(row.independentSourceIndices).toBe(true);expect(row.independentSourceCoordinates).toBe(true);}
});

test.skipIf(!folder)("IfcOpenShell independently verifies the original native TXT to IFC route",async()=>{
 const metadata=JSON.parse(await readFile(`${folder}/oracle/native-step-import.json`,"utf8"));
 expect(metadata.implementation).toBe("IfcOpenShell");expect(metadata.schema).toBe("IFC4X3_ADD2");expect(metadata.entities).toHaveLength(35);expect(metadata.validation).toEqual([]);
 expect(metadata.lengthScale).toBe(.001);expect(metadata.geometry[0].triangles).toBe(12);expect(metadata.geometry[0].bounds).toEqual([[.5,-.5,0],[1.5,.5,2]]);
 const graph=await parseIfc4x3Step(await readFile(`${folder}/native-step-import.ifc`,"utf8"));expect(graph.entities).toHaveLength(35);const projection=await extractIfc4x3Triangles(graph);expect(projection.artifact?.[0]?.indices).toHaveLength(36);
});

test.skipIf(!folder)("three.js independently reads the native Rust route GLB",async()=>{
 const bytes=await readFile(`${folder}/native-triangulated.glb`);
 const result=await new Promise<{scene:import("three").Group}>((resolve,reject)=>new GLTFLoader().parse(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer,"",resolve,reject));
 result.scene.updateMatrixWorld(true);
 const bounds=new Box3().setFromObject(result.scene,true);
 expect(bounds.min.toArray()).toEqual([.5,0,-.5]);
 expect(bounds.max.toArray()).toEqual([1.5,2,.5]);
 let triangles=0;
 result.scene.traverse(child=>{const mesh=child as import("three").Mesh;if(mesh.isMesh)triangles+=mesh.geometry.getIndex()!.count/3;});
 expect(triangles).toBe(12);
});

test.skipIf(!folder)("IfcOpenShell independently verifies native BIM spatial emission",async()=>{
 const metadata=JSON.parse(await readFile(`${folder}/oracle/native-bim.json`,"utf8"));
 expect(metadata.validation).toEqual([]);expect(metadata.types.IfcBuildingStorey).toBe(2);
 const result=await ifc4x3ToBimSpatial(await parseIfc4x3Step(await readFile(`${folder}/native-bim.ifc`,"utf8")));
 expect(result.artifact).toBeDefined();expect(Object.values(result.artifact!.storeys!).map(s=>s.height)).toEqual([3,3]);
});

test.skipIf(!folder)("IfcOpenShell independently verifies native unloaded FEM emission",async()=>{
 const metadata=JSON.parse(await readFile(`${folder}/oracle/native-fem.json`,"utf8"));
 expect(metadata.validation).toEqual([]);expect(metadata.types.IfcStructuralCurveMember).toBe(2);expect(metadata.types.IfcStructuralLoadCase).toBe(0);
 const result=await ifc4x3ToFem3d(await parseIfc4x3Step(await readFile(`${folder}/native-fem.ifc`,"utf8")));
 expect(result.artifact).toBeDefined();expect(result.artifact!.sections.length).toBe(1);
 expect(ifcNumber({kind:"real",value:result.artifact!.sections[0]!.j})).toBe(4e-5);
});

test.skipIf(!folder)("IfcOpenShell independently verifies funded native FEM owner and host emission",async()=>{
 const metadata=JSON.parse(await readFile(`${folder}/oracle/native-fem-owned.json`,"utf8"));
 expect(metadata.implementation).toBe("IfcOpenShell");expect(metadata.schema).toBe("IFC4X3_ADD2");expect(metadata.validation).toEqual([]);
 expect(metadata.types.IfcStructuralPointConnection).toBe(3);expect(metadata.types.IfcStructuralCurveMember).toBe(2);
 const result=await ifc4x3ToFem3d(await parseIfc4x3Step(await readFile(`${folder}/native-fem-owned.ifc`,"utf8")));
 expect(result.artifact).toBeDefined();expect(result.artifact!.materials.length).toBe(1);expect(result.artifact!.sections.length).toBe(1);expect(result.artifact!.elements.length).toBe(2);
 const members=result.artifact!.elements as Array<{materialId:string;sectionId:string}>;
 expect(members[0]!.materialId).toBe(members[1]!.materialId);expect(members[0]!.sectionId).toBe(members[1]!.sectionId);expect(result.artifact!.sections[0]!.j).toBe(4e-5);
});

test.skipIf(!folder)("IfcOpenShell independently reconstructs the native Rust route IFC",async()=>{
 const metadata=JSON.parse(await readFile(`${folder}/oracle/native-triangulated.json`,"utf8"));
 expect(metadata.implementation).toBe("IfcOpenShell");
 expect(metadata.schema).toBe("IFC4X3_ADD2");
 expect(metadata.geometry[0].triangles).toBe(12);
 expect(metadata.geometry[0].bounds).toEqual([[.5,-.5,0],[1.5,.5,2]]);
 const source=await parseIfc4x3Step(await readFile(`${folder}/native-triangulated.ifc`,"utf8"));
 const projection=await extractIfc4x3Triangles(source);
 expect(projection.artifact?.[0]?.indices.length).toBe(36);
});
