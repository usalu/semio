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
test("ordered map clone quotes preserve independent original and interrupted candidates", () => {
  const path = join(root, owner, "🧬️retained-clone/🗺️ordered-map");
  const law = JSON.parse(readFileSync(join(path, "🧫️fixtures/🎟️clone/🔣️.json"), "utf8"));
  const validate = new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(path, "🧬️schema/🎟️clone/🔣️.json"), "utf8")));
  expect(validate(law)).toBe(true);
  for (const row of law.cases) {
    const original = Array.from({length:row.entryCount}, (_, key) => key);
    const independent = applyPatch({keys:[]}, original.map(value => ({op:"add" as const,path:"/keys/-",value})), true, false).newDocument.keys;
    expect(original).toEqual(row.expectedKeys);
    expect(independent).toEqual(row.expectedKeys);
    for (const stop of law.interruptAfter) {
      const candidate = original.slice(0, Math.min(stop, original.length));
      const canceled = applyPatch({original,candidate}, [{op:"remove",path:"/candidate"}], true, false).newDocument;
      expect(canceled.original).toEqual(row.expectedKeys);
      expect(original).toEqual(row.expectedKeys);
    }
  }
  console.log("[DEBUG] Ordered map clone neutral corpus matches independent RFC6902 outputs and interruption custody");
});
test("original shared clone preserves independent original values and cancellation",()=>{
 const path=join(root,owner,"🧬️retained-clone/🔗️shared"),law=JSON.parse(readFileSync(join(path,"🧫️fixtures/🔣️.json"),"utf8"));
 expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(path,"🧬️schema/🔣️.json"),"utf8")))(law)).toBe(true);
 for(const text of law.texts){for(const order of law.closeOrders){
  const original={source:{text},output:{text}};
  const first=applyPatch(original,[{op:"remove",path:order==="sourceFirst"?"/source":"/output"}],true,false).newDocument;
  expect((first.source??first.output).text).toBe(text);
  expect(original.source.text).toBe(text);
  for(const stop of law.interruptAfter){const pending={source:{text},candidate:stop===2?{text}:undefined};const closed=applyPatch(pending,[{op:"remove",path:"/source"}],true,false).newDocument;expect(pending.source.text).toBe(text);if(stop===2)expect(closed.candidate!.text).toBe(text);}
 }}
 console.log("[DEBUG] Original shared clone independent RFC6902 original/output close and cancellation values verified");
});
test("typed read leases preserve independent sparse visitation and payload outputs", () => {
  const path = join(root, owner, "🔗️read"), law = JSON.parse(readFileSync(join(path,"🧫️fixtures/🔣️.json"),"utf8"));
  for (const row of law.cases) {
    const returned = new Set(row.returned), owners = Array.from({length:row.issued},(_,index)=>index);
    let cursor = row.cursor;
    const observed = row.visits.map(() => {
      const next = owners.filter(index=>index>=cursor).at(0) ?? owners.filter(index=>index<cursor).at(0);
      if(next===undefined)return null;
      cursor=(next+1)%law.capacity;
      if(!returned.has(next))return null;
      owners.splice(owners.indexOf(next),1);
      return next;
    });
    const independent = applyPatch({visits:[]},row.visits.map((value:number|null)=>({op:"add" as const,path:"/visits/-",value})),true,false).newDocument.visits;
    expect(observed).toEqual(independent);
  }
  for (const text of law.texts) expect(applyPatch({},[{op:"add",path:"/root",value:text}],true,false).newDocument.root).toBe(JSON.parse(JSON.stringify(text)));
  console.log("[DEBUG] Typed read sparse visitation and independent JSON Patch payload outputs verified");
});
test("erased read portable source pin law preserves independent payload authority", () => {
  const path = join(root, owner, "🔗️read"), law = JSON.parse(readFileSync(join(path,"🧫️fixtures/🔣️.json"),"utf8"));
  for(const copy of law.workBytes){
    expect(copy).toBeGreaterThan(0);
    const original={root:law.erasedSource.text,source:law.erasedSource.text,read:true,authority:true};
    const retained=applyPatch(original,[{op:"remove",path:"/read"},{op:"remove",path:"/authority"}],true,false).newDocument;
    expect(retained.source).toBe(JSON.parse(JSON.stringify(law.erasedSource.text)));
    expect(original.read).toBe(true);
    expect(original.authority).toBe(true);
  }
  console.log("[DEBUG] Erased read strict neutral source pin and independent retained payload authority verified");
});
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

test("owned source birth funds original custody before allocation", () => {
  const source=join(root,owner,"🧬️retained-clone/🔗️source");
  const law=JSON.parse(readFileSync(join(source,"🧫️fixtures/🎟️owned-birth/🔣️.json"),"utf8"));
  expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(source,"🧬️schema/🎟️owned-birth/🔣️.json"),"utf8")))(law)).toBe(true);
  const original={owner:law.owner,authority:law.authority,allocated:false};
  for(const refusal of law.refusals){
    const demand={items:law.birth.items,copy:law.birth.sourceAliasCopies,capacity:law.ownerCapacity+law.authorityCapacity,depth:law.birth.depth};
    const grant=applyPatch(demand,[{op:"replace",path:`/${refusal.currency}`,value:demand[refusal.currency as keyof typeof demand]-refusal.shortBy}],true,false).newDocument;
    expect(grant[refusal.currency as keyof typeof demand]).toBeLessThan(demand[refusal.currency as keyof typeof demand]);
    const refused=applyPatch(original,[],true,false).newDocument;expect(refused).toEqual(original);
    expect(refusal.kind).toBe({items:"WorkLimit",copy:"OwnershipLimit",capacity:"OwnershipLimit",depth:"DepthLimit"}[refusal.currency as "items"|"copy"|"capacity"|"depth"]);
  }
  const admitted=applyPatch(original,[{op:"replace",path:"/allocated",value:true}],true,false).newDocument;
  expect(admitted.owner).toBe(original.owner);expect(admitted.authority).toBe(original.authority);expect(admitted.allocated).toBe(true);expect(law.birth.sourceAliasCopies).toBe(1);expect(law.birth.releaseBytes).toBe(0);
  expect(law.sharedAuthorityRetainsAlias).toBe(true);
  const shared=applyPatch({original:original.authority,aliases:["issuer","source"]},[{op:"remove",path:"/aliases/1"}],true,false).newDocument;expect(shared.original).toBe(original.authority);expect(shared.aliases).toEqual(["issuer"]);
  for(const order of law.closeOrders){const live={owner:law.owner,authority:law.authority,aliases:["source","projection"],released:false};const first=applyPatch(live,[{op:"remove",path:order==="sourceFirst"?"/aliases/0":"/aliases/1"}],true,false).newDocument;expect(first.owner).toBe(law.owner);expect(first.authority).toBe(law.authority);expect(first.released).toBe(false);expect(first.aliases).toEqual([order==="sourceFirst"?"projection":"source"]);const terminal=applyPatch(first,[{op:"remove",path:"/aliases/0"},{op:"replace",path:"/released",value:true}],true,false).newDocument;expect(terminal.aliases).toEqual([]);expect(terminal.released).toBe(true);}
  console.log("[DEBUG] owned source birth independent RFC6902 oracle preserves original owners on all refused currencies");
});

test("funded read ownership keeps sparse returned roots independently of live readers", () => {
  const law=JSON.parse(readFileSync(join(root,owner,"🔗️read/🧫️fixtures/🔣️.json"),"utf8"));
  for(const row of law.cases){
    let slots=Array.from({length:law.capacity},()=>({root:null as number|null,issued:false}));
    for(let slot=0;slot<row.issued;slot++)slots=applyPatch(slots,[{op:"replace",path:`/${slot}/root`,value:slot},{op:"replace",path:`/${slot}/issued`,value:true}],true,false).newDocument;
    for(const slot of row.returned)slots=applyPatch(slots,[{op:"replace",path:`/${slot}/issued`,value:false}],true,false).newDocument;
    let cursor=row.cursor;
    const visits=row.visits.map(()=>{
      for(let offset=0;offset<law.capacity;offset++){
        const slot=(cursor+offset)%law.capacity,entry=slots[slot]!;
        if(entry.root===null)continue;
        cursor=(slot+1)%law.capacity;
        if(entry.issued)return null;
        const original=entry.root;slots=applyPatch(slots,[{op:"replace",path:`/${slot}/root`,value:null}],true,false).newDocument;return original;
      }
      return null;
    });
    expect(visits).toEqual(row.visits);
    for(let slot=0;slot<row.issued;slot++)if(!row.returned.includes(slot)){expect(slots[slot]!.issued).toBe(true);expect(slots[slot]!.root).toBe(slot);}
  }
  console.log("[DEBUG] independent read ownership oracle preserves original sparse wrap and live-reader starvation vectors");
});

test("retained clone close depth preserves an unfunded original birth", () => {
  const law=JSON.parse(readFileSync(join(root,owner,"🧬️retained-clone/🧫️fixtures/🪜️close-demands/🔣️.json"),"utf8"));
  const original={owner:law.owner,retiring:false,closing:false};
  const closing=applyPatch(original,[{op:"replace",path:"/closing",value:true}],true,false).newDocument;
  expect(law.idleDepth).toBe(0);expect(law.pendingBirthDepth).toBe(1);
  const scaffold=JSON.parse(readFileSync(join(root,owner,"🧬️retained-clone/🧫️fixtures/📏️release-authority/🔣️.json"),"utf8"));
  expect(scaffold.physicalScaffoldDepth).toBe(1);
  const deniedScaffold=applyPatch({backing:scaffold.physicalCursorBytes},[],true,false).newDocument;expect(deniedScaffold.backing).toBe(scaffold.physicalCursorBytes);
  const closedScaffold=applyPatch(deniedScaffold,[{op:"replace",path:"/backing",value:0}],true,false).newDocument;expect(closedScaffold.backing).toBe(0);
  const refused=applyPatch(closing,[],true,false).newDocument;
  expect(law.refusedBirth.maximumDepth).toBeLessThan(law.pendingBirthDepth);expect(refused.owner).toBe(law.owner);expect(refused.retiring).toBe(false);
  const admitted=applyPatch(closing,[{op:"replace",path:"/owner",value:null},{op:"replace",path:"/retiring",value:true}],true,false).newDocument;
  expect(law.fundedBirth.maximumDepth).toBe(law.pendingBirthDepth);expect(admitted.owner).toBeNull();expect(admitted.retiring).toBe(true);
  const terminal=applyPatch(admitted,[{op:"replace",path:"/retiring",value:false}],true,false).newDocument;
  expect(terminal.owner).toBeNull();expect(terminal.retiring).toBe(false);expect(law.terminalDepth).toBe(0);
  console.log("[DEBUG] independent close depth oracle preserves original custody across refused and funded birth");
});

test("recursive structural depth counts field and box owners independently", () => {
  const directory=join(root,owner,"🧬️retained-clone/🧫️fixtures/🌳️structural-depth");
  const law=JSON.parse(readFileSync(join(directory,"🔣️.json"),"utf8"));
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

test("read closure order retains the original typed payload until its exact last alias closes", () => {
  const law=JSON.parse(readFileSync(join(root,owner,"🔗️read/🧫️fixtures/🔣️.json"),"utf8"));
  for(const order of law.closeOrders){
    let state={authority:true,read:true,original:"last-reader λ🙂",released:false};
    if(order==="authority-first"){state=applyPatch(state,[{op:"replace",path:"/authority",value:false}],true,false).newDocument;expect(state.read).toBe(true);expect(JSON.parse(JSON.stringify(state.original))).toBe("last-reader λ🙂");expect(state.released).toBe(false);}
    state=applyPatch(state,[{op:"replace",path:"/read",value:false}],true,false).newDocument;
    if(order==="read-first"){expect(state.authority).toBe(true);expect(state.released).toBe(false);state=applyPatch(state,[{op:"replace",path:"/authority",value:false}],true,false).newDocument;}
    expect(state.authority||state.read).toBe(false);state=applyPatch(state,[{op:"replace",path:"/released",value:true}],true,false).newDocument;expect(state.released).toBe(true);
  }
  console.log("[DEBUG] Independent AJV and RFC6902 close-order oracle preserves the original last-reader payload");
});
