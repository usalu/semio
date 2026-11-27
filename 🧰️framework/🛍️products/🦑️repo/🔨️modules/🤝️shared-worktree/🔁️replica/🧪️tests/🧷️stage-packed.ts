import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {chmodSync,existsSync,mkdtempSync,readFileSync,rmSync,copyFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {devNull,tmpdir} from "node:os";
import {inflateSync} from "node:zlib";
import * as packgate from "../🧷️packgate/🟦️.ts";
import fixture from "./🧫️fixtures/🧷️stage-packed.json" with {type:"json"};

const fixtureGit=(root:string)=>(args:string[],stdin?:Buffer)=>{
  const result=Bun.spawnSync(["git",...args],{cwd:root,env:{...process.env,GIT_CONFIG_GLOBAL:devNull,GIT_CONFIG_NOSYSTEM:"1"},stdin,stdout:"pipe",stderr:"pipe"});
  if(result.exitCode!==0)throw Error(result.stderr.toString());
  return result.stdout.toString().trim();
 };

test.skipIf(Bun.spawnSync(["git","--version"]).exitCode!==0)("exact staged blobs use proven fleet packs or materialize canonical loose objects",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"sgp-")),db=new Database(":memory:");
 const git=fixtureGit(root);
 try{
  git(["init","-q","--object-format=sha1"]);
  const directory=join(root,".git","objects","pack"),payloads=new Map(fixture.objects.map(row=>[row.id,Buffer.from(row.value.repeat(row.values))]));
  const loose=(oid:string)=>join(root,".git","objects",oid.slice(0,2),oid.slice(2)),oids=new Map<string,string>(),stems=new Map<string,string>();
  for(const [id,value] of payloads){
   const oid=git(["hash-object",...(id==="missing"?[]:["-w"]),"--stdin"],value);oids.set(id,oid);
   expect(createHash("sha1").update(Buffer.from("blob "+value.length+"\0")).update(value).digest("hex")).toBe(oid);
   if(id==="missing")continue;
   const stem="pack-"+git(["pack-objects",join(directory,"pack")],Buffer.from(oid+"\n"));stems.set(id,stem);
   rmSync(loose(oid));
  }
  const wrong=stems.get("wrongPair")!;
  for(const extension of ["idx","pack"])copyFileSync(join(directory,wrong+"."+extension),join(directory,"pack-local-"+wrong+"."+extension));
  chmodSync(join(directory,wrong+".idx"),0o644);
  copyFileSync(join(directory,stems.get("unshared")!+".idx"),join(directory,wrong+".idx"));
  const before=readdirSync(directory).sort(),live=new Set<string>();
  for(const id of ["covered","wrongPair","packWithoutIndex"]){
   const path=packgate.packDirectoryV1+stems.get(id)!;
   live.add(path+".pack");if(id!=="packWithoutIndex")live.add(path+".idx");
  }
  const expected=db.query("select json_extract(value,'$.id') id,json_extract(value,'$.outcome') outcome from json_each(?) order by key").all(JSON.stringify(fixture.rows)) as {id:string;outcome:string}[];
  const run=packgate.prepareStagePackedObjectsV1;
  expect(typeof run).toBe("function");
  const abort=new AbortController(),reason=Error("private object read cancelled");abort.abort(reason);
  await expect(run(root,[oids.get("unshared")!],path=>live.has(path),abort.signal)).rejects.toBe(reason);
  expect(existsSync(loose(oids.get("unshared")!))).toBe(false);
  const result=await run(root,fixture.request.map(id=>oids.get(id)!),path=>live.has(path));
  for(const outcome of ["covered","loose","pending"] as const)expect<unknown>(result[outcome]).toEqual(expected.filter(row=>row.outcome===outcome).map(row=>oids.get(row.id)));
  for(const row of expected){
   const oid=oids.get(row.id)!;
   expect(existsSync(loose(oid)),row.id).toBe(row.outcome==="loose");
   if(row.outcome==="loose"){
    const canonical=inflateSync(readFileSync(loose(oid))),data=payloads.get(row.id)!;
    expect(canonical).toEqual(Buffer.concat([Buffer.from("blob "+data.length+"\0"),data]));
   }
  }
  expect(readdirSync(directory).sort()).toEqual(before);
  expect(existsSync(join(root,".git","semio-packs.json"))).toBe(false);
 }finally{db.close();rmSync(root,{recursive:true,force:true});}
},20000);

test.skipIf(Bun.spawnSync(["git","--version"]).exitCode!==0)("SHA-256 packed stages recover blobs and leave malformed or non-blob IDs pending",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"sgp-"));
 const git=fixtureGit(root);
 try{
  git(["init","-q","--object-format=sha256"]);
  const data=Buffer.from("sha256 exact packed payload\n"),oid=git(["hash-object","-w","--stdin"],data),tree=git(["mktree"],Buffer.alloc(0));
  const loose=(name:string)=>join(root,".git","objects",name.slice(0,2),name.slice(2));
  git(["pack-objects",join(root,".git","objects","pack","pack")],Buffer.from(tree+"\n"+oid+"\n"));
  rmSync(loose(oid));rmSync(loose(tree));
  const object=Buffer.concat([Buffer.from("blob "+data.length+"\0"),data]);
  expect(createHash("sha256").update(object).digest("hex")).toBe(oid);
  expect(await packgate.prepareStagePackedObjectsV1(root,[tree,oid,"../invalid"],()=>true)).toEqual({covered:[],loose:[oid],pending:[tree,"../invalid"]});
  expect(inflateSync(readFileSync(loose(oid)))).toEqual(object);
  expect(existsSync(loose(tree))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
},20000);
