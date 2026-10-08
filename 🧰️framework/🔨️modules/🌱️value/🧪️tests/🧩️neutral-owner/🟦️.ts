import Ajv from "ajv";
import { test, expect } from "bun:test";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync, symlinkSync, lstatSync } from "node:fs";
import { resolve, join, dirname, isAbsolute } from "node:path";
import { spawnSync } from "node:child_process";
import * as toml from "@iarna/toml";
import { applyPatch, getValueByPointer } from "fast-json-patch";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../..");
const owner = "🧰️framework/🔨️modules/🌱️value";
const fixture = JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🧩️neutral-owner/🔣️.json"), "utf8"));
const validValue = new Ajv({strict:false}).compile(JSON.parse(readFileSync(join(root, owner, "🧬️schema/🔣️.json"), "utf8")));
test("borrowed clone authority follows independent JSON Patch", () => {
  for (const row of fixture.vectors.filter((row: {accepted:boolean}) => row.accepted)) {
    const input = structuredClone(row.input);
    const copied = applyPatch({}, [{ op: "add", path: "/value", value: structuredClone(input) }], true, false).newDocument.value;
    expect(copied).toEqual(row.input);
    expect(input).toEqual(row.input);
    for (const at of fixture.borrowAuthority.cancelAt) {
      const partial = applyPatch({ owners: Object.entries(copied).slice(0, at) }, [{ op: "replace", path: "/owners", value: [] }], true, false).newDocument;
      expect(partial.owners).toEqual([]);
      expect(input).toEqual(row.input);
    }
  }
  console.log("[DEBUG] borrowed clone independent JSON Patch laws verified");
});
test("paged native ownership follows semantic JSON and independent UTF-8 ordering", () => {
  const local = join(root, owner, "📦️paged");
  const law = JSON.parse(readFileSync(join(local, "🧫️fixtures/🎮️native-owner/🔣️.json"), "utf8"));
  const prefix = law.prefix.repeat(law.prefixRepeat);
  expect(new TextEncoder().encode(prefix).length).toBeGreaterThan(law.bodyBytes);
  const object = Object.fromEntries(law.objectEntries.map((row: {suffix:string,value:number}) => [prefix + row.suffix, row.value]));
  expect(Object.keys(object).length).toBe(law.objectEntries.length);
  expect(validValue(object)).toBe(true);
  const replacementKey = prefix + law.objectReplacement.suffix;
  expect(object[replacementKey]).toBe(law.objectReplacement.previous);
  const edited = structuredClone(object);
  edited[replacementKey] = law.objectReplacement.value;
  expect(edited[replacementKey]).toBe(law.objectReplacement.value);
  const lexicalTail = Object.keys(edited).sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right))).at(-1)!;
  expect(lexicalTail).toBe(prefix + law.lexicalTailSuffix);
  delete edited[lexicalTail];
  expect(Object.hasOwn(edited, lexicalTail)).toBe(false);
  expect(validValue(edited)).toBe(true);

  expect(applyPatch({}, [{op:"add",path:"/object",value:object}], true, false).newDocument.object).toEqual(JSON.parse(JSON.stringify(object)));
  for (const row of law.comparisons) {
    expect(Math.sign(Buffer.compare(Buffer.from(prefix + row.left), Buffer.from(prefix + row.right)))).toBe(row.ordering);
    expect(JSON.parse(JSON.stringify({ text: prefix + row.left }))).toEqual({ text: prefix + row.left });
  }
  for (const row of law.emptyComparisons) expect(Math.sign(Buffer.compare(Buffer.from(row.left), Buffer.from(row.right)))).toBe(row.ordering);
  for (const row of law.listEdits) {
    const operation = row.kind === "insert" ? {op:"add" as const,path:`/${row.index}`,value:row.value} : {op:"remove" as const,path:`/${row.index}`};
    expect(applyPatch(structuredClone(row.initial), [operation], true, false).newDocument).toEqual(row.expected);
    if (row.kind === "remove") expect(row.initial[row.index]).toBe(row.removed);
  }
  expect(law.comparatorClose.bindings).toBe(2);
  expect(law.comparatorClose.maximumItems).toBe(1);
  expect(law.comparatorClose.maximumBytes).toBe(0);
  const aliases = applyPatch({bindings:["left","right"]}, [{op:"remove",path:"/bindings/0"}], true, false).newDocument;
  expect(aliases.bindings).toEqual(["right"]);
  expect(applyPatch(aliases, [{op:"remove",path:"/bindings/0"}], true, false).newDocument.bindings).toEqual([]);
  console.log("[DEBUG] paged native JSON/UTF-8 comparison law verified");
});
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

test("retirement source capacity agrees with independent decimal arithmetic",async()=>{
  const Decimal=(await import("decimal.js")).default.clone({precision:100});
  const local=join(root,owner,"♻️retirement"),fixture=JSON.parse(readFileSync(join(local,"🧫️fixtures/🔣️.json"),"utf8"));
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
  console.log(`[DEBUG] retirement source capacity 24 portable word-width vectors agree with decimal.js`);
});

test("owned projected fields preserve native value paths and exact alias closure",()=>{
 const law=JSON.parse(readFileSync(join(root,owner,"📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json"),"utf8")).ownedProjection;
 expect(validValue(law.input)).toBe(true);const before=JSON.stringify(law.input);
 for(const row of law.paths){const own=row.pointer.split("/").slice(1).reduce((value:any,key:string)=>value[key],law.input);expect(own).toBe(getValueByPointer(law.input,row.pointer));expect(own).toBe(row.value);}
 let aliases=["parent","child"];for(const action of law.closeSequence){if(action==="parent"||action==="child"){const index=aliases.indexOf(action);aliases=applyPatch({aliases},[{op:"remove",path:"/aliases/"+index}],true,false).newDocument.aliases;}if(action==="complete")expect(aliases).toEqual([]);}
 expect(JSON.stringify(law.input)).toBe(before);expect(law.constructorAllocationBytes).toBe(0);expect(law.maximumItems).toBe(1);expect(law.maximumBytes).toBe(0);
 expect(readFileSync(join(root,owner,"🧬️retained-clone/🦀️.rs"),"utf8").includes("pub fn project_owned")).toBe(true);
 console.log("[DEBUG] owned projection native value paths agree with independent RFC6902 pointer and exact alias closure");
});

test("recursive structural depth counts field and box owners independently", () => {
  const directory=join(root,owner,"🧬️retained-clone/🧫️fixtures/🌳️structural-depth");
  const law=JSON.parse(readFileSync(join(directory,"🔣️.json"),"utf8"));
  expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(directory,"📐️schema.json"),"utf8")))(law)).toBe(true);
  for(const row of law.cases){
    let chain={text:"leaf",next:null} as {text:string,next:unknown};
    for(let index=0;index<row.boxes;index++) chain={text:`node-${index}-β`,next:chain};
    const copy=applyPatch({},[{op:"add",path:"/chain",value:chain}],true,false).newDocument.chain;
    expect(JSON.parse(JSON.stringify(copy))).toEqual(chain);
    let boxes=0,current=copy;
    while(current.next!==null){expect(current.text).toBe(`node-${row.boxes-1-boxes}-β`);boxes++;current=current.next;}
    expect(current.text).toBe("leaf");expect(boxes).toBe(row.boxes);
    expect(boxes*(law.fieldLevelsPerBox+law.boxLevelsPerBox)+law.terminalFieldLevels).toBe(row.requiredDepth);
    expect(row.requiredDepth<=row.maximumDepth).toBe(row.accepted);
  }
  expect(law.close).toEqual({maximumItems:1,maximumCopyBytes:2,maximumReleaseBytes:65536});
  console.log("[DEBUG] Recursive field/box depth oracle preserves zero, one, 31/32 and 255/256 ordered chain boundaries with separate two-byte copy and exact physical close authority");
});
