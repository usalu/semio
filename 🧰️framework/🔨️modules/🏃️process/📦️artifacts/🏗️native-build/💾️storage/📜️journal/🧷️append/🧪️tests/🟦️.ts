import{test,expect}from"bun:test";
import Ajv from"ajv";
import{appendFileSync,chmodSync,mkdtempSync,readFileSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{open}from"node:fs/promises";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import schema from"../🧬️schema/🔣️.json";
import{appendObservedChildFundingJournalV1,retireChildFundingJournalV1,childFundingJournalTextsV1,childFundingJournalAttemptsV1}from"../🟦️.ts";
const withDescriptor=async<T>(opener:()=>Promise<Awaited<ReturnType<typeof open>>>,body:(file:Awaited<ReturnType<typeof open>>)=>Promise<T>):Promise<T>=>{const file=await opener();try{return await body(file);}finally{await file.close();}};
const scratch=():string=>mkdtempSync(join(tmpdir(),"child-funding-journal-"));
const check=():void=>{};
const python=(path:string,suffix:string):string=>{const result=spawnSync("python3",["-I","-c","import sys;sys.stdout.write(open(sys.argv[1],'rb').read().decode()+sys.argv[2])",path,suffix],{encoding:"utf8"});if(result.status!==0)throw Error(result.stderr);return result.stdout;};

test("messages are closed bilingual schema DATA agreeing with an independent Ajv",()=>{
 const validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/Text"});
 for(const row of Object.values(schema.definitions.Messages.const))expect(validate(row)).toBe(true);
 expect(validate({en:"x"})).toBe(false);
 expect(childFundingJournalAttemptsV1).toBe(schema.definitions.Protocol.const.maximumAttempts);
 const texts=childFundingJournalTextsV1("changed",{path:"/j",attempts:4});
 expect(texts.en).toContain("/j");expect(texts.de).toContain("4 Versuche");
});

test("the plain append equals an independent Python append",async()=>{
 const directory=scratch();try{
  const path=join(directory,"storage-events.jsonl"),observed='{"a":1}\n',suffix='{"b":2}\n';writeFileSync(path,observed);
  const expected=python(path,suffix);
  expect(await appendObservedChildFundingJournalV1({path,observed,suffix,limit:1048576,check,withDescriptor})).toBe("appended");
  expect(readFileSync(path,"utf8")).toBe(expected);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("m12 sequence: a sibling appends between observing and the descriptor, the retirement observes again instead of failing the partition",async()=>{
 const directory=scratch();try{
  const path=join(directory,"storage-events.jsonl"),first='{"event":"committed","id":"graph-namespace"}\n',sibling='{"event":"retired","id":"graph-namespace"}\n';writeFileSync(path,first);
  let raced=false,observations=0;
  const racing=async<T>(opener:()=>Promise<Awaited<ReturnType<typeof open>>>,body:(file:Awaited<ReturnType<typeof open>>)=>Promise<T>):Promise<T>=>{if(!raced){raced=true;appendFileSync(path,sibling);}return withDescriptor(opener,body);};
  const retirement='{"event":"retired","id":"graph-capacity"}\n';
  const result=await retireChildFundingJournalV1({path,first,observe:async()=>{observations++;return readFileSync(path,"utf8");},suffixFor:()=>retirement,limit:1048576,check,withDescriptor:racing});
  expect(observations).toBe(1);
  expect(result.observed).toBe(first+sibling);
  expect(readFileSync(path,"utf8")).toBe(first+sibling+retirement);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a touched modification time, change time and mode no longer refuse the append, which the old stamp check did",async()=>{
 const directory=scratch();try{
  const path=join(directory,"storage-events.jsonl"),observed='{"a":1}\n',suffix='{"b":2}\n';writeFileSync(path,observed);
  const touching=async<T>(opener:()=>Promise<Awaited<ReturnType<typeof open>>>,body:(file:Awaited<ReturnType<typeof open>>)=>Promise<T>):Promise<T>=>{utimesSync(path,new Date(1e12),new Date(1e12));chmodSync(path,0o640);return withDescriptor(opener,body);};
  expect(await appendObservedChildFundingJournalV1({path,observed,suffix,limit:1048576,check,withDescriptor:touching})).toBe("appended");
  expect(readFileSync(path,"utf8")).toBe(observed+suffix);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a same-size rewrite and a replaced inode answer changed without writing",async()=>{
 const directory=scratch();try{
  const path=join(directory,"storage-events.jsonl"),observed='{"a":1}\n',suffix='{"b":2}\n';writeFileSync(path,'{"a":2}\n');
  expect(await appendObservedChildFundingJournalV1({path,observed,suffix,limit:1048576,check,withDescriptor})).toBe("changed");
  expect(readFileSync(path,"utf8")).toBe('{"a":2}\n');
  writeFileSync(path,observed);
  const replacing=async<T>(opener:()=>Promise<Awaited<ReturnType<typeof open>>>,body:(file:Awaited<ReturnType<typeof open>>)=>Promise<T>):Promise<T>=>{rmSync(path);writeFileSync(path,observed);return withDescriptor(opener,body);};
  expect(await appendObservedChildFundingJournalV1({path,observed,suffix,limit:1048576,check,withDescriptor:replacing})).toBe("changed");
  expect(readFileSync(path,"utf8")).toBe(observed);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a journal that never settles ends after the bounded attempts with a bilingual typed refusal",async()=>{
 const directory=scratch();try{
  const path=join(directory,"storage-events.jsonl");writeFileSync(path,"x\n");
  const hostile=async<T>(opener:()=>Promise<Awaited<ReturnType<typeof open>>>,body:(file:Awaited<ReturnType<typeof open>>)=>Promise<T>):Promise<T>=>{appendFileSync(path,"y\n");return withDescriptor(opener,body);};
  let observations=0;
  const error=await retireChildFundingJournalV1({path,first:"x\n",observe:async()=>{observations++;return readFileSync(path,"utf8");},suffixFor:()=>"z\n",limit:1048576,check,withDescriptor:hostile}).catch((value:unknown)=>value);
  expect(observations).toBe(childFundingJournalAttemptsV1-1);
  expect(error).toBeInstanceOf(Error);expect(Reflect.get(error as object,"kind")).toBe("changed");expect(Reflect.get(Reflect.get(error as object,"texts"),"de")).toContain("Versuche");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a hard link, a symlink and the byte bound are refused outright",async()=>{
 const directory=scratch();try{
  const path=join(directory,"storage-events.jsonl"),link=join(directory,"other");writeFileSync(path,"a\n");
  expect(await appendObservedChildFundingJournalV1({path,observed:"a\n",suffix:"b\n",limit:3,check,withDescriptor}).catch((e:Error)=>e.message)).toContain("bound exceeded");
  spawnSync("ln",[path,link]);
  expect(await appendObservedChildFundingJournalV1({path,observed:"a\n",suffix:"b\n",limit:1048576,check,withDescriptor}).catch((e:{kind:string})=>e.kind)).toBe("notOriginal");
 }finally{rmSync(directory,{recursive:true,force:true});}
});
