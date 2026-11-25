/** 🔖️ Contract: a finished wave is journaled in bounded memory: the rows of its owners (log-backed lists, as the inference hands them over) are written part by part, so the resident set a wave adds does not grow with the owners or rows of the wave (the 16:48 whole read died with 11 GB right after storing 455 fragments, when the journal joined every row of the wave into one string), and the parts read back to exactly the rows that went in. */
import {expect,test} from "bun:test";
import {dirname,join} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";

const here=dirname(fileURLToPath(import.meta.url)),generations=join(here,"../.."),url=(path:string)=>pathToFileURL(join(generations,path)).href;
const run=(owners:number,per:number)=>{
 const script=`
  import{mkdtempSync,rmSync,readFileSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
  const{createRowLogV1}=await import(${JSON.stringify(url("🗃️row-log/🟨️.mjs"))}),{createBuildJournalV1,decodeJournalRowsV1,unpackJournalRowsV1,BUILD_JOURNAL_V1}=await import(${JSON.stringify(url("🔖️journal/🟨️.mjs"))}),{createNodeFilePortV1}=await import(${JSON.stringify(url("🗃️file-ports/🟨️.mjs"))});
  const directory=mkdtempSync(join(tmpdir(),"journal-memory-")),log=createRowLogV1({path:join(directory,"rl","a.rows")}),upserts=new Map(),append=new Map();
  const hex=character=>character.repeat(64);
  for(let owner=0;owner<${owners};owner++){const builder=log.builder();for(let from=0;from<${per};from+=250)builder.add(Array.from({length:Math.min(250,${per}-from)},(_unused,at)=>({kind:"file",path:"/w/pkg/p"+owner+"/src/f"+(from+at)+".ts",fact:{size:from+at,mtimeMs:1.7e12+from+at,ino:owner*100000+from+at,sha256:String(owner*100000+from+at).padStart(64,"0")}})));(owner%2?append:upserts).set("o"+String(owner).padStart(6,"0"),builder.finish());}
  const port=createNodeFilePortV1();let peak=0,largest=0;const sample=()=>{peak=Math.max(peak,process.memoryUsage().rss);};
  const fs={...port,async write(path,bytes,options){sample();largest=Math.max(largest,bytes.length);return port.write(path,bytes,options);}};
  const journal=createBuildJournalV1({fs,root:join(directory,"store"),manifestPath:()=>"",publish:async()=>({}),settler:{}});
  Bun.gc(true);await new Promise(resolve=>setTimeout(resolve,50));
  const before=process.memoryUsage().rss;peak=before;const timer=setInterval(sample,5);
  const carry={files:[],waves:0,names:[],done:0,dropped:new Set(),returned:new Set(),derived:new Set()};
  await journal.wave({key:hex("a"),current:undefined,identity:hex("b"),implementation:hex("c"),carry,steps:[],upserts,append,owners:{},env:{digest:hex("d")}});
  clearInterval(timer);sample();
  const seen=new Map();let bytes=0;for(const file of carry.files){const raw=unpackJournalRowsV1(readFileSync(join(directory,"store",BUILD_JOURNAL_V1.directory,file.file)));bytes+=raw.length;for(const entry of decodeJournalRowsV1(raw)){const key=entry.mode+entry.name;seen.set(key,(seen.get(key)??0)+entry.rows.length);}}
  const exact=[...upserts.keys()].every(name=>seen.get("u"+name)===${per})&&[...append.keys()].every(name=>seen.get("a"+name)===${per});
  console.log(JSON.stringify({growth:(peak-before)/1048576,files:carry.files.length,largest:largest/1048576,bytes:bytes/1048576,exact,entries:seen.size}));log.close();rmSync(directory,{recursive:true,force:true});
 `;
 const result=Bun.spawnSync([process.execPath,"-e",script]);
 const text=result.stdout.toString().trim().split("\n").at(-1)??"";
 if(!text.startsWith("{"))throw new Error(result.stderr.toString().slice(0,2000));
 return JSON.parse(text) as {growth:number;files:number;largest:number;bytes:number;exact:boolean;entries:number};
};

test("a wave of log-backed rows is journaled in parts of bounded size and adds less resident set than the bytes it writes (the whole-string journal added four and a half times as much)",()=>{
 const small=run(400,1000),large=run(1600,1000);
 expect(large.entries).toBe(1600);expect(small.exact).toBe(true);expect(large.exact).toBe(true);
 expect(large.bytes).toBeGreaterThan(small.bytes*3);
 expect(large.files).toBeGreaterThan(small.files*2);
 expect(large.largest,"largest part in MiB").toBeLessThan(16);
 expect(large.growth,"MiB added by a 1,600 owner wave").toBeLessThan(large.bytes*0.8);
 expect(large.growth-small.growth,"MiB for 1,200 further owners").toBeLessThan(small.growth*2);
},240000);
