import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";

test("Node readline independently decodes the native control leader vectors", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⌨️controls/🔣️.json",import.meta.url),"utf8"));
  const source="const {PassThrough}=require('node:stream'),{emitKeypressEvents}=require('node:readline'),fixture=JSON.parse(process.argv[1]),input=new PassThrough(),events=[];emitKeypressEvents(input);input.on('keypress',(_,key)=>events.push({key:key.sequence==='\\u0000'&&key.ctrl?' ':!key.ctrl&&key.sequence.length===1?key.sequence:key.name==='space'?' ':key.name,ctrl:Boolean(key.ctrl)}));input.end(Buffer.from(fixture.bytes));setImmediate(()=>console.log(JSON.stringify(events)));";
  const child=Bun.spawn(["node","-e",source,JSON.stringify(fixture)],{stdout:"pipe",stderr:"pipe"});
  const output=await new Response(child.stdout).text();expect(await child.exited,await new Response(child.stderr).text()).toBe(0);expect(JSON.parse(output)).toEqual(fixture.events);
});

test("JavaScript independently selects the shared searchable launcher vectors", () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔎️launcher/🔣️.json",import.meta.url),"utf8"));
  for(const vector of fixture.cases){const tokens=vector.filter.toLocaleLowerCase().split(/\s+/).filter(Boolean),index=fixture.options.findIndex((value:string)=>tokens.every((token:string)=>value.toLocaleLowerCase().includes(token)));expect(index<0?null:index).toBe(vector.index);}
  for(const vector of fixture.pointer){const tokens=vector.filter.toLocaleLowerCase().split(/\s+/).filter(Boolean),visible=fixture.options.map((value:string,index:number)=>({value,index})).filter(({value}:{value:string})=>tokens.every((token:string)=>value.toLocaleLowerCase().includes(token))),offset=Math.max(0,vector.selected-(vector.height-2));expect(vector.row<1?null:visible[offset+vector.row-1]?.index??null).toBe(vector.index);}
});

test("Ajv and ordinary object projection independently resolve optional dashboard preferences", () => {
  const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/⚙️preferences/🔣️.json",import.meta.url),"utf8"));
  const vectors=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⚙️preferences/🔣️.json",import.meta.url),"utf8"));
  const validate=new Ajv({strict:false}).compile(schema);
  for(const value of vectors.valid)expect(validate(value),JSON.stringify(validate.errors)).toBe(true);
  for(const value of vectors.invalid)expect(validate(value),JSON.stringify(value)).toBe(false);
  for(const vector of vectors.cases)expect(Object.assign({},vectors.defaults,...vector.layers)).toEqual(vector.expected);
});

test("Ajv independently validates the shared dashboard protocol vectors", () => {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🌀️daemon/🔣️.json", import.meta.url), "utf8"));
  const vectors = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🌀️control-plane/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: false }).compile(schema);
  for (const message of vectors.valid) expect(validate(message), JSON.stringify(validate.errors)).toBe(true);
  for (const message of vectors.invalid) expect(validate(message), JSON.stringify(message)).toBe(false);
});


test("Nx independently schedules every inferred command discovery vector", async () => {
  const {createTaskGraph} = await import("nx/src/tasks-runner/create-task-graph.js");
  const {graph,expected} = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🌳️inferred-targets/🔣️.json",import.meta.url),"utf8"));
  const tasks = expected.flatMap((id:string) => { const split=id.lastIndexOf(":"); return Object.keys(createTaskGraph(graph,{},[id.slice(0,split)],[id.slice(split+1)],undefined,{}).tasks); }).sort();
  expect(tasks).toEqual(expected);
});

test("Ajv and Nx independently admit every configured renderer language selection", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🗣️launch-axes/🔣️.json",import.meta.url),"utf8"));
  const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🗣️launch-axes/🔣️.json",import.meta.url),"utf8"));
  const validate=new Ajv({strict:false}).compile(schema);
  expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
  const {createTaskGraph}=await import("nx/src/tasks-runner/create-task-graph.js");
  const targets=Object.fromEntries(fixture.renderers.map((row:{target:string})=>[row.target,{executor:"nx:run-commands",options:{command:"bun fixture"}}]));
  const graph={nodes:{"@semio-tech/framework-os-dev":{name:"@semio-tech/framework-os-dev",type:"lib",data:{root:"dev",targets}}},dependencies:{"@semio-tech/framework-os-dev":[]}};
  for(const row of fixture.renderers)for(const locale of fixture.locales)for(const terminology of fixture.terminologies){
    const expected={SEMIO_LOCKED_LOCALE:locale,SEMIO_LOCKED_TERMINOLOGY:terminology};
    const tasks=createTaskGraph(graph,{},["@semio-tech/framework-os-dev"],[row.target],undefined,expected).tasks;
    expect(Object.keys(tasks)).toEqual([`@semio-tech/framework-os-dev:${row.target}`]);
    expect(tasks[Object.keys(tasks)[0]!].overrides).toEqual(expected);
  }
});
