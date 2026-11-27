/** 🧪️ Contract: a server says its 507 and 503 refusals in the log: the first at once, then one summed line per window, never one line per request. */
import {expect,test} from "bun:test";
import {createRemoteCacheRefusalLogV1} from "../🟦️.ts";

const rig=(intervalMilliseconds=60_000)=>{let clock=1_000_000;const lines:string[]=[];const log=createRemoteCacheRefusalLogV1({report:line=>lines.push(line),intervalMilliseconds,now:()=>clock});return{lines,log,advance:(ms:number)=>{clock+=ms;}};};

test("the first refusal is reported at once, the burst after it is summed into one line per window",()=>{
 const test=rig();
 test.log.record("capacity","volume 96 % full");
 expect(test.lines.length).toBe(1);expect(test.lines[0]).toContain("refused 1 write(s) with 507");expect(test.lines[0]).toContain("volume 96 % full");
 for(let index=0;index<999;index++){test.advance(10);test.log.record("capacity","volume 96 % full");}
 expect(test.lines.length,"nine seconds of 999 refusals add nothing").toBe(1);
 test.advance(60_000);test.log.record("capacity","volume 95 % full");
 expect(test.lines.length).toBe(2);expect(test.lines[1]).toContain("refused 1000 write(s) with 507");expect(test.lines[1]).toContain("(1001 since start)");expect(test.lines[1]).toContain("95 % full");
 expect(test.log.total("capacity")).toBe(1001);
});

test("every line says it in English and in German, and busy refusals are counted apart from capacity ones",()=>{
 const test=rig();
 test.log.record("busy");test.log.record("capacity","x");
 expect(test.lines.length).toBe(2);
 expect(test.lines[0]).toContain("503 busy");expect(test.lines[0]).toContain(" / ");expect(test.lines[0]).toContain("ausgelastet");
 expect(test.lines[1]).toContain("507");expect(test.lines[1]).toContain("abgelehnt");
 expect(test.log.total("busy")).toBe(1);expect(test.log.total("capacity")).toBe(1);
});

test("a flush says what the window still holds, and a failing report never reaches the request",()=>{
 const test=rig();
 test.log.record("capacity","a");test.advance(5_000);test.log.record("capacity","b");test.log.record("capacity","c");
 test.log.flush();
 expect(test.lines.length).toBe(2);expect(test.lines[1]).toContain("refused 2 write(s)");
 test.log.flush();expect(test.lines.length,"nothing pending, nothing said").toBe(2);
 const broken=createRemoteCacheRefusalLogV1({report:()=>{throw Error("closed");},intervalMilliseconds:1000});
 expect(()=>broken.record("busy")).not.toThrow();
});
