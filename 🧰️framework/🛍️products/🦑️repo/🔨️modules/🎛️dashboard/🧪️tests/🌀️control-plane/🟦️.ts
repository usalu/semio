import { expect, test } from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

test("Node readline independently decodes the native control leader vectors", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/⌨️controls/🔣️.json",import.meta.url),"utf8"));
  const source="const {PassThrough}=require('node:stream'),{emitKeypressEvents}=require('node:readline'),fixture=JSON.parse(process.argv[1]),input=new PassThrough(),events=[];emitKeypressEvents(input);input.on('keypress',(_,key)=>events.push({key:key.sequence==='\\u0000'&&key.ctrl?' ':!key.ctrl&&key.sequence.length===1?key.sequence:key.name==='space'?' ':key.name,ctrl:Boolean(key.ctrl)}));input.end(Buffer.from(fixture.bytes));setImmediate(()=>console.log(JSON.stringify(events)));";
  const child=Bun.spawn(["node","-e",source,JSON.stringify(fixture)],{stdout:"pipe",stderr:"pipe"});
  const output=await new Response(child.stdout).text();expect(await child.exited,await new Response(child.stderr).text()).toBe(0);expect(JSON.parse(output)).toEqual(fixture.events);
});

test("JavaScript independently selects the shared searchable launcher vectors", () => {
  const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔎️launcher/🔣️.json",import.meta.url),"utf8"));
  for(const vector of fixture.cases){const tokens=vector.filter.toLocaleLowerCase().split(/\s+/).filter(Boolean),index=fixture.options.findIndex((value:string)=>tokens.every((token:string)=>value.toLocaleLowerCase().includes(token)));expect(index<0?null:index).toBe(vector.index);}
  for(const vector of fixture.pointer){const tokens=vector.filter.toLocaleLowerCase().split(/\s+/).filter(Boolean),visible=fixture.options.map((value:string,index:number)=>({value,index})).filter(({value}:{value:string})=>tokens.every((token:string)=>value.toLocaleLowerCase().includes(token))),offset=Math.max(0,vector.selected-(vector.height-2));expect(vector.row<1?null:visible[offset+vector.row-1]?.index??null).toBe(vector.index);}
});

test("Ajv and ordinary object projection independently resolve optional dashboard preferences", () => {
  const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/⚙️preferences/🔣️.json",import.meta.url),"utf8"));
  const vectors=JSON.parse(readFileSync(new URL("../🧫️fixtures/⚙️preferences/🔣️.json",import.meta.url),"utf8"));
  const validate=new Ajv({strict:false}).compile(schema);
  for(const value of vectors.valid)expect(validate(value),JSON.stringify(validate.errors)).toBe(true);
  for(const value of vectors.invalid)expect(validate(value),JSON.stringify(value)).toBe(false);
  for(const vector of vectors.cases)expect(Object.assign({},vectors.defaults,...vector.layers)).toEqual(vector.expected);
});

test("Ajv independently validates the shared dashboard protocol vectors", () => {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🌀️daemon/🔣️.json", import.meta.url), "utf8"));
  const vectors = JSON.parse(readFileSync(new URL("../🧫️fixtures/🌀️control-plane/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: false }).compile(schema);
  for (const message of vectors.valid) expect(validate(message), JSON.stringify(validate.errors)).toBe(true);
  for (const message of vectors.invalid) expect(validate(message), JSON.stringify(message)).toBe(false);
});


/** 🧮️ Runs Nx's own `createTaskGraph` in a child whose workspace is a private empty directory, so the oracle never loads the repository's bounded Nx namespace. */
async function nxTaskGraphOracle(nodes:Record<string,unknown>,calls:readonly{projects:readonly string[];targets:readonly string[];overrides:Record<string,string>}[]):Promise<{tasks:string[];overrides:unknown}[]>{
  const root=mkdtempSync(join(tmpdir(),"nx-oracle-"));
  try{
    writeFileSync(join(root,"nx.json"),"{}");
    const source="const fs=require('node:fs'),input=JSON.parse(fs.readFileSync(0,'utf8')),{createTaskGraph}=require(process.argv[1]),graph={nodes:input.nodes,dependencies:Object.fromEntries(Object.keys(input.nodes).map(name=>[name,[]]))};process.stdout.write(JSON.stringify(input.calls.map(call=>{const tasks=createTaskGraph(graph,{},call.projects,call.targets,undefined,call.overrides).tasks,ids=Object.keys(tasks);return{tasks:ids,overrides:ids.length?tasks[ids[0]].overrides:null};})));";
    const environment=Object.fromEntries(Object.entries(process.env).filter(([name])=>!name.startsWith("NX_")&&!name.startsWith("SEMIO_NX_")));
    const child=Bun.spawn([process.execPath,"-e",source,Bun.resolveSync("nx/src/tasks-runner/create-task-graph.js",import.meta.dir)],{cwd:root,env:{...environment,NX_WORKSPACE_ROOT_PATH:root,NX_DAEMON:"false"},stdin:new Blob([JSON.stringify({nodes,calls})]),stdout:"pipe",stderr:"pipe"});
    const output=await new Response(child.stdout).text();expect(await child.exited,await new Response(child.stderr).text()).toBe(0);
    return JSON.parse(output);
  }finally{rmSync(root,{recursive:true,force:true});}
}

test("Nx independently schedules every inferred command discovery vector", async () => {
  const {graph,expected} = JSON.parse(readFileSync(new URL("../🧫️fixtures/🌳️inferred-targets/🔣️.json",import.meta.url),"utf8"));
  const nodes=Object.fromEntries(Object.entries<unknown>(graph.projects).map(([name,data])=>[name,{name,type:"lib",data}]));
  const results=await nxTaskGraphOracle(nodes,expected.map((id:string)=>{const split=id.lastIndexOf(":");return{projects:[id.slice(0,split)],targets:[id.slice(split+1)],overrides:{}};}));
  expect(results.flatMap(result=>result.tasks).sort()).toEqual(expected);
});

test("Nx independently schedules every configured renderer language selection", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🗣️launch-axes/🔣️.json",import.meta.url),"utf8"));
  const project="@semio-tech/framework-os-dev";
  const targets=Object.fromEntries(fixture.renderers.map((row:{target:string})=>[row.target,{executor:"nx:run-commands",options:{command:"bun fixture"}}]));
  const rows=fixture.renderers.flatMap((row:{target:string})=>fixture.locales.flatMap((locale:string)=>fixture.terminologies.map((terminology:string)=>({target:row.target,expected:{SEMIO_LOCKED_LOCALE:locale,SEMIO_LOCKED_TERMINOLOGY:terminology}}))));
  const results=await nxTaskGraphOracle({[project]:{name:project,type:"lib",data:{root:"dev",targets}}},rows.map((row:{target:string;expected:Record<string,string>})=>({projects:[project],targets:[row.target],overrides:row.expected})));
  rows.forEach((row:{target:string;expected:Record<string,string>},index:number)=>{expect(results[index]!.tasks).toEqual([`${project}:${row.target}`]);expect(results[index]!.overrides).toEqual(row.expected);});
});

test("Ajv independently validates every dashboard declaration of the registry workspace vector and the workspace root", () => {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🎮️registry/🔣️.json", import.meta.url), "utf8"));
  const workspace = JSON.parse(readFileSync(new URL("../🧫️fixtures/🎮️registry/🏗️workspace.json", import.meta.url), "utf8"));
  const root = JSON.parse(readFileSync(new URL("../../../../../../../📋️project.json", import.meta.url), "utf8"));
  const ajv = new Ajv2020({ strict: false });
  ajv.addSchema(schema);
  const validate = (definition: string) => ajv.getSchema(`${schema.$id}#/$defs/${definition}`)!;
  const manifests = workspace.files.filter((file: { path: string }) => file.path.endsWith("📋️project.json"));
  const commands = workspace.files.filter((file: { path: string }) => file.path.endsWith("🎮️commands.json"));
  expect(manifests.length).toBeGreaterThan(0);
  expect(commands.length).toBeGreaterThan(0);
  for (const file of manifests) expect(validate("ProjectManifest")(file.json), `${file.path}: ${JSON.stringify(validate("ProjectManifest").errors)}`).toBe(true);
  for (const file of commands) expect(validate("TicketCommands")(file.json), `${file.path}: ${JSON.stringify(validate("TicketCommands").errors)}`).toBe(true);
  expect(validate("ProjectManifest")(root), JSON.stringify(validate("ProjectManifest").errors)).toBe(true);
  const declared = (manifest: { metadata?: { semio?: { dashboard?: { tools?: unknown[]; compounds?: unknown[] } } } }) => (manifest.metadata?.semio?.dashboard?.tools?.length ?? 0) + (manifest.metadata?.semio?.dashboard?.compounds?.length ?? 0);
  expect(manifests.reduce((count: number, file: { json: Parameters<typeof declared>[0] }) => count + declared(file.json), 0)).toBeGreaterThan(0);
  expect(validate("ProjectManifest")({ metadata: { semio: { dashboard: { tools: [{ id: "no command" }] } } } })).toBe(false);
  expect(validate("ProjectManifest")({ metadata: { semio: { dashboard: { launch: [] } } } })).toBe(false);
});

test("Ajv sees every message type, session status and error code of the daemon schema in a valid vector", () => {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🌀️daemon/🔣️.json", import.meta.url), "utf8"));
  const vectors = JSON.parse(readFileSync(new URL("../🧫️fixtures/🌀️control-plane/🔣️.json", import.meta.url), "utf8"));
  const types = ["ClientMessage", "ServerMessage"].flatMap((name) => schema.definitions[name].oneOf.map((variant: { properties: { type: { const: string } } }) => variant.properties.type.const));
  for (const type of types) expect(vectors.valid.some((message: { type: string }) => message.type === type), `message type ${type}`).toBe(true);
  const sessions = vectors.valid.flatMap((message: { session?: { status: string }; sessions?: { status: string }[] }) => [message.session, ...(message.sessions ?? [])]).filter(Boolean);
  for (const status of schema.definitions.Session.properties.status.enum) expect(sessions.some((session: { status: string }) => session.status === status), `session status ${status}`).toBe(true);
  const error = schema.definitions.ServerMessage.oneOf.find((variant: { properties: { type: { const: string } } }) => variant.properties.type.const === "error");
  for (const code of error.properties.code.enum) expect(vectors.valid.some((message: { code?: string }) => message.code === code), `error code ${code}`).toBe(true);
});

test("Node Buffer independently builds the shared output and input frames", () => {
  const vectors = JSON.parse(readFileSync(new URL("../🧫️fixtures/🌀️control-plane/🔣️.json", import.meta.url), "utf8"));
  for (const frame of vectors.frames) {
    const id = Buffer.from(frame.session_id, "utf8"), data = Buffer.from(frame.data_hex, "hex");
    const payload = Buffer.concat([Buffer.from([id.length & 255, id.length >> 8]), id, data]);
    const length = Buffer.alloc(4);
    length.writeUInt32LE(payload.length + 1);
    expect(Buffer.concat([length, Buffer.from([frame.kind]), payload]).toString("hex"), frame.name).toBe(frame.frame_hex);
  }
});

test("strip-ansi and a regular expression independently decide the ready table", async () => {
  const { default: stripAnsi } = await import("strip-ansi");
  const table = JSON.parse(readFileSync(new URL("../🧫️fixtures/🟢️ready/🔣️.json", import.meta.url), "utf8"));
  for (const row of table.cases as { name: string; ready: { port: number; path: string; printed?: boolean }; chunks: string[]; url: string | null }[]) {
    const pattern = new RegExp(String.raw`http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):${row.ready.port}(?!\d)` + (row.ready.printed ? String.raw`\S*` : ""));
    const lines = stripAnsi(row.chunks.join("")).split(/[\r\n\t]/);
    const found = lines.map((line) => pattern.exec(line)?.[0]).find((match) => match !== undefined);
    expect(found === undefined ? null : row.ready.printed ? found : found + row.ready.path, row.name).toBe(row.url);
  }
});

test("Node environment projection independently isolates task process authority", () => {
  const boundary = JSON.parse(readFileSync(new URL("../🧫️fixtures/🌀️control-plane/🔣️.json", import.meta.url), "utf8")).environmentBoundary;
  const client = Object.fromEntries(boundary.client.filter(([key]:string[]) => !boundary.discarded.includes(key)));
  const projected = { ...client, ...Object.fromEntries(boundary.declared), TERM:"xterm-256color", COLORTERM:"truecolor" };
  expect(Object.entries(projected)).toEqual(boundary.expected);
});

test("every independent oracle of this file proves a named scenario of its feature", () => {
  const own = readFileSync(new URL(import.meta.url), "utf8");
  const proofs: [string, string, string][] = [
    ["⚙️preferences", "Resolving layered preferences", "Ajv and ordinary object projection independently resolve optional dashboard preferences"],
    ["⚙️preferences", "Keyboard and mouse use the same command selection", "JavaScript independently selects the shared searchable launcher vectors"],
    ["🚀️launcher", "Journeys match the shared vectors", "JavaScript independently selects the shared searchable launcher vectors"],
    ["🟢️ready", "Decide every case of the shared table", "strip-ansi and a regular expression independently decide the ready table"],
    ["✉️ipc", "Every valid protocol vector survives the codec unchanged", "Ajv independently validates the shared dashboard protocol vectors"],
    ["✉️ipc", "Every invalid protocol vector is refused", "Ajv independently validates the shared dashboard protocol vectors"],
    ["✉️ipc", "The vectors cover the whole schema", "Ajv sees every message type, session status and error code of the daemon schema in a valid vector"],
    ["✉️ipc", "Frames carry the same bytes in every implementation", "Node Buffer independently builds the shared output and input frames"],
    ["⌨️controls", "A portable control key opens optional settings", "Node readline independently decodes the native control leader vectors"],
    ["⌨️controls", "Unicode input survives terminal decoding", "Node readline independently decodes the native control leader vectors"],
  ];
  for (const [folder, scenario, title] of proofs) {
    const feature = readFileSync(new URL(`../${folder}/🥒️.feature`, import.meta.url), "utf8");
    expect([...feature.matchAll(/^\s*Scenario:\s*(.+?)\s*$/gm)].map((match) => match[1]), folder).toContain(scenario);
    expect(own.includes(`test(${JSON.stringify(title)}`), `${folder}: ${title}`).toBe(true);
  }
});
