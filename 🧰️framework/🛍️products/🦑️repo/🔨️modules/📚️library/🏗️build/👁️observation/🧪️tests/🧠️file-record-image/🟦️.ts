/** 🧠️ Contract: the analysis of a settled file is computed, hashed and stored once per process while the file keeps its stamp, every later asker receives its own copy of the same analysis and replays the same file observation, a changed or still racy file is analysed again, and releasing the inference memory forgets the images; the analysis equals an independent Python computation over the same bytes. */
import{beforeAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const api=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs");
const{memoizedFileRecord,observeInference,captureObservedSourceInputsV1,releaseInferenceMemoryV1}=api as unknown as{captureObservedSourceInputsV1:(compute:()=>unknown)=>Promise<{value:unknown;observations:{kind:string;path:string;fact:{sha256?:string}}[]}>;memoizedFileRecord:(namespace:string,path:string,compute:(bytes:Buffer)=>unknown)=>unknown;observeInference:(options:{cachePath:string;key:unknown},compute:()=>unknown)=>Promise<unknown>;releaseInferenceMemoryV1:()=>void};
const python=testPythonV1();
beforeAll(async()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(root,{recursive:true});await observeInference({cachePath:join(root,"file-record-image-load.json"),key:"file-record-image-load"},()=>undefined);});
const pythonAnalysis=(file:string)=>{
 const run=Bun.spawnSync([python!,"-I","-c","import sys,json\nb=open(sys.argv[1],'rb').read()\nprint(json.dumps({'length':len(b),'upper':b.decode('utf-8').upper(),'lines':b.count(b'\\n'),'sha256':__import__('hashlib').sha256(b).hexdigest()},ensure_ascii=True))",file]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()) as{length:number;upper:string;lines:number;sha256:string};
};
const fresh=()=>{const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"file-record-image-"));mkdirSync(join(root,".cache"),{recursive:true});return root;};
const scope=(root:string,name:string,body:()=>unknown)=>observeInference({cachePath:join(root,".cache",name+".json"),key:[name]},()=>({value:body()}));
const settle=()=>Bun.sleepSync(2300);

test("a settled file is analysed once per process, each asker owns its copy, and the analysis equals the Python computation",async()=>{
 const root=fresh(),file=join(root,"one.txt"),cachePath=join(root,".cache","a.json");
 writeFileSync(file,"alpha\nbeta é\n");settle();releaseInferenceMemoryV1();
 let analyses=0;const analyse=(bytes:Buffer)=>{analyses++;return{length:bytes.byteLength,upper:bytes.toString("utf8").toUpperCase(),lines:bytes.toString("utf8").split("\n").length-1};};
 const expected=pythonAnalysis(file),results:unknown[]=[];
 await observeInference({cachePath,key:["a"]},async()=>{
  for(let round=0;round<4;round++){
   const captured=await captureObservedSourceInputsV1(()=>memoizedFileRecord("law-image-v1",file,analyse));
   results.push(captured.value);
   const rows=captured.observations.filter(row=>row.kind==="file"&&row.path===file);
   expect(rows.length,"round "+round).toBe(1);
   expect(rows[0]!.fact.sha256,"round "+round).toBe(expected.sha256);
  }
  return{done:true};
 });
 expect(analyses).toBe(1);
 expect(results[0]).toEqual({length:expected.length,upper:expected.upper,lines:expected.lines});
 for(const result of results)expect(result).toEqual(results[0]);
 expect(new Set(results).size).toBe(results.length);
 (results[0] as{upper:string}).upper="mutated";
 expect(results[1]).toEqual({length:expected.length,upper:expected.upper,lines:expected.lines});
 rmSync(root,{recursive:true,force:true});
},30_000);

test("a changed file and a released memory are analysed again; a namespace is its own key",async()=>{
 const root=fresh(),file=join(root,"two.txt");
 writeFileSync(file,"first\n");settle();releaseInferenceMemoryV1();
 let analyses=0;const analyse=(bytes:Buffer)=>{analyses++;return{length:bytes.byteLength,text:bytes.toString("utf8")};};
 const ask=(namespace="law-image-v1")=>memoizedFileRecord(namespace,file,analyse);
 await scope(root,"one",()=>{ask();ask();});
 expect(analyses).toBe(1);
 await scope(root,"two",ask);
 expect(analyses).toBe(1);
 await scope(root,"three",()=>ask("law-image-other-v1"));
 expect(analyses).toBe(2);
 releaseInferenceMemoryV1();
 await scope(root,"four",ask);
 expect(analyses).toBe(3);
 writeFileSync(file,"second line\n");settle();
 let after:unknown;
 await scope(root,"five",()=>{after=ask();});
 expect(analyses).toBe(4);
 expect(after).toEqual({length:12,text:"second line\n"});
 await scope(root,"six",ask);
 expect(analyses).toBe(4);
 rmSync(root,{recursive:true,force:true});
},60_000);

test("a file written moments ago is never remembered: every ask analyses its current bytes",async()=>{
 const root=fresh(),file=join(root,"three.txt");
 releaseInferenceMemoryV1();
 let analyses=0;const analyse=(bytes:Buffer)=>{analyses++;return bytes.toString("utf8");};
 const seen:unknown[]=[];
 for(const [name,text] of [["a","racy one\n"],["b","racy two\n"],["c","racy two\n"]] as const){
  writeFileSync(file,text);
  await scope(root,name,()=>{seen.push(memoizedFileRecord("law-image-v1",file,analyse));});
 }
 expect(analyses).toBe(3);
 expect(seen).toEqual(["racy one\n","racy two\n","racy two\n"]);
 expect(readFileSync(file,"utf8")).toBe("racy two\n");
 rmSync(root,{recursive:true,force:true});
},30_000);
