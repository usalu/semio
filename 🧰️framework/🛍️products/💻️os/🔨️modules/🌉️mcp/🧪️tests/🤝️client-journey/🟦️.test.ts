import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {inferenceClientVerdictV1,createHeadlessCallerSurfaceV1,inferenceCancellationVerdictV1,verifyActiveInferenceCancellationV1,type McpClientSessionV1,type McpClientStep} from "../../🟦️.ts";
import {encodeMcpComponentModelManifestV1} from "../../📡️stdio/📦️component-models/🟦️.ts";
import modelSchema from "../../📡️stdio/📦️component-models/🔣️.json";
import {OS_MCP_SCHEMA_DOCUMENT} from "../../🧬️schema/🟦️.ts";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json";
import inference from "./inference.json";
import context from "./context.json";
import cancellation from "./cancellation.json";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";
import initial from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🧫️initial-artifact.json";
import bitmapSchema from "../../../../../../../✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json";

test("typed creation sends distinct schema-admitted fixtures to the owner before persisting", () => {
 const oracle = new Ajv({strict:false}).addFormat("base64", /^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/u).compile(bitmapSchema);
 expect(oracle(initial.solve)).toBe(true);
 expect(oracle(initial.cancellation)).toBe(true);
 expect(oracle({...initial.solve, input:{...initial.solve.input, pixels:initial.invalidPixels}})).toBe(false);
 const db = new Database(":memory:");
 try {
  const size = db.query("SELECT json_extract(?1, '$.output.width') * json_extract(?1, '$.output.height') AS pixels");
  expect([initial.solve, initial.cancellation].map(row => size.get(JSON.stringify(row)))).toEqual([{pixels:1}, {pixels:4096}]);
  expect(atob(initial.result.pixels).length).toBe(1);
 } finally { db.close(); }
 const handler = readFileSync(new URL("../../🗿️artifact/🦀️.rs", import.meta.url), "utf8");
 const workspace = readFileSync(new URL("../../🏠️workspace/🛂️gateway-backend/🧩️artifact-creation-and-routing/🦀️.rs", import.meta.url), "utf8");
 expect(handler).toContain("create_plugin_artifact_cancellably(&artifact_id, declared, initial, scope)");
 expect(workspace.match(/import_initial_artifact[\s\S]*AppCommand::ReadArtifact[\s\S]*write_if_current\(/u)).not.toBeNull();
});

test("the cancellation fixture yields two distinct overlapping stripe patterns", () => {
 const {input, model} = initial.cancellation;
 expect(model).toEqual({patternSize:2, symmetry:1, periodicInput:true});
 const pixels = Buffer.from(input.pixels, "base64");
 expect(pixels.toString("base64")).toBe(input.pixels);
 expect([...pixels]).toEqual([0, 1]);
 expect(pixels.length).toBe(input.width * input.height);
 expect(input.palette.length).toBe(2);
 const patterns = Array.from(pixels, (_, index) => {
  const x = index % input.width;
  const y = Math.floor(index / input.width);
  return Array.from({length:model.patternSize ** 2}, (_, offset) => {
   const column = (x + offset % model.patternSize) % input.width;
   const row = (y + Math.floor(offset / model.patternSize)) % input.height;
   return pixels[row * input.width + column];
  }).join("");
 });
 expect([...new Set(patterns)].sort()).toEqual(["0101", "1010"]);
});

test("cancellation evidence requires a progressing active job and an unchanged artifact",()=>{
 const oracle=new Ajv({strict:false}).compile({type:"object",required:["active","terminal","unchanged"],properties:{active:{type:"object",required:["state","progress"],properties:{state:{enum:["ACCEPTED","RUNNING"]},progress:{type:"array",minItems:1}}},terminal:{type:"object",required:["state","cancelRequested"],properties:{state:{const:"CANCELLED"},cancelRequested:{const:true},commit:{type:"null"}}},unchanged:{const:true}}});
 for(const row of cancellation.cases){expect(Boolean(oracle(row))).toBe(row.expected);expect(inferenceCancellationVerdictV1(row.active,row.terminal,row.unchanged)).toBe(row.expected);}
});

test("cancellation gives the active job time to report progress and requires both persisted bytes", async () => {
 const oracle = new Ajv().compile({type:"object",required:["packBase64","sprBase64"],properties:{packBase64:{type:"string",minLength:1},sprBase64:{type:"string",minLength:1}}});
 const db = new Database(":memory:");
 try {
  const unchanged = db.query("SELECT json_extract(?1, '$.packBase64') = json_extract(?2, '$.packBase64') AND json_extract(?1, '$.sprBase64') = json_extract(?2, '$.sprBase64') AS valid");
  for (const row of cancellation.documentCases) {
   expect(Boolean(oracle(row.before) && oracle(row.after) && (unchanged.get(JSON.stringify(row.before),JSON.stringify(row.after)) as {valid:number}).valid), row.name).toBe(row.expected);
   let activeProgress = row.progressDelayMs === 0;
   let cancelComplete = false;
   let snapshotCount = 0;
   const timers: ReturnType<typeof setTimeout>[] = [];
   const job = () => cancelComplete ? {state:"CANCELLED",cancelRequested:true,progress:[]} : {state:"RUNNING",cancelRequested:false,progress:activeProgress ? [{fraction:0.2}] : []};
   const call = async (name: string): ReturnType<McpClientSessionV1["call"]> => {
    switch (name) {
     case "artifact_create": return {structuredContent:{}};
     case "artifact_snapshot": {
      const snapshot = snapshotCount++ === 0 ? row.before : row.after;
      const content = Object.fromEntries(Object.entries(snapshot));
      return {structuredContent:{untrusted:{schema:"semio.mcp.untrusted-content/v1",content}}};
     }
     case "inference_submit":
      if (row.progressDelayMs) timers.push(setTimeout(() => { activeProgress = true; }, row.progressDelayMs));
      return {structuredContent:{jobHandle:"job-1",job:job()}};
     case "inference_cancel":
      if (row.cancelDelayMs) timers.push(setTimeout(() => { cancelComplete = true; }, row.cancelDelayMs));
      else cancelComplete = true;
      return {structuredContent:{job:job()}};
     case "inference_events": return {structuredContent:{job:job()}};
     default: throw Error(`Unexpected inference fixture call ${name}`);
    }
   };
   const steps: McpClientStep[] = [];
   try {
    await verifyActiveInferenceCancellationV1({call}, pins, "artifact", step => steps.push(step));
    expect(steps.map(step => step.ok), row.name).toEqual([true, row.expected]);
   } finally { timers.forEach(clearTimeout); }
  }
 } finally { db.close(); }
});

test("headless caller owns an explicit schema-admitted surface within the published declaration",()=>{
 const valid=new Ajv({strict:false}).compile(OS_MCP_SCHEMA_DOCUMENT.$defs.ActionPrepareInput);
 const db=new Database(":memory:");
 db.run("CREATE TABLE declarations(app TEXT,mode TEXT,kind TEXT)");
 for(const mode of context.app.modes)for(const kind of context.app.windowKinds)db.run("INSERT INTO declarations VALUES(?,?,?)",[context.app.id,mode.id,kind.id]);
 for(const row of context.cases){
  const match=db.query("SELECT COUNT(*) AS count FROM declarations WHERE app=? AND mode=? AND kind=?").get(row.preferences.appId,row.preferences.modeId,row.preferences.windowKindId) as {count:number};
  const reference={capabilityId:"test.mutate",target:{pluginId:context.binding.pluginId,artifactRef:context.binding.artifactId,appId:row.preferences.appId},context:{locale:row.preferences.locale,terminology:row.preferences.terminology,activeModeId:row.preferences.modeId,windowId:context.binding.windowId,windowInstances:[{id:context.binding.windowId,windowKindId:row.preferences.windowKindId}],sessionIdentity:{userId:context.binding.principal,displayName:context.binding.principal}}};
  expect(match.count===1&&Boolean(valid(reference))).toBe(row.expected);
  if(row.expected){const surface=createHeadlessCallerSurfaceV1(context.app,context.binding,row.preferences);expect(surface.context).toEqual(reference.context);expect(surface.target).toEqual(reference.target);expect(Object.isFrozen(surface.context)).toBe(true);}
  else expect(()=>createHeadlessCallerSurfaceV1(context.app,context.binding,row.preferences)).toThrow();
 }
 db.close();
});

test("terminal inference output is typed and bound to the requested artifact",()=>{
 const oracle=new Ajv().compile({type:"object",required:["structuredContent"],properties:{isError:{const:false},structuredContent:{type:"object",required:[...Object.keys(inference.binding),"jobId","status","complete","payloadBytes","payload"],properties:{...Object.fromEntries(Object.entries(inference.binding).map(([name,value])=>[name,{const:value}])),jobId:{type:"string",minLength:1},status:{const:"SUCCEEDED"},complete:{const:true},payloadBytes:{type:"integer",minimum:1},payload:inference.outputSchema}}}});
 for(const row of inference.cases){
  expect(Boolean(oracle(row.reply))).toBe(row.expected);
  expect(inferenceClientVerdictV1(row.reply,inference.binding,inference.outputSchema).ok).toBe(row.expected);
 }
});

test("the native manifest binds each model to its mode, crate, and unique plugin", () => {
 const oracle = new Ajv().compile(modelSchema);
 for (const row of corpus.cases) {
  const db = new Database(":memory:");
  try {
   db.exec("CREATE TABLE owners (plugin TEXT PRIMARY KEY, relative_path TEXT CHECK(relative_path IS NOT NULL));");
   const manifest = {version: corpus.version, mode: row.mode, models: row.models.map(({location, ...model}) => ({...model, root: join(process.cwd(), ...location.split("/"))}))};
   let valid = Boolean(oracle(manifest));
   for (const model of row.models) {
    const key = createHash("sha256").update(model.cratePath).digest("hex").slice(0, 24);
    const expected = `browser/${row.mode}/components/${key}`;
    if (model.location !== expected || model.cratePath.split("/").includes("..") || !db.query("INSERT OR IGNORE INTO owners VALUES (?, ?)").run(model.pluginId, expected).changes) valid = false;
   }
   expect(valid, row.name).toBe(row.expected);
   if (row.expected) expect(JSON.parse(encodeMcpComponentModelManifestV1(manifest))).toEqual(manifest);
   else expect(() => encodeMcpComponentModelManifestV1(manifest), row.name).toThrow();
  } finally { db.close(); }
 }
});
