/** 🧬️ General codec results agree with the defining closed schema and independent binary and relational oracles. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import {runInNewContext} from "node:vm";
import {readFileSync} from "node:fs";
import {transformSync} from "esbuild";
import { ACTOR_CODEC_REFUSAL, actorCodecAnswer, admitActorCodecRequestV1, admitActorCodecAnswerV1, actorCodecRuntimeSourceV1, actorCodecSchema, type ActorCodecExports, type ActorCodecAnswer, type ActorCodecDocumentPair, type ActorCodecRequest } from "../../📦️packages/🟦️typescript/🧬️codec/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import fixtureSchema from "./🧫️fixtures/🧬️.json";

type Pair = { readonly packHex: string; readonly sprHex: string };
type ClosedAnswer = { readonly ok: Uint8Array | ActorCodecDocumentPair | readonly [string, string] } | { readonly fault: Uint8Array };
type Equal<A, B> = (<T>() => T extends A ? 1 : 2) extends (<T>() => T extends B ? 1 : 2) ? true : false;
const closedAnswer: Equal<ActorCodecAnswer, ClosedAnswer> = true;
const bytes = (hex: string): Uint8Array => Uint8Array.from(hex.match(/../g) ?? [], pair => Number.parseInt(pair, 16));
const hex = (value: Uint8Array): string => Array.from(value, byte => byte.toString(16).padStart(2, "0")).join("");
const pair = (value: Pair): ActorCodecDocumentPair => ({ pack: bytes(value.packHex), spr: bytes(value.sprHex) });

/** 🧾️ Each typed export has its own fixture value; fault identity is preserved through the actual owner. */
test("Codec exports return only the three schema-defined successes or the original guest fault", async () => {
  expect(closedAnswer).toBe(true);
  const ajv = new Ajv({ strict: true }).addSchema(actorCodecSchema);
  expect(ajv.validate(fixtureSchema, fixture), JSON.stringify(ajv.errors)).toBe(true);
  const answerSchema = ajv.compile({ $ref: actorCodecSchema.$id + "#/$defs/ActorCodecAnswerProjection" });
  for (const invalid of fixture.invalidAnswers) expect(answerSchema(invalid)).toBe(false);
  expect(fixture.refusals).toEqual(ACTOR_CODEC_REFUSAL);
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE codec_result(id TEXT PRIMARY KEY, expected TEXT NOT NULL, observed TEXT NOT NULL)");
    for (const row of fixture.cases) {
      const component = row.component, calls: string[] = [], pairs: ActorCodecDocumentPair[] = [], fault = bytes(component.faultHex ?? "");
      const owners = { "pack-schema-hash": bytes(component.bytesHex ?? ""), genesis: pair(component.pair ?? { packHex: "", sprHex: "" }), "print-mirror": [component.mirror?.[0] ?? "", component.mirror?.[1] ?? ""] as readonly [string, string] };
      const hostTrap = new Error(component.throws ?? "Unused host trap");
      const refuse = (): void => {
        if (component.throws) throw hostTrap;
        if (component.faultHex !== undefined) throw Object.assign(new Error("guest fault"), { payload: { tag: "fault", val: fault } });
      };
      const codec: ActorCodecExports | undefined = component.exported === false ? undefined : {
        packSchemaHash: async kind => { calls.push(kind); refuse(); return owners["pack-schema-hash"]; },
        genesis: async (kind, documentId) => { calls.push(kind, documentId); refuse(); return owners.genesis; },
        printMirror: async (kind, value) => { calls.push(kind); pairs.push(value); refuse(); return owners["print-mirror"]; },
      };
      const request = (row.request.pair ? { ...row.request, pair: pair(row.request.pair) } : row.request) as ActorCodecRequest;
      let answer: ActorCodecAnswer | undefined, observed: object = {};
      try { answer = await actorCodecAnswer(codec, request); }
      catch (error) { if (component.throws) expect(error).toBe(hostTrap); observed = { refusal: error instanceof Error ? error.message : String(error) }; }
      if (answer) {
        if ("ok" in answer) expect(answer.ok).toBe(owners[request.operation]);
        if ("fault" in answer) { expect(answer.fault).toBe(fault); observed = { faultHex: hex(answer.fault) }; }
        else if (answer.ok instanceof Uint8Array) observed = { okHex: hex(answer.ok) };
        else if ("pack" in answer.ok) observed = { okPair: { packHex: hex(answer.ok.pack), sprHex: hex(answer.ok.spr) } };
        else observed = { okMirror: [...answer.ok] };
        expect(answerSchema(observed), row.id).toBe(true);
      }
      for (const value of [component.bytesHex, component.faultHex, component.pair?.packHex, component.pair?.sprHex, row.request.pair?.packHex, row.request.pair?.sprHex]) {
        if (value !== undefined) { expect([...bytes(value)]).toEqual([...Buffer.from(value, "hex")]); expect(hex(bytes(value))).toBe(Buffer.from(value, "hex").toString("hex")); }
      }
      if (calls.length) expect(calls).toEqual(row.request.operation === "genesis" ? [row.request.artifactKind, row.request.documentId!] : [row.request.artifactKind]);
      if (row.request.pair && request.operation === "print-mirror") { expect(pairs).toHaveLength(1); expect(pairs[0]).toBe(request.pair); }
      expect(observed, row.id).toEqual(row.expected);
      db.query("INSERT INTO codec_result VALUES(?,?,?)").run(row.id, JSON.stringify(row.expected), JSON.stringify(observed));
    }
    expect(db.query("SELECT count(*) AS mismatches FROM codec_result WHERE json(expected)<>json(observed)").get()).toEqual({ mismatches: 0 });
    expect(db.query("SELECT count(*) AS cases FROM codec_result").get()).toEqual({ cases: fixture.cases.length });
  } finally { db.close(); }
});


/** 🪞️ The portable JSON projection reconstructs only fixture owners, preserving malformed fields for actual admission. */
function runtimeAnswer(projection: Record<string, unknown>): unknown {
 const output:Record<string,unknown>={};
 for(const[name,value]of Object.entries(projection)){
  if(name==="okHex"||name==="faultHex")output[name==="okHex"?"ok":"fault"]=typeof value==="string"?bytes(value):value;
  else if(name==="okPair"&&value!==null&&typeof value==="object"){const pair:Record<string,unknown>={};for(const[name,item]of Object.entries(value))pair[name==="packHex"?"pack":name==="sprHex"?"spr":name]=typeof item==="string"&&(name==="packHex"||name==="sprHex")?bytes(item):item;output.ok=pair;}
  else output[name==="okMirror"?"ok":name]=value;
 }
 return output;
}

/** 🎯️ Each actual answer-law call receives only its operation's defining request fields. */
function operationRequest(operation:ActorCodecRequest["operation"]):ActorCodecRequest {
 if(operation==="pack-schema-hash")return{operation,artifactKind:"probe"};
 if(operation==="genesis")return{operation,artifactKind:"probe",documentId:"document"};
 return{operation,artifactKind:"probe",pair:pair({packHex:"00",sprHex:"ff"})};
}
/** 🧫️ Decodes only the portable request fixture's byte projection while retaining malformed fields. */
function runtimeRequest(value:unknown):unknown {
 if(value===null||typeof value!=="object"||Array.isArray(value))return value;
 return Object.fromEntries(Object.entries(value).map(([name,item])=>[name,name==="pair"&&item!==null&&typeof item==="object"&&!Array.isArray(item)?Object.fromEntries(Object.entries(item).map(([key,value])=>[key==="packHex"?"pack":key==="sprHex"?"spr":key,(key==="packHex"||key==="sprHex")&&typeof value==="string"?bytes(value):value])):item]));
}

test("closed portable requests agree with Ajv Buffer and SQLite before any guest or ShardClient effect",async()=>{
 const {ShardClient}=await import("../../📮️shard-client/🟦️.ts"),oracle=new Ajv({strict:true}).addSchema(actorCodecSchema).compile({$ref:actorCodecSchema.$id+"#/$defs/ActorCodecRequest"}),db=new Database(":memory:");db.exec("CREATE TABLE requests(id TEXT, expected INTEGER, actual INTEGER, independent INTEGER)");
 try{for(const row of fixture.requestCases){const request=runtimeRequest(row.request),effects:string[]=[],slot={},success=new Uint8Array([1]),codec:ActorCodecExports={packSchemaHash:async()=>{effects.push("guest");return success;},genesis:async()=>{effects.push("guest");return{pack:success,spr:success};},printMirror:async(_kind,value)=>{effects.push("guest");expect(value===(request as {pair:unknown}).pair).toBe(true);return["body","history"];}},client={requireShard:()=>{effects.push("slot");return slot;},captureActorActivation:()=>{effects.push("capture");return{activationGeneration:1n};},nextRequestId:()=>{effects.push("sequence");return"request";},send:async(_slot:unknown,message:{request:unknown})=>{effects.push("transport");expect(message.request).toBe(request);return{fault:success};}};let accepted=false;
  try{expect(admitActorCodecRequestV1(request)===request).toBe(true);accepted=true;}catch(error){if(!("refusal"in row)||typeof row.refusal!=="string")throw error;expect((error as Error).message).toBe(row.refusal);}expect(accepted,row.id).toBe(row.accepted);const independent=oracle(row.request);expect(independent,row.id).toBe(row.accepted);db.query("INSERT INTO requests VALUES(?,?,?,?)").run(row.id,Number(row.accepted),Number(accepted),Number(independent));
  if(row.accepted){await Reflect.apply(actorCodecAnswer,undefined,[codec,request]);await Reflect.apply(ShardClient.prototype.codec,client,["actor",request]);expect(effects).toEqual(["guest","slot","capture","sequence","transport"]);if(request!==null&&typeof request==="object"&&"pair"in request){const owned=request.pair as ActorCodecDocumentPair,projected=(row.request as {pair:{packHex:string;sprHex:string}}).pair;expect([...owned.pack]).toEqual([...Buffer.from(projected.packHex,"hex")]);expect([...owned.spr]).toEqual([...Buffer.from(projected.sprHex,"hex")]);}}
  else{const refusal="refusal"in row?row.refusal:"missing refusal";await expect(Reflect.apply(actorCodecAnswer,undefined,[codec,request])).rejects.toThrow(refusal);await expect(Reflect.apply(ShardClient.prototype.codec,client,["actor",request])).rejects.toThrow(refusal);expect(effects).toEqual([]);}
 }
 expect(db.query("SELECT count(*) AS mismatches FROM requests WHERE expected<>actual OR actual<>independent").get()).toEqual({mismatches:0});}finally{db.close();}
});

test("foreign request pairs preserve complete identity and accessor symbol inherited or spoofed inputs cause no effects",async()=>{
 const {ShardClient}=await import("../../📮️shard-client/🟦️.ts"),foreign=runInNewContext('({operation:"print-mirror",artifactKind:"作品",pair:{pack:new Uint8Array([1]),spr:new Uint8Array([2])}})') as Extract<ActorCodecRequest,{operation:"print-mirror"}>;expect(foreign.pair.pack instanceof Uint8Array).toBe(false);expect(admitActorCodecRequestV1(foreign)).toBe(foreign);expect(admitActorCodecRequestV1(foreign).pair).toBe(foreign.pair);expect(admitActorCodecRequestV1(foreign).pair.pack).toBe(foreign.pair.pack);expect(admitActorCodecRequestV1(foreign).pair.spr).toBe(foreign.pair.spr);
 let reads=0,effects=0;const codec:ActorCodecExports={packSchemaHash:async()=>{effects++;return foreign.pair.pack;},genesis:async()=>{effects++;return foreign.pair;},printMirror:async(_kind,value)=>{effects++;expect(value).toBe(foreign.pair);return["body","history"];}},client={requireShard:()=>{effects++;return{};},captureActorActivation:()=>{effects++;return{activationGeneration:1n};},nextRequestId:()=>{effects++;return"request";},send:async(_slot:unknown,message:{request:unknown})=>{effects++;expect(message.request).toBe(foreign);return{fault:foreign.pair.pack};}};
 await actorCodecAnswer(codec,foreign);await Reflect.apply(ShardClient.prototype.codec,client,["actor",foreign]);expect(effects).toBe(5);effects=0;
 const bad=[{get operation(){reads++;return"pack-schema-hash";},artifactKind:"note"},{operation:"pack-schema-hash",get artifactKind(){reads++;return"note";}},{operation:"genesis",artifactKind:"note",get documentId(){reads++;return"document";}},{operation:"print-mirror",artifactKind:"note",get pair(){reads++;return foreign.pair;}},{...foreign,pair:{get pack(){reads++;return foreign.pair.pack;},spr:foreign.pair.spr}},{...foreign,pair:{pack:foreign.pair.pack,get spr(){reads++;return foreign.pair.spr;}}},{operation:"pack-schema-hash",artifactKind:"note",[Symbol("extra")]:true},{...foreign,pair:{...foreign.pair,[Symbol("extra")]:true}},Object.create({operation:"pack-schema-hash",artifactKind:"note"}),{operation:{[Symbol.toPrimitive](){reads++;return"pack-schema-hash";},toString(){reads++;return"pack-schema-hash";}},artifactKind:"note"},{operation:"pack-schema-hash",artifactKind:{toString(){reads++;return"note";}}},{operation:"genesis",artifactKind:"note",documentId:{[Symbol.toPrimitive](){reads++;return"document";}}},{...foreign,pair:{pack:Object.defineProperty(new Int8Array([1]),Symbol.toStringTag,{value:"Uint8Array"}),spr:foreign.pair.spr}}];
 for(const request of bad){expect(()=>admitActorCodecRequestV1(request)).toThrow("actor-codec.request");await expect(Reflect.apply(actorCodecAnswer,undefined,[codec,request])).rejects.toThrow("actor-codec.request");await expect(Reflect.apply(ShardClient.prototype.codec,client,["actor",request])).rejects.toThrow("actor-codec.request");}expect(reads).toBe(0);expect(effects).toBe(0);
});

test("response correlation retains the admitted primitive operation while the original request owner changes during await",async()=>{
 const {ShardClient}=await import("../../📮️shard-client/🟦️.ts"),hash=new Uint8Array([1]),request={operation:"pack-schema-hash",artifactKind:"probe"} as const,codec:ActorCodecExports={packSchemaHash:async()=>{Object.defineProperty(request,"operation",{value:"genesis"});return hash;},genesis:async()=>({pack:hash,spr:hash}),printMirror:async()=>["one","two"]};
 const guestAnswer=await actorCodecAnswer(codec,request);expect("ok"in guestAnswer&&guestAnswer.ok===hash).toBe(true);
 const receiving={operation:"pack-schema-hash",artifactKind:"probe"} as const,answer={ok:hash},client={requireShard:()=>({}),captureActorActivation:()=>({activationGeneration:1n}),nextRequestId:()=>"request",send:async(_slot:unknown,message:{request:unknown})=>{expect(message.request).toBe(receiving);Object.defineProperty(receiving,"operation",{value:"genesis"});return answer;}};
 expect(await Reflect.apply(ShardClient.prototype.codec,client,["actor",receiving])).toBe(answer);
});

test("selected export access retains admitted scalar inputs and the original pair without reconstructing a request",async()=>{
 const pack=new Uint8Array([1]),spr=new Uint8Array([2]),owned={pack,spr},genesis={operation:"genesis",artifactKind:"original-kind",documentId:"original-document"} as const,mirror={operation:"print-mirror",artifactKind:"original-kind",pair:owned} as const,calls:unknown[][]=[];
 const codec:ActorCodecExports={packSchemaHash:async()=>pack,get genesis(){Object.defineProperty(genesis,"artifactKind",{value:"changed-kind"});Object.defineProperty(genesis,"documentId",{value:"changed-document"});return async(kind:string,documentId:string)=>{calls.push([kind,documentId]);return owned;};},get printMirror(){Object.defineProperty(mirror,"artifactKind",{value:"changed-kind"});Object.defineProperty(mirror,"pair",{value:{pack:new Uint8Array(),spr:new Uint8Array()}});return async(kind:string,value:ActorCodecDocumentPair)=>{calls.push([kind,value]);return["one","two"]as const;};}};
 await actorCodecAnswer(codec,genesis);await actorCodecAnswer(codec,mirror);expect(calls[0]).toEqual(["original-kind","original-document"]);expect(calls[1]?.[0]).toBe("original-kind");expect(calls[1]?.[1]===owned).toBe(true);expect(owned.pack).toBe(pack);expect(owned.spr).toBe(spr);
});

test("operation-specific runtime admission refuses malformed guest success and preserves complete owner identity",async()=>{
 const ajv=new Ajv({strict:true}).addSchema(actorCodecSchema),oracle=ajv.compile({$ref:actorCodecSchema.$id+"#/$defs/ActorCodecOperationAnswerProjection"}),db=new Database(":memory:");
 db.exec("CREATE TABLE admission(id TEXT PRIMARY KEY,expected INTEGER,actual INTEGER,independent INTEGER)");
 try{for(const row of fixture.runtimeCases){const operation=row.operation as ActorCodecRequest["operation"],owner=runtimeAnswer(row.answer),request=operationRequest(operation);let admitted=false;try{expect(admitActorCodecAnswerV1(operation,owner)===owner).toBe(true);admitted=true;}catch(error){expect((error as Error).message).toBe("actor-codec.answer:"+operation);}expect(admitted,row.id).toBe(row.accepted);const independent=oracle({operation,answer:row.answer});expect(independent,row.id).toBe(row.accepted);db.query("INSERT INTO admission VALUES(?,?,?,?)").run(row.id,Number(row.accepted),Number(admitted),Number(independent));
  if(owner!==null&&typeof owner==="object"&&"ok"in owner){const success=owner.ok,codec={packSchemaHash:async()=>success,genesis:async()=>success,printMirror:async()=>success} as ActorCodecExports;if(row.accepted){const answer=await actorCodecAnswer(codec,request);expect("ok"in answer&&answer.ok===success).toBe(true);if(operation==="genesis"&&success!==null&&typeof success==="object"&&"pack"in success&&"spr"in success){expect("ok"in answer&&answer.ok===success).toBe(true);expect((answer as {ok:ActorCodecDocumentPair}).ok.pack===success.pack).toBe(true);expect((answer as {ok:ActorCodecDocumentPair}).ok.spr===success.spr).toBe(true);}}else if(Object.keys(owner).length===1)await expect(actorCodecAnswer(codec,request)).rejects.toThrow("actor-codec.answer:"+operation);}
 }
 expect(db.query("SELECT count(*)AS mismatches FROM admission WHERE expected<>actual OR actual<>independent").get()).toEqual({mismatches:0});expect(db.query("SELECT count(*)AS cases FROM admission").get()).toEqual({cases:fixture.runtimeCases.length});
 }finally{db.close();}
});

test("genuine foreign-realm bytes, pairs, mirrors and guest faults retain original owners while spoofed brands refuse",async()=>{
 const foreign=runInNewContext('({bytes:new Uint8Array([0,255]),pair:{pack:new Uint8Array([1]),spr:new Uint8Array([2])},mirror:["foreign","history"]})') as {bytes:Uint8Array;pair:ActorCodecDocumentPair;mirror:readonly[string,string]};expect(foreign.bytes instanceof Uint8Array).toBe(false);expect(foreign.pair.pack instanceof Uint8Array).toBe(false);expect([...foreign.bytes]).toEqual([...Buffer.from([0,255])]);
 const requests=[{operation:"pack-schema-hash",artifactKind:"kind"},{operation:"genesis",artifactKind:"kind",documentId:"document"},{operation:"print-mirror",artifactKind:"kind",pair:foreign.pair}] as const,successes=[foreign.bytes,foreign.pair,foreign.mirror],codec:ActorCodecExports={packSchemaHash:async()=>foreign.bytes,genesis:async()=>foreign.pair,printMirror:async(_kind,value)=>{expect(value).toBe(foreign.pair);return foreign.mirror;}};
 const inline=runInNewContext(actorCodecRuntimeSourceV1()+";({admitActorCodecAnswerV1,actorCodecAnswer:codecAnswer})") as {admitActorCodecAnswerV1:typeof admitActorCodecAnswerV1;actorCodecAnswer:typeof actorCodecAnswer};
 for(let index=0;index<requests.length;index++){const request=requests[index]!,owner={ok:successes[index]};expect(admitActorCodecAnswerV1(request.operation,owner)===owner).toBe(true);expect(inline.admitActorCodecAnswerV1(request.operation,owner)===owner).toBe(true);expect((await actorCodecAnswer(codec,request) as {ok:unknown}).ok).toBe(successes[index]);expect((await inline.actorCodecAnswer(codec,request) as {ok:unknown}).ok).toBe(successes[index]);}
 const fault=runInNewContext('new Uint8Array([7,8])') as Uint8Array,guest=Object.assign(new Error("original guest"),{payload:{tag:"fault",val:fault}}),faultCodec:ActorCodecExports={...codec,genesis:async()=>{throw guest;}};expect((await actorCodecAnswer(faultCodec,requests[1]) as {fault:Uint8Array}).fault).toBe(fault);expect((await inline.actorCodecAnswer(faultCodec,requests[1]) as {fault:Uint8Array}).fault).toBe(fault);
 const spoof=Object.defineProperty(new Int8Array([1]),Symbol.toStringTag,{value:"Uint8Array"}),bad=[spoof,new Uint8ClampedArray([1]),new DataView(new ArrayBuffer(1)),[1],{0:1,length:1,[Symbol.toStringTag]:"Uint8Array"}];for(const value of bad){expect(()=>admitActorCodecAnswerV1("pack-schema-hash",{ok:value})).toThrow("actor-codec.answer");expect(()=>admitActorCodecAnswerV1("genesis",{fault:value})).toThrow("actor-codec.answer");const trap=Object.assign(new Error("original invalid fault"),{payload:{tag:"fault",val:value}});try{await actorCodecAnswer({...codec,genesis:async()=>{throw trap;}},requests[1]);throw Error("Expected trap");}catch(error){expect(error).toBe(trap);}}
 const extraPair={...foreign.pair,extra:true},extraMirror=Object.assign(["one","two"],{extra:true}),symbolAnswer={ok:foreign.bytes,[Symbol("extra")]:true};for(const[operation,value]of[["genesis",extraPair],["print-mirror",extraMirror]]as const)expect(()=>admitActorCodecAnswerV1(operation,{ok:value})).toThrow("actor-codec.answer");expect(()=>admitActorCodecAnswerV1("pack-schema-hash",symbolAnswer)).toThrow("actor-codec.answer");let read=false;expect(()=>admitActorCodecAnswerV1("pack-schema-hash",{get ok(){read=true;return foreign.bytes;}})).toThrow("actor-codec.answer");expect(read).toBe(false);
 await expect(actorCodecAnswer({}as ActorCodecExports,requests[0])).rejects.toThrow("actor-codec.unexported");
});


test("ShardClient codec receives transport unknown through the defining operation-aware answer admission",async()=>{
 const {ShardClient}=await import("../../📮️shard-client/🟦️.ts");for(const row of fixture.runtimeCases){const owner=runtimeAnswer(row.answer),slot={},request=operationRequest(row.operation as ActorCodecRequest["operation"]),messages:unknown[]=[],client={requireShard:()=>slot,captureActorActivation:()=>({activationGeneration:1n}),nextRequestId:()=>"codec-receive",send:async(received:unknown,message:unknown,id:unknown)=>{expect(received).toBe(slot);expect(id).toBe("codec-receive");messages.push(message);return owner;}};
  const receiving=Reflect.apply(ShardClient.prototype.codec,client,["actor",request]);if(row.accepted)expect((await receiving)===owner,row.id).toBe(true);else await expect(receiving).rejects.toThrow("actor-codec.answer:"+row.operation);expect(messages).toEqual([{kind:"codec",requestId:"codec-receive",actorId:"actor",activationGeneration:1n,request}]);
 }
});


test("identifier-minified defining codec serialization retains its complete zero-import lexical scope",async()=>{
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),emitted=transformSync(source,{loader:"ts",format:"cjs",minifyIdentifiers:true,target:"es2022"}).code,exports=runInNewContext("const module={exports:{}};const exports=module.exports;"+emitted+";module.exports") as {actorCodecRuntimeSourceV1:()=>string},runtime=runInNewContext(exports.actorCodecRuntimeSourceV1()+";({codecAnswer,admitActorCodecRequestV1,admitActorCodecAnswerV1})")as {codecAnswer:typeof actorCodecAnswer;admitActorCodecRequestV1:typeof admitActorCodecRequestV1;admitActorCodecAnswerV1:typeof admitActorCodecAnswerV1};
 const owner=new Uint8Array([1,2]),codec:ActorCodecExports={packSchemaHash:async()=>owner,genesis:async()=>({pack:owner,spr:owner}),printMirror:async()=>["one","two"]},request={operation:"pack-schema-hash",artifactKind:"probe"}as const;expect(runtime.admitActorCodecRequestV1(request)).toBe(request);expect((await runtime.codecAnswer(codec,request)as {ok:Uint8Array}).ok).toBe(owner);await expect(runtime.codecAnswer(codec,{...request,documentId:"unused"})).rejects.toThrow("actor-codec.request");let coerced=false;expect(()=>runtime.admitActorCodecRequestV1({operation:{toString(){coerced=true;return"pack-schema-hash";}},artifactKind:"probe"})).toThrow("actor-codec.request");expect(coerced).toBe(false);await expect(runtime.codecAnswer({...codec,genesis:async()=>owner as never},{operation:"genesis",artifactKind:"probe",documentId:"document"})).rejects.toThrow("actor-codec.answer:genesis");
});
