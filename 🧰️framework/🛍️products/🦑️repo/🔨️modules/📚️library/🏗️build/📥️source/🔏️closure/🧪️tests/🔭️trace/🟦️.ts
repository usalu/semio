/** 🔭️ Contract: a process that runs a captured snapshot through the shared loader writes a source trace only when the entry enables it, and then records exactly the captured modules it loaded, the captured files it read as bytes, the live files it read and the action it declared, each once; an independent bundler (esbuild) names the same module set; the fixture lives in `./🔣️.json`. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {build} from "esbuild";
import {buildSourceClosureLoaderV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");

function run(traced:boolean){
 const base=mkdtempSync(join(realpathSync(tmpdir()),"source-trace-")),root=join(base,"root");mkdirSync(root);
 for(const[path,text]of Object.entries({...corpus.files,...corpus.live})){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 const rows=Object.entries(corpus.files).sort(([a],[b])=>a<b?-1:1).map(([path,text])=>({path,sha256:hash(text),base64:Buffer.from(text).toString("base64"),imports:path===corpus.entry?corpus.imports:{}})),snapshot=JSON.stringify({version:1,root,entry:corpus.entry,files:rows}),snapshotPath=join(base,"implementation.json"),runner=join(base,"📜️script.ts"),trace=join(base,"trace");
 writeFileSync(snapshotPath,snapshot);writeFileSync(runner,buildSourceClosureLoaderV1);
 const result=spawnSync(process.execPath,[runner,snapshotPath,hash(snapshot),"one","two"],{encoding:"utf8",env:{...process.env,TRACE_DIR:traced?trace:""}});
 return {base,root,trace,result};
}

test("the corpus is closed by its schema",()=>{const ajv=new Ajv({strict:false});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);expect(ajv.validate(schema,{...corpus,extra:1})).toBe(false);expect(ajv.validate(schema,{...corpus,expectedModules:"x"})).toBe(false);});

test("a run that never enables the trace writes none and behaves like a plain captured run",()=>{
 const row=run(false);
 try{expect(row.result.status,row.result.stderr).toBe(0);expect(row.result.stdout).toContain("lazy-loaded");expect(existsSync(row.trace)).toBe(false);}finally{rmSync(row.base,{recursive:true,force:true});}
});

test("an enabled trace records each loaded module, captured read, live read and the declared action once, and a bundler agrees on the modules",async()=>{
 const row=run(true);
 try{
  expect(row.result.status,row.result.stderr).toBe(0);
  const files=readdirSync(row.trace);expect(files.length).toBe(1);
  const lines=readFileSync(join(row.trace,files[0]!),"utf8").split("\n").filter(Boolean),of=(kind:string)=>lines.filter(line=>line.startsWith(kind+" ")).map(line=>line.slice(2));
  expect(lines[0]!.startsWith("s ")).toBe(true);expect(JSON.parse(lines[0]!.slice(2)).entry).toBe(corpus.entry);
  expect(of("m").sort()).toEqual(corpus.expectedModules);expect(of("r").sort()).toEqual(corpus.expectedReads);expect(of("x").sort()).toEqual(corpus.expectedLive);expect(of("a").map(text=>JSON.parse(text))).toEqual([corpus.expectedAction]);
  expect(new Set(lines).size).toBe(lines.length);
  const oracle=await build({absWorkingDir:row.root,entryPoints:[corpus.entry],bundle:true,write:false,metafile:true,platform:"node",format:"esm",logLevel:"silent"});
  expect(Object.keys(oracle.metafile!.inputs).map(path=>path.replace(/ with \{.*\}$/u,"")).sort()).toEqual(corpus.expectedModules);
 }finally{rmSync(row.base,{recursive:true,force:true});}
});
