/** 🌊️ Contract: a raw filesystem error is classified once, by one closed table, as a race with a peer (`transient`, `persistent`) or as no race; the verdict equals the independent Python oracle on every vector, the real operating system gives the errno names the table assumes (Python `os` is the oracle), and a race on a real tree is recognised only when a directory around the path changed since the process started. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {lstatSync,mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../../🧬️schema/🌊️races.json";
import churn from "../../🧬️schema/🔣️.json";
import vectors from "./🔣️.json";
import {testPythonV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {decideFilesystemRaceV1,filesystemErrorFromTextV1,filesystemRaceOfErrorV1,filesystemRaceOfTextV1,filesystemRaceTableV1,observeRaceWorldV1,processRaceWorldV1,type RaceFactsV1,type RaceWorldV1} from "../../🌊️races/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),table=filesystemRaceTableV1();
const oracle=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"..","..","🧬️schema","🌊️races.json"),join(here,"🔣️.json")]).stdout.toString()) as {decide:Record<string,unknown>;observe:Record<string,unknown>;text:Record<string,unknown>;errnos:Record<string,string|null>};
const worldOf=(row:{platform:string;lstat:Record<string,unknown>}):RaceWorldV1=>({root:"/w",startedAtMs:vectors.startedAtMs,platform:row.platform,lstat:path=>{if(!Object.hasOwn(row.lstat,path))throw Error("Unexpected stat of "+path);return row.lstat[path] as ReturnType<RaceWorldV1["lstat"]>;}});
let root="",sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"races-")));root=join(sandbox,"w");mkdirSync(join(root,"src"),{recursive:true});writeFileSync(join(root,"AGENTS.md"),"x");writeFileSync(join(root,"package.json"),"{}");writeFileSync(join(root,"src","a.ts"),"a");});
afterAll(()=>{rmSync(sandbox,{recursive:true,force:true});});
const errorOf=(action:()=>unknown):NodeJS.ErrnoException=>{try{action();}catch(error){return error as NodeJS.ErrnoException;}throw Error("An error was expected");};
const worldAt=(startedAtMs:number):RaceWorldV1=>({...processRaceWorldV1({SEMIO_WORKSPACE_ROOT:root}),root,startedAtMs});

test("the table is closed under its schema and speaks the vocabulary of the churn policy",()=>{
 const ajv=new Ajv({strict:false}),code=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Code"});
 for(const name of [...Object.keys(table.codes.all),...Object.keys(table.codes.win32)])expect(code(name),name).toBe(true);
 for(const kind of Object.values({...table.codes.all,...table.codes.win32}))expect(schema.definitions.Kind.enum).toContain(kind);
 expect(Object.keys(table.codes.all).filter(name=>Object.hasOwn(table.codes.win32,name))).toEqual([]);
 expect(new Set(table.readSyscalls).size).toBe(table.readSyscalls.length);
 for(const reason of [...Object.values(table.reasons),table.persistentReason])expect(Object.keys(churn.definitions.Protocol.const.reasons.en)).toContain(reason);
 expect(Number.isInteger(table.witness.slackMilliseconds)&&table.witness.slackMilliseconds>=0).toBe(true);
 expect(table.ownedSegments.length).toBeGreaterThan(3);
});

test("every decision vector is answered as written and exactly like the independent Python oracle",()=>{
 expect(Object.keys(oracle.decide).sort()).toEqual(vectors.decide.map(row=>row.id).sort());
 for(const row of vectors.decide){
  const decision=decideFilesystemRaceV1({...row.facts,syscall:row.facts.syscall??undefined,root:row.facts.root??undefined,changeMs:row.facts.changeMs??undefined} as unknown as RaceFactsV1);
  expect({disposition:decision.disposition,reason:decision.reason??null,cause:decision.cause},row.id).toEqual(row.expected as never);
  expect({disposition:decision.disposition,reason:decision.reason??null,cause:decision.cause},row.id).toEqual(oracle.decide[row.id] as never);
 }
 expect(vectors.decide.length).toBeGreaterThan(60);
});

test("the probe and the witness of a change are read from the world exactly like the oracle, for present, missing, shadowed and unreadable paths",()=>{
 expect(Object.keys(oracle.observe).sort()).toEqual(vectors.observe.map(row=>row.id).sort());
 for(const row of vectors.observe){
  const seen=observeRaceWorldV1(row.path,worldOf(row));
  expect({probe:seen.probe,changeMs:seen.changeMs??null},row.id).toEqual(row.expected as never);
  expect({probe:seen.probe,changeMs:seen.changeMs??null},row.id).toEqual(oracle.observe[row.id] as never);
 }
});

test("the printed form of a filesystem error yields the same code, syscall and first absolute path as the oracle, and a text without them is no error",()=>{
 expect(Object.keys(oracle.text).sort()).toEqual(vectors.text.map(row=>row.id).sort());
 for(const row of vectors.text){
  expect(filesystemErrorFromTextV1(row.text,row.platform)??null,row.id).toEqual(row.expected as never);
  expect(filesystemErrorFromTextV1(row.text,row.platform)??null,row.id).toEqual(oracle.text[row.id] as never);
 }
 const old=processRaceWorldV1();
 expect(filesystemRaceOfTextV1("no error here",old).disposition).toBe("none");
 expect(filesystemRaceOfTextV1("ENOENT: no such file or directory, lstat 'relative/x'",old).cause).toBe("shape");
});

test("the operating system raises the errno names the table assumes for every race of a read, as the Python os module does",()=>{
 const base=join(sandbox,"errno");mkdirSync(base);
 const file=join(base,"f"),directory=join(base,"d"),moved=join(base,"moved");writeFileSync(file,"x");mkdirSync(directory);
 const code=(action:()=>unknown):string|null=>{try{action();return null;}catch(error){return (error as NodeJS.ErrnoException).code??null;}};
 const answer:Record<string,string|null>={
  "lstat-missing":code(()=>lstatSync(join(base,"missing"))),
  "lstat-under-file":code(()=>lstatSync(join(file,"x"))),
  "open-missing-parent":code(()=>require("node:fs").readFileSync(join(base,"gone","x"))),
  "open-directory":code(()=>require("node:fs").readFileSync(directory)),
  "scandir-file":code(()=>require("node:fs").readdirSync(file)),
  "scandir-missing":code(()=>require("node:fs").readdirSync(join(base,"missing"))),
  "readlink-missing":code(()=>require("node:fs").readlinkSync(join(base,"missing"))),
 };
 symlinkSync(join(base,"loop"),join(base,"loop"));answer["open-symlink-loop"]=code(()=>require("node:fs").readFileSync(join(base,"loop")));
 renameSync(directory,moved);answer["lstat-after-directory-move"]=code(()=>lstatSync(join(directory,"x")));
 rmSync(file);mkdirSync(file);answer["open-file-replaced-by-directory"]=code(()=>require("node:fs").readFileSync(file));
 answer["lstat-below-moved-directory"]=code(()=>lstatSync(join(directory,"nope","deeper")));
 expect(answer).toEqual(oracle.errnos);
 for(const name of Object.values(answer))expect(name===null||Object.hasOwn(table.codes.all,name),String(name)).toBe(true);
});

test("on a real tree a deleted, renamed or replaced source is a race only because a directory around it changed after the process started",()=>{
 const target=join(root,"src","a.ts");
 const before=Date.now()-60_000,after=Date.now()+60_000;
 const vanished=errorOf(()=>require("node:fs").readFileSync(join(root,"src","gone.ts")));
 const early=filesystemRaceOfErrorV1(vanished,worldAt(before)),late=filesystemRaceOfErrorV1(vanished,worldAt(after));
 expect([early.disposition,early.reason,early.cause]).toEqual(["persistent","vanished","absent"]);
 expect([late.disposition,late.cause]).toEqual(["none","unchanged"]);
 renameSync(target,target+".moved");
 const renamed=errorOf(()=>require("node:fs").readFileSync(target));
 expect(filesystemRaceOfErrorV1(renamed,worldAt(before)).disposition).toBe("persistent");
 renameSync(target+".moved",target);
 expect(filesystemRaceOfErrorV1(renamed,worldAt(before)).disposition).toBe("transient");
 const second=errorOf(()=>require("node:fs").readFileSync(join(root,"src","moved-away.ts")));
 writeFileSync(join(root,"src","moved-away.ts"),"back");
 const returned=filesystemRaceOfErrorV1(second,worldAt(before));
 expect([returned.disposition,returned.reason,returned.cause]).toEqual(["transient","vanished","present"]);
 mkdirSync(join(root,"d"));const replaced=Object.assign(errorOf(()=>require("node:fs").readFileSync(join(root,"d"))),{path:join(root,"d")});
 expect(filesystemRaceOfErrorV1(replaced,worldAt(before)).reason).toBe("identity");
 const outside=errorOf(()=>require("node:fs").readFileSync(join(sandbox,"outside.txt")));
 expect(filesystemRaceOfErrorV1(outside,worldAt(before)).cause).toBe("outside");
 const owned=errorOf(()=>require("node:fs").readFileSync(join(root,".cache","x")));
 expect(filesystemRaceOfErrorV1(owned,worldAt(before)).cause).toBe("owned");
 const written=errorOf(()=>require("node:fs").renameSync(join(root,"src","nope"),join(root,"src","nope2")));
 expect(filesystemRaceOfErrorV1(written,worldAt(before)).cause).toBe("write");
 for(const value of [undefined,null,"text",42,new Error("plain"),Object.assign(new Error("x"),{code:"ENOENT"}),Object.assign(new Error("x"),{path:"/p"})])expect(filesystemRaceOfErrorV1(value,worldAt(before)).disposition).toBe("none");
});

test("the workspace root of the process comes from the environment when it names a workspace and otherwise from the root markers above the working directory",()=>{
 expect(processRaceWorldV1({SEMIO_WORKSPACE_ROOT:root}).root).toBe(root);
 const empty=join(sandbox,"empty");mkdirSync(empty);
 const saved=process.cwd();try{process.chdir(join(root,"src"));expect(processRaceWorldV1({}).root).toBe(root);process.chdir(empty);expect(processRaceWorldV1({SEMIO_WORKSPACE_ROOT:empty}).root).toBeUndefined();}finally{process.chdir(saved);}
 expect(dirname(root)).toBe(sandbox);
});
