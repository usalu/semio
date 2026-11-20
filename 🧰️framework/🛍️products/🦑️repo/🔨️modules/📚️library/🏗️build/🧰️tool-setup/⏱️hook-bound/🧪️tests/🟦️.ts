/** ⏱️ Law: a tool hook is a declared step: it speaks every heartbeat in both languages with the tool, the step and the seconds of its bound, it is stopped at its bound with the typed bilingual refusal even when it ignores its signal, a cancellation of the caller is not a timeout, and a verification that timed out is a refusal of the setup, never a missing tool that is provisioned again. */
import {expect,test} from "bun:test";
import {verifyToolV1,type ToolContextV1,type ToolRowV1} from "../../🟦️.ts";
import {TOOL_HOOK_POLICY_V1,ToolHookTimeoutV1,runToolHookBoundedV1} from "../🟦️.ts";

const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const request=(signal:AbortSignal=new AbortController().signal)=>({tool:"tectonic",step:"verify" as const,script:"C:\\semio\\print\\toolchain\\script.ts",signal});

test("the verify step is declared with a bound of sixty heartbeats, half the stage timeout, above the longest silent verification seen, and an English and a German name",()=>{
 expect(TOOL_HOOK_POLICY_V1.steps.verify.boundMilliseconds).toBe(60*TOOL_HOOK_POLICY_V1.heartbeatMilliseconds);expect(TOOL_HOOK_POLICY_V1.steps.verify.boundMilliseconds).toBeGreaterThan(1730000);expect(TOOL_HOOK_POLICY_V1.steps.verify.boundMilliseconds).toBe(3600000/2);
 expect(TOOL_HOOK_POLICY_V1.steps.verify.en).not.toBe(TOOL_HOOK_POLICY_V1.steps.verify.de);
});

test("a hook that finishes in time returns its value and says nothing",async()=>{
 const lines:string[]=[];
 expect(await runToolHookBoundedV1(request(),async()=>"ok",{say:line=>lines.push(line),heartbeatMilliseconds:20,boundMilliseconds:500})).toBe("ok");
 await sleep(60);expect(lines).toEqual([]);
});

test("a slow hook speaks every heartbeat in both languages with the tool, the step, the seconds and the bound",async()=>{
 const lines:string[]=[];
 expect(await runToolHookBoundedV1(request(),async()=>{await sleep(130);return 7;},{say:line=>lines.push(line),heartbeatMilliseconds:30,boundMilliseconds:900000})).toBe(7);
 expect(lines.length).toBeGreaterThanOrEqual(6);expect(lines.length%2).toBe(0);
 expect(lines[0]).toContain("tectonic: verifying the pinned tool / Prüfung des gepinnten Werkzeugs");expect(lines[0]).toContain(" of 900 s;");expect(lines[0]).toContain("script.ts");
 expect(lines[1]).toContain("tectonic: verifying the pinned tool / Prüfung des gepinnten Werkzeugs");expect(lines[1]).toContain(" von 900 s;");
});

test("a hook that never ends and ignores its signal is stopped at its bound with the typed refusal in both languages, and its signal fires",async()=>{
 let seen:AbortSignal|undefined;const started=Date.now();
 const failure=await runToolHookBoundedV1(request(),signal=>{seen=signal;return new Promise<never>(()=>undefined);},{say:()=>undefined,heartbeatMilliseconds:20,boundMilliseconds:150}).catch(error=>error as ToolHookTimeoutV1);
 expect(failure).toBeInstanceOf(ToolHookTimeoutV1);expect(failure.name).toBe("ToolHookTimeout");expect(Date.now()-started).toBeLessThan(2000);expect(seen!.aborted).toBe(true);expect(seen!.reason).toBe(failure);
 expect(failure.texts.en).toContain("tectonic");expect(failure.texts.en).toContain("verifying the pinned tool");expect(failure.texts.en).toContain("within 0 s");
 expect(failure.texts.de).toContain("Prüfung des gepinnten Werkzeugs");expect(failure.texts.de).toContain("innerhalb von 0 s");expect(failure.texts.en).not.toBe(failure.texts.de);
});

test("a cancellation of the caller passes through unchanged and is never the timeout",async()=>{
 const caller=new AbortController(),reason=new Error("stage cancelled");
 const running=runToolHookBoundedV1(request(caller.signal),signal=>new Promise<never>((_,reject)=>signal.addEventListener("abort",()=>reject(signal.reason),{once:true})),{say:()=>undefined,heartbeatMilliseconds:20,boundMilliseconds:5000});
 caller.abort(reason);
 expect(await running.catch(error=>error)).toBe(reason);
});

const row={id:"tectonic",summary:{en:"x",de:"x"},pin:{format:"value",value:"0.16.9"},probe:{mode:"hook",arguments:[],pattern:""},locate:[{kind:"hook"}],setup:{kind:"script",script:"s.ts",prepare:["prepare"],verify:["verify"]}} as unknown as ToolRowV1;
const contextOf=(hook:ToolContextV1["hook"])=>({root:"/x",environment:{},hook}) as unknown as ToolContextV1;

test("a verification that timed out is a refusal of the setup, a verification that failed is still a missing tool",async()=>{
 const signal=new AbortController().signal;
 await expect(verifyToolV1(row,contextOf(async()=>{throw new ToolHookTimeoutV1("tectonic","verify","s.ts",1800000);}),signal)).rejects.toBeInstanceOf(ToolHookTimeoutV1);
 expect(await verifyToolV1(row,contextOf(async()=>({found:null,location:null})),signal)).toMatchObject({status:"missing"});
 expect(await verifyToolV1(row,contextOf(async()=>{throw new Error("hook broke");}),signal)).toMatchObject({status:"failed",message:"hook broke"});
 expect(await verifyToolV1(row,contextOf(async()=>({found:"0.16.9",location:"s.ts"})),signal)).toMatchObject({status:"ok"});
});

test("the system tool context runs the verify hook of a script tool only through the bounded step",async()=>{
 const {readFileSync}=await import("node:fs"),{join}=await import("node:path"),source=readFileSync(join(import.meta.dir,"../../⌨️command/🟦️.ts"),"utf8");
 expect(source).toContain('runToolHookBoundedV1({tool: row.id, step: "verify"');
 expect(source.match(/captureOwnedCommandV1\(process\.execPath, \[join\(root, \.\.\.script\.split/gu)?.length).toBe(1);
});
