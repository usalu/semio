import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020";
import { Database } from "bun:sqlite";
import law from "./🔣️.json";
import schema from "./🧬️.json";
import { coreParameterVector } from "../core-signature.🟦️.ts";
import { readPluginRuntimeSource } from "../../../../⚛️reactor/🏃️runtime/📖️source-text/🟦️.ts";

/** 🔌️ Compares each channel fixture signature with both the guest declaration and Core selector. */
test("all 36 current owned crossings carry the declared scalar authority", () => {
  expect(new Ajv({ strict: true }).validate(schema, law)).toBe(true);
  const source = readFileSync(resolve(import.meta.dir, "../../../🔌️owned-semio-boundary/🦀️.rs"), "utf8");
  const runtime = readPluginRuntimeSource();
  const core = source.slice(source.indexOf("pub(super) fn core_type"));
  const arms = [...core.matchAll(/((?:Self::\w+\s*(?:\|\s*)?)+)=>\s*(?:\{\s*)?FunctionType\s*\{\s*parameters:\s*([\s\S]*?),\s*results:\s*([\s\S]*?)\s*\}/gu)];
  const constants = new Map([...source.matchAll(/pub const (SEMIO_OWNED_\w+_EXPORT)\s*:\s*&str\s*=\s*"([^"]+)";/gu)].map(([, constant, name]) => [constant, name]));
  const selectors = new Map([...source.matchAll(/Self::(\w+)\s*=>\s*(SEMIO_OWNED_\w+_EXPORT)/gu)].map(([, variant, constant]) => [variant, constant]));
  const db = new Database(":memory:");
  try {
    db.run("create table exports(variant text primary key,parameters text,results text)");
    for (const row of law.exports) {
      db.run("insert into exports values(?,?,?)", [row.variant, JSON.stringify(row.parameters), JSON.stringify(row.results)]);
      const arm = arms.find(match => new RegExp(`\\bSelf::${row.variant}\\b`).test(match[1]))?.[0].replace(/\s+/gu, "");
      expect(arm, row.variant).toContain(`parameters:${coreParameterVector(row.parameters)}`);
      expect(arm, row.variant).toContain(`results:vec![${row.results.map(type => `ValueType::${type.toUpperCase()}`).join(",")}]`);
      const name = constants.get(selectors.get(row.variant) ?? "");
      expect(name, row.variant).toBeDefined();
      const signature = new RegExp(`\\bfn\\s+${name}\\(([^()]*)\\)\\s*(?:->\\s*(\\w+))?`, "u").exec(runtime);
      expect(signature, name).not.toBeNull();
      const actualParameters = signature![1].split(",").map(parameter => parameter.trim()).filter(Boolean)
        .map(parameter => parameter.split(":")[1]?.trim());
      expect(actualParameters, name).toEqual(row.parameters.map(type => type.replace("i", "u")));
      expect(signature![2], name).toBe(row.results[0]?.replace("i", "u"));
    }
    expect(db.query("select count(*) as count from exports").get()).toEqual({ count: 36 });
  } finally {
    db.close();
  }
});

test("the independent WebAssembly engine executes the complete authored scalar profile",()=>{const uleb=(n:number)=>{const b:number[]=[];do{const v=n&127;n=Math.floor(n/128);b.push(v|(n?128:0));}while(n);return b;},section=(id:number,b:number[])=>[id,...uleb(b.length),...b],types=[36,...law.exports.flatMap(row=>[96,row.parameters.length,...row.parameters.map(p=>p==="i32"?127:126),row.results.length,...row.results.map(p=>p==="i32"?127:126)])],funcs=[36,...law.exports.map((_,i)=>i)],exports=[36,...law.exports.flatMap((row,i)=>{const name=[...Buffer.from(row.variant)];return[name.length,...name,0,i];})],bodies=[36,...law.exports.flatMap(row=>{const b=[0,...(row.results.length?[row.results[0]==="i32"?65:66,0]:[]),11];return[b.length,...b];})],bytes=new Uint8Array([0,97,115,109,1,0,0,0,...section(1,types),...section(3,funcs),...section(7,exports),...section(10,bodies)]);expect(WebAssembly.validate(bytes)).toBe(true);const instance=new WebAssembly.Instance(new WebAssembly.Module(bytes));for(const row of law.exports){const invoke=instance.exports[row.variant] as(...args:(number|bigint)[])=>number|bigint|undefined;expect(invoke(...row.parameters.map(p=>p==="i32"?0:0n))).toBe(!row.results.length?undefined:row.results[0]==="i32"?0:0n);}});
