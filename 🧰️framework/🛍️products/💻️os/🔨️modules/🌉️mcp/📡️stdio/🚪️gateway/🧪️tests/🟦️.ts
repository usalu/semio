import Ajv from "ajv";
import {Database} from "bun:sqlite";
import policy from "../🧬️schema/🔣️.json";
import { expect, test } from "bun:test";
import { CallToolResultSchema, CancelledNotificationSchema, InitializeRequestSchema, InitializeResultSchema, JSONRPCErrorSchema, JSONRPCResponseSchema, PromptListChangedNotificationSchema, ResourceListChangedNotificationSchema, ToolListChangedNotificationSchema, ListPromptsResultSchema, ListResourceTemplatesResultSchema, ListResourcesResultSchema, ListToolsResultSchema } from "@modelcontextprotocol/sdk/types.js";
import { PassThrough } from "node:stream";
import { serveWarmMcpGateway, WarmMcpGateway, type WarmMcpFrame, type WarmMcpServerPort, warmMcpReply } from "../🟦️.ts";
import corpus from "./🔣️.json";

type Step = { closed?: boolean; exited?: number | null; signal?: string; at?: number; client?: WarmMcpFrame; server?: WarmMcpFrame; child?: number; staged?: number; changed?: boolean; usable?: boolean; elapsed?: number; failed?: number; reason?: string };
type Transcript = { restarted?: boolean; abandoned?: string; toClient?: WarmMcpFrame; toServer?: WarmMcpFrame; child?: number; stopped?: number };

/** 🔁️ Replays a journey; `child` names the native server (0 when absent) a frame goes to or comes from. */
const run = (steps: readonly Step[]): { seen: Transcript[]; logs: string[] } => {
  const seen: Transcript[] = [];
  const logs: string[] = [];
  const ports: WarmMcpServerPort[] = [];
  const scheduled: { milliseconds: number; run: () => void; live: boolean }[] = [];
  let clock = 0;
  const tagged = (index: number): { child?: number } => (index === 0 ? {} : { child: index });
  const gateway = new WarmMcpGateway({
    toClient: (frame) => void seen.push({ toClient: frame }),
    startServer: () => {
      const index = ports.length;
      const port: WarmMcpServerPort = { send: (frame) => void seen.push({ toServer: frame, ...tagged(index) }), stop: () => void seen.push({ stopped: index }) };
      ports.push(port);
      return port;
    },
    log: (line) => void logs.push(line),
    now: () => clock,
    abandon: (reason) => void seen.push({ abandoned: reason }),
    schedule: (milliseconds, run) => {
      const entry = { milliseconds, run, live: true };
      scheduled.push(entry);
      return () => { entry.live = false; };
    },
  });
  for (const step of steps) {
    if (step.at !== undefined) clock = step.at;
    if (step.client !== undefined) gateway.fromClient(step.client);
    else if (step.server !== undefined) gateway.fromServer(step.server, ports[step.child ?? 0]);
    else if (step.exited !== undefined) seen.push({ restarted: gateway.serverExited(step.exited, ports[step.child ?? 0], step.signal) });
    else if (step.usable) gateway.markUsable();
    else if (step.closed) gateway.clientClosed();
    else if (step.elapsed !== undefined) for (const entry of scheduled) { if (entry.live && entry.milliseconds <= step.elapsed) { entry.live = false; entry.run(); } }
    else if (step.failed !== undefined) gateway.portFailed(ports[step.failed]!, step.reason ?? "");
    else if (step.staged !== undefined) gateway.stagingFinished(step.staged, step.changed);
  }
  return { seen, logs };
};

const replay = (steps: readonly Step[]): Transcript[] => run(steps).seen;

for (const row of corpus.replies) {
  test(`warm gateway reply: ${row.name}`, () => {
    expect(warmMcpReply(row.request as WarmMcpFrame)).toEqual(row.reply as WarmMcpFrame | null);
  });
}

for (const journey of corpus.journeys) {
  test(`warm gateway journey: ${journey.name}`, () => {
    const { seen, logs } = run(journey.steps as Step[]);
    expect(seen).toEqual(journey.transcript as Transcript[]);
    if ("logs" in journey) expect(logs).toEqual(journey.logs as string[]);
  });
}

type ModelStep = Step & { built?: string; changed?: boolean; failed?: number | string; reason?: string; closed?: boolean };
type ModelTranscript = Transcript & { prepare?: { pluginId: string; cratePath: string } };

/** 🧱️ Replays a journey whose launcher builds models: `built`/`failed` settle the pending build of that plugin, `closed` ends the session. */
const runModels = async (steps: readonly ModelStep[]): Promise<{ seen: ModelTranscript[]; logs: string[]; aborted: string[] }> => {
  const seen: ModelTranscript[] = [];
  const logs: string[] = [];
  const aborted: string[] = [];
  const ports: WarmMcpServerPort[] = [];
  const pending = new Map<string, { settle: (changed: boolean) => void; refuse: (error: Error) => void }>();
  const gateway = new WarmMcpGateway({
    toClient: (frame) => void seen.push({ toClient: frame }),
    startServer: () => {
      const index = ports.length;
      const port: WarmMcpServerPort = { send: (frame) => void seen.push({ toServer: frame, ...(index === 0 ? {} : { child: index }) }), stop: () => void seen.push({ stopped: index }) };
      ports.push(port);
      return port;
    },
    log: (line) => void logs.push(line),
    prepareModel: (model, signal) => new Promise<boolean>((settle, refuse) => {
      seen.push({ prepare: model });
      signal.addEventListener("abort", () => { aborted.push(model.pluginId); refuse(new Error("aborted")); });
      pending.set(model.pluginId, { settle, refuse });
    }),
  });
  for (const step of steps) {
    if (step.client !== undefined) gateway.fromClient(step.client);
    else if (step.server !== undefined) gateway.fromServer(step.server, ports[step.child ?? 0]);
    else if (step.usable) gateway.markUsable();
    else if (step.staged !== undefined) gateway.stagingFinished(step.staged, step.changed);
    else if (step.built !== undefined) pending.get(step.built)!.settle(step.changed ?? true);
    else if (typeof step.failed === "string") pending.get(step.failed)!.refuse(new Error(step.reason ?? ""));
    else if (step.closed) gateway.clientClosed();
    await new Promise<void>((wake) => setTimeout(wake, 0));
  }
  return { seen, logs, aborted };
};

for (const journey of corpus.modelJourneys) {
  test(`warm gateway model journey: ${journey.name}`, async () => {
    const { seen, logs, aborted } = await runModels(journey.steps as ModelStep[]);
    expect(seen).toEqual(journey.transcript as ModelTranscript[]);
    expect(logs).toEqual(journey.logs);
    expect(aborted).toEqual(journey.steps.some((step) => "closed" in step) ? ["note"] : []);
  });
}

test("a gateway whose launcher builds no models ignores missing-model refusals", async () => {
  const journey = corpus.modelJourneys[0]!;
  const gateway = new WarmMcpGateway({ toClient: () => {}, startServer: () => ({ send: () => {}, stop: () => {} }), log: (line) => void logs.push(line) });
  const logs: string[] = [];
  for (const step of journey.steps.slice(0, 6) as ModelStep[]) {
    if (step.client !== undefined) gateway.fromClient(step.client);
    else if (step.server !== undefined) gateway.fromServer(step.server);
    else if (step.usable) gateway.markUsable();
  }
  await new Promise<void>((wake) => setTimeout(wake, 0));
  expect(logs).toEqual([]);
});

test("warm gateway replies satisfy the reference SDK schemas", () => {
  for (const row of corpus.replies) {
    const reply = warmMcpReply(row.request as WarmMcpFrame) as { result?: unknown } | null;
    if (reply === null) continue;
    if (row.request.method === "initialize") expect(InitializeResultSchema.safeParse(reply.result).success).toBe(true);
    const discovery = { "tools/list": ListToolsResultSchema, "resources/list": ListResourcesResultSchema, "resources/templates/list": ListResourceTemplatesResultSchema, "prompts/list": ListPromptsResultSchema }[row.request.method as string];
    if (discovery !== undefined) expect(discovery.safeParse(reply.result).success).toBe(true);
  }
  const failed = replay(corpus.journeys.find((journey) => journey.name === "staging-failed")!.steps as Step[]).flatMap((entry) => (entry.toClient?.error === undefined ? [] : [entry.toClient]));
  expect(failed.map((frame) => frame.id)).toEqual([3, 4, 5]);
  for (const frame of failed) expect(JSONRPCErrorSchema.safeParse(frame).success).toBe(true);
});

test("queued cancellation agrees with the reference SDK and SQLite", () => {
  const journey = corpus.journeys.find(row => row.name === "cancel-before-dispatch")!;
  const staged = journey.steps.findIndex(step => step.staged === 0);
  const frames = journey.steps.slice(0, staged).map(step => step.client);
  const { params } = CancelledNotificationSchema.parse(frames.pop());
  const late = CancelledNotificationSchema.parse(journey.steps[staged + 1]!.client);
  const db = new Database(":memory:");
  try {
    const outstanding = db.query("select value from json_each(?1) where json_extract(value, '$.id') is not ?2 order by key")
      .all(JSON.stringify(frames), params.requestId) as { value: string }[];
    expect(replay(journey.steps as Step[]).map(entry => entry.toServer)).toEqual([
      ...outstanding.map(row => JSON.parse(row.value)),
      { jsonrpc: "2.0", ...late },
    ]);
  } finally {
    db.close();
  }
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
  send({ jsonrpc: "2.0", id: 3, method: "ping" });
  await until(() => lines.some((frame) => frame.id === 3));
  expect(ListToolsResultSchema.parse(lines.find((frame) => frame.id === 2)!.result).tools).toEqual([]);
  expect(lines.some((frame) => frame.method === "notifications/tools/list_changed")).toBe(false);
  release(0);
  await until(() => lines.some((frame) => frame.method === "notifications/prompts/list_changed"));
  expect(lines.filter((frame) => frame.id === 2)).toHaveLength(1);
  send({ jsonrpc: "2.0", id: 4, method: "tools/list" });
  await until(() => lines.some((frame) => frame.id === 4));
  expect(ListToolsResultSchema.parse(lines.find((frame) => frame.id === 4)!.result).tools[0]?.name).toBe("peer-tool");
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

test("a peer that exits before the native handshake fails queued requests and the launcher", async () => {
  const input = new PassThrough();
  const output = new PassThrough();
  let wire = "";
  output.on("data", (chunk: Buffer) => { wire += chunk.toString(); });
  const { peerStatus, gatewayStatus, errorCode } = corpus.earlyPeerExit;
  const served = serveWarmMcpGateway({ stage: async () => 0, command: () => process.execPath, args: ["-e", `process.exit(${peerStatus})`], cwd: process.cwd(), env: process.env, input, output });
  for (const frame of [
    corpus.replies[0]!.request,
    { jsonrpc: "2.0", method: "notifications/initialized" },
    { jsonrpc: "2.0", id: 2, method: "tools/call", params: { name: "x" } },
  ]) input.write(JSON.stringify(frame) + "\n");
  try {
    expect(await served).toBe(gatewayStatus);
    const failure = wire.trim().split("\n").map(line => JSON.parse(line)).find(frame => frame.id === 2);
    expect(JSONRPCErrorSchema.parse(failure).error.code).toBe(errorCode);
  } finally {
    input.end();
  }
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


const refreshJourneys = corpus.journeys.filter((journey) => journey.name.startsWith("usable-"));

test("refresh wording and handshake budget agree with the authored schema", () => {
  expect(new Ajv({ strict: false }).validate(policy.definitions.Refresh, corpus.refresh)).toBe(true);
  expect(new Ajv({ strict: false }).validate(policy.definitions.Handshake, corpus.handshake)).toBe(true);
});

test("refresh journeys emit frames the reference SDK schemas accept", () => {
  const announcements = [ToolListChangedNotificationSchema, ResourceListChangedNotificationSchema, PromptListChangedNotificationSchema];
  for (const journey of refreshJourneys) {
    const seen = replay(journey.steps as Step[]);
    const handshakes = seen.flatMap((entry) => (entry.toServer?.method === "initialize" ? [entry.toServer] : []));
    const original = (journey.steps[0] as Step).client!;
    for (const frame of handshakes) {
      expect(InitializeRequestSchema.safeParse(frame).success).toBe(true);
      expect(frame.params).toEqual(original.params);
    }
    const methods = seen.flatMap((entry) => (entry.toClient?.method?.endsWith("/list_changed") ? [entry.toClient] : []));
    expect(methods.length % 3).toBe(0);
    for (let index = 0; index < methods.length; index += 3) announcements.forEach((schema, offset) => expect(schema.safeParse(methods[index + offset]).success).toBe(true));
    for (const entry of seen) {
      const frame = entry.toClient;
      if (frame?.result !== undefined && frame.id !== 1) {
        expect(JSONRPCResponseSchema.safeParse(frame).success).toBe(true);
        expect(CallToolResultSchema.safeParse(frame.result).success).toBe(true);
      }
    }
  }
});

test("a replaced server is ended only after the requests it held are answered, as SQLite counts them", () => {
  const journey = refreshJourneys.find((row) => row.name.startsWith("usable-then-changed-swaps"))!;
  const seen = replay(journey.steps as Step[]);
  const db = new Database(":memory:");
  try {
    const held = db.query("select json_extract(value, '$.toServer.id') id from json_each(?1) where json_extract(value, '$.toServer.method') = 'tools/call' and json_extract(value, '$.child') is null order by key")
      .all(JSON.stringify(seen)) as { id: number }[];
    const stopped = seen.findIndex((entry) => entry.stopped === 0);
    const answered = seen.slice(0, stopped).flatMap((entry) => (entry.toClient?.result !== undefined && held.some((row) => row.id === entry.toClient!.id) ? [entry.toClient.id] : []));
    expect(held.map((row) => row.id)).toEqual([2, 3]);
    expect(answered).toEqual([2, 3]);
    expect(seen.filter((entry) => entry.toServer?.method === "tools/call" && entry.child === 1).map((entry) => entry.toServer!.id)).toEqual([4]);
  } finally {
    db.close();
  }
});

const refreshPeer = (label: string): string => String.raw`const rl=require('node:readline').createInterface({input:process.stdin});rl.on('line',line=>{const m=JSON.parse(line);const out=o=>process.stdout.write(JSON.stringify({jsonrpc:'2.0',...o})+'\n');if(m.method==='initialize')out({id:m.id,result:{protocolVersion:m.params.protocolVersion,capabilities:{tools:{}},serverInfo:{name:'semio',version:'${label}'}}});else if(m.method==='tools/call')setTimeout(()=>out({id:m.id,result:{content:[{type:'text',text:'${label}'}]}}),m.params.name==='slow'?400:0);else if(m.method==='tools/list')out({id:m.id,result:{tools:[{name:'${label}',inputSchema:{type:'object'}}]}})});rl.on('close',()=>process.exit(0));`;

const collect = (output: PassThrough): { lines: WarmMcpFrame[]; until: (predicate: () => boolean) => Promise<void> } => {
  const lines: WarmMcpFrame[] = [];
  let buffer = "";
  output.on("data", (chunk: Buffer) => {
    buffer += chunk.toString();
    for (let end = buffer.indexOf("\n"); end >= 0; end = buffer.indexOf("\n")) {
      lines.push(JSON.parse(buffer.slice(0, end)) as WarmMcpFrame);
      buffer = buffer.slice(end + 1);
    }
  });
  const until = async (predicate: () => boolean): Promise<void> => {
    const deadline = Date.now() + 15000;
    while (!predicate()) {
      if (Date.now() > deadline) throw new Error(`timed out; saw ${JSON.stringify(lines)}`);
      await new Promise((wake) => setTimeout(wake, 20));
    }
  };
  return { lines, until };
};

test("a usable server answers before staging ends and a changed refresh swaps real processes behind an in-flight request", async () => {
  const input = new PassThrough();
  const output = new PassThrough();
  const { lines, until } = collect(output);
  let release!: () => void;
  const finished = new Promise<void>((resolve) => (release = resolve));
  let spawned = 0;
  const served = serveWarmMcpGateway({
    stage: async (_progress, _signal, serving) => { serving(); await finished; return 0; },
    changed: () => true,
    command: () => process.execPath,
    get args() { return ["-e", refreshPeer(`peer${spawned++}`)]; },
    cwd: process.cwd(),
    env: process.env,
    input,
    output,
  });
  const send = (frame: WarmMcpFrame): void => void input.write(`${JSON.stringify(frame)}\n`);
  send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "t", version: "1" } } });
  send({ jsonrpc: "2.0", method: "notifications/initialized" });
  send({ jsonrpc: "2.0", id: 2, method: "tools/call", params: { name: "slow" } });
  await until(() => lines.filter((frame) => frame.method === "notifications/prompts/list_changed").length === 1);
  release();
  await until(() => lines.filter((frame) => frame.method === "notifications/prompts/list_changed").length === 2);
  send({ jsonrpc: "2.0", id: 3, method: "tools/list" });
  await until(() => lines.some((frame) => frame.id === 3) && lines.some((frame) => frame.id === 2));
  expect(ListToolsResultSchema.parse(lines.find((frame) => frame.id === 3)!.result).tools[0]?.name).toBe("peer1");
  expect(CallToolResultSchema.parse(lines.find((frame) => frame.id === 2)!.result).content[0]).toEqual({ type: "text", text: "peer0" });
  expect(lines.every((frame) => frame.jsonrpc === "2.0")).toBe(true);
  input.end();
  expect(await served).toBe(0);
}, 30000);

test("discovery is held while a published server is expected and released at once when none can serve", async () => {
  for (const serves of [true, false]) {
    const input = new PassThrough();
    const output = new PassThrough();
    const { lines, until } = collect(output);
    let decide = (_value: boolean): void => {}, finish = (): void => {};
    const expected = new Promise<boolean>((resolve) => { decide = resolve; }), finished = new Promise<void>((resolve) => { finish = resolve; });
    const served = serveWarmMcpGateway({
      expected,
      stage: async (_progress, _signal, serving) => { const ok = await expected; if (ok) serving(); await finished; return 0; },
      changed: () => false,
      command: () => process.execPath,
      args: ["-e", refreshPeer("only")],
      cwd: process.cwd(),
      env: process.env,
      input,
      output,
    });
    const send = (frame: WarmMcpFrame): void => void input.write(`${JSON.stringify(frame)}\n`);
    send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "t", version: "1" } } });
    send({ jsonrpc: "2.0", method: "notifications/initialized" });
    send({ jsonrpc: "2.0", id: 2, method: "tools/list" });
    await new Promise((wake) => setTimeout(wake, 100));
    expect(lines.some((frame) => frame.id === 2)).toBe(false);
    decide(serves);
    await until(() => lines.some((frame) => frame.id === 2));
    const announced = lines.findIndex((frame) => frame.method === "notifications/tools/list_changed"), answered = lines.findIndex((frame) => frame.id === 2);
    expect(serves ? announced !== -1 || answered !== -1 : announced === -1).toBe(true);
    finish();
    input.end();
    expect(await served).toBe(0);
  }
}, 30000);

test("a second usable publication swaps the real server before staging ends", async () => {
  const input = new PassThrough();
  const output = new PassThrough();
  const { lines, until } = collect(output);
  let starts = 0, second = (): void => {}, finish = (): void => {};
  const published = new Promise<void>((resolve) => { second = resolve; }), finished = new Promise<void>((resolve) => { finish = resolve; });
  const served = serveWarmMcpGateway({
    stage: async (_progress, _signal, serving) => { serving(); await published; serving(); await finished; return 0; },
    changed: () => false,
    command: () => { starts++; return process.execPath; },
    args: ["-e", refreshPeer("only")],
    cwd: process.cwd(),
    env: process.env,
    input,
    output,
  });
  const announced = (): number => lines.filter((frame) => frame.method === "notifications/tools/list_changed").length;
  const send = (frame: WarmMcpFrame): void => void input.write(`${JSON.stringify(frame)}\n`);
  send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "t", version: "1" } } });
  send({ jsonrpc: "2.0", method: "notifications/initialized" });
  await until(() => announced() === 1);
  second();
  await until(() => announced() === 2);
  expect(starts).toBe(2);
  send({ jsonrpc: "2.0", id: 2, method: "tools/list" });
  await until(() => lines.some((frame) => frame.id === 2));
  finish();
  input.end();
  expect(await served).toBe(0);
}, 30000);

test("an unchanged or failed refresh leaves the first real process serving", async () => {
  for (const outcome of [{ status: 0, changed: false }, { status: 1, changed: true }]) {
    const input = new PassThrough();
    const output = new PassThrough();
    const { lines, until } = collect(output);
    let starts = 0;
    const served = serveWarmMcpGateway({
      stage: async (_progress, _signal, serving) => { serving(); return outcome.status; },
      changed: () => outcome.changed,
      command: () => { starts++; return process.execPath; },
      args: ["-e", refreshPeer("only")],
      cwd: process.cwd(),
      env: process.env,
      input,
      output,
    });
    const send = (frame: WarmMcpFrame): void => void input.write(`${JSON.stringify(frame)}\n`);
    send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "t", version: "1" } } });
    send({ jsonrpc: "2.0", method: "notifications/initialized" });
    await until(() => lines.some((frame) => frame.method === "notifications/prompts/list_changed"));
    await new Promise((wake) => setTimeout(wake, 100));
    send({ jsonrpc: "2.0", id: 2, method: "tools/list" });
    await until(() => lines.some((frame) => frame.id === 2));
    expect(starts).toBe(1);
    expect(lines.filter((frame) => frame.method === "notifications/tools/list_changed")).toHaveLength(1);
    input.end();
    expect(await served).toBe(0);
  }
}, 30000);

test("the serving server crashing answers its request, restarts behind the client and the session ends with the client", async () => {
  const input = new PassThrough();
  const output = new PassThrough();
  const { lines, until } = collect(output);
  let release!: () => void;
  const finished = new Promise<void>((resolve) => (release = resolve));
  let spawned = 0;
  const dying = refreshPeer("dying").replace("if(m.method==='initialize')", "if(m.method==='tools/call'&&m.params.name==='die')process.exit(3);if(m.method==='initialize')");
  const served = serveWarmMcpGateway({
    stage: async (_progress, _signal, serving) => { serving(); await finished; return 0; },
    changed: () => true,
    command: () => process.execPath,
    get args() { return ["-e", spawned++ === 0 ? refreshPeer("old") : dying]; },
    cwd: process.cwd(),
    env: process.env,
    input,
    output,
  });
  const send = (frame: WarmMcpFrame): void => void input.write(`${JSON.stringify(frame)}\n`);
  send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "t", version: "1" } } });
  send({ jsonrpc: "2.0", method: "notifications/initialized" });
  send({ jsonrpc: "2.0", id: 2, method: "tools/call", params: { name: "slow" } });
  await until(() => lines.filter((frame) => frame.method === "notifications/prompts/list_changed").length === 1);
  release();
  await until(() => lines.filter((frame) => frame.method === "notifications/prompts/list_changed").length === 2);
  send({ jsonrpc: "2.0", id: 3, method: "tools/call", params: { name: "die" } });
  await until(() => lines.some((frame) => frame.id === 3));
  const refused = lines.find((frame) => frame.id === 3)!.error as { code: number; data: { cause: string; retryable: boolean; exit: { code: number | null } } };
  expect([refused.code, refused.data.cause, refused.data.retryable, refused.data.exit.code]).toEqual([-32603, "server-exited", true, 3]);
  await until(() => lines.filter((frame) => frame.method === "notifications/prompts/list_changed").length === 3);
  input.end();
  expect(await served).toBe(0);
}, 30000);

const withoutInitializeReply = (seen: readonly Transcript[]): Transcript[] => seen.filter((entry, index) => !(index === 0 && entry.toClient?.id === 1));

for (const hold of corpus.discoveryHolds) {
  test(`warm gateway discovery hold: ${hold.name}`, () => {
    const { seen } = run(hold.steps as Step[]);
    const summary = withoutInitializeReply(seen).map((entry) => (entry.toClient ? { toClient: { id: entry.toClient.id, method: entry.toClient.method, result: entry.toClient.result, error: (entry.toClient.error as { code?: number } | undefined)?.code } } : entry));
    expect(JSON.parse(JSON.stringify(summary))).toEqual(hold.expected);
  });
}
