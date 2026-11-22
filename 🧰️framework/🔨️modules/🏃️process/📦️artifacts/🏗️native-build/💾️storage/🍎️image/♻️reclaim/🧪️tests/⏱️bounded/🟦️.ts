import {expect,test} from "bun:test";
import {createNativeImageLinuxReclaimPortV1,nativeImageBoundedRunnerV1,NativeImageCommandInterruptedV1} from "../../🟦️.ts";

/** ⏱️ Contract: every command of the loop-image cycle ends by its deadline or by cancellation with the bilingual typed error, and a command that ends in time is unchanged. No real image is touched; the clock is fake. */
const clock=()=>{let at=0;const timers:{at:number;run:()=>void;off:boolean}[]=[];return{setTimer:(run:()=>void,ms:number)=>{const row={at:at+ms,run,off:false};timers.push(row);return row;},clearTimer:(row:unknown)=>{(row as {off:boolean}).off=true;},advance:(ms:number)=>{at+=ms;for(const row of timers.filter(row=>!row.off&&row.at<=at)){row.off=true;row.run();}}};};
const hang=()=>new Promise<string>(()=>undefined);

test("a hung command is rejected at its deadline with the typed bilingual error",async()=>{
 const time=clock(),run=nativeImageBoundedRunnerV1(hang,{milliseconds:5000,setTimer:time.setTimer,clearTimer:time.clearTimer}),pending=run("fstrim",["-v","/scratch"]);
 time.advance(4999);time.advance(1);
 const error=await pending.catch(failure=>failure) as NativeImageCommandInterruptedV1;
 expect(error).toBeInstanceOf(NativeImageCommandInterruptedV1);
 expect(error.reason).toBe("deadline");expect(error.message).toContain("fstrim -v /scratch");expect(error.message).toContain("5 s");expect(error.message).toContain(" / Image-Befehl ");
});

test("cancellation rejects at once, also before the start",async()=>{
 const time=clock(),controller=new AbortController(),run=nativeImageBoundedRunnerV1(hang,{milliseconds:5000,signal:controller.signal,setTimer:time.setTimer,clearTimer:time.clearTimer}),pending=run("mount",["-o","discard"]);
 controller.abort();
 expect(((await pending.catch(failure=>failure)) as NativeImageCommandInterruptedV1).reason).toBe("cancelled");
 expect(((await run("losetup",[]).catch(failure=>failure)) as NativeImageCommandInterruptedV1).reason).toBe("cancelled");
});

test("a command that ends in time keeps its result and its error",async()=>{
 const time=clock();
 expect(await nativeImageBoundedRunnerV1(async()=>"ok",{setTimer:time.setTimer,clearTimer:time.clearTimer})("losetup",[])).toBe("ok");
 const failure=Error("boom");
 expect(await nativeImageBoundedRunnerV1(async()=>{throw failure;},{setTimer:time.setTimer,clearTimer:time.clearTimer})("losetup",[]).catch(error=>error)).toBe(failure);
});

test("the Linux port bounds losetup, mount and fstrim through its injected signal",async()=>{
 const controller=new AbortController(),calls:string[]=[],port=createNativeImageLinuxReclaimPortV1({image:"/x.img",signal:controller.signal,run:async(program)=>{calls.push(program);return program.endsWith("losetup")?"/dev/loop9\n":program.endsWith("mount")||program==="mount"?"":hang();}});
 await port.attach();
 const pending=port.trim!();controller.abort();
 expect(((await pending.catch(failure=>failure)) as NativeImageCommandInterruptedV1).reason).toBe("cancelled");
 port.dispose();
});
