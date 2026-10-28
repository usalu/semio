import {expect,test} from "bun:test";
import {readFile} from "node:fs/promises";
import cases from "./gltf-accessor-cases.json" with {type:"json"};
import {gltfToIfc4x3,readIfcGltfGlb,writeIfcGltfGlb} from "../gltf.ts";

const source=async()=>readIfcGltfGlb(await readFile(new URL("box.glb",import.meta.url)));
test.each(cases.invalid)("neutral dense accessor refusal: $name",async row=>{
 const graph=await source(),doc=graph.document,primitive=doc.meshes[0]!.primitives[0]!,index=primitive.attributes.find(([name])=>name==="POSITION")![1],position=doc.accessors[Number(index)]!,view=doc.bufferViews[Number(position.bufferView)]!;
 if(row.edit==="misalignedViews"){graph.buffers[0]!.unshift(0);doc.buffers[0]!.byteLength++;for(const item of doc.bufferViews)item.byteOffset++;}
 else if(row.edit==="shortDeclaredBuffer")doc.buffers[0]!.byteLength=1n;
 else if(row.edit==="wrongPositionTarget")view.target=34963n;
 else graph.buffers[Number(view.buffer)]![Number(view.byteOffset+position.byteOffset)]=256;
 const outcome=await gltfToIfc4x3(graph);expect(outcome.artifact).toBeUndefined();expect(outcome.fidelity).toBe("unsupported");
});
test.each(cases.invalidBytes)("GLB export refuses a nonbyte value %s before normalization",async value=>{
 const graph=await source();graph.buffers[0]![0]=value;await expect(writeIfcGltfGlb(graph)).rejects.toHaveProperty("code","glb.bytes");
});
test.each(cases.invalidByteCounts)("GLB read and write refuse a zero-length declared buffer %s",async count=>{
 const graph=await source();graph.document.buffers[0]!.byteLength=BigInt(count);graph.buffers[0]=Array(count).fill(0);await expect(writeIfcGltfGlb(graph)).rejects.toHaveProperty("code","glb.length");
 const json=new TextEncoder().encode('{"asset":{"version":"2.0"},"buffers":[{"byteLength":0}]}'),size=Math.ceil(json.length/4)*4,bytes=new Uint8Array(28+size),view=new DataView(bytes.buffer);view.setUint32(0,0x46546c67,true);view.setUint32(4,2,true);view.setUint32(8,bytes.length,true);view.setUint32(12,size,true);view.setUint32(16,0x4e4f534a,true);bytes.fill(32,20,20+size);bytes.set(json,20);view.setUint32(20+size,0,true);view.setUint32(24+size,0x004e4942,true);await expect(readIfcGltfGlb(bytes)).rejects.toHaveProperty("code","glb.length");
});
test.each(["read","write"])("original cancellation reaches interior GLB %s bytes",async route=>{
 const graph=await source(),count=cases.binaryCancellation.count;graph.document.buffers[0]!.byteLength=BigInt(count);graph.buffers[0]=Array(count).fill(0);const original=new AbortController();let seen=false;const control={signal:original.signal,progress:(progress:{phase:string;completed:number})=>{if(progress.phase===cases.binaryCancellation.phase&&progress.completed>=cases.binaryCancellation.minimumCompleted){seen=true;original.abort();}}},bytes=await writeIfcGltfGlb(graph);await expect(route==="read"?readIfcGltfGlb(bytes,control):writeIfcGltfGlb(graph,control)).rejects.toHaveProperty("code","io.cancelled");expect(seen).toBe(true);
});
test("original cancellation reaches one dense accessor before graph publication",async()=>{
 const graph=await source(),position=graph.document.accessors[Number(graph.document.meshes[0]!.primitives[0]!.attributes.find(([name])=>name==="POSITION")![1])]!,view=graph.document.bufferViews[Number(position.bufferView)]!,count=cases.cancellation.count;position.count=BigInt(count);position.byteOffset=0n;view.buffer=0n;view.byteOffset=0n;view.byteLength=BigInt(count*12);view.byteStride=undefined;graph.buffers[0]=Array(count*12).fill(0);graph.document.buffers[0]!.byteLength=BigInt(count*12);const original=new AbortController();let seen=false;
 await expect(gltfToIfc4x3(graph,{signal:original.signal,batchSize:64,progress:progress=>{if(progress.phase===cases.cancellation.phase&&progress.completed>=cases.cancellation.minimumCompleted){seen=true;original.abort();}}})).rejects.toHaveProperty("code","io.cancelled");expect(seen).toBe(true);
});
