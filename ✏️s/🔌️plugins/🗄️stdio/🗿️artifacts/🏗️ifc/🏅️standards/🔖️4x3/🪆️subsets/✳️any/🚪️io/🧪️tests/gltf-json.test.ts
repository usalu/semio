import {expect,test} from "bun:test";
import {readFile} from "node:fs/promises";
import cases from "./gltf-json-cases.json" with {type:"json"};
import {gltfToIfc4x3,readIfcGltfGlb,writeIfcGltfGlb} from "../gltf.ts";

test.each(cases.cases)("neutral natural glTF JSON: $name",async row=>{
 const json=new TextEncoder().encode(row.prefix+row.json),size=Math.ceil(json.length/4)*4,bytes=new Uint8Array(32+size),view=new DataView(bytes.buffer);view.setUint32(0,0x46546c67,true);view.setUint32(4,2,true);view.setUint32(8,bytes.length,true);view.setUint32(12,size,true);view.setUint32(16,0x4e4f534a,true);bytes.fill(32,20,20+size);bytes.set(json,20);view.setUint32(20+size,4,true);view.setUint32(24+size,0x004e4942,true);bytes[28+size]=7;
 if(row.accepted)expect((await readIfcGltfGlb(bytes)).buffers).toEqual([[7]]);else await expect(readIfcGltfGlb(bytes)).rejects.toBeDefined();
});
test("unsupported document versions cannot pass direct geometry or GLB export routes",async()=>{
 for(const [version,minVersion] of [["1.0",undefined],["2.0","2.1"]] as const){const graph=await readIfcGltfGlb(await readFile(new URL("box.glb",import.meta.url)));graph.document.asset.version=version;graph.document.asset.minVersion=minVersion;const imported=await gltfToIfc4x3(graph);expect(imported.diagnostics[0]?.code).toBe("gltf.unsupported-semantics");expect(Boolean(imported.artifact)).toBe(false);await expect(writeIfcGltfGlb(graph)).rejects.toHaveProperty("code","gltf.version");}
});
