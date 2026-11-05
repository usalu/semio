import {test,expect} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,writeFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {verifyStagedPluginComponent,inferenceClientVerdictV1,createHeadlessCallerSurfaceV1,inferenceCancellationVerdictV1} from "../../🟦️.ts";
import {OS_MCP_SCHEMA_DOCUMENT} from "../../🧬️schema/🟦️.ts";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json";
import inference from "./inference.json";
import context from "./context.json";
import cancellation from "./cancellation.json";

test("cancellation evidence requires a progressing active job and an unchanged artifact",()=>{
 const oracle=new Ajv({strict:false}).compile({type:"object",required:["active","terminal","unchanged"],properties:{active:{type:"object",required:["state","progress"],properties:{state:{enum:["ACCEPTED","RUNNING"]},progress:{type:"array",minItems:1}}},terminal:{type:"object",required:["state","cancelRequested"],properties:{state:{const:"CANCELLED"},cancelRequested:{const:true},commit:{type:"null"}}},unchanged:{const:true}}});
 for(const row of cancellation.cases){expect(Boolean(oracle(row))).toBe(row.expected);expect(inferenceCancellationVerdictV1(row.active,row.terminal,row.unchanged)).toBe(row.expected);}
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

test("client preflight admits only the current explicitly selected component publication",()=>{
 const generated=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!generated)throw Error("Explicit ticket generated directory required");
 mkdirSync(generated,{recursive:true});
 const root=mkdtempSync(join(generated,"guest-preflight-"));
 const oracle=new Ajv().compile({type:"object",required:["catalog","plugin","component","matching"],properties:{catalog:{const:true},plugin:{const:true},component:{const:"prototype"},matching:{const:true}}});
 try{
  for(const [index,row]of corpus.cases.entries()){
   const repo=join(root,String(index)),catalog=join(repo,"catalog"),owner=join(repo,"plugin"),crate="plugin/packages/rust";
   mkdirSync(catalog,{recursive:true});
   mkdirSync(join(owner,"packages/rust/dist/components",row.component),{recursive:true});
   const bytes=Buffer.from("genuine preflight test component");
   const digest=createHash("sha256").update(row.matching?bytes:Buffer.from("other generation")).digest("hex");
   if(row.catalog)writeFileSync(join(catalog,"🦀️catalog.json"),JSON.stringify({version:1,plugins:row.plugin?[{pluginId:"note",cratePath:crate,wasmOut:"note.wasm",role:"plugin",extends:null,capabilities:[],activationEvents:[],host:null}]:[],playgrounds:[]}));
   writeFileSync(join(owner,"🔣️.json"),JSON.stringify({hashes:{wasmSha256:digest}}));
   writeFileSync(join(owner,"packages/rust/dist/components",row.component,"note.wasm"),bytes);
   expect(Boolean(oracle(row))).toBe(row.expected);
   expect(verifyStagedPluginComponent(repo,"note",catalog,"prototype").ok).toBe(row.expected);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
