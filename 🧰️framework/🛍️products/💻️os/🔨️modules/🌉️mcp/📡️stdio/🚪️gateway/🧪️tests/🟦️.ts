import Ajv from "ajv";
import {Database} from "bun:sqlite";
import policy from "../🧬️schema/🔣️.json";
import { expect, test } from "bun:test";
import { InitializeResultSchema, JSONRPCErrorSchema, ListToolsResultSchema } from "@modelcontextprotocol/sdk/types.js";
import { PassThrough } from "node:stream";
import { serveWarmMcpGateway, WarmMcpGateway, type WarmMcpFrame, warmMcpReply } from "../🟦️.ts";
import corpus from "./🔣️.json";

type Step = { client?: WarmMcpFrame; server?: WarmMcpFrame; staged?: number };
type Transcript = { toClient?: WarmMcpFrame; toServer?: WarmMcpFrame };

const replay = (steps: readonly Step[]): Transcript[] => {
  const seen: Transcript[] = [];
  const gateway = new WarmMcpGateway({ toClient: (frame) => void seen.push({ toClient: frame }), startServer: () => ({ send: (frame) => void seen.push({ toServer: frame }), stop: () => {} }), log: () => {} });
  for (const step of steps) {
    if (step.client !== undefined) gateway.fromClient(step.client);
    else if (step.server !== undefined) gateway.fromServer(step.server);
    else if (step.staged !== undefined) gateway.stagingFinished(step.staged);
  }
  return seen;
};

for (const row of corpus.replies) {
  test(`warm gateway reply: ${row.name}`, () => {
    expect(warmMcpReply(row.request as WarmMcpFrame)).toEqual(row.reply as WarmMcpFrame | null);
  });
}

for (const journey of corpus.journeys) {
  test(`warm gateway journey: ${journey.name}`, () => {
    expect(replay(journey.steps as Step[])).toEqual(journey.transcript as Transcript[]);
  });
}

test("warm gateway replies satisfy the reference SDK schemas", () => {
  for (const row of corpus.replies) {
    const reply = warmMcpReply(row.request as WarmMcpFrame) as { result?: unknown } | null;
    if (reply === null) continue;
    if (row.request.method === "initialize") expect(InitializeResultSchema.safeParse(reply.result).success).toBe(true);
    if (row.request.method === "tools/list") expect(ListToolsResultSchema.safeParse(reply.result).success).toBe(true);
  }
  const failed = replay(corpus.journeys.find((journey) => journey.name === "staging-failed")!.steps as Step[]).flatMap((entry) => (entry.toClient?.error === undefined ? [] : [entry.toClient]));
  expect(failed.length).toBe(2);
  for (const frame of failed) expect(JSON.stringify(JSONRPCErrorSchema.safeParse(frame).success)).toBe("true");
});

const peer = String.raw`const rl=require('node:readline').createInterface({input:process.stdin});rl.on('line',line=>{const m=JSON.parse(line);if(m.method==='initialize')process.stdout.write(JSON.stringify({jsonrpc:'2.0',id:m.id,result:{protocolVersion:m.params.protocolVersion,capabilities:{tools:{}},serverInfo:{name:'semio',version:'peer'}}})+'\n');else if(m.method==='tools/list')process.stdout.write('not json\n'+JSON.stringify({jsonrpc:'2.0',id:m.id,result:{tools:[{name:'peer-tool',inputSchema:{type:'object'}}]}})+'\n')});rl.on('close',()=>process.exit(0));`;

test("warm gateway serves a client while staging runs, then splices in the real server over stdio", async () => {
  const input = new PassThrough();
  const output = new PassThrough();
  const lines: WarmMcpFrame[] = [];
  const waiting: Array<() => void> = [];
  let buffer = "";
  output.on("data", (chunk: Buffer) => {
    buffer += chunk.toString();
    for (let end = buffer.indexOf("\n"); end >= 0; end = buffer.indexOf("\n")) {
      lines.push(JSON.parse(buffer.slice(0, end)) as WarmMcpFrame);
      buffer = buffer.slice(end + 1);
      for (const wake of waiting.splice(0)) wake();
    }
  });
  const until = async (predicate: () => boolean): Promise<void> => {
    const deadline = Date.now() + 15000;
    while (!predicate()) {
      if (Date.now() > deadline) throw new Error(`timed out; saw ${JSON.stringify(lines)}`);
      await new Promise<void>((wake) => {
        waiting.push(wake);
        setTimeout(wake, 50);
      });
    }
  };
  let release: (status: number) => void = () => {};
  const staged = new Promise<number>((resolve) => (release = resolve));
  const served = serveWarmMcpGateway({ stage: () => staged, command: () => process.execPath, args: ["-e", peer], cwd: process.cwd(), env: process.env, input, output });
  const send = (frame: WarmMcpFrame): void => void input.write(`${JSON.stringify(frame)}\n`);
  send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "t", version: "1" } } });
  await until(() => lines.some((frame) => frame.id === 1));
  expect(InitializeResultSchema.safeParse(lines.find((frame) => frame.id === 1)!.result).success).toBe(true);
  send({ jsonrpc: "2.0", method: "notifications/initialized" });
  send({ jsonrpc: "2.0", id: 2, method: "tools/list" });
  await until(() => lines.some((frame) => frame.id === 2));
  expect((lines.find((frame) => frame.id === 2)!.result as { tools: unknown[] }).tools).toEqual([]);
  release(0);
  await until(() => lines.some((frame) => frame.method === "notifications/prompts/list_changed"));
  send({ jsonrpc: "2.0", id: 3, method: "tools/list" });
  await until(() => lines.some((frame) => frame.id === 3));
  expect(ListToolsResultSchema.parse(lines.find((frame) => frame.id === 3)!.result).tools[0]?.name).toBe("peer-tool");
  expect(lines.every((frame) => frame.jsonrpc === "2.0")).toBe(true);
  input.end();
  expect(await served).toBe(0);
}, 30000);

test("warm gateway ends cleanly when the client leaves during staging", async () => {
  const input = new PassThrough();
  let aborted = false;
  const served = serveWarmMcpGateway({ stage: (_progress, signal) => new Promise<number>(() => signal.addEventListener("abort", () => (aborted = true))), command: () => process.execPath, args: [], cwd: process.cwd(), env: process.env, input, output: new PassThrough() });
  input.end();
  expect(await served).toBe(0);
  expect(aborted).toBe(true);
});

test("warm gateway never starts a peer after client EOF and late staging settlement", async () => {
 const input = new PassThrough();
 let release!: (status: number) => void, starts = 0;
 const served = serveWarmMcpGateway({stage: () => new Promise<number>(resolve => {release = resolve;}), command: () => {starts++; return process.execPath;}, args: ["-e", "process.exit(0)"], cwd: process.cwd(), env: process.env, input, output: new PassThrough()});
 input.write(JSON.stringify(corpus.replies[0]!.request) + "\n");
 input.write(JSON.stringify({jsonrpc:"2.0",method:"notifications/initialized"}) + "\n");
 input.end();
 expect(await served).toBe(0);
 release(0);
 await new Promise(resolve => setTimeout(resolve, 20));
 expect(starts).toBe(0);
});

test("native peer shutdown is bounded when EOF and graceful termination are ignored", async () => {
 const input=new PassThrough(),output=new PassThrough();let announced!:()=>void;const ready=new Promise<void>(resolve=>{announced=resolve;});
 let pid:number|undefined;output.on("data",chunk=>{for(const line of String(chunk).trim().split("\n")){const frame=JSON.parse(line);if(frame.method==="peer/ready"){pid=frame.params.pid;announced();}}});
 const peer=`process.on("SIGTERM",()=>{});require("node:readline").createInterface({input:process.stdin}).on("line",line=>{const m=JSON.parse(line);if(m.method==="initialize")console.log(JSON.stringify({jsonrpc:"2.0",id:m.id,result:{protocolVersion:m.params.protocolVersion,capabilities:{},serverInfo:{name:"peer",version:"1"}}}));if(m.method==="notifications/initialized")console.log(JSON.stringify({jsonrpc:"2.0",method:"peer/ready",params:{pid:process.pid}}));});setInterval(()=>{},1000);`;
 const served=serveWarmMcpGateway({stage:async()=>0,command:()=>process.execPath,args:["-e",peer],cwd:process.cwd(),env:process.env,input,output});
 input.write(JSON.stringify(corpus.replies[0]!.request)+"\n");input.write(JSON.stringify({jsonrpc:"2.0",method:"notifications/initialized"})+"\n");
 await ready;const started=performance.now();input.end();let timer:ReturnType<typeof setTimeout>|undefined;try{await Promise.race([served,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error("Bounded peer shutdown required")),1500);})]);}finally{clearTimeout(timer);if(pid)try{process.kill(pid,"SIGKILL");}catch{}}expect(performance.now()-started).toBeLessThan(corpus.shutdown.graceMilliseconds+corpus.shutdown.terminationMilliseconds+corpus.shutdown.killMilliseconds+500);
},3000);

test("neutral shutdown timing agrees with authored schema and SQLite budget", () => {
 expect(new Ajv({strict:false}).validate(policy.definitions.Shutdown,corpus.shutdown)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ?1+?2+?3 maximum").get(corpus.shutdown.graceMilliseconds,corpus.shutdown.terminationMilliseconds,corpus.shutdown.killMilliseconds)).toEqual({maximum:750});}finally{db.close();}
});

test("modern discover-first preserves the exact native answer and SDK schema", () => {
 const journey=corpus.journeys.find(row=>row.name==="modern-discover-first-is-native-and-stateless")!;const seen=replay(journey.steps as Step[]),answer=seen.find(row=>row.toClient?.id===10)?.toClient as {result:unknown}|undefined;expect(answer).toBeDefined();expect(InitializeResultSchema.parse(answer!.result)).toEqual(InitializeResultSchema.parse((journey.transcript as Transcript[]).find(row=>row.toClient?.id===10)?.toClient?.result));expect(seen.some(row=>row.toServer?.method==="initialize"||row.toServer?.method==="notifications/initialized")).toBe(false);
});
