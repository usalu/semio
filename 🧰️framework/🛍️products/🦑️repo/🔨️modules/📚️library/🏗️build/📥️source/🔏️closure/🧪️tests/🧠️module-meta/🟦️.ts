import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{closeSync as closeFileV1,mkdirSync as makeDirectoryV1,mkdtempSync as makeTemporaryV1,openSync as openFileV1,realpathSync as realPathV1,rmSync as removeTreeV1,writeFileSync as writeFileSyncV1}from"node:fs";
import{spawnSync as spawnSyncV1}from"node:child_process";
import{tmpdir as temporaryDirectoryV1}from"node:os";
import{dirname,join}from"node:path";
import{buildSourceClosureStreamLoaderV1}from"../../🟦️.ts";
import fixture from"./🔣️.json";

const environmentV1={...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"};

/** 🧱️ The snapshot text of a file tree rooted at `root`: rows sorted as the capture writes them, every relative import mapped to its own path. */
function snapshotOfV1(root:string,files:Readonly<Record<string,string>>):{text:string;sha256:string}{
 const rows=Object.entries(files).sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(([path,body])=>({path,sha256:createHash("sha256").update(body).digest("hex"),base64:Buffer.from(body).toString("base64"),imports:Object.fromEntries([...body.matchAll(/from"\.\/([^"]+)"/gu)].map(match=>["./"+match[1]!,match[1]!]))})),text=JSON.stringify({version:1,root,entry:"entry.ts",files:rows});
 return{text,sha256:createHash("sha256").update(text).digest("hex")};
}

/** 🏃️ Runs the tree under the aux loader, the snapshot an inherited file descriptor 3 as the command publishes it. */
function runLoadedV1(area:string,files:Readonly<Record<string,string>>):{status:number|null;out:string;err:string}{
 const root=join(area,"loaded"),{text,sha256}=snapshotOfV1(root,files),file=join(area,"snapshot.json");writeFileSyncV1(file,text);const descriptor=openFileV1(file,"r");
 try{const run=spawnSyncV1(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",sha256],{encoding:"utf8",env:environmentV1,stdio:["ignore","pipe","pipe",descriptor]});return{status:run.status,out:run.stdout,err:run.stderr};}finally{closeFileV1(descriptor);}
}

/** 🌍️ Runs the same tree natively from disk by Bun's own module loader, the independent implementation the loader is checked against. */
function runNativeV1(area:string,files:Readonly<Record<string,string>>):{status:number|null;out:string;err:string}{
 const root=join(area,"native");for(const[path,body]of Object.entries(files)){makeDirectoryV1(dirname(join(root,path)),{recursive:true});writeFileSyncV1(join(root,path),body);}
 const run=spawnSyncV1(process.execPath,["--eval",`const module=await import(${JSON.stringify(join(root,"entry.ts"))});await module.runCapturedBuildSourceV1();`],{encoding:"utf8",env:environmentV1});return{status:run.status,out:run.stdout,err:run.stderr};
}

const filesOfV1=(files:Readonly<Record<string,string|undefined>>):Record<string,string>=>Object.fromEntries(Object.entries(files).filter((entry):entry is[string,string]=>typeof entry[1]==="string"));
const expandV1=(value:unknown,root:string):unknown=>typeof value==="string"?value.replaceAll("{root}",root):Array.isArray(value)?value.map(item=>expandV1(item,root)):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,expandV1(item,root)])):value;

test("module meta of a captured module names that module and equals Bun's native loader and the fixture",()=>{
 const area=realPathV1(makeTemporaryV1(join(temporaryDirectoryV1(),"mm-")));
 try{
  for(const row of fixture.cases){
   const files=filesOfV1(row.files),loaded=runLoadedV1(area,files),native=runNativeV1(area,files);
   expect(loaded.err).toBe("");expect(native.err).toBe("");expect(loaded.status).toBe(0);expect(native.status).toBe(0);
   const got=JSON.parse(loaded.out),reference=JSON.parse(native.out.split(join(area,"native")).join(join(area,"loaded")));
   expect(got).toEqual(expandV1(row.expected,join(area,"loaded")));expect(got).toEqual(reference);
  }
 }finally{removeTreeV1(area,{recursive:true,force:true});}
},60000);

/** 📏️ A tree of `count` modules, each a few kilobytes that use their own module meta, under one entry that reports the resident set of the process after loading them all. */
function crowdV1(count:number):Record<string,string>{
 const body=(index:number)=>`export const v${index}=import.meta.url.length+${index};\n`+Array.from({length:60},(_,k)=>`export function g${index}_${k}(a:number){return a*${k}+"${"x".repeat(40)}${k}".length;}`).join("\n"),files:Record<string,string>={};
 for(let index=0;index<count;index++)files[`m${index}.ts`]=body(index);
 files["entry.ts"]=Array.from({length:count},(_,index)=>`import{v${index}}from"./m${index}.ts";`).join("")+`export async function runCapturedBuildSourceV1(){let s=0;${Array.from({length:count},(_,index)=>`s+=v${index};`).join("")}console.log(JSON.stringify({sum:s,rssMegabytes:process.memoryUsage().rss/1048576}));}`;
 return files;
}

test("a captured crowd of modules shares its transpilers: loading 600 modules stays far below the one-transpiler-per-module footprint",()=>{
 const area=realPathV1(makeTemporaryV1(join(temporaryDirectoryV1(),"mm-")));
 try{
  const small=runLoadedV1(area,crowdV1(50)),large=runLoadedV1(area,crowdV1(600));
  expect(small.err).toBe("");expect(large.err).toBe("");expect(large.status).toBe(0);
  const grown=JSON.parse(large.out).rssMegabytes-JSON.parse(small.out).rssMegabytes;
  expect(grown).toBeLessThan(65);
 }finally{removeTreeV1(area,{recursive:true,force:true});}
},120000);
