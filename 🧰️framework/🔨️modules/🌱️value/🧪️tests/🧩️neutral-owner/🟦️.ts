import { test, expect } from "bun:test";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync, symlinkSync, lstatSync } from "node:fs";
import { resolve, join, dirname, isAbsolute } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../..");
const owner = "🧰️framework/🔨️modules/🌱️value";
const fixture = JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🧩️neutral-owner/🔣️.json"), "utf8"));
/** 🗂️ Admits explicitly owned compiler storage without following directory links. */
function compilerStorage(path: string): string {
  if (!isAbsolute(path)) throw new Error("Compiler storage must be an absolute directory");
  const directory = resolve(path);
  for (let current = directory;; current = dirname(current)) {
    try { const state = lstatSync(current); if (!state.isDirectory() || state.isSymbolicLink()) throw new Error(`Compiler storage directory refused: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(directory, { recursive: true });
  return directory;
}
test("neutral value owner portable corpus admits independent Ajv", () => {
  const schema = JSON.parse(readFileSync(join(root, owner, "🧬️schema/🧩️neutral-owner/🔣️.json"), "utf8"));
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
});
test("paged value portable corpus admits independent Ajv", () => {
  const fixture = JSON.parse(readFileSync(join(root, owner, "📦️paged/🧫️fixtures/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(root, owner, "📦️paged/🧬️schema/🔣️.json"), "utf8"));
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
});
test("compiler storage respects physical caller directory authority", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name the ticket artifact directory");
  const directory = mkdtempSync(join(output, "value-storage-admission-"));
  const regular = join(directory, "regular"), file = join(directory, "file"), link = join(directory, "linked");
  mkdirSync(regular); writeFileSync(file, "owned");
  symlinkSync(regular, link, process.platform === "win32" ? "junction" : "dir");
  const paths: Record<string, string> = { directory: regular, missing: join(directory, "missing", "child"), relative: "relative-storage", file, link, "linked-ancestor": join(link, "child") };
  const oracle = spawnSync("node", ["--input-type=module", "-e", `import {lstatSync} from 'node:fs';import {isAbsolute,dirname} from 'node:path';const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(path=>{if(!isAbsolute(path))return false;for(let current=path;;current=dirname(current)){try{const entry=lstatSync(current);if(!entry.isDirectory()||entry.isSymbolicLink())return false}catch(error){if(error.code!=='ENOENT')throw error}if(dirname(current)===current)return true}})));`, JSON.stringify(fixture.compilerStorage.map((row: {kind:string}) => paths[row.kind]))], { encoding: "utf8" });
  expect(oracle.status).toBe(0);
  const expected = fixture.compilerStorage.map((row: {accepted:boolean}) => row.accepted);
  expect(JSON.parse(oracle.stdout)).toEqual(expected);
  expect(fixture.compilerStorage.map((row: {kind:string}) => { try { compilerStorage(paths[row.kind]); return true; } catch { return false; } })).toEqual(expected);
});
test("actual value derives build and execute with product trees absent", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name the ticket artifact directory");
  const copy = mkdtempSync(join(output, "neutral-value-owner-"));
  const members = [owner + "/✨️derive/📦️packages/🦀️rust", "🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust"];
  if (existsSync(join(root, owner, "📦️packages/🦀️rust/Cargo.toml"))) members.push(owner + "/📦️packages/🦀️rust");
  for (const source of [owner, "🧰️framework/🔨️modules/🚪️io/🔤️base64"]) {
    mkdirSync(resolve(copy, source, ".."), { recursive: true });
    cpSync(join(root, source), join(copy, source), { recursive: true, filter: path => !/(?:^|[\\/])(?:dist|target|node_modules|🗑️generated)(?:[\\/]|$)/u.test(path) });
  }
  const source = toml.parse(readFileSync(join(root, "Cargo.toml"), "utf8")) as any;
  writeFileSync(join(copy, "Cargo.toml"), toml.stringify({ workspace: { resolver: "2", members, package: source.workspace.package, lints: source.workspace.lints, dependencies: { "semio-framework-value": { path: owner + "/📦️packages/🦀️rust" } } } }));
  expect(existsSync(join(copy, "🧰️framework/🛍️products"))).toBe(false);
  expect(existsSync(join(copy, "✏️s"))).toBe(false);
  const cargo = { ...process.env, CARGO_TARGET_DIR: compilerStorage(process.env.CARGO_TARGET_DIR ?? join(copy, "target")), CARGO_BUILD_BUILD_DIR: compilerStorage(process.env.CARGO_BUILD_BUILD_DIR ?? join(copy, "compiler")) };
  await runOwnedCommand("cargo", ["clean", "--offline", "--manifest-path", join(copy, "Cargo.toml"), "-p", "semio-framework-value", "-p", "semio-framework-value-derive"], copy, "value:fresh-captured-units", 120000, { env: cargo });
  console.log("value:products-absent fresh-captured-units=value,value-derive thirdparty-artifacts=caller-owned");
  await runOwnedCommand("cargo", ["test", "--offline", "--manifest-path", join(copy, "Cargo.toml"), "-p", "semio-framework-value", "-p", "semio-framework-value-derive", "--", "--nocapture"], copy, "value:products-absent", 120000, { env: cargo });
}, 120000);

test("retirement source capacity corpus is closed and agrees with independent decimal arithmetic",async()=>{
  const Decimal=(await import("decimal.js")).default.clone({precision:100});
  const local=join(root,owner,"♻️retirement"),fixture=JSON.parse(readFileSync(join(local,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(local,"🧬️schema/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
  expect(validate(fixture)).toBe(true);
  const cases=fixture.capacityAdmission.cases;expect(cases.length).toBe(12);expect(new Set(cases.map((row:any)=>row.id)).size).toBe(12);
  expect(new TextEncoder().encode(fixture.capacityAdmission.source).length).toBe(fixture.capacityAdmission.sourceBytes);
  const observations=[];
  for(const bits of [32,64]){
    const maximum=(1n<<BigInt(bits))-1n,signed=maximum>>1n;
    const number=(value:string)=>value==="usizeMax"?maximum:value==="isizeMax"?signed:BigInt(value);
    for(const row of cases){
      const source=number(row.sourceBytes),multiple=number(row.multiples),scaffold=number(row.scaffoldBytes),owned=number(row.ownedBytes),capacity=source*multiple+scaffold;
      const reference=new Decimal(source.toString()).mul(multiple.toString()).add(scaffold.toString());
      const accepted=capacity<=signed&&capacity>=owned,oracle=reference.lte(signed.toString())&&reference.gte(owned.toString());
      expect(reference.toFixed(0)).toBe(capacity.toString());expect(accepted).toBe(oracle);expect(accepted).toBe(row.accepted);if(accepted)expect(capacity.toString()).toBe(number(row.maximumBytes).toString());
      observations.push({bits,id:row.id,accepted,maximumBytes:accepted?capacity.toString():null});
    }
  }
  expect(observations.length).toBe(24);
  const hostile=[{...fixture,extra:true},{...fixture,capacityAdmission:{...fixture.capacityAdmission,extra:true}},{...fixture,leases:{...fixture.leases,extra:true}},{...fixture,continuation:{...fixture.continuation,extra:true}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===0?{...row,accepted:"true"}:row)}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===0?{...row,sourceBytes:"-1"}:row)}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===0?{...row,maximumBytes:null}:row)}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===3?{...row,maximumBytes:"4"}:row)}},{...fixture,leases:{...fixture.leases,orders:fixture.leases.orders.map((row:any,index:number)=>index===0?[0,0,2]:row)}}];
  for(const value of hostile)expect(validate(value)).toBe(false);
  for(const key of ["leases","capacityAdmission","continuation"]){const value={...fixture};delete value[key];expect(validate(value)).toBe(false);}
  console.log(`[DEBUG] retirement source capacity 24 portable word-width vectors agree with decimal.js; twelve closed-schema hostiles refused`);
});
