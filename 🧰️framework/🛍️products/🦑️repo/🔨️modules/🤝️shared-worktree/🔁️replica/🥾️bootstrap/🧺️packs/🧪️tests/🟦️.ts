import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {packExcludedV1,planPacksV1,readPackNamesV1} from "../🟦️.ts";

const scratch:string[]=[],hasGit=spawnSync("git",["--version"]).status===0,gitTest=hasGit?test:test.skip;
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const git=(cwd:string,args:readonly string[],input?:Buffer):string=>{
 const result=spawnSync("git",[...args],{cwd,input,encoding:"utf8",maxBuffer:1<<28,env:{...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.org",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.org",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_SYSTEM:"/dev/null"}});
 if(result.status!==0)throw Error(`git ${args.join(" ")}: ${result.stderr}`);
 return result.stdout;
};

function repository():{root:string;directory:string;stems:string[]}{
 const root=mkdtempSync(join(tmpdir(),"packs-"));
 scratch.push(root);
 git(root,["init","-q"]);git(root,["config","gc.auto","0"]);git(root,["config","maintenance.auto","false"]);
 const directory=join(root,".git","objects","pack"),commit=(name:string)=>{writeFileSync(join(root,name),`${name}\n`.repeat(2000)+Math.random());git(root,["add","-A"]);git(root,["commit","-q","-m",name]);};
 commit("a");git(root,["repack","-q","-d"]);
 commit("b");git(root,["repack","-q","-d"]);
 const held=new Map(readdirSync(directory).map(name=>[name,readFileSync(join(directory,name))] as const));
 commit("c");git(root,["repack","-q","-a","-d"]);
 for(const [name,data] of held)writeFileSync(join(directory,name),data);
 return {root,directory,stems:[...new Set(readdirSync(directory).map(name=>name.replace(/\.[a-z]+$/u,"")))].sort()};
}

gitTest("the object names of an index equal git show-index and the plan keeps the one pack that holds every object, calling the two older packs covered",async()=>{
 const {directory,stems,root}=repository();
 expect(stems.length).toBe(3);
 const plan=await planPacksV1(directory);
 expect(plan.keep.length).toBe(1);
 expect([...plan.covered].sort()).toEqual(stems.filter(stem=>stem!==plan.keep[0]));
 for(const stem of stems){
  const table=await readPackNamesV1(join(directory,`${stem}.idx`));
  const oracle=git(root,["show-index"],readFileSync(join(directory,`${stem}.idx`))).split("\n").filter(Boolean).map(line=>line.split(" ")[1]!).sort();
  const ours:string[]=[];
  for(let index=0;index<table!.count;index++)ours.push(table!.names.subarray(index*20,index*20+20).toString("hex"));
  expect(ours).toEqual(oracle);
 }
 expect(plan.bytesDropped).toBeGreaterThan(0);
 expect(plan.objects).toBe(Number(git(root,["count-objects","-v"]).match(/in-pack: (\d+)/u)![1]!)-plan.covered.reduce((sum,stem)=>sum+git(root,["show-index"],readFileSync(join(directory,`${stem}.idx`))).split("\n").filter(Boolean).length,0));
});

gitTest("an unreadable index keeps its pack, a pack without index and an index without pack are orphans, tmp files and locks are garbage",async()=>{
 const {directory,stems}=repository(),plan0=await planPacksV1(directory);
 const covered=plan0.covered[0]!;
 writeFileSync(join(directory,`${covered}.idx`),"damaged");
 writeFileSync(join(directory,"pack-1111111111111111111111111111111111111111.pack"),"x");
 writeFileSync(join(directory,"pack-2222222222222222222222222222222222222222.idx"),"x");
 writeFileSync(join(directory,"tmp_pack_Ab12Cd"),"x");writeFileSync(join(directory,"multi-pack-index"),"x");writeFileSync(join(directory,"pack-3333.lock"),"x");
 const plan=await planPacksV1(directory);
 expect(plan.keep).toContain(covered);
 expect(plan.orphans).toEqual(["pack-1111111111111111111111111111111111111111","pack-2222222222222222222222222222222222222222"]);
 expect(plan.garbage).toEqual(expect.arrayContaining(["tmp_pack_Ab12Cd","multi-pack-index","pack-3333.lock"]));
 expect(stems.length).toBe(3);
});

test("the archive filter drops temporary files, locks, gc.log and every pack file outside the kept stems and nothing else",()=>{
 const plan={keep:["pack-aa"],covered:["pack-bb"],orphans:[],garbage:[],objects:1,bytesKept:1,bytesDropped:1};
 const base=".git/objects/pack/";
 for(const dropped of [`${base}pack-bb.pack`,`${base}pack-bb.idx`,`${base}multi-pack-index`,`${base}tmp_pack_X`,`${base}pack-aa.lock`,`${base}.tmp-1-pack-aa.pack`,".git/objects/ab/tmp_obj_Xy",".git/gc.log",".git/gc.pid",`${base}multi-pack-index-1.bitmap`])expect(packExcludedV1(dropped,plan),dropped).toBe(true);
 for(const kept of [`${base}pack-aa.pack`,`${base}pack-aa.idx`,`${base}pack-aa.rev`,`${base}pack-aa.mtimes`,`${base}pack-aa.keep`,".git/objects/ab/cdef",".git/HEAD",".git/packed-refs","src/gc.log",".git/objects/info/commit-graph"])expect(packExcludedV1(kept,plan),kept).toBe(false);
 expect(packExcludedV1(`${base}pack-zz.pack`,undefined)).toBe(false);
});
