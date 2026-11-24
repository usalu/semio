import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireSlotV1,processAliveV1,processZombieV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"semio-zombie-"));roots.push(root);return root;};
const parents:Array<ReturnType<typeof spawn>>=[];
afterAll(()=>{for(const parent of parents)parent.kill("SIGKILL");for(const root of roots)rmSync(root,{recursive:true,force:true});});
const posix=process.platform!=="win32"&&spawnSync("perl",["-e","1"]).status===0;
const makeZombie=async():Promise<number>=>{
 const parent=spawn("perl",["-e",'my $p=fork();if(!$p){exit 0;}print "$p\\n";$|=1;sleep 60;'],{stdio:["ignore","pipe","ignore"]});parents.push(parent);
 const pid=await new Promise<number>(done=>parent.stdout!.once("data",data=>done(Number(String(data).trim()))));
 for(let attempt=0;attempt<50&&!processZombieV1(pid);attempt++)await new Promise(done=>setTimeout(done,50));
 return pid;
};
const age=(path:string,seconds:number)=>{const then=new Date(Date.now()-seconds*1000);utimesSync(path,then,then);};
test("a process that exited but was never reaped is a zombie, not alive; a running one is alive; Windows has no zombies",()=>{
 expect(processZombieV1(1,{platform:"win32",state:()=>"Z"})).toBe(false);
 expect(processZombieV1(1,{platform:"linux",state:()=>"Z"})).toBe(true);
 expect(processZombieV1(1,{platform:"darwin",state:()=>"S"})).toBe(false);
 expect(processZombieV1(1,{platform:"darwin",state:()=>undefined})).toBe(false);
 expect(processAliveV1(process.pid)).toBe(true);
});
test.skipIf(!posix)("a real zombie is detected as not alive",async()=>{
 const pid=await makeZombie();expect(processZombieV1(pid)).toBe(true);expect(processAliveV1(pid)).toBe(false);
});
test.skipIf(!posix)("a compile slot and the ticket queue head held by zombies are reclaimed instead of blocking every waiter forever",async()=>{
 const root=fresh(),zombie=await makeZombie(),slots=join(root,"s"),tickets=join(root,"q");mkdirSync(slots,{recursive:true});mkdirSync(tickets,{recursive:true});
 const slot=join(slots,"0.lock"),ticket=join(tickets,"000000000000001-dead0000-"+String(zombie));
 writeFileSync(slot,JSON.stringify({pid:zombie,token:"t",atMs:1}));writeFileSync(ticket,JSON.stringify({pid:zombie,token:"x",atMs:1}));
 age(slot,3600);age(ticket,3600);
 const claim=await Promise.race([acquireSlotV1(root,1),new Promise<undefined>(done=>setTimeout(()=>done(undefined),5000))]);
 expect(claim,"slot granted although its holder and the head ticket are zombies").toBeDefined();claim!.release();
});
test.skipIf(!posix)("a live holder keeps its claim however old the file is, and a zombie holder with a fresh heartbeat is not judged yet",async()=>{
 const root=fresh(),directory=join(root,"k"),claim=tryClaimV1(directory,"live")!;expect(claim).toBeDefined();
 age(claim.path,3600);expect(tryClaimV1(directory,"live")).toBeUndefined();claim.release();
 const zombie=await makeZombie();mkdirSync(directory,{recursive:true});const path=join(directory,"fresh.lock");
 writeFileSync(path,JSON.stringify({pid:zombie,token:"t",atMs:1}));age(path,1.5);
 expect(tryClaimV1(directory,"fresh")).toBeUndefined();age(path,60);
 const taken=tryClaimV1(directory,"fresh");expect(taken).toBeDefined();taken!.release();
});
