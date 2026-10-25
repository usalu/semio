/** 🌊️ Contract: the streaming JSON writer produces exactly the text of the standard indented serialisation (checked against `JSON.stringify` and the independent Python model) in pieces of bounded size, honours backpressure and cancellation, and writes a 400 MB text without ever holding more than a piece. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{heapStats}from"bun:jsc";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{writeJsonStreamV1}from"../🟨️.mjs";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py");
const cases=[
 {name:"scalars",indent:2,value:{a:1,b:"two",c:true,d:null,e:[],f:{},g:-7}},
 {name:"nested",indent:2,value:{projects:{"@semio/x":{name:"x",targets:{build:{execution:{kind:"command",command:"run \"x\"\n"},inputs:["{workspaceRoot}/a.ts",{env:"TOKEN"}]}}}},dependencies:[{source:"a",target:"b"}],unavailable:[]}},
 {name:"unicode and escapes",indent:2,value:["🧰️framework/📦️packages","tab\there","quote\"back\\slash","control\u0001\u001f","é ü 日本語"]},
 {name:"compact",indent:0,value:{a:[1,2,{b:[]}],c:{}}},
 {name:"wide indent",indent:4,value:[[1,[2,[3,[4]]]],{k:{k:{k:{k:1}}}}]},
 {name:"top-level scalar",indent:2,value:"text"},
 {name:"top-level empty",indent:2,value:[]},
];
const stream=async(value:unknown,options:object={})=>{const parts:string[]=[];await writeJsonStreamV1(value,text=>{parts.push(text);},options);return parts;};

test("the streamed text equals JSON.stringify and the independent model for every case and every piece size",async()=>{
 for(const row of cases)for(const chunkCharacters of[1,7,64,65536])expect((await stream(row.value,{indent:row.indent,chunkCharacters})).join(""),row.name+" / "+chunkCharacters).toBe(JSON.stringify(row.value,null,row.indent));
 if(python){
  const directory=mkdtempSync(join(tmpdir(),"json-stream-"));
  try{const path=join(directory,"corpus.json");writeFileSync(path,JSON.stringify({cases}));const done=Bun.spawnSync([python,"-I",oracle,"text",path]);expect(done.exitCode).toBe(0);const expected=JSON.parse(done.stdout.toString()) as string[];for(const[index,row]of cases.entries())expect((await stream(row.value,{indent:row.indent})).join(""),row.name).toBe(expected[index]);}
  finally{rmSync(directory,{recursive:true,force:true});}
 }
});

test("serialisation rules of JSON.stringify are kept: omitted members, null elements, toJSON and rejected indents",async()=>{
 const value={kept:1,gone:undefined,fn:()=>1,list:[undefined,()=>1,Symbol("s"),2],date:new Date(0),custom:{toJSON:()=>({x:1})}};
 expect((await stream(value)).join("")).toBe(JSON.stringify(value,null,2));
 await expect(writeJsonStreamV1({},()=>undefined,{indent:11})).rejects.toThrow("build.json.indent-out-of-range");
 await expect(writeJsonStreamV1(undefined,()=>undefined)).rejects.toThrow("build.json.value-required");
});

test("pieces are bounded, written in order with backpressure, and an abort stops between pieces",async()=>{
 const value={rows:Array.from({length:2000},(_,i)=>({i,text:"row "+i}))},sizes:number[]=[];let busy=false,overlap=false;
 await writeJsonStreamV1(value,async text=>{if(busy)overlap=true;busy=true;sizes.push(text.length);await new Promise(resolve=>setImmediate(resolve));busy=false;},{chunkCharacters:4096});
 expect(overlap).toBe(false);expect(Math.max(...sizes)).toBeLessThan(4096+256);expect(sizes.length).toBeGreaterThan(10);
 const controller=new AbortController();let written=0;
 await expect(writeJsonStreamV1(value,()=>{if(++written===3)controller.abort(Error("stopped"));},{chunkCharacters:4096,signal:controller.signal})).rejects.toThrow("stopped");
 expect(written).toBe(3);
});

test("a value of 400 MB of text is written without a resident copy of it",async()=>{
 const shared="payload-"+"x".repeat(1000),value={rows:Array.from({length:380_000},(_,i)=>({i,text:shared,list:[shared]}))};
 const live=()=>{Bun.gc(true);const statistics=heapStats();return statistics.heapSize+statistics.extraMemorySize;};
 let total=0,peak=0,pieces=0;const base=live();
 await writeJsonStreamV1(value,text=>{total+=text.length;if(++pieces%100===0)peak=Math.max(peak,live()-base);},{chunkCharacters:1<<20});
 expect(total).toBeGreaterThan(400_000_000);expect(peak).toBeLessThan(64*1048576);
},300000);
