/** 🎟️ Host staging never dispatches an incomplete or cancelled request. */
import {test,expect} from "bun:test";
import {StagedActionHost,type StagedActionSource} from "./session.ts";
const owner={token:{action:"insertItem",window:"inspection",generation:"0",revision:"a".repeat(64)},locale:"en" as const,controllerId:"test"};
test("cancel aborts the original producer and performs no mutation",async()=>{
 const host=new StagedActionHost();let signal:AbortSignal|undefined,executed=0;
 const source:StagedActionSource={owner,surface:"7:staged-inspection",current:()=>true,revision:()=>owner.token.revision,subscribe:()=>()=>{},query:async(_request,input)=>{signal=input;return new Promise(()=>{});},execute:async()=>{executed++;}};
 const result=host.open(source,{});await Promise.resolve();await Promise.resolve();expect(host.snapshot()?.ready).toBe(false);host.execute();host.cancel();expect(await result).toBe("cancelled");expect(signal?.aborted).toBe(true);expect(executed).toBe(0);
});
