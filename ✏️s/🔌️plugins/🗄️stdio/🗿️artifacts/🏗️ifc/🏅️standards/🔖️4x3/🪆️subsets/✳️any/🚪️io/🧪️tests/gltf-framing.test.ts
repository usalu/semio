import {expect,test} from "bun:test";
import {readFile} from "node:fs/promises";
import {Box3} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import cases from "./gltf-framing-cases.json" with {type:"json"};
import {ifc4x3ToGltf,gltfToIfc4x3,readIfcGltfGlb,writeIfcGltfGlb} from "../gltf.ts";
import {extractIfc4x3Triangles} from "../geometry.ts";
import {ifcReal,parseIfc4x3Step} from "../step.ts";
import {ifcMatrixOrientation} from "../orientation.ts";
import {ifcIdentity} from "../geometry.ts";

function glb(row:typeof cases.cases[number]):Uint8Array {
 const json=new TextEncoder().encode(cases.json+row.jsonTail),size=Math.ceil(json.length/4)*4,bytes=new Uint8Array(32+size+(row.unknownChunk?12:0)),view=new DataView(bytes.buffer);view.setUint32(0,0x46546c67,true);view.setUint32(4,2,true);view.setUint32(8,bytes.length,true);view.setUint32(12,size,true);view.setUint32(16,0x4e4f534a,true);bytes.fill(32,20,20+size);bytes.set(json,20);view.setUint32(20+size,4,true);view.setUint32(24+size,0x004e4942,true);bytes[28+size]=7;bytes.fill(row.binPadding,29+size,32+size);if(row.unknownChunk){view.setUint32(32+size,4,true);view.setUint32(36+size,0x52545845,true);bytes.set([1,2,3,4],40+size);}return bytes;
}
test.each(cases.cases)("neutral GLB framing: $name",async row=>{
 if(row.accepted){expect((await readIfcGltfGlb(glb(row))).buffers).toEqual([[7]]);}else await expect(readIfcGltfGlb(glb(row))).rejects.toBeDefined();
});
test.each(cases.scales)("represented nonsingular scale %s retains independent world bounds",async scale=>{
 const source=await readIfcGltfGlb(await readFile(new URL("box.glb",import.meta.url))),value=ifcReal(scale);if(value.kind!=="real")throw Error("real required");const node=source.document.nodes[0]!;node.matrix=undefined;node.rotation=undefined;node.translation=undefined;node.scale=[value.value,value.value,value.value];const bytes=await writeIfcGltfGlb(source);const parsed=await new Promise<{scene:import("three").Group}>((resolve,reject)=>new GLTFLoader().parse(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer,"",resolve,reject));parsed.scene.updateMatrixWorld(true);const expected=new Box3().setFromObject(parsed.scene,true);const outcome=await gltfToIfc4x3(source);expect(outcome.artifact).toBeDefined();const mesh=(await extractIfc4x3Triangles(outcome.artifact!)).artifact![0]!;expect([0,1,2].map(k=>Math.min(...mesh.positions.map(point=>point[k]!)))).toEqual(expected.min.toArray());expect([0,1,2].map(k=>Math.max(...mesh.positions.map(point=>point[k]!)))).toEqual(expected.max.toArray());
});
test.each(cases.orientations)("neutral represented orientation: $name",row=>{
 const matrix=ifcIdentity();for(let r=0;r<3;r++)for(let c=0;c<3;c++)matrix[c*4+r]=row.linear[r*3+c]!;expect(Number(ifcMatrixOrientation(matrix))).toBe(row.sign);
});
test("nonfinite transform entries cannot publish an orientation",()=>{
 for(const value of [Infinity,-Infinity,NaN])for(const index of [0,12]){const matrix=ifcIdentity();matrix[index]=value;expect(ifcMatrixOrientation(matrix)).toBe(0);}
});
test.each(cases.cancellationRoutes)("original cancellation survives replaced options: %s",async route=>{
 const bytes=await readFile(new URL("box.glb",import.meta.url)),gltf=await readIfcGltfGlb(bytes),ifc=await parseIfc4x3Step(await readFile(new URL("triangulated.ifc",import.meta.url),"utf8"));const original=new AbortController(),replacement=new AbortController();let calls=0;const control={signal:original.signal,progress:()=>{calls++;control.signal=replacement.signal;control.progress=()=>{};original.abort();}};const run=route==="readGlb"?()=>readIfcGltfGlb(bytes,control):route==="writeGlb"?()=>writeIfcGltfGlb(gltf,control):route==="importGltf"?()=>gltfToIfc4x3(gltf,control):()=>ifc4x3ToGltf(ifc,control);await expect(run()).rejects.toHaveProperty("code","io.cancelled");expect(calls).toBe(1);
});
