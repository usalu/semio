import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import schema from "../../../../../../../../🔨️modules/🏃️process/🪪️agent-sessions/🧬️schema/🔣️.json";
import {agentSessionAddressV1,agentSessionStoreV1,agentSessionWorktreeKeyV1,currentAgentSessionV1,listAgentSessionsV1,openAgentSessionV1,deriveAgentSessionV1,resetAgentSessionsV1,touchAgentSessionV1,readAgentSessionV1,type AgentSessionV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {createTestSandboxV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {attachAgentSessionV1} from "../../🟦️.ts";
import {runSessionCommandV1,type SessionCommandContextV1} from "../🟦️.ts";

const scratch=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"sc-"));
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
const ajv=new Ajv({strict:true});ajv.addSchema(schema,"s");
const fresh=(name:string)=>{const directory=join(scratch,name.slice(0,2));mkdirSync(join(directory,"r"),{recursive:true});mkdirSync(join(directory,"t"),{recursive:true});return {directory,registry:join(directory,"r"),parent:join(directory,"t"),root:join(directory,"w")};};
const run=async(setup:ReturnType<typeof fresh>,args:string[],extra:Partial<SessionCommandContextV1>&{language?:string}={}):Promise<{code:number;lines:string[]}>=>{
 resetAgentSessionsV1();const lines:string[]=[],{language,...rest}=extra,environment={SEMIO_LANGUAGE:language??"en",...(rest.environment??{})} as NodeJS.ProcessEnv;
 const code=await runSessionCommandV1(setup.root,args,{write:line=>lines.push(line),registry:setup.registry,parent:setup.parent,ancestry:()=>[],alive:()=>false,usage:async()=>undefined,...rest,environment});
 return {code,lines};
};

test("open creates a session for an explicit agent once, attaches afterwards, speaks English and German and prints the export line and the closed JSON shape",async()=>{
 resetAgentSessionsV1();const setup=fresh("open");
 const first=await run(setup,["open","--agent=reviewer","--export"]);
 expect(first.code).toBe(0);expect(first.lines[0]).toMatch(/^Agent session a[a-z0-9]+ opened for agent reviewer \(human, explicit name\): cargo lane a[a-z0-9]+, test root /u);expect(first.lines[1]).toMatch(/^SEMIO_AGENT_SESSION=[0-9a-f]{12}\.a[a-z0-9]+$/u);
 const again=await run(setup,["open","--agent","reviewer"],{now:()=>Date.now()+5000});expect(again.lines[0]).toContain("is in use");
 const german=await run(setup,["open","--agent=reviewer"],{language:"de"});expect(german.lines[0]).toMatch(/^Agent-Sitzung a[a-z0-9]+ von Agent reviewer \(human\) wird verwendet\.$/u);
 const json=await run(setup,["open","--agent=planner","--weight=3","--json"]);const data=JSON.parse(json.lines.join("\n"));
 expect(data.created).toBe(true);expect(data.address).toBe(agentSessionAddressV1(data.session));expect(data.environment).toEqual({SEMIO_AGENT_SESSION:data.address});expect(ajv.getSchema("s#/definitions/AgentSession")!(data.session),JSON.stringify(ajv.getSchema("s#/definitions/AgentSession")!.errors)).toBe(true);
 expect(data.session.budget.weight).toBe(3);expect(data.paths.temp).toBe(join(data.paths.sandboxes,"t"));expect(existsSync(data.paths.temp)).toBe(true);
 const attached=JSON.parse((await run(setup,["open","--agent=planner","--json"],{now:()=>Date.now()+5000})).lines.join("\n"));expect(attached.created).toBe(false);expect(attached.session.id).toBe(data.session.id);
});

test("open identifies the agent by the client's session variable, by the connection and by the owner process, and without any hint by the default session",async()=>{
 resetAgentSessionsV1();const setup=fresh("identify");
 const byVariable=JSON.parse((await run(setup,["open","--json"],{environment:{CODEX_THREAD_ID:"thread-1"} as NodeJS.ProcessEnv})).lines.join("\n"));expect(byVariable.session.agent).toEqual({id:"thread-1",client:"codex",origin:"environment"});
 const byConnection=JSON.parse((await run(setup,["open","--client=claude-code","--client-pid=11","--owner-pid=22","--json"])).lines.join("\n"));expect(byConnection.session.agent.origin).toBe("connection");expect(byConnection.session.anchors).toEqual([{pid:22,started:""}]);expect(byConnection.session.agent.client).toBe("claude");
 const same=JSON.parse((await run(setup,["open","--client=claude-code","--client-pid=11","--owner-pid=22","--json"])).lines.join("\n"));expect(same.session.id).toBe(byConnection.session.id);
 const other=JSON.parse((await run(setup,["open","--client=claude-code","--client-pid=11","--owner-pid=23","--json"])).lines.join("\n"));expect(other.session.id).not.toBe(byConnection.session.id);
 const process_=JSON.parse((await run(setup,["open","--json"],{ancestry:()=>[{pid:900,started:"s",command:"codex exec"}]})).lines.join("\n"));expect(process_.session.agent.origin).toBe("process");expect(process_.session.anchors).toEqual([{pid:900,started:"s"}]);
 const none=JSON.parse((await run(setup,["open","--json"])).lines.join("\n"));expect(none.session.id).toBe("d");expect(none.session.agent.origin).toBe("default");
});

test("refusals are readable in both languages: unknown options, bad numbers, bad identifiers, a missing session and a live session that is not forced",async()=>{
 resetAgentSessionsV1();const setup=fresh("refuse");
 const refused=async(args:string[],language:string,extra:Partial<SessionCommandContextV1>={})=>{try{await run(setup,args,{language,...extra});return "";}catch(error){return (error as Error).message;}};
 expect(await refused(["open","--bogus"],"en")).toBe("Unknown session option: --bogus");expect(await refused(["open","--bogus"],"de")).toBe("Unbekannte Sitzungsoption: --bogus");
 expect(await refused(["open","--agent"],"en")).toBe("Unknown session option: --agent");expect(await refused(["open","--json=1"],"en")).toBe("Unknown session option: --json=1");
 expect(await refused(["open","--agent=x","--weight=0"],"en")).toContain("whole number from 1 to 16");expect(await refused(["open","--agent=x","--weight=17"],"de")).toContain("von 1 bis 16");expect(await refused(["open","--agent=x","--weight=a"],"en")).toContain("1 to 16");
 expect(await refused(["open","--agent=x","--owner-pid=-3"],"en")).toBe("A process id is a positive whole number: -3");
 expect(await refused(["open","--agent=///"],"en")).toBe("");
 expect(await refused(["status","BAD"],"en")).toContain("Agent session identifiers");expect(await refused(["status","a0000"],"en")).toBe("No agent session a0000 in this worktree.");expect(await refused(["status","a0000"],"de")).toBe("Keine Agent-Sitzung a0000 in diesem Arbeitsbaum.");
 expect(await refused(["close"],"en")).toContain("Expected: repo session open");expect(await refused(["close","a","b"],"en")).toBe("Unknown session option: b");expect(await refused([],"en")).toContain("Expected: repo session open");expect(await refused(["nonsense"],"de")).toContain("Erwartet: repo session open");
 const opened=JSON.parse((await run(setup,["open","--agent=live","--owner-pid=4242","--json"])).lines.join("\n"));
 expect(await refused(["close",opened.session.id],"en",{alive:pid=>pid===4242})).toContain("is still live");expect(await refused(["close",opened.session.id],"de",{alive:pid=>pid===4242})).toContain("noch aktiv");
 const forced=await run(setup,["close",opened.session.id,"--force"],{alive:pid=>pid===4242});expect(forced.lines[0]).toMatch(/^Agent session a[a-z0-9]+ closed: /u);
 expect(readAgentSessionV1(agentSessionStoreV1(setup.registry,agentSessionWorktreeKeyV1(setup.root)),opened.session.id)).toBeUndefined();
});

test("list shows the sessions of the worktree or of every worktree with state and share, status shows one session with the usage probe, as text and as JSON",async()=>{
 resetAgentSessionsV1();const setup=fresh("list"),now=10_000_000;
 const key=agentSessionWorktreeKeyV1(setup.root),store=agentSessionStoreV1(setup.registry,key),other=agentSessionStoreV1(setup.registry,"ffffffffffff");
 for(const agent of["a1","b1"])openAgentSessionV1(store,deriveAgentSessionV1({agent,environment:{},ancestry:[]}),{now});
 openAgentSessionV1(other,{...deriveAgentSessionV1({agent:"elsewhere",environment:{},ancestry:[]})},{now});
 const text=await run(setup,["list"],{now:()=>now});expect(text.lines.length).toBe(3);expect(text.lines[0]).toContain("SESSION");expect(text.lines.join("\n")).toContain("50.0%");
 const all=JSON.parse((await run(setup,["list","--all","--json"],{now:()=>now})).lines.join("\n"));expect(all.sessions.length).toBe(3);for(const row of all.sessions)expect(ajv.getSchema("s#/definitions/Row")!(row),JSON.stringify(ajv.getSchema("s#/definitions/Row")!.errors)).toBe(true);
 expect((await run(setup,["list"],{now:()=>now,language:"de"})).lines[0]).toContain("SITZUNG");
 const id=listAgentSessionsV1(store,{now})[0]!.session.id,usage={leases:2,grants:1,sandboxes:3};
 const status=await run(setup,["status",id],{now:()=>now,usage:async()=>usage});expect(status.lines.join("\n")).toContain("Leases held:");expect(status.lines.join("\n")).toContain(`Session:                  ${id}`);expect(status.lines.find(row=>row.startsWith("Test sandbox root"))).toContain(join(setup.parent,"st")===""?"":id.slice(0,6));
 const json=JSON.parse((await run(setup,["status",id,"--json"],{now:()=>now,usage:async()=>usage})).lines.join("\n"));expect(json.usage).toEqual(usage);expect(json.token).toBe(false);expect(status.lines.join("\n")).toContain("Service token:");expect(json.live).toBe(2);expect(json.shareBasisPoints).toBe(5000);expect(json.state).toBe("live");
 const de=await run(setup,["status",id],{now:()=>now,usage:async()=>usage,language:"de"});expect(de.lines.join("\n")).toContain("Gehaltene Leases:");
 const current=await run(setup,["status","--json"],{now:()=>now,environment:{CLAUDE_CODE_SESSION_ID:"cur"} as NodeJS.ProcessEnv});expect(JSON.parse(current.lines.join("\n")).session.agent.id).toBe("cur");
});

test("close releases the sandboxes of a gone session, keeps a session whose sandbox is alive, and close --dead collects every gone session",async()=>{
 resetAgentSessionsV1();const setup=fresh("close"),now=Date.now()+10_000_000;
 const key=agentSessionWorktreeKeyV1(setup.root),store=agentSessionStoreV1(setup.registry,key);
 const mk=(agent:string)=>openAgentSessionV1(store,deriveAgentSessionV1({agent,environment:{},ancestry:[]}),{now:Date.now()}).session;
 const gone=mk("gone"),busy=mk("busy"),idle=mk("idle");
 const sandbox=(session:AgentSessionV1)=>createTestSandboxV1({route:"c",parent:setup.parent,reclaim:false,session});
 const mine=sandbox(gone),live=sandbox(busy);
 expect(existsSync(mine.root)).toBe(true);
 const closing=await run(setup,["close",gone.id,"--force"],{now:()=>now,alive:pid=>pid===process.pid});
 expect(closing.lines[0]).toContain("closed: ");expect(existsSync(mine.root)).toBe(true);mine.dispose();
 const kept=await run(setup,["close",busy.id,"--force","--json"],{now:()=>now,alive:pid=>pid===process.pid});expect(JSON.parse(kept.lines.join("\n"))).toMatchObject({id:busy.id,closed:true,kept:1});expect(existsSync(live.root)).toBe(true);live.dispose();
 const dead=await run(setup,["close","--dead"],{now:()=>now+1_000_000_000});expect(dead.lines[0]).toMatch(/^Collected \d+ gone agent session\(s\)\.$/u);
 const rows=listAgentSessionsV1(store,{now:now+1_000_000_000,alive:()=>false});expect(rows.length).toBe(0);expect(idle.id).toBeTruthy();
 const json=JSON.parse((await run(setup,["close","--dead","--json"],{now:()=>now+1_000_000_000})).lines.join("\n"));expect(json).toEqual({version:1,collected:[]});
});

test("the door attaches the process to its session, sets the one address variable for its children, collects gone sessions once per interval and never fails the command",()=>{
 resetAgentSessionsV1();const setup=fresh("door"),environment={SEMIO_AGENT_SESSION_DIRECTORY:setup.registry,CLAUDE_CODE_SESSION_ID:"door-1"} as NodeJS.ProcessEnv,warnings:string[]=[];
 const session=attachAgentSessionV1(setup.root,environment,line=>warnings.push(line));
 expect(session?.agent.id).toBe("door-1");expect(environment.SEMIO_AGENT_SESSION).toBe(agentSessionAddressV1(session!));expect(warnings).toEqual([]);
 for(const name of Object.keys(environment))expect(["SEMIO_AGENT_SESSION_DIRECTORY","CLAUDE_CODE_SESSION_ID","SEMIO_AGENT_SESSION"]).toContain(name);
 resetAgentSessionsV1();const broken={SEMIO_AGENT_SESSION_DIRECTORY:"/dev/null/nope",CLAUDE_CODE_SESSION_ID:"x"} as NodeJS.ProcessEnv;
 expect(attachAgentSessionV1(setup.root,broken,line=>warnings.push(line))).toBeUndefined();expect(warnings.length).toBe(1);expect(warnings[0]).toContain("[session] agent session not attached");expect(broken.SEMIO_AGENT_SESSION).toBeUndefined();
 touchAgentSessionV1(agentSessionStoreV1(setup.registry,session!.worktree),session!.id,Date.now(),true);expect(currentAgentSessionV1({registry:setup.registry,environment:{SEMIO_AGENT_SESSION:agentSessionAddressV1(session!)} as never}).id).toBe(session!.id);
});

test("an adapter still gets the session of its identity when the store cannot be written, and the door reports it",()=>{
 resetAgentSessionsV1();const environment={CLAUDE_CODE_SESSION_ID:"degraded-1"};
 const session=currentAgentSessionV1({environment,registry:"/dev/null/nope",worktree:"/work/tree"});
 expect(session.agent).toEqual({id:"degraded-1",client:"claude",origin:"environment"});expect(session.resources.cargo.lane).toBe(session.id);
 expect(()=>currentAgentSessionV1({environment:{...environment,CLAUDE_CODE_SESSION_ID:"degraded-2"},registry:"/dev/null/nope",worktree:"/work/tree",strict:true})).toThrow();
 expect(listAgentSessionsV1(agentSessionStoreV1("/dev/null/nope","3c20d9163894"))).toEqual([]);
});

test("listing every worktree of a registry that does not exist yet is empty",async()=>{
 resetAgentSessionsV1();const setup=fresh("no");
 expect(JSON.parse((await run({...setup,registry:join(setup.directory,"absent")},["list","--all","--json"])).lines.join("\n"))).toEqual({version:1,sessions:[]});
});

test("source law: no library module outside the tests reads a removed per-tool identity variable",()=>{
 const library=join(import.meta.dir,"..","..","..","..");const found:string[]=[],removed=[/SEMIO_AGENT_ID/u,/SEMIO_CARGO_LANE/u,/SEMIO_SESSION\b/u];
 const walk=(directory:string,depth:number)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){const path=join(directory,entry.name);if(entry.isDirectory()){if(!["node_modules","🧪️tests","🧫️fixtures","target","dist"].includes(entry.name)&&depth<12)walk(path,depth+1);}else if(entry.name.endsWith(".ts")||entry.name.endsWith(".mjs")){const text=readFileSync(path,"utf8");for(const pattern of removed)if(pattern.test(text))found.push(path.slice(library.length)+" "+pattern.source);}}};
 walk(library,0);expect(found).toEqual([]);
 writeFileSync(join(scratch,"unused"),"");
});
