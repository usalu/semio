import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {randomBytes} from "node:crypto";
import {consolidateGitV1,ConsolidationFaultV1} from "../🟦️.ts";

const scratch:string[]=[],hasGit=spawnSync("git",["--version"]).status===0,gitTest=hasGit?test:test.skip;
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const git=(cwd:string,args:readonly string[]):string=>{
 const result=spawnSync("git",[...args],{cwd,encoding:"utf8",maxBuffer:1<<28,env:{...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.org",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.org",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_SYSTEM:"/dev/null"}});
 if(result.status!==0)throw Error(`git ${args.join(" ")}: ${result.stderr}`);
 return result.stdout;
};
const objects=(root:string):string[]=>git(root,["cat-file","--batch-all-objects","--batch-check=%(objectname)"]).split("\n").filter(Boolean).sort();
const packs=(directory:string):string[]=>readdirSync(directory).filter(name=>name.endsWith(".pack")).sort();

function repository(big=false):{root:string;directory:string}{
 const root=mkdtempSync(join(tmpdir(),"consolidate-"));
 scratch.push(root);
 git(root,["init","-q"]);git(root,["config","gc.auto","0"]);git(root,["config","maintenance.auto","false"]);
 const directory=join(root,".git","objects","pack"),commit=(name:string,data?:Buffer)=>{writeFileSync(join(root,name),data??`${name}\n`.repeat(3000)+randomBytes(64).toString("hex"));git(root,["add","-A"]);git(root,["commit","-q","-m",name]);};
 commit("a",big?randomBytes(70*2**20):undefined);git(root,["repack","-q","-d"]);
 commit("b");git(root,["repack","-q","-d"]);
 const held=new Map(readdirSync(directory).map(name=>[name,readFileSync(join(directory,name))] as const));
 commit("c");git(root,["repack","-q","-a","-d"]);
 for(const [name,data] of held)writeFileSync(join(directory,name),data);
 for(const name of readdirSync(directory))chmodSync(join(directory,name),0o444);
 git(root,["multi-pack-index","write"]);
 return {root,directory};
}

const quick={waitMilliseconds:0,pollMilliseconds:1,needBytes:1,space:async()=>undefined};

gitTest("a dry run changes nothing and reports the waste",async()=>{
 const {root,directory}=repository(),before=readdirSync(directory).sort();
 const plan=await consolidateGitV1({worktree:root,apply:false});
 expect(plan.outcome).toBe("planned");
 expect(plan.retired).toBeGreaterThanOrEqual(2);
 expect(readdirSync(directory).sort()).toEqual(before);
});

gitTest("a run leaves one pack that holds exactly the objects of before, git fsck accepts it, HEAD resolves and no staging file, multi-pack-index or lock remains",async()=>{
 const {root,directory}=repository(),names=objects(root),head=git(root,["rev-parse","HEAD"]);
 const published:string[]=[];
 const result=await consolidateGitV1({worktree:root,apply:true,...quick,published:async path=>{published.push(path);return true;}});
 expect(result.outcome).toBe("consolidated");
 expect(result.retired).toBeGreaterThanOrEqual(3);
 expect(packs(directory).length).toBe(1);
 expect(packs(directory)[0]).toBe(`${result.pack}.pack`);
 expect(published).toEqual([`.git/objects/pack/${result.pack}.pack`,`.git/objects/pack/${result.pack}.idx`]);
 expect(readdirSync(directory).some(name=>name.startsWith("tmp_")||name.startsWith("multi-pack-index"))).toBe(false);
 expect(existsSync(join(root,".git","semio-consolidate.lock"))).toBe(false);
 git(root,["fsck","--full","--strict"]);
 expect(git(root,["rev-parse","HEAD"])).toBe(head);
 expect(objects(root)).toEqual(names);
 expect(result.bytesFreed).toBeGreaterThan(0);
 expect(result.objects).toBe(names.length);
 expect((await consolidateGitV1({worktree:root,apply:true,...quick})).outcome).toBe("nothing");
});

for(const point of ["pack-written","placed","published","retire:1","retire:2"])
gitTest(`a crash at ${point} leaves a repository git fsck accepts and with all objects, and the next run converges to one pack`,async()=>{
 const {root,directory}=repository(),names=objects(root);
 await expect(consolidateGitV1({worktree:root,apply:true,...quick,faults:point})).rejects.toBeInstanceOf(ConsolidationFaultV1);
 git(root,["fsck","--full"]);
 expect(objects(root)).toEqual(names);
 if(point==="pack-written")expect(readdirSync(directory).some(name=>name.startsWith("tmp_consolidate_"))).toBe(true);
 const result=await consolidateGitV1({worktree:root,apply:true,...quick,minOrphanAgeMilliseconds:0});
 expect(["consolidated","nothing"]).toContain(result.outcome);
 expect(packs(directory).length).toBe(1);
 expect(readdirSync(directory).filter(name=>!/^pack-[0-9a-f]{40}\.(pack|idx|rev|keep)$/u.test(name))).toEqual([]);
 git(root,["fsck","--full","--strict"]);
 expect(objects(root)).toEqual(names);
});

gitTest("a new pack that stays unpublished retires nothing",async()=>{
 const {root,directory}=repository(),before=packs(directory),names=objects(root);
 const result=await consolidateGitV1({worktree:root,apply:true,...quick,waitMilliseconds:30,published:async()=>false});
 expect(result.outcome).toBe("unpublished");
 expect(result.retired).toBe(0);
 expect(packs(directory)).toEqual(expect.arrayContaining(before));
 expect(packs(directory).length).toBe(before.length+1);
 git(root,["fsck","--full"]);
 expect(objects(root)).toEqual(names);
});

gitTest("not enough free space refuses before anything is written",async()=>{
 const {root,directory}=repository(),before=readdirSync(directory).sort(),events:string[]=[];
 const result=await consolidateGitV1({worktree:root,apply:true,...quick,space:async()=>({free:1000,total:10**9}),needBytes:10**6,sink:event=>events.push(`${event.event}:${event.reason}`)});
 expect(result.outcome).toBe("space");
 expect(events).toContain("skipped:space");
 expect(readdirSync(directory).sort()).toEqual(before);
});

gitTest("read-only packs of 64 MiB and more are retired, an old read-only orphan and an old read-only tmp pack of that size are removed, and the repository stays whole",async()=>{
 const {root,directory}=repository(true),names=objects(root);
 const big=packs(directory).filter(name=>statSync(join(directory,name)).size>=64*2**20);
 expect(big.length).toBeGreaterThanOrEqual(1);
 expect(statSync(join(directory,big[0]!)).mode&0o222).toBe(0);
 const hugeOrphan=join(directory,"pack-1111111111111111111111111111111111111111.pack"),hugeTmp=join(directory,"tmp_pack_Zz9Zz9");
 writeFileSync(hugeOrphan,randomBytes(70*2**20));writeFileSync(hugeTmp,randomBytes(70*2**20));
 chmodSync(hugeOrphan,0o444);chmodSync(hugeTmp,0o444);
 const old=new Date(Date.now()-3*3_600_000);
 utimesSync(hugeOrphan,old,old);utimesSync(hugeTmp,old,old);
 const result=await consolidateGitV1({worktree:root,apply:true,...quick,now:()=>Date.now()+3*3_600_000});
 expect(result.outcome).toBe("consolidated");
 expect(existsSync(hugeOrphan)||existsSync(hugeTmp)).toBe(false);
 for(const name of big)expect(existsSync(join(directory,name))).toBe(false);
 expect(packs(directory).length).toBe(1);
 git(root,["fsck","--full"]);
 expect(objects(root)).toEqual(names);
});

gitTest("a young orphan stays, a lock of a living process refuses and the lock of a dead process is taken over",async()=>{
 const {root,directory}=repository();
 writeFileSync(join(directory,"pack-2222222222222222222222222222222222222222.pack"),"half");
 const lock=join(root,".git","semio-consolidate.lock");
 mkdirSync(lock);writeFileSync(join(lock,"pid"),String(process.pid));
 expect((await consolidateGitV1({worktree:root,apply:true,...quick})).outcome).toBe("locked");
  rmSync(lock,{recursive:true});mkdirSync(lock);writeFileSync(join(lock,"pid"),String(2**22+12345));
 const result=await consolidateGitV1({worktree:root,apply:true,...quick});
 expect(result.outcome).toBe("consolidated");
 expect(existsSync(join(directory,"pack-2222222222222222222222222222222222222222.pack"))).toBe(true);
 git(root,["fsck","--full"]);
});
