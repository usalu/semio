/** 🗞️ Contract: rows that the journal vouches for skip their probe and nothing else does; an unsound skip would seal a stale graph. */
import {expect,test} from "bun:test";
import {chmodSync,mkdirSync,mkdtempSync,rmSync,statSync,symlinkSync,writeFileSync,readdirSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {journalCleanRowsV1,journalMembershipRecordsV1,linkIndexV1,readJournalAttestationV1,writeJournalAttestationV1,type JournalAttestationV1,type JournalFeedV1,type JournalRowV1} from "../🟦️.ts";
import {buildGraphMembershipV1} from "../../../📁️workspace/🟦️.ts";

type ChangeV1=readonly [string,0|1,string];
const feedOf=(rows:readonly ChangeV1[]|"refuse",excluded:readonly string[]=[]):JournalFeedV1=>({
  excluder:()=>path=>excluded.some(name=>path===name||path.startsWith(name+"/")),
  cursor:async()=>({epoch:"e1",seq:10}),
  since:async()=>rows==="refuse"?{exact:false}:{exact:true,cursor:{epoch:"e1",seq:12},rows},
});
const memory=(initial?:JournalAttestationV1)=>{const box={value:initial};return {box,read:()=>box.value,write:(value:JournalAttestationV1)=>{box.value=value;}};};
const realRoot=(directories:string[])=>{const root=mkdtempSync(join(tmpdir(),"journal-root-"));for(const directory of directories)mkdirSync(join(root,directory),{recursive:true});return root;};
const rowsOf=(root:string,paths:Record<string,string>):JournalRowV1[]=>Object.entries(paths).map(([path,kind])=>({kind,path:root+"/"+path,fact:path.length}));

test("the clean set equals an independent CPython model over random change sets",async()=>{
  const root=realRoot(["a/b","c","d/e"]),kinds=["file","stat","exists","readdir","lstat","analysis","realpath"];
  let seed=7;const random=()=>(seed=(seed*1103515245+12345)%2147483648)/2147483648;
  const directories=["a","a/b","c","d/e","d"],paths:string[]=[];
  for(const directory of directories)for(let at=0;at<4;at++)paths.push(directory+"/f"+at);
  paths.push("top.txt","a","d");
  for(let round=0;round<30;round++){
    const rows:JournalRowV1[]=paths.map(path=>({kind:kinds[Math.floor(random()*kinds.length)],path:root+"/"+path,fact:1}));
    const changes:ChangeV1[]=[];
    for(let at=0;at<Math.floor(random()*4);at++)changes.push([["a","m","r","n"][Math.floor(random()*4)] as string,random()<0.5?1:0,paths[Math.floor(random()*paths.length)]]);
    const excluded=random()<0.3?["c"]:[],store=memory({version:1,memo:"m",epoch:"e1",seq:3});
    const journal=await journalCleanRowsV1({feed:feedOf(changes,excluded),root,read:store.read,write:store.write,insensitive:false},"m",rows);
    const actual=rows.map(row=>journal.clean(row));
    const model=spawnSync("python3",["-I","-c",`
import json,sys
d=json.load(sys.stdin);rows=d["rows"];ch=d["changes"];ex=d["excluded"];root=d["root"]+"/"
kinds={"file","exists","stat","lstat","fileExists","directoryExists","readdir","readdirNames"}
def touched(p):
    for k,dr,c in ch:
        if c==p or c.startswith(p+"/") or p.startswith(c+"/") and not (dr==1 and k=="m"): return True
    return False
def excluded(p): return any(p==e or p.startswith(e+"/") for e in ex)
print(json.dumps([r["kind"] in kinds and r["path"].startswith(root) and not touched(r["path"][len(root):]) and not excluded(r["path"][len(root):]) for r in rows]))
`],{input:JSON.stringify({rows,changes,excluded,root}),encoding:"utf8"});
    expect(model.status).toBe(0);
    expect(actual).toEqual(JSON.parse(model.stdout));
  }
  rmSync(root,{recursive:true,force:true});
});

test("no skipped row is stale on a real tree after real edits (ground truth is the file system)",async()=>{
  const root=mkdtempSync(join(tmpdir(),"journal-clean-"));
  try{
    mkdirSync(join(root,"s/inner"),{recursive:true});mkdirSync(join(root,"t"),{recursive:true});
    for(const name of ["s/a.txt","s/inner/b.txt","t/c.txt","t/d.txt"])writeFileSync(join(root,name),name);
    const stamp=(path:string)=>{try{return JSON.stringify([statSync(join(root,path)).size,statSync(join(root,path)).mtimeMs]);}catch{return "gone";}};
    const names=["s/a.txt","s/inner/b.txt","t/c.txt","t/d.txt","t/new.txt"];
    const rows:JournalRowV1[]=names.map(name=>({kind:"file",path:join(root,name),fact:stamp(name)}));
    writeFileSync(join(root,"s/a.txt"),"changed longer");writeFileSync(join(root,"t/new.txt"),"x");
    const store=memory({version:1,memo:"m",epoch:"e1",seq:3}),journal=await journalCleanRowsV1({feed:feedOf([["m",0,"s/a.txt"],["a",0,"t/new.txt"],["m",1,"t"]]),root,read:store.read,write:store.write},"m",rows);
    for(const row of rows){
      if(journal.clean(row))expect(stamp(row.path.slice(root.length+1))).toBe(row.fact as string);
    }
    expect(journal.clean(rows[1])).toBe(true);
    expect(journal.clean(rows[0])).toBe(false);
    expect(journal.clean(rows[4])).toBe(false);
    expect(journal.clean({...rows[1],fact:"other"})).toBe(false);
    expect(readdirSync(root).length).toBe(2);
  }finally{rmSync(root,{recursive:true,force:true});}
});

test("a refusal, a foreign memo, a missing attestation or a dead feed make every row probed",async()=>{
  const rows=rowsOf("/w",{"a/f":"file"}),store=()=>memory({version:1,memo:"m",epoch:"e1",seq:3});
  for(const [feed,memo,held] of [[feedOf("refuse"),"m",store()],[feedOf([]),"other",store()],[feedOf([]),"m",memory()],[feedOf([]),undefined,store()]] as const){
    const journal=await journalCleanRowsV1({feed,root:"/w",read:held.read,write:held.write,insensitive:false},memo,rows);
    expect(journal.clean(rows[0])).toBe(false);
  }
  const dead:JournalFeedV1={excluder:()=>()=>false,cursor:()=>new Promise(()=>undefined),since:async()=>({exact:false})},held=store();
  const journal=await journalCleanRowsV1({feed:dead,root:"/w",read:held.read,write:held.write,timeoutMilliseconds:30},"m",rows);
  expect(journal.clean(rows[0])).toBe(false);
  await journal.seal("m");expect(held.box.value?.seq).toBe(3);
});

test("a cursor taken before the probe is sealed and later questions start there; unchanged memo rows then all skip",async()=>{
  const root=realRoot(["a"]),rows=rowsOf(root,{"a/f":"file","a/g":"stat","b":"readdir"}),store=memory();
  const first=await journalCleanRowsV1({feed:feedOf([]),root,read:store.read,write:store.write,insensitive:false},"m",rows);
  expect(rows.map(row=>first.clean(row))).toEqual([false,false,false]);
  await first.seal("m");expect(store.box.value).toMatchObject({version:1,memo:"m",epoch:"e1",seq:10,links:{checked:["a"],linked:[]}});
  const second=await journalCleanRowsV1({feed:feedOf([]),root,read:store.read,write:store.write,insensitive:false},"m",rows);
  expect(rows.map(row=>second.clean(row))).toEqual([true,true,true]);
  expect(second.counts()).toEqual({skipped:3,checked:3,changed:0});
  rmSync(root,{recursive:true,force:true});
});

test("the attestation file round-trips atomically and foreign content is ignored",()=>{
  const directory=mkdtempSync(join(tmpdir(),"journal-attest-")),path=join(directory,"x.verified.json");
  try{
    expect(readJournalAttestationV1(path)).toBeUndefined();
    writeJournalAttestationV1(path,{version:1,memo:"m",epoch:"e",seq:4});
    expect(readJournalAttestationV1(path)).toEqual({version:1,memo:"m",epoch:"e",seq:4});
    expect(readdirSync(directory)).toEqual(["x.verified.json"]);
    writeFileSync(path,JSON.stringify({version:2,memo:"m",epoch:"e",seq:4}));expect(readJournalAttestationV1(path)).toBeUndefined();
    writeFileSync(path,"{");expect(readJournalAttestationV1(path)).toBeUndefined();
    expect(existsSync(path)).toBe(true);
  }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the observation hold skips exactly the rows its clean predicate vouches for",async()=>{
  const {observationsHold,observationsHoldWithinRoundV1}=await import("../../../../👁️observation/🟨️.mjs");
  const root=mkdtempSync(join(tmpdir(),"journal-hold-"));
  try{
    const rows=[{kind:"exists",path:join(root,"missing-a"),fact:true},{kind:"exists",path:join(root,"missing-b"),fact:true}] as never[];
    expect(await observationsHold(rows)).toBe(false);
    expect(await observationsHold(rows,64,new Set(),new Set(),undefined,undefined,undefined,false,()=>true)).toBe(true);
    const stale:string[]=[];
    expect(await observationsHold(rows,64,new Set(),new Set(),undefined,row=>{stale.push(row.path);},undefined,true,row=>row.path.endsWith("-a"))).toBe(false);
    expect(stale).toEqual([join(root,"missing-b")]);
    expect(await observationsHoldWithinRoundV1(rows,64,undefined,undefined,undefined,()=>true)).toBe(true);
  }finally{rmSync(root,{recursive:true,force:true});}
});

test("membership hash: vouched entries keep their record, changed ones are probed, the digest equals a fresh hash and an independent CPython hash",async()=>{
  const root=mkdtempSync(join(tmpdir(),"journal-member-"));
  try{
    mkdirSync(join(root,"p/q"),{recursive:true});
    const files=["a.txt","p/b.txt","p/q/c.txt","z.txt"];
    for(const name of files)writeFileSync(join(root,name),name);
    const file=join(root,"membership.json"),python=(names:string[])=>{
      const run=spawnSync("python3",["-I","-c","import hashlib,json,os,sys\nr=sys.argv[1];n=json.loads(sys.argv[2])\nprint(hashlib.sha256(\"\\n\".join(json.dumps([f,\"file\",os.lstat(os.path.join(r,f)).st_mode],separators=(\",\",\":\"),ensure_ascii=False) for f in sorted(n)).encode()).hexdigest())",root,JSON.stringify(names)],{encoding:"utf8"});
      expect(run.status).toBe(0);return run.stdout.trim();
    };
    const first=await journalMembershipRecordsV1({feed:feedOf([]),root,file,insensitive:false});
    const digest0=await buildGraphMembershipV1(root,files,undefined,new Set(),undefined,first);
    await first.done();expect(first.counts()).toEqual({reused:0,probed:4});
    expect(digest0).toBe(python(files));
    chmodSync(join(root,"p/b.txt"),0o600);
    const second=await journalMembershipRecordsV1({feed:feedOf([["m",0,"p/b.txt"]]),root,file,insensitive:false});
    const digest1=await buildGraphMembershipV1(root,files,undefined,new Set(),undefined,second);
    await second.done();
    expect(second.counts()).toEqual({reused:3,probed:1});
    expect(digest1).toBe(await buildGraphMembershipV1(root,files));
    expect(digest1).toBe(python(files));
    expect(digest1).not.toBe(digest0);
    const third=await journalMembershipRecordsV1({feed:feedOf([]),root,file,insensitive:false});
    expect(await buildGraphMembershipV1(root,files,undefined,new Set(),undefined,third)).toBe(digest1);
    expect(third.counts()).toEqual({reused:4,probed:0});
    const refused=await journalMembershipRecordsV1({feed:feedOf("refuse"),root,file,insensitive:false});
    await buildGraphMembershipV1(root,files,undefined,new Set(),undefined,refused);
    expect(refused.counts().reused).toBe(0);
  }finally{rmSync(root,{recursive:true,force:true});}
});

test("nothing below a symbolic-link directory is vouched for; probed directories are remembered until a change touches them",async()=>{
  const root=mkdtempSync(join(tmpdir(),"journal-link-"));
  try{
    mkdirSync(join(root,"real"));mkdirSync(join(root,"plain"));writeFileSync(join(root,"real/x"),"x");writeFileSync(join(root,"plain/y"),"y");
    symlinkSync(join(root,"real"),join(root,"alias"));
    const index=linkIndexV1(root,undefined);
    expect(await index.among(["alias/x","plain/y","real/x"])).toEqual(["alias"]);
    const state=index.state();
    expect([...state.checked].sort()).toEqual(["plain","real"]);expect(state.linked).toEqual(["alias"]);
    rmSync(join(root,"plain"),{recursive:true});symlinkSync(join(root,"real"),join(root,"plain"));
    const quiet=linkIndexV1(root,state,{rows:[],excluded:()=>false,insensitive:false});
    expect(await quiet.among(["plain/y"])).toEqual([]);
    const touched=linkIndexV1(root,state,{rows:[["a",0,"plain"]],excluded:()=>false,insensitive:false});
    expect(await touched.among(["plain/y"])).toEqual(["plain"]);
  }finally{rmSync(root,{recursive:true,force:true});}
});

test("a vouched module resolution row still equals a fresh resolution after random file edits (ground truth is a fresh process), and an edit beside a target un-vouches exactly its rows",async()=>{
  const {realpathSync,unlinkSync,mkdirSync:mkdir}=await import("node:fs"),{dirname}=await import("node:path");
  const root=realpathSync(mkdtempSync(join(tmpdir(),"journal-resolve-")));
  try{
    const files=["a/x.js","a/y.js","b/z.js","b/dir/index.js","b/dir/package.json","c/w.js","a/f.js","b/f.js","c/f.js"];
    for(const name of files){mkdir(join(root,dirname(name)),{recursive:true});writeFileSync(join(root,name),name==="b/dir/package.json"?"{}":"module.exports=1;");}
    const asks:[string,string][]=[["a/f.js","./x"],["a/f.js","./y.js"],["a/f.js","./gone"],["a/f.js","../b/z"],["b/f.js","./dir"],["b/f.js","./z"],["b/f.js","../c/w"],["c/f.js","./w"],["c/f.js","./later"],["c/f.js","../a/x"],["c/f.js","node:fs"],["a/f.js","left-pad"]];
    const resolver=join(root,"resolve.cjs");
    const resolveAll=()=>{
      writeFileSync(resolver,`const {createRequire}=require("node:module");const root=${JSON.stringify(root)};const out={};for(const [from,request] of ${JSON.stringify(asks)}){const key=JSON.stringify([root+"/"+from,request]);try{const found=createRequire(root+"/"+from).resolve(request);out[key]=found.startsWith("/")?require("node:fs").realpathSync(found):found;}catch(error){out[key]="!"+(error.code??"EUNKNOWN");}}process.stdout.write(JSON.stringify(out));`);
      const run=spawnSync(process.execPath,[resolver],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as Record<string,string>;
    };
    const recorded=resolveAll(),rows:JournalRowV1[]=Object.entries(recorded).map(([path,fact])=>({kind:"resolve",path,fact}));
    let seed=5;const random=(n:number)=>(seed=(seed*1103515245+12345)%2147483648)%n;
    const dirs=["a","b","b/dir","c"],edits:Array<()=>ChangeV1[]>=[
      ()=>{const dir=dirs[random(dirs.length)],name=["gone","later","x","z","w"][random(5)]+".js";writeFileSync(join(root,dir,name),"1");return [["a",0,dir+"/"+name],["m",1,dir]];},
      ()=>{const name=["a/x.js","a/y.js","b/z.js","c/w.js"][random(4)];try{unlinkSync(join(root,name));}catch{}return [["r",0,name],["m",1,dirname(name)]];},
      ()=>{writeFileSync(join(root,"b/z.js"),"module.exports=2;");return [["m",0,"b/z.js"]];},
      ()=>{writeFileSync(join(root,"a/unrelated.txt"),"u");return [["a",0,"a/unrelated.txt"],["m",1,"a"]];},
    ];
    let vouched=0,withdrawn=0;
    for(let round=0;round<8;round++){
      const changes:ChangeV1[]=[];for(let at=0;at<1+random(2);at++)changes.push(...edits[random(edits.length)]());
      const store=memory({version:1,memo:"m",epoch:"e1",seq:3}),journal=await journalCleanRowsV1({feed:feedOf(changes),root,read:store.read,write:store.write,insensitive:false},"m",rows),fresh=resolveAll();
      for(const row of rows)if(journal.clean(row)){vouched++;expect(fresh[row.path]===row.fact,row.path+" after "+JSON.stringify(changes)).toBe(true);}else if(fresh[row.path]!==row.fact)withdrawn++;
      rows.splice(0,rows.length,...Object.entries(fresh).map(([path,fact])=>({kind:"resolve",path,fact})));
    }
    expect(vouched).toBeGreaterThan(20);expect(withdrawn).toBeGreaterThan(0);
    const quiet=await journalCleanRowsV1({feed:feedOf([]),root,read:()=>({version:1,memo:"m",epoch:"e1",seq:3}),write:()=>undefined,insensitive:false},"m",rows);
    const bare=rows.filter(row=>JSON.parse(row.path)[1]==="left-pad"||JSON.parse(row.path)[1]==="node:fs");
    expect(bare.map(row=>quiet.clean(row))).toEqual(bare.map(row=>JSON.parse(row.path)[1]==="node:fs"));
    expect(rows.filter(row=>!bare.includes(row)).every(row=>quiet.clean(row))).toBe(true);
  }finally{rmSync(root,{recursive:true,force:true});}
});
