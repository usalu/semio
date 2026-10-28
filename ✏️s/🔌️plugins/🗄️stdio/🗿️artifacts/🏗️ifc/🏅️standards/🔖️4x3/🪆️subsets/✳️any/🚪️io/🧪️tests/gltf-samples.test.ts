import {expect,test} from "bun:test";
import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {Texture,Vector3,type Mesh} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import cases from "./gltf-samples/cases.json" with {type:"json"};
import provenance from "./gltf-samples/provenance.json" with {type:"json"};
import {parseGltfDocument} from "../../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
import {gltfToIfc4x3,readIfcGltfGlb} from "../gltf.ts";
import {extractIfc4x3Triangles} from "../geometry.ts";

if(typeof ProgressEvent==="undefined")Object.defineProperty(globalThis,"ProgressEvent",{value:class extends Event {lengthComputable:boolean;loaded:number;total:number;constructor(type:string,init:{lengthComputable?:boolean;loaded?:number;total?:number}={}){super(type);this.lengthComputable=init.lengthComputable??false;this.loaded=init.loaded??0;this.total=init.total??0;}}});

test.each(cases.cases)("original Khronos $name retains independent world triangles",async row=>{
 const bytes=await readFile(new URL("gltf-samples/"+row.file,import.meta.url)),original=provenance.cases.find(item=>item.file===row.file)!;expect(createHash("sha256").update(bytes).digest("hex")).toBe(original.sha256);expect(provenance.commit).toBe(cases.sourceCommit);expect(createHash("sha256").update(await readFile(new URL("gltf-samples/"+original.name+"-README.md",import.meta.url))).digest("hex")).toBe(original.readmeSha256);
 const binary=row.file.endsWith(".glb"),snapshot=binary?await readIfcGltfGlb(bytes):(()=>{const document=parseGltfDocument(JSON.parse(bytes.toString("utf8")));return{schema:"stdio.gltf" as const,document,sourceForm:"json" as const,buffers:document.buffers.map(buffer=>Array.from(Buffer.from(buffer.uri!.split(",")[1]!,"base64")))};})();
 const loader=new GLTFLoader();loader.register(()=>({name:"semio_geometry_only_texture",loadTexture:()=>Promise.resolve(new Texture())}));const independent=await loader.parseAsync(binary?bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer:bytes.toString("utf8"),"");independent.scene.updateMatrixWorld(true);
 const meshes:{positions:number[][];indices:number[]}[]=[];independent.scene.traverse(child=>{const mesh=child as Mesh;if(!mesh.isMesh)return;const positions=Array.from({length:mesh.geometry.getAttribute("position").count},(_,index)=>{const point=mesh.getVertexPosition(index,new Vector3()).applyMatrix4(mesh.matrixWorld);return[point.x,-point.z,point.y];}),indices=Array.from(mesh.geometry.index?.array??Array.from({length:positions.length},(_,index)=>index));if(mesh.matrixWorld.determinant()<0)for(let i=0;i<indices.length;i+=3)[indices[i+1],indices[i+2]]=[indices[i+2]!,indices[i+1]!];meshes.push({positions,indices});});
 const imported=await gltfToIfc4x3(snapshot,{batchSize:4096});expect(Boolean(imported.artifact)).toBe(true);const actual=(await extractIfc4x3Triangles(imported.artifact!,{batchSize:4096})).artifact!;expect(actual.length).toBe(row.primitives);expect(actual.reduce((sum,mesh)=>sum+mesh.positions.length,0)).toBe(row.vertices);expect(actual.reduce((sum,mesh)=>sum+mesh.indices.length/3,0)).toBe(row.triangles);expect(actual.map(mesh=>mesh.indices)).toEqual(meshes.map(mesh=>mesh.indices));let maximumError=0;for(let m=0;m<actual.length;m++){expect(actual[m]!.positions.length).toBe(meshes[m]!.positions.length);for(let p=0;p<actual[m]!.positions.length;p++)for(let k=0;k<3;k++){const a=actual[m]!.positions[p]![k]!,b=meshes[m]!.positions[p]![k]!;maximumError=Math.max(maximumError,Math.abs(a-b)/Math.max(1,Math.abs(a),Math.abs(b)));}}expect(maximumError).toBeLessThanOrEqual(cases.relativeErrorLimit);expect([0,1,2].map(k=>Math.min(...actual.flatMap(mesh=>mesh.positions.map(point=>point[k]!))))).toEqual(row.ifcBounds[0]!);expect([0,1,2].map(k=>Math.max(...actual.flatMap(mesh=>mesh.positions.map(point=>point[k]!))))).toEqual(row.ifcBounds[1]!);
});
