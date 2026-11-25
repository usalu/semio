import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import ts from "typescript";
import { drainTypedOperationBatches, scanTypedOperationPages, TYPED_OPERATION_PAGE_MAGIC, wireTurnStatusTag } from "../../../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import fixture from "./🔣️.json";

function definition(path: string, name: string): string {
  const tree = ts.createSourceFile(path, readFileSync(new URL(path,import.meta.url),"utf8"), ts.ScriptTarget.Latest, true);
  return tree.statements.find(row => (ts.isFunctionDeclaration(row) || ts.isClassDeclaration(row)) && row.name?.text === name)!.getText(tree).replace(/^export /u, "");
}

const program = ts.transpileModule([
  definition("../../../🧾️typed-operations-and-handle/🟦️.ts", "WgpuTypedOperationDrive"),
  definition("../🟦️.ts", "createTurnDrivers"),
].join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;

type Turn = typeof fixture.cases[number]["turns"][number];
function typedPage() {
  const body = new Uint8Array(TYPED_OPERATION_PAGE_MAGIC.length + 30);
  body.set(TYPED_OPERATION_PAGE_MAGIC);
  const header = new DataView(body.buffer, TYPED_OPERATION_PAGE_MAGIC.length);
  header.setUint32(0, 7, true);
  header.setBigUint64(4, 1n, true);
  header.setBigUint64(12, 1n, true);
  header.setUint8(25, 4);
  return { tag: "send-message", val: { target: { tag: "shell", val: 7 }, payload: body } };
}

for (const sample of fixture.cases) test(sample.name, async () => {
  using database = new Database(":memory:");
  const accepted = database.query("select sum(case json_extract(value,'$.transportError') when 1 then 0 else 1+coalesce(json_extract(value,'$.followUps'),0) end) as total from json_each(?)").get(JSON.stringify(sample.turns)) as {total:number};
  expect(accepted.total).toBe(sample.accepted);
  const page0={id:0},page1={id:1},pages=[page0,page1];
  let submitted=0,observed=0,released=false;
  const activation={assertActive(){if(released)throw Error("activation revoked");}};
  const settled=Promise.withResolvers<{error?:Error;frames?:Uint8Array[]}>();
  const ports={
    WGPU_TYPED_OPERATION_SETTLE_LIMIT:64, WGPU_TYPED_OPERATION_DRAIN_BUDGET:4096, WGPU_TYPED_OPERATION_EFFECT_CAPACITY:4096,
    WGPU_SPAWNED_JOB_ROUNDS:2, drainTypedOperationBatches, wireTurnStatusTag, scanTypedOperationPages,
    createTurnOutcomeBroadcast:()=>({push:settled.resolve,stream:null}), createShardCommandIngressPages:()=>pages,
    serializeWgpuActorCall:(_actor:string,work:()=>Promise<void>)=>work(), yieldWgpuUi:async()=>undefined,
    getShardClient:()=>({}), takeSpawnedJobs:()=>[], forgetSpawnedJobs:()=>{}, shellFrameBytes:()=>null,
    pendingTurnEffects:new Map(), stashLeftoverHostEffects:()=>{},
    submitTurn:async(_actor:string,events:readonly unknown[],page:typeof page0|undefined,lease:typeof activation)=>{
      expect(lease).toBe(activation);
      lease.assertActive();
      const turn:Turn=sample.turns[submitted++]!;
      expect(page).toBe(turn.page===null?undefined:pages[turn.page]);
      expect(events).toHaveLength(turn.acks??0);
      if(turn.transportError)throw Error("transport refused");
      return {turn, status:"idle", uiPatches:[], effects:turn.owesAck?[typedPage()]:[], deferredEvents:turn.deferred, commandIngress:turn.ingress?{tag:turn.ingress}:undefined};
    },
  };
  const create=new Function(...Object.keys(ports),program+";return createTurnDrivers;")(...Object.values(ports));
  const route={lifecycle:{activation},accept:async(result:{turn:Turn})=>{
    observed+=1+(result.turn.followUps??0);
    if(result.turn.revoke)released=true;
    return Array.from({length:result.turn.followUps??0},()=>({status:"idle",uiPatches:[],effects:[]}));
  }};
  const driver=create({requireUiRoute:()=>route,requireActorId:()=>"actor",executeFor:()=>undefined,state:{eventSeq:0},closingInstances:new Set(),actorIdByInstance:new Map([[7,"actor"]]),pluginId:"fixture"});
  driver.channelHandle.enqueue(7,[Uint8Array.of(1)]);
  const result=await settled.promise;
  expect(result.error?.message??null).toBe(sample.error);
  expect(submitted).toBe(sample.turns.length);
  expect(observed).toBe(sample.accepted);
});

test("extension completion hands an owed page acknowledgement to its background drain", async () => {
  const source = ts.transpileModule([definition("../../../🧾️typed-operations-and-handle/🟦️.ts", "WgpuTypedOperationDrive"), definition("../../📞️extension-and-request-actors/🟦️.ts", "createExtensionCompletion")].join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  let handoff: readonly unknown[] | undefined;
  const ports = {
    scanTypedOperationPages, wireTurnStatusTag, WGPU_TYPED_OPERATION_SETTLE_LIMIT: 0,
    GUEST_HOST_ANSWER_CEILING_BYTES: 1024, guestAnswerPages: (answer: Uint8Array) => ({ prologue: [], terminal: answer }),
    pendingTurnEffects: new Map(), wgpuHostEffect: () => null, shellFrameBytes: () => null, admitSpawnedJob: () => false,
    serializeWgpuActorCall: (_actor: string, work: () => Promise<unknown>) => work(),
    submitTurn: async () => ({ status: "more-work", deferredEvents: 0, uiPatches: [], effects: [typedPage()] }),
  };
  const create = new Function(...Object.keys(ports), source + ";return createExtensionCompletion;")(...Object.values(ports));
  const completion = create({ requireActorId: () => "actor", requireUiRoute: () => ({ accept: async () => [] }), executeFor: () => undefined, drainSpawnedJobs: async () => ({ frames: [] }), turnOutcomes: { push() {} }, drainTypedOperations: (_id: number, acknowledgements: readonly unknown[]) => { handoff = acknowledgements; } });
  await completion.captureExtensionCompletion(7, 1n).complete({ ok: Uint8Array.of(1) });
  expect(handoff).toHaveLength(1);
});

test("background drain submits a handed-off acknowledgement once", async () => {
  const ack = { kind: "fixture-ack" };
  const submitted: unknown[][] = [];
  const ports = {
    WGPU_TYPED_OPERATION_DRAIN_BUDGET: 4, WGPU_TYPED_OPERATION_EFFECT_CAPACITY: 4, WGPU_SPAWNED_JOB_ROUNDS: 2,
    scanTypedOperationPages, wireTurnStatusTag, createTurnOutcomeBroadcast: () => ({ push() {}, stream: null }),
    drainTypedOperationBatches: async (_budget: number, _live: unknown, step: () => Promise<unknown>) => { await step(); return { polls: 1, stopped: "idle" }; },
    serializeWgpuActorCall: (_actor: string, work: () => Promise<unknown>) => work(), yieldWgpuUi: async () => {},
    getShardClient: () => ({}), takeSpawnedJobs: () => [], forgetSpawnedJobs: () => {}, shellFrameBytes: () => null,
    pendingTurnEffects: new Map(), stashLeftoverHostEffects: () => {},
    submitTurn: async (_actor: string, events: unknown[]) => { submitted.push(events); return { status: "idle", uiPatches: [], effects: [], deferredEvents: 0 }; },
  };
  const create = new Function(...Object.keys(ports), program + ";return createTurnDrivers;")(...Object.values(ports));
  const driver = create({ requireUiRoute: () => ({ lifecycle: { activation: {} }, accept: async () => [] }), requireActorId: () => "actor", executeFor: () => undefined, state: {}, closingInstances: new Set(), actorIdByInstance: new Map([[7, "actor"]]), pluginId: "fixture" });
  await driver.drainTypedOperations(7, [ack]);
  expect(submitted).toEqual([[ack]]);
});
