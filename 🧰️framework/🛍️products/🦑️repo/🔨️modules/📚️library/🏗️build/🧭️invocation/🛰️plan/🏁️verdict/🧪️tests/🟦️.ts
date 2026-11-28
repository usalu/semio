/** 🏁️ Contract: a plan request never ends silently. A request that finishes (result or error) prints nothing here; a process that exits (also a drained event loop with the request unsettled) prints the bilingual verdict, with a nonzero exit code. Checked with real processes.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {planVerdictTextV1} from "../🟦️.ts";

const module=new URL("../🟦️.ts",import.meta.url).href;
const run=async(body:string)=>{
 const child=Bun.spawn([process.execPath,"-e",`import {guardPlanVerdictV1} from ${JSON.stringify(module)};const guard=guardPlanVerdictV1();${body}`],{stdout:"pipe",stderr:"pipe",stdin:"ignore"});
 const [out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 return{out,err,code};
};

test("the verdict text is in both languages and names how the request ended",()=>{
 const text=planVerdictTextV1("signal SIGTERM");
 expect(text).toContain("ended without an answer (signal SIGTERM)");expect(text).toContain("endete ohne Antwort (signal SIGTERM)");expect(text.endsWith("\n")).toBe(true);
});

test("a request that finished prints no verdict and keeps the exit code",async()=>{
 const{err,code}=await run("guard.done();process.exitCode=0;");
 expect(err).toBe("");expect(code).toBe(0);
});

test("an event loop that ran dry with the request unsettled prints the verdict and exits nonzero",async()=>{
 const{err,code}=await run("void new Promise(()=>{});");
 expect(err).toContain("ended without an answer");expect(err).toContain("endete ohne Antwort");expect(code).not.toBe(0);
});

test("an exit with code 0 while the request is open is turned into a failure with the verdict",async()=>{
 const{err,code}=await run("setTimeout(()=>process.exit(0),20);setInterval(()=>undefined,1000);");
 expect(err).toContain("ended without an answer");expect(code).toBe(1);
});

test("the guard never keeps the event loop alive and never hooks a signal, so a request that is open when the loop runs dry ends instead of hanging",async()=>{
 const{err,code}=await run("if(process.listenerCount('SIGTERM')+process.listenerCount('SIGINT')+process.listenerCount('SIGHUP')!==0)throw Error('signal hook');void new Promise(()=>{});");
 expect(err).toContain("endete ohne Antwort");expect(code).not.toBe(0);
});
