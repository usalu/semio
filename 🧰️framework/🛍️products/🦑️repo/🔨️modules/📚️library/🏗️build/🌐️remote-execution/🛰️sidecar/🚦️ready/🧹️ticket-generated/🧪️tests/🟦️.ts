/** 🧹️ Contract: a `🗑️generated` folder of a ticket that nothing has written to for the idle limit goes away on its own (the replica never copies or deletes these ignored folders, so a lane that left one behind on a worker filled its disk), a folder a lane still writes to stays, and a short volume shortens the limit. The independent oracle is the Python standard library walking the same tree, and the root `.gitignore` is read with the matcher of the replica to prove that the swept names are ignored, i.e. that the sweep never deletes anything another machine holds.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{compileIgnoreV1}from"../../../../../../../../🔨️modules/🤝️shared-worktree/🚧️boundary/🟦️.ts";
import{TICKET_GENERATED_POLICY_V1 as policy,createTicketGeneratedSweepV1,sweepTicketGeneratedV1,ticketGeneratedFoldersV1,ticketGeneratedPressureV1}from"../🟦️.ts";

const HOUR=3600000,NOW=Date.UTC(2026,10,27,12,0,0);
const ticket="🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22";
const build=():string=>{
 const root=mkdtempSync(join(tmpdir(),"ticket-generated-")),dirs=new Map<string,number>(),base=join(root,".🧬semio");
 const at=(path:string,hoursAgo:number,size:number=10)=>{
  const file=join(base,path);mkdirSync(join(file,".."),{recursive:true});writeFileSync(file,Buffer.alloc(size,1));
  const when=(NOW-hoursAgo*HOUR)/1000;utimesSync(file,when,when);
  for(let dir=join(file,"..");dir.length>=base.length;dir=join(dir,"..")){dirs.set(dir,Math.max(dirs.get(dir)??0,when));}
 };
 at(`${ticket}/OLD/🗑️generated/target/a.o`,50,1000);at(`${ticket}/OLD/🗑️generated/target/b.o`,60,500);
 at(`${ticket}/BUSY/🗑️generated/target/a.o`,50);at(`${ticket}/BUSY/🗑️generated/deep/er/live.log`,0.2);
 at(`${ticket}/MID/🗑️generated/x`,5);
 at(`${ticket}/NOFE0F/🗑generated/x`,70);
 at(`${ticket}/KEEP/notes.md`,500);at(`${ticket}/KEEP/🧰️scripts/run.sh`,500);
 for(const [dir,when] of dirs)utimesSync(dir,when,when);
 return root;
};
const oracle=(root:string,idleMs:number):string[]=>{
 const run=spawnSync("python3",["-I","-c","import os,sys,json\nbase=os.path.join(sys.argv[1],'.🧬semio','🦑️repo','🎫️tickets');limit=float(sys.argv[2]);out=[]\nfor y in sorted(os.listdir(base)):\n for m in sorted(os.listdir(os.path.join(base,y))):\n  for d in sorted(os.listdir(os.path.join(base,y,m))):\n   for t in sorted(os.listdir(os.path.join(base,y,m,d))):\n    for g in sorted(os.listdir(os.path.join(base,y,m,d,t))):\n     if 'generated' not in g or not os.path.isdir(os.path.join(base,y,m,d,t,g)):continue\n     p=os.path.join(base,y,m,d,t,g);newest=os.lstat(p).st_mtime\n     for r,ds,fs in os.walk(p):\n      for n in ds+fs:newest=max(newest,os.lstat(os.path.join(r,n)).st_mtime)\n     if newest*1000<limit:out.append(os.path.relpath(p,sys.argv[1]))\nprint(json.dumps(out))",root,String(NOW-idleMs)]);
 expect(run.status,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString());
};

test("the sweep removes exactly the folders that an independent walk finds idle, and nothing else",async()=>{
 const root=build();
 try{
  const expected=oracle(root,policy.idleMilliseconds).map(path=>path.normalize("NFC")).sort();
  expect(expected.length).toBe(2);
  const found=await sweepTicketGeneratedV1(root,{now:()=>NOW}),removed=found.filter(row=>row.state==="removed").map(row=>row.path.normalize("NFC")).sort();
  expect(removed).toEqual(expected);
  expect(found.filter(row=>row.state==="active").map(row=>row.path.split("/")[6]).sort()).toEqual(["BUSY","MID"]);
  const old=found.find(row=>row.path.includes("/OLD/"))!;
  expect([old.files,old.bytes,old.idleHours]).toEqual([2,1500,50]);
  for(const path of expected)expect(existsSync(join(root,path))).toBe(false);
  expect(existsSync(join(root,".🧬semio",ticket,"KEEP/notes.md"))).toBe(true);
  expect(existsSync(join(root,".🧬semio",ticket,"BUSY/🗑️generated/deep/er/live.log"))).toBe(true);
  expect(existsSync(join(root,".🧬semio",ticket,"MID/🗑️generated/x"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a short volume shortens the idle limit, and a dry run reports without removing",async()=>{
 const root=build();
 try{
  const short={freeBytes:16*2**30,totalBytes:475*2**30};
  expect(ticketGeneratedPressureV1(short)).toBe(true);
  expect(ticketGeneratedPressureV1({freeBytes:200*2**30,totalBytes:475*2**30})).toBe(false);
  expect(ticketGeneratedPressureV1({freeBytes:null,totalBytes:null})).toBe(false);
  expect(ticketGeneratedPressureV1(null)).toBe(false);
  const lines:string[]=[],dry=await sweepTicketGeneratedV1(root,{now:()=>NOW,volume:()=>short,dryRun:true,report:line=>lines.push(line)});
  expect(dry.filter(row=>row.state==="would-remove").map(row=>row.path.split("/")[6]).sort()).toEqual(["MID","NOFE0F","OLD"]);
  expect(dry.filter(row=>row.state==="removed").length).toBe(0);
  expect(existsSync(join(root,".🧬semio",ticket,"MID/🗑️generated/x"))).toBe(true);
  expect(lines.length).toBe(3);expect(lines[0]).toContain(" / ");expect(lines[0]).toContain("16 GiB");
  const expected=oracle(root,policy.pressureIdleMilliseconds).length;
  expect(expected).toBe(3);
  const real=await sweepTicketGeneratedV1(root,{now:()=>NOW,volume:()=>short});
  expect(real.filter(row=>row.state==="removed").length).toBe(3);
  expect(existsSync(join(root,".🧬semio",ticket,"BUSY/🗑️generated/deep/er/live.log"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("links are never followed, a folder over the entry budget is kept as unknown, a failing removal never stops the next and the signal cancels",async()=>{
 const root=build(),outside=mkdtempSync(join(tmpdir(),"ticket-outside-"));
 try{
  writeFileSync(join(outside,"precious"),"x");utimesSync(join(outside,"precious"),1,1);
  mkdirSync(join(root,".🧬semio",ticket,"LINK"),{recursive:true});symlinkSync(outside,join(root,".🧬semio",ticket,"LINK/🗑️generated"));
  expect(ticketGeneratedFoldersV1(root).some(path=>path.includes("/LINK/"))).toBe(false);
  const small=await sweepTicketGeneratedV1(root,{now:()=>NOW,entryBudget:1,dryRun:true});
  expect(small.filter(row=>row.state==="unknown").length).toBeGreaterThan(0);
  const failing=await sweepTicketGeneratedV1(root,{now:()=>NOW,remove:async path=>{if(path.includes("/OLD/"))throw Error("busy");rmSync(path,{recursive:true,force:true});}});
  expect(failing.find(row=>row.path.includes("/NOFE0F/"))!.state).toBe("removed");
  expect(failing.find(row=>row.path.includes("/OLD/"))!.state).toBe("failed");
  await expect(sweepTicketGeneratedV1(root,{now:()=>NOW,signal:AbortSignal.abort()})).rejects.toThrow();
  expect(existsSync(join(outside,"precious"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(outside,{recursive:true,force:true});}
});

test("the background sweep waits for its first delay, runs one at a time and every interval, and survives a failing sweep",async()=>{
 let clock=NOW,calls=0,fail=false;
 const sweep=createTicketGeneratedSweepV1({root:"/nowhere",now:()=>clock,sweep:async()=>{calls++;if(fail)throw Error("x");return[];}});
 expect(await sweep.tick()).toBe(false);
 clock+=policy.firstDelayMilliseconds;expect(await sweep.tick()).toBe(true);expect(calls).toBe(1);
 clock+=policy.intervalMilliseconds-1;expect(await sweep.tick()).toBe(false);
 clock+=1;fail=true;expect(await sweep.tick()).toBe(false);expect(calls).toBe(2);
 fail=false;clock+=policy.intervalMilliseconds;expect(await sweep.tick()).toBe(true);expect(calls).toBe(3);
});

test("the swept names are ignored by the root .gitignore, so the replica holds nothing of them and no other machine loses a file",()=>{
 const text=readFileSync(join(import.meta.dir,"../../../../../../../../../../../.gitignore"),"utf8"),ignore=compileIgnoreV1(text);
 for(const name of policy.names)expect(ignore.ignores(`.🧬semio/${ticket}/ANY/${name}`,true)).toBe(true);
 expect(ignore.ignores(`.🧬semio/${ticket}/ANY/notes.md`,false)).toBe(false);
});
