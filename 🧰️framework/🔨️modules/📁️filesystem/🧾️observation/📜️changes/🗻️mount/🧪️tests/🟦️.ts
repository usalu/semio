/** 🗻️ Contract: a change journal may only vouch for a workspace whose file system reports every edit to the machine that runs the watcher. Network shares, virtual and FUSE file systems of container and VM hosts (`fakeowner`, `virtiofs`, `9p`, `grpcfuse`, `fuse.*`, `nfs*`, `cifs`, `smb*`) and Windows network drives receive edits from the other side without a watcher event; there the client refuses with `remote-file-system` (EN/DE) and every consumer hashes. The decision is checked on mount tables as the kernel and the OS print them against an independent Python model, and on the real host against `df`. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import {renderChangeRefusalV1} from "../../🟦️.ts";
import {createChangeJournalClientV1} from "../../🧭️client/🟦️.ts";
import {openVouchV1} from "../../🪪️vouch/🟦️.ts";
import {CHANGE_FILE_SYSTEM_POLICY_V1,changeFileSystemVerdictV1,darwinMountOfV1,inspectChangeFileSystemV1,linuxMountOfV1,type ChangeFileSystemFactsV1,type ChangeFileSystemPortsV1} from "../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),python=(()=>{try{return spawnSync("python3",["-I","-c","print(1)"]).status===0?"python3":"python";}catch{return"python";}})();
const vectors=corpus.vectors as (ChangeFileSystemFactsV1&{name:string;expect:{type:string;mount:string}|null})[];
const factsOf=(vector:(typeof vectors)[number]):ChangeFileSystemFactsV1=>{const {name:_name,expect:_expect,...facts}=vector;return facts;};

test("the declared type rules are closed under their schema",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({...schema,$ref:"#/definitions/Policy"} as object);
 expect(CHANGE_FILE_SYSTEM_POLICY_V1.version).toBe(1);
 const rule=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/TypeRule"} as object);
 expect(rule(CHANGE_FILE_SYSTEM_POLICY_V1.linux)).toBe(true);expect(rule(CHANGE_FILE_SYSTEM_POLICY_V1.darwin)).toBe(true);
 expect(validate(schema.definitions.Policy.const)).toBe(true);
 const verdict=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Verdict"} as object);
 for(const vector of vectors)if(vector.expect)expect(verdict(vector.expect),vector.name).toBe(true);
 for(const name of["fakeowner","virtiofs","9p","grpcfuse","fuse.sshfs","nfs4","cifs","smb3","drvfs"])expect(changeFileSystemVerdictV1({platform:"linux",root:"/m/x",mountinfo:`1 0 0:1 / /m rw - ${name} s rw\n`})?.type).toBe(name);
});

test("every mount table vector is decided as the independent Python model decides it",()=>{
 for(const vector of vectors)expect(changeFileSystemVerdictV1(factsOf(vector))??null,vector.name).toEqual(vector.expect);
 const run=spawnSync(python,["-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"../🧬️schema/🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(vectors.map(vector=>vector.expect));
 expect(vectors.filter(vector=>vector.expect!==null).length).toBeGreaterThan(10);expect(vectors.filter(vector=>vector.expect===null).length).toBeGreaterThan(8);
});

test("the host is asked for the facts of its platform only, a failing question is an unknown and a verdict names the refused mount",async()=>{
 const asked:string[]=[];
 const ports=(platform:string,answers:Record<string,string|undefined>,fail=false):ChangeFileSystemPortsV1=>({platform,readText:async path=>{asked.push("read "+path);if(fail)throw Error("denied");return answers[path];},run:async(program,args)=>{asked.push("run "+program+" "+args.length);if(fail)throw Error("denied");return answers[program];}});
 const virtio="1 0 0:1 / / rw - overlay o rw\n2 1 0:9 / /workspaces/semio rw - virtiofs m rw\n";
 expect(await inspectChangeFileSystemV1("/workspaces/semio/a",ports("linux",{"/proc/self/mountinfo":virtio}))).toEqual({type:"virtiofs",mount:"/workspaces/semio"});
 expect(await inspectChangeFileSystemV1("/workspaces/semio/a",ports("linux",{},true))).toBeUndefined();
 expect(await inspectChangeFileSystemV1("/Volumes/s/a",ports("darwin",{"/sbin/mount":"//u@h/s on /Volumes/s (smbfs, nodev)\n"}))).toEqual({type:"smbfs",mount:"/Volumes/s"});
 expect(await inspectChangeFileSystemV1("/Volumes/s/a",ports("darwin",{},true))).toBeUndefined();
 expect(await inspectChangeFileSystemV1("Z:\\repo",ports("win32",{"powershell.exe":"4\r\n"}))).toEqual({type:"network-drive",mount:"Z:"});
 expect(await inspectChangeFileSystemV1("C:\\repo",ports("win32",{"powershell.exe":"3\r\n"}))).toBeUndefined();
 expect(await inspectChangeFileSystemV1("\\\\h\\s\\repo",ports("win32",{}))).toEqual({type:"unc",mount:"\\\\h\\s"});
 expect(await inspectChangeFileSystemV1("/x",ports("freebsd",{}))).toBeUndefined();
 expect(asked).toContain("read /proc/self/mountinfo");expect(asked).toContain("run /sbin/mount 0");expect(asked.filter(line=>line.startsWith("run powershell")).length).toBe(2);
});

test("the client refuses every call on a remote file system without starting a service, in both languages, and a vouch on it proves nothing",async()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"mount-")));
 try{
  let started=0,asked=0;
  const client=createChangeJournalClientV1({directory,root:directory,start:async()=>{started++;},fileSystem:async()=>{asked++;return {type:"virtiofs",mount:"/workspaces/semio"};}});
  for(const answer of[await client.cursor(),await client.since({epoch:"0123456789abcdef",seq:1},[""])]){
   expect(answer).toMatchObject({exact:false,reason:"remote-file-system",path:"/workspaces/semio",detail:"virtiofs"});
   if(!("reason" in answer))throw Error("refusal expected");
   expect(renderChangeRefusalV1(answer,"en")).toBe("Cannot vouch: /workspaces/semio lies on a virtiofs file system that does not report edits made on its other side; the tree is hashed.");
   expect(renderChangeRefusalV1(answer,"de")).toContain("Kann nicht einstehen: /workspaces/semio liegt auf einem Dateisystem vom Typ virtiofs");
  }
  expect(started).toBe(0);expect(asked).toBe(1);
  const vouch=await openVouchV1({client,directory:join(directory,"vouch"),consumer:"mount-law",key:"k"});
  expect(vouch.mode).toBe("walk");expect(vouch.refusal).toBe("remote-file-system");expect(vouch.skippable(["a","b"]).size).toBe(0);
  expect((await vouch.settled(["a"])).size).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a question that fails or a local file system leaves the client as it was",async()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"mount-")));
 try{
  for(const fileSystem of[async()=>undefined,async()=>{throw Error("no answer");}]){
   let started=0;
   const client=createChangeJournalClientV1({directory,root:directory,start:async()=>{started++;},fileSystem});
   expect(await client.cursor()).toMatchObject({exact:false,reason:"no-service"});expect(started).toBe(1);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const df=(path:string):string|undefined=>{
 const run=spawnSync("df",["-P",path],{encoding:"utf8"});if(run.status!==0)return undefined;
 const row=run.stdout.trim().split("\n")[1]??"",match=/^\S+\s+\d+\s+\d+\s+\d+\s+\d+%\s+(.*)$/u.exec(row);return match?.[1];
};
test("on the real host the mount found in the mount table is the one `df` reports, and a temporary directory is a local file system",async()=>{
 const platform=process.platform;
 if(platform!=="linux"&&platform!=="darwin")return;
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"mount-")));
 try{
  const real:ChangeFileSystemPortsV1={platform,readText:async path=>{const run=spawnSync("cat",[path],{encoding:"utf8"});return run.status===0?run.stdout:undefined;},run:async(program,args)=>{const run=spawnSync(program,[...args],{encoding:"utf8"});return run.status===0?run.stdout:undefined;}};
  const table=platform==="linux"?(await real.readText("/proc/self/mountinfo"))!:(await real.run("/sbin/mount",[]))!;
  const found=platform==="linux"?linuxMountOfV1(table,directory):darwinMountOfV1(table,directory);
  const plain=(path?:string):string|undefined=>path===undefined?undefined:path==="/System/Volumes/Data"?"/":path.startsWith("/System/Volumes/Data/")?path.slice("/System/Volumes/Data".length):path;
  expect(plain(found?.mount)).toBe(plain(df(directory)));
  expect(await inspectChangeFileSystemV1(directory,real)).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
