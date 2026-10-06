/** 🎨️ Proves portable polygon attributes and their defining fixture owner. */
import {test,expect} from "bun:test";
import {readFileSync,mkdirSync,writeFileSync,existsSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import Ajv from "ajv/dist/2020.js";
import JSON5 from "json5";
import {Matrix4,Vector3} from "three";

import {runBudgetedTestCommand} from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import {testLevelBudgetMs} from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
const root = fileURLToPath(new URL("../../../", import.meta.url)), fixturePath = resolve(root, "🧫️fixtures/🎨️attributes/🔣️.json"), polygonPath = resolve(root, "🧬️schema/🥽️polygon/🔣️.json"), nativePath = resolve(root, "🧪️tests/🔬️mesh-data-from-value-round-trip/🦀️.rs");
const read=(path:string)=>readFileSync(path,"utf8"),fixture=()=>JSON.parse(read(fixturePath));
const sha=(body:string)=>createHash("sha256").update(body).digest("hex");
function nativeInputs(source:string,path:string):string[]{return [...source.matchAll(/include_str!\("([^"]*🎨️attributes\/🔣️.json)"\)/g)].map(match=>resolve(dirname(path),match[1]!));}
test("portable attribute corpus has closed language-neutral admission",()=>{
 const text = read(fixturePath), value = JSON.parse(text);
 expect(JSON5.parse(text)).toEqual(value);
 
});
test("canonical polygon schema retains indexed channels and rejects invalid declarations",()=>{
 const value=fixture(),schema=JSON.parse(read(polygonPath)),validate=new Ajv({strict:true}).compile(schema);
 expect(schema.$id).toBe("semio:mesh:polygon");for(const key of ["mesh","indexedMesh"])expect(validate(value[key])).toBe(true);
 const domain=structuredClone(value.mesh);domain.attributes.normal.domain="edge";expect(validate(domain)).toBe(false);
 const dimensions=structuredClone(value.mesh);dimensions.attributes.uv.values[0]=[0,0,0];expect(validate(dimensions)).toBe(false);
});
test("owned attribute witnesses agree with independent geometry and indexed expansion",()=>{
 const value=fixture(),channel=value.indexedMesh.attributes.normal,expanded=channel.indices.map((index:number)=>channel.values[index]);
 expect(expanded).toEqual(value.mesh.attributes.normal.values);expect(new Float32Array(expanded.flat()).length).toBe(value.expected.cornerCount*3);
 const components=value.mesh.attributes.normal.values[0],scale=value.normalTransform.scale,normal=new Vector3(components[0],components[1],components[2]).transformDirection(new Matrix4().makeScale(scale[0],scale[1],scale[2]).invert().transpose());
 value.expected.normalAfterScale.forEach((component:number,axis:number)=>expect(normal.toArray()[axis]).toBeCloseTo(component,6));
 const uv=value.indexedMesh.attributes.uv;expect(uv.values[uv.indices[0]]).not.toEqual(uv.values[uv.indices[3]]);expect(value.indexedMesh.attributes.labels.values.length).toBe(1);
});
test("all native attribute reads bind directly to this defining lower owner",()=>{
 const paths=nativeInputs(read(nativePath),nativePath);expect(paths.length).toBe(2);for(const path of paths){expect(path).toBe(fixturePath);expect(existsSync(path)).toBe(true);}
});
test("native fixture exposure survives physical deletion of specific trees",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Caller-owned test output required");
 const sandbox = resolve(output, "mesh-attribute-deletion"), native = resolve(sandbox, relative(root, nativePath));
 for(const path of assets){const target=resolve(sandbox,relative(root,path));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,read(path));}
 const node=Bun.which("node");if(!node)throw Error("Node test oracle is required");
 const code="const fs=require('node:fs'),p=require('node:path'),c=require('node:crypto');const source=process.argv[1],fixture=process.argv[2],text=fs.readFileSync(source,'utf8'),routes=[...text.matchAll(/include_str!\\(\"([^\\\"]*🎨️attributes\\/🔣️.json)\"\\)/g)].map(m=>p.resolve(p.dirname(source),m[1]));if(routes.length!==2||routes.some(x=>x!==fixture))throw Error('Noncanonical fixture owner');const bodies=routes.map(x=>fs.readFileSync(x,'utf8'));console.log(JSON.stringify({routes:routes.length,digests:bodies.map(x=>c.createHash('sha256').update(x).digest('hex'))}));";
 let stdout="";await runBudgetedTestCommand(node,["--eval",code,native,resolve(sandbox,relative(root,fixturePath))],{cwd:sandbox,env:process.env,budgetMs:testLevelBudgetMs(),throwOnFailure:true,captureStdout:{limitBytes:4096,onChunk:bytes=>{stdout+=Buffer.from(bytes).toString("utf8");}}});
 expect(JSON.parse(stdout)).toEqual({routes:2,digests:[sha(read(fixturePath)),sha(read(fixturePath))]});
 for(const name of ["✏️s","s","hub","🛍️products"])expect(existsSync(resolve(sandbox,name))).toBe(false);
 console.log("[DEBUG] Neutral mesh attribute inputs survived a source projection with all specific trees absent");
});
