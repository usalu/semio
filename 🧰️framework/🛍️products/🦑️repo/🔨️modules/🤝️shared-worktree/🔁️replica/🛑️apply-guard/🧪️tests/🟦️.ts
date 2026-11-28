/** 🛑️ Contract: a file from the store is not written when the volume would keep less than the reserve free; the verdict is the same as the one of an independent calculation, reads of the volume are shared, and an unmeasurable volume never holds a write. The real `statfs` of this machine is compared with the `df` command.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {APPLY_GUARD_POLICY_V1 as policy,applyReserveV1,createApplyGuardV1,judgeApplyV1,readApplySpaceV1} from "../🟦️.ts";

const GIB=2**30;

test("a write waits when less than the reserve would stay free, and the reserve is the bytes plus the share of the volume",()=>{
 const total=475*GIB,reserve=2*GIB+Math.ceil(total*0.03);
 expect(applyReserveV1(total)).toBe(reserve);
 expect(judgeApplyV1({free:reserve+1,total}).short).toBe(false);
 expect(judgeApplyV1({free:reserve,total}).short).toBe(false);
 expect(judgeApplyV1({free:reserve-1,total}).short).toBe(true);
 expect(judgeApplyV1({free:reserve+10,total},11).short).toBe(true);
 const short=judgeApplyV1({free:14*GIB,total});
 expect(short.short).toBe(true);expect(short.message).toContain("Disk short");expect(short.message).toContain("Platte knapp");
 expect(judgeApplyV1({free:14*GIB,total}).reserve).toBe(reserve);
 expect(policy.reserveBasisPoints).toBe(300);
});

test("a volume that cannot be measured never holds a write",()=>{
 expect(judgeApplyV1(undefined).short).toBe(false);
 expect(judgeApplyV1({free:0,total:0}).short).toBe(false);
 expect(judgeApplyV1({free:Number.NaN,total:GIB}).short).toBe(false);
});

test("one reading is shared by concurrent checks and reused until the cache ends",async()=>{
 let clock=0,reads=0,free=10*GIB;
 const guard=createApplyGuardV1({directory:"/x",now:()=>clock,read:async()=>{reads++;await Promise.resolve();return{free,total:475*GIB};}});
 const first=await Promise.all([guard.check(),guard.check(),guard.check()]);
 expect(reads).toBe(1);expect(first.every(row=>row.short)).toBe(true);
 free=400*GIB;clock+=policy.cacheMilliseconds-1;expect((await guard.check()).short).toBe(true);expect(reads).toBe(1);
 clock+=1;expect((await guard.check()).short).toBe(false);expect(reads).toBe(2);
});

test("the reading of the real volume agrees with the df command",async()=>{
 const space=await readApplySpaceV1(tmpdir());
 expect(space).toBeDefined();
 const run=spawnSync("df",["-Pk",tmpdir()]);
 expect(run.status).toBe(0);
 const row=run.stdout.toString().trim().split("\n")[1]!.split(/\s+/),blocks=Number(row[1])*1024;
 expect(space!.total).toBeGreaterThan(blocks*0.9);expect(space!.total).toBeLessThan(blocks*1.1);
 expect(space!.free).toBeGreaterThan(0);expect(space!.free).toBeLessThanOrEqual(space!.total);
});
