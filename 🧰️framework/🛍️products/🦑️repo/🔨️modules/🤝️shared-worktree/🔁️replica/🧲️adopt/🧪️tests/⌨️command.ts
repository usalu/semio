import {afterAll,expect,test} from "bun:test";
import {chmodSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,sep} from "node:path";
import replicaFixture from "../../🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};
import {writeAccessV1} from "../../../🔐️access/🟦️.ts";
import {runReplicaCommandV1,parseReplicaArgumentsV1} from "../../⌨️command/🟦️.ts";
import {closeReplicaV1,joinV1,openReplicaV1,publishOnceV1,seedV1,type ReplicaOptionsV1} from "../../🟦️.ts";

const scratch:string[]=[],fast={pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`adopt-cli-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const environment={...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.test",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.test",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"};
const git=(directory:string,...args:string[]):string=>{
 const result=Bun.spawnSync(["git",...args],{cwd:directory,env:environment,stdout:"pipe",stderr:"pipe"});
 if(result.exitCode!==0)throw Error(`git ${args.join(" ")}: ${result.stderr.toString()}`);
 return result.stdout.toString();
};
function write(directory:string,path:string,text:string):void{const file=join(directory,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);}
async function command(worktree:string,args:string[],language:"en"|"de"="en"){
 const lines:string[]=[],notices:string[]=[],result=await runReplicaCommandV1([...args,"--lang",language],{root:worktree,environment:{},signal:new AbortController().signal,write:line=>lines.push(line),notice:line=>notices.push(line)});
 return {code:result.exitCode,lines,notices,text:notices.join("\n")};
}
async function build(){
 const store=temp("store"),server=temp("server"),adopter=temp("adopter"),options:ReplicaOptionsV1={worktree:server,storeRoot:store,originId:"g1-server",groupId:"g1",...fast};
 write(server,".gitignore",replicaFixture.gitignore);write(server,"a.txt","alpha\n");write(server,"b.txt","bravo\n");write(server,"\u00fcber.txt","umlaut\n");write(server,"tool.sh","#!/bin/sh\n");
 git(server,"init","-q","-b","main");git(server,"add","-A");git(server,"commit","-q","-m","initial");
 await seedV1(options);
 cpSync(server,adopter,{recursive:true,filter:source=>!source.slice(server.length).includes(`${sep}.cache`)});
 write(server,"b.txt","bravo fleet\n");
 const replica=await openReplicaV1(options);
 try{await publishOnceV1(replica);}finally{await closeReplicaV1(replica);}
 write(adopter,"b.txt","bravo local, much longer\n");write(adopter,"only-here.txt","x\n");
 return {store,server,adopter};
}

test("join takes --adopt, --dry-run, --force and --sample, and the relocate command takes its own closed options",()=>{
 expect(parseReplicaArgumentsV1(["join","--adopt","--dry-run","--force","--sample","5"])).toMatchObject({operation:"join"});
 expect(parseReplicaArgumentsV1(["relocate","--to","/x","--phase","move","--wait","30","--dry-run"]).values).toEqual({"--to":"/x","--phase":"move","--wait":"30"});
 expect(()=>parseReplicaArgumentsV1(["relocate","--adopt"])).toThrow();
 expect(()=>parseReplicaArgumentsV1(["join","--phase","move"])).toThrow();
});

posixTest("join --adopt --dry-run prints the plan in English and German and changes nothing, join --adopt adopts and reports, and the adopt options are refused without --adopt",async()=>{
 const {store,adopter}=await build(),base=["--worktree",adopter,"--store",store,"--group","g1","--origin","g1-m02"];
 const dry=await command(adopter,["join","--adopt","--dry-run","--sample","2",...base]);
 expect(dry.code).toBe(0);expect(dry.text).toContain("Dry run: planning the adoption");expect(dry.text).toContain("only here, published as new work");expect(dry.text).toContain("only-here.txt");expect(dry.text).toContain("b.txt");expect(dry.text).toContain("b.txt: changed here and in the fleet since this repository's base");expect(dry.text).toContain("both sides changed it since this repository's base");expect(dry.text).toContain("Conflicts: 1. Stop and report if there are more than 50");expect(dry.text).toContain("Evidence:");expect(dry.text).toContain("New local work: 1 paths");expect(dry.text).toContain("Git: fleet tip");expect(dry.text).toContain("Dry run finished: nothing was changed.");
 expect(existsSync(join(adopter,".cache"))).toBe(false);
 const german=await command(adopter,["join","--adopt","--dry-run",...base],"de");
 expect(german.text).toContain("Probelauf: plane die Übernahme");expect(german.text).toContain("Konflikte: 1. Bei mehr als 50 anhalten");expect(german.text).toContain("beide Seiten haben es seit der Basis dieses Repositorys geändert");expect(german.text).toContain("Probelauf beendet: nichts wurde verändert.");
 await expect(command(adopter,["join","--dry-run",...base])).rejects.toThrow("belong to join --adopt");
 await expect(command(adopter,["join","--force",...base])).rejects.toThrow("--dry-run, --force and --sample belong to join --adopt");
 const real=await command(adopter,["join","--adopt",...base]);
 expect(real.code).toBe(0);expect(real.text).toContain("Adopted:");expect(real.text).toContain("1 conflicts");expect(readFileSync(join(adopter,"b.txt"),"utf8")).toBe("bravo fleet\n");expect(real.text).toContain(".cache/f/lost/adopt-");
 expect(real.lines.some(line=>line.includes('"stage":"adopt-plan"'))).toBe(true);
 const again=await command(adopter,["join","--adopt",...base]);
 expect(again.code).toBe(0);
 const status=await command(adopter,["status",...base.slice(0,4)]);
 expect(status.code).toBe(0);
},120_000);

posixTest("join --adopt --dry-run names the decomposed names it reads as NFC and the paths that differ only in the executable bit, in English and German, and join --adopt renames and decides them",async()=>{
 const {store,adopter}=await build(),base=["--worktree",adopter,"--store",store,"--group","g1","--origin","g1-m02"],composed="über.txt";
 rmSync(join(adopter,composed));write(adopter,composed.normalize("NFD"),"umlaut\n");chmodSync(join(adopter,"tool.sh"),0o755);
 if(!readdirSync(adopter).includes(composed.normalize("NFD")))return;
 const dry=await command(adopter,["join","--adopt","--dry-run",...base]);
 expect(dry.code).toBe(0);expect(dry.text).toContain("Names that are not NFC (macOS tar extracts them like that): 1. The adoption renames them to NFC before it plans.");expect(dry.text).toContain(`renamed to NFC: ${composed}`);expect(dry.text).toContain("tool.sh: edited here on top of the fleet's version");expect(dry.text).toContain("(same bytes, only the executable bit differs)");
 expect(readdirSync(adopter)).toContain(composed.normalize("NFD"));
 const german=await command(adopter,["join","--adopt","--dry-run",...base],"de");
 expect(german.text).toContain("Namen, die nicht NFC sind (macOS tar entpackt sie so): 1. Die Übernahme benennt sie vor dem Planen in NFC um.");expect(german.text).toContain(`in NFC umbenannt: ${composed}`);expect(german.text).toContain("(gleiche Bytes, nur das Ausführbar-Bit weicht ab)");
 const real=await command(adopter,["join","--adopt",...base]);
 expect(real.code).toBe(0);expect(real.text).toContain(`renamed to NFC: ${composed}`);
 expect(readdirSync(adopter)).toContain(composed);expect(readdirSync(adopter)).not.toContain(composed.normalize("NFD"));
},120_000);

posixTest("relocate --dry-run prints the plan, relocate runs its phases and names every refusal in the chosen language",async()=>{
 const {store,server,adopter}=await build(),old=temp("old");
 await joinV1({worktree:old,storeRoot:store,originId:"g1-m01",groupId:"g1",...fast});
 const target=join(adopter),args=["--worktree",old,"--to",target];
 const dry=await command(old,["relocate","--dry-run",...args]);
 expect(dry.code).toBe(0);expect(dry.text).toContain("Relocating this machine");expect(dry.text).toContain("Dry run finished");
 expect(existsSync(join(target,".cache"))).toBe(false);
 expect((await command(old,["relocate","--to",old,"--worktree",old])).text).toContain("The new worktree is the old one.");
 expect((await command(old,["relocate","--to",old,"--worktree",old],"de")).text).toContain("Das neue Arbeitsverzeichnis ist das alte.");
 expect((await command(old,["relocate","--phase","finish",...args])).text).toContain("needs the earlier phases first");
 const running=await openReplicaV1({worktree:old,storeRoot:store,originId:"g1-m01",groupId:"g1",...fast});
 try{
  expect((await command(old,["relocate","--phase","prepare",...args])).code).toBe(0);
  expect((await command(old,["relocate","--phase","move",...args])).text).toContain("still running");
 }finally{await closeReplicaV1(running);}
 expect((await command(old,["relocate","--phase","move",...args])).code).toBe(0);
 const done=await command(old,["relocate","--phase","finish",...args]);
  expect(done.code).toBe(0);expect(done.text).toContain("Relocation step reached: finished");expect(done.text).toContain("verified");
 void server;
},120_000);

posixTest("status and fleet say worktree-access-denied with the exact consent hint while the replica of an origin waits for the operating system",async()=>{
 const {store,adopter}=await build(),base=["--worktree",adopter,"--store",store,"--group","g1","--origin","g1-m02"];
 await command(adopter,["join","--adopt",...base]);
 await writeAccessV1(store,{version:1,originId:"g1-m02",state:"denied",wallMs:Date.now(),worktree:adopter,platform:"darwin",error:"EPERM"});
 const status=await command(adopter,["status","--worktree",adopter,"--store",store]);
 expect(status.text).toContain("worktree-access-denied: the replica cannot read");expect(status.text).toContain("Files & Folders");
 const german=await command(adopter,["status","--worktree",adopter,"--store",store],"de");
 expect(german.text).toContain("worktree-access-denied: die Replica kann");expect(german.text).toContain("Dateien & Ordner");
 const json=await command(adopter,["status","--json","--worktree",adopter,"--store",store]);
 expect(JSON.parse(json.lines[0]!).access).toMatchObject({state:"denied",originId:"g1-m02"});
 const fleet=await command(adopter,["fleet","--worktree",adopter,"--store",store]);
 expect(fleet.code).toBe(1);expect(fleet.text).toContain("g1-m02 cannot read its worktree");
 await writeAccessV1(store,{version:1,originId:"g1-m02",state:"waiting-for-consent",wallMs:Date.now(),worktree:adopter,platform:"linux",error:"no answer"});
 expect((await command(adopter,["status","--worktree",adopter,"--store",store])).text).toContain("Give the account that runs the replica read and write permission");
},120_000);
