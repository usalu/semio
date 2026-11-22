/** 🪦️ Law: the producer publications of gone processes are swept by the product and by an independent Python walk alike; a live process, a young folder, a link and a foreign name stay, and a process that ends retires its own folder. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {PRODUCER_PUBLICATION_V1,producerAliveV1,sweepDeadProducerPublicationsV1} from "../🟨️.mjs";

const now=Date.now(),HASHES=["a","b","c"].map(letter=>letter.repeat(64));
let root="",selections="",outside="",gone=0,alsoGone=0;
const endedPid=():number=>{const run=spawnSync(process.execPath,["--eval","process.stdout.write(String(process.pid))"],{encoding:"utf8"});return Number(run.stdout);};
const publication=(hash:string,name:string,ageMs:number):string=>{const folder=join(selections,hash,"inference",name);mkdirSync(folder,{recursive:true});writeFileSync(join(folder,"emoji-project-json.publication.json"),"x".repeat(5000));const time=(now-ageMs)/1000;utimesSync(folder,time,time);return folder;};
const names=():string[]=>HASHES.flatMap(hash=>{const folder=join(selections,hash,"inference");return existsSync(folder)?readdirSync(folder).filter(name=>name.startsWith("p-")).map(name=>join(folder,name)):[];}).sort();
const python=(dead:readonly number[]):string[]=>{
 const code="import json,os,sys,re\nroot,dead,now,quiet=json.loads(sys.stdin.read())\nout=[]\nfor h in sorted(os.listdir(root)):\n d=os.path.join(root,h,'inference')\n if not os.path.isdir(d):continue\n for n in sorted(os.listdir(d)):\n  m=re.fullmatch(r'p-([1-9][0-9]{0,9})',n);p=os.path.join(d,n)\n  if m and os.path.isdir(p) and not os.path.islink(p) and int(m.group(1)) in dead and now-os.stat(p).st_mtime*1000>=quiet:out.append(p)\nprint(json.dumps(out))";
 const run=spawnSync("python3",["-I","-c",code],{input:JSON.stringify([selections,dead,now,PRODUCER_PUBLICATION_V1.quietMs]),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);
};
const lay=():void=>{
 rmSync(selections,{recursive:true,force:true});mkdirSync(selections,{recursive:true});
 publication(HASHES[0]!,"p-"+gone,3600000);publication(HASHES[0]!,"p-"+process.pid,3600000);publication(HASHES[1]!,"p-"+alsoGone,3600000);
 publication(HASHES[1]!,"p-"+(alsoGone+1),1000);publication(HASHES[2]!,"p-abc",3600000);publication(HASHES[2]!,"q-"+gone,3600000);
 writeFileSync(join(selections,HASHES[2]!,"inference","p-"+(alsoGone+2)),"file");
 symlinkSync(outside,join(selections,HASHES[2]!,"inference","p-"+(gone+7)));
};
beforeAll(()=>{
 root=mkdtempSync(join(tmpdir(),"producer-publications-"));selections=join(root,"selections");outside=join(root,"outside");mkdirSync(outside,{recursive:true});writeFileSync(join(outside,"keep.json"),"keep");
 gone=endedPid();alsoGone=endedPid();if(alsoGone===gone)alsoGone=gone+3;
});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

test("the sweep names the same folders as an independent Python walk and removes only those",async()=>{
 lay();const dead=[gone,alsoGone,gone+7,alsoGone+1,alsoGone+2].filter(pid=>!producerAliveV1(pid)),before=names();
 const plan=await sweepDeadProducerPublicationsV1(selections,{now:()=>now,dryRun:true,alive:pid=>!dead.includes(pid)});
 expect([...plan.candidates].sort()).toEqual(python(dead).sort());
 expect(plan.candidates.length).toBeGreaterThanOrEqual(2);expect(plan.removed).toBe(0);expect(plan.bytes).toBeGreaterThanOrEqual(5000);expect(names()).toEqual(before);
 const done=await sweepDeadProducerPublicationsV1(selections,{now:()=>now,alive:pid=>!dead.includes(pid)});
 expect(done.removed).toBe(plan.candidates.length);expect(done.candidates).toEqual(plan.candidates);
 expect(names()).toEqual(before.filter(path=>!plan.candidates.includes(path)));
 expect(existsSync(join(selections,HASHES[0]!,"inference","p-"+process.pid))).toBe(true);
 expect(existsSync(join(selections,HASHES[1]!,"inference","p-"+(alsoGone+1)))).toBe(true);
 expect(existsSync(join(outside,"keep.json"))).toBe(true);expect(existsSync(join(selections,HASHES[2]!,"inference","p-abc"))).toBe(true);
 expect(existsSync(join(selections,HASHES[2]!,"inference","q-"+gone))).toBe(true);
});

test("a second sweep finds nothing, a missing store is not an error and a cancelled sweep stops",async()=>{
 const again=await sweepDeadProducerPublicationsV1(selections,{now:()=>now,alive:pid=>pid===process.pid});expect(again.removed).toBe(0);
 expect((await sweepDeadProducerPublicationsV1(join(root,"missing"),{})).cancelled).toBe(false);
 lay();const controller=new AbortController();controller.abort();
 const stopped=await sweepDeadProducerPublicationsV1(selections,{now:()=>now,alive:()=>false,signal:controller.signal});expect(stopped.cancelled).toBe(true);expect(stopped.removed).toBe(0);
});

test("a live process keeps its folder, a young folder stays, and the pid check answers like the system",async()=>{
 lay();const live=await sweepDeadProducerPublicationsV1(selections,{now:()=>now,alive:()=>true});expect([live.candidates.length,live.removed]).toEqual([0,0]);
 const young=await sweepDeadProducerPublicationsV1(selections,{now:()=>now,alive:()=>false,quietMs:2*3600000});expect(young.removed).toBe(0);
 expect(producerAliveV1(process.pid)).toBe(true);expect(producerAliveV1(gone)).toBe(false);
});

test("a process retires its own folder when it ends, keeps a link, and a killed process leaves its folder for the sweep",()=>{
 lay();const own=join(selections,HASHES[0]!,"inference","p-own"),linked=join(root,"linked"),target=join(root,"link-target");
 mkdirSync(own,{recursive:true});writeFileSync(join(own,"x.json"),"x");mkdirSync(target,{recursive:true});writeFileSync(join(target,"keep.json"),"keep");symlinkSync(target,linked);
 const module=join(import.meta.dir,"..","🟨️.mjs"),code=`import(${JSON.stringify(module)}).then(m=>{m.retireProducerPublicationAtExitV1(${JSON.stringify(own)});m.retireProducerPublicationAtExitV1(${JSON.stringify(own)});m.retireProducerPublicationAtExitV1(${JSON.stringify(linked)});});`;
 const run=spawnSync(process.execPath,["--eval",code],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 expect(existsSync(own)).toBe(false);expect(existsSync(join(target,"keep.json"))).toBe(true);expect(existsSync(linked)).toBe(true);
 const killed=join(selections,HASHES[0]!,"inference","p-killed");mkdirSync(killed,{recursive:true});
 const crash=spawnSync(process.execPath,["--eval",`import(${JSON.stringify(module)}).then(m=>{m.retireProducerPublicationAtExitV1(${JSON.stringify(killed)});process.kill(process.pid,"SIGKILL");});`],{encoding:"utf8"});
 expect(crash.signal).toBe("SIGKILL");expect(existsSync(killed)).toBe(true);
});
