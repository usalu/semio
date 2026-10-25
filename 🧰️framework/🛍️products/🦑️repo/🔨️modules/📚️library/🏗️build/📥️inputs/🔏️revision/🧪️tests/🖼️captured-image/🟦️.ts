/** 🖼️ Contract: a bundle compiled with captured Source images answers its reads from those bytes, so a Source edited after the capture cannot refuse it; a Source without an image still refuses a changed byte. */
import{expect,test}from"bun:test";
import{copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{capturedSourceImagesV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts";
import {mirrorRevisionReaderDirectoryV1} from "../🪞️mirror/🟦️.ts";

const module=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),contract=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url));
const oracle=(text:string)=>{const run=Bun.spawnSync(["python3","-I","-c","import hashlib,sys;print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest())"],{stdin:new TextEncoder().encode(text)});expect(run.exitCode).toBe(0);return run.stdout.toString().trim();};
const relative=(path:string)=>path.slice(1);
async function load(sources:(root:string)=>{path:string;sha256:string}[],images:(root:string)=>unknown){
 const root=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"captured-image-"))),own=[module,contract].map(path=>({path:relative(path),sha256:oracle(readFileSync(path,"utf8"))}));
 const rows=[...own,...sources(root)].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),file=join(mirrorRevisionReaderDirectoryV1(root),"🟨️.mjs");
 writeFileSync(file,new Bun.Transpiler({loader:"js",define:{"import.meta.semioPhysicalSourceURL":JSON.stringify(pathToFileURL(module).href),"import.meta.semioCapturedPhysicalSources":JSON.stringify({version:1,root:"/",sources:rows}),"import.meta.semioCapturedPhysicalSourceImages":JSON.stringify(images(root))??"undefined"}}).transformSync(readFileSync(module,"utf8").replace("from\"./","from\""+pathToFileURL(join(module,"..")).href+"/")));
 return{root,loaded:await import(pathToFileURL(file).href+"?"+Math.random())};
}
const data="{\"captured\":true}\n";

test("a captured image answers text and identity reads even after the Source was edited",async()=>{
 const sha=oracle(data);
 const made=await load(root=>[{path:relative(join(root,"data.json")),sha256:sha}],root=>({version:1,images:[{path:relative(join(root,"data.json")),byteLength:Buffer.byteLength(data),text:data}]}));
 try{
  const path=join(made.root,"data.json");writeFileSync(path,"EDITED AFTER THE CAPTURE\n");
  expect(made.loaded.capturePhysicalSourceTextV1(pathToFileURL(path),1024)).toBe(data);
  expect(made.loaded.captureBoundedSourceRevisionV1(path,1024,false)).toEqual({sha256:sha,byteLength:Buffer.byteLength(data),text:null});
  expect(()=>made.loaded.captureBoundedSourceRevisionV1(path,4,false)).toThrow("Compiler-captured admitted Source image identity required");
 }finally{rmSync(made.root,{recursive:true,force:true});}
});

test("a captured Source without an image still refuses a changed byte, and an image that does not match its identity is refused at load",async()=>{
 const sha=oracle(data),plain=await load(root=>[{path:relative(join(root,"data.json")),sha256:sha}],()=>undefined);
 try{const path=join(plain.root,"data.json");writeFileSync(path,data);expect(plain.loaded.captureBoundedSourceRevisionV1(path,1024,true).text).toBe(data);writeFileSync(path,"changed\n");expect(()=>plain.loaded.captureBoundedSourceRevisionV1(path,1024,true)).toThrow("Compiler-captured admitted Source image identity required");}finally{rmSync(plain.root,{recursive:true,force:true});}
 await expect(load(root=>[{path:relative(join(root,"data.json")),sha256:sha}],root=>({version:1,images:[{path:relative(join(root,"data.json")),byteLength:Buffer.byteLength("other\n"),text:"other\n"}]}))).rejects.toThrow("Captured physical Source image must match its identity");
 await expect(load(root=>[{path:relative(join(root,"data.json")),sha256:sha}],()=>({version:1,images:[{path:"unknown.json",byteLength:1}]}))).rejects.toThrow("Captured physical Source image must name one admitted Source");
});

test("the compiler carries the text of data Sources that no module imports and only the length of the others",()=>{
 const code="export const x = 1;\n",json="{\"a\":1}\n",binary=new Uint8Array([0xff,0xfe,0x00]),sha=(value:string)=>oracle(value);
 const manifest={version:1,mode:"x",entry:"entry.ts",resolutions:[],sources:[{path:"data.json",sha256:sha(json)},{path:"entry.ts",sha256:sha(code)},{path:"lib.ts",sha256:sha(code)},{path:"raw.bin",sha256:"0".repeat(64)}]};
 const files=new Map<string,Uint8Array>([["data.json",new TextEncoder().encode(json)],["entry.ts",new TextEncoder().encode(code)],["lib.ts",new TextEncoder().encode(code)],["raw.bin",binary]]);
 const rows=capturedSourceImagesV1(manifest as never,files,new Map([["entry.ts",{"./lib.ts":"lib.ts"}]]));
 expect(rows).toEqual([{path:"data.json",byteLength:Buffer.byteLength(json),text:json},{path:"entry.ts",byteLength:Buffer.byteLength(code)},{path:"lib.ts",byteLength:Buffer.byteLength(code)},{path:"raw.bin",byteLength:3}]);
});

test("an imported data Source that a module names by a relative path literal carries its text, so a runtime read of it survives an edit (independent regex oracle)",()=>{
 const entry="import schema from \"./sub/schema.json\" with {type:\"json\"};import other from \"./other.json\" with {type:\"json\"};export const text=new URL(\"./sub/schema.json\",import.meta.url);export{schema,other};\n",schema="{\"s\":1}\n",other="{\"o\":2}\n",sha=(value:string)=>oracle(value);
 const manifest={version:1,mode:"x",entry:"entry.ts",resolutions:[],sources:[{path:"entry.ts",sha256:sha(entry)},{path:"other.json",sha256:sha(other)},{path:"sub/schema.json",sha256:sha(schema)}]};
 const encode=(value:string)=>new TextEncoder().encode(value),files=new Map<string,Uint8Array>([["entry.ts",encode(entry)],["other.json",encode(other)],["sub/schema.json",encode(schema)]]);
 const rows=capturedSourceImagesV1(manifest as never,files,new Map([["entry.ts",{"./sub/schema.json":"sub/schema.json","./other.json":"other.json"}]]));
 const run=Bun.spawnSync(["python3","-I","-c","import re,sys;print(sorted(set(re.findall(r'new URL\\(\"\\./([^\"]+\\.json)\"',sys.stdin.read()))))"],{stdin:new TextEncoder().encode(entry)});expect(run.exitCode).toBe(0);
 expect(run.stdout.toString().trim()).toBe("['sub/schema.json']");
 expect(rows).toEqual([{path:"entry.ts",byteLength:Buffer.byteLength(entry)},{path:"other.json",byteLength:Buffer.byteLength(other)},{path:"sub/schema.json",byteLength:Buffer.byteLength(schema),text:schema}]);
});
