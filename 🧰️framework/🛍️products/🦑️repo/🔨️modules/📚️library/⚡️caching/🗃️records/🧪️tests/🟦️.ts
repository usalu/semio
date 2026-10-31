/** 🗃️ Record store laws over `./🧫️fixtures/🔣️.json`: every scenario and a random operation sequence over two concurrent store handles
 * agree with an in-memory model, every published index satisfies `../🧬️schema/🔣️.json` (Ajv), and the packs on disk are exactly the
 * indexed ones (fast-glob). Damaged bytes must read as a miss, never as a wrong value. */
import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, readlinkSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import fg from "fast-glob";
import {Database} from "bun:sqlite";
import { moduleSourceClosureDigest, openRecordStore, RECORD_STORE_PACK_LIMIT, recordKey, openRecordStoreDescriptorCount, RECORD_STORE_DESCRIPTOR_LIMIT, type RecordStore } from "../🟦️.ts";

type Operation = { op: "put" | "get" | "flush" | "reopen" | "crash" | "corrupt" | "retain" | "compact"; store: string; key?: string; value?: string; expect?: string | null };
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "./🧫️fixtures/🔣️.json"), "utf8")) as { schemaVersion: number; scenarios: { id: string; operations: Operation[] }[];retirementCases:readonly {id:string;attempts:number;commits:number;outcome:string}[] };
const validate = new Ajv({ strict: false, allErrors: true }).compile(JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8")));
const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
mkdirSync(artifacts, { recursive: true });

/** 🎲️ A small deterministic generator so property laws are reproducible. */
function random(seed: number): () => number {
  let state = seed;
  return () => { state = (state * 1103515245 + 12345) & 0x7fffffff; return state / 0x80000000; };
}

/** 🧪️ Replays operations against real store handles in one directory and checks each `get` against the expectation. */
function replay(directory: string, operations: readonly Operation[], check: (operation: Operation, actual: string | null) => void): void {
  const handles = new Map<string, RecordStore>(), handle = (name: string): RecordStore => { if (!handles.has(name)) handles.set(name, openRecordStore(directory)); return handles.get(name)!; };
  const key = (name: string): string => recordKey("record-store-law", name);
  for (const operation of operations) {
    const store = handle(operation.store);
    if (operation.op === "put") store.put(key(operation.key!), Buffer.from(operation.value!, "utf8"));
    else if (operation.op === "get") { const value = store.get(key(operation.key!)); check(operation, value === undefined ? null : Buffer.from(value).toString("utf8")); }
    else if (operation.op === "flush") store.flush();
    else if (operation.op === "retain") store.retainAccessed();
    else if (operation.op === "compact") store.compact();
    else if (operation.op === "reopen") { store.close(); handles.delete(operation.store); }
    else if (operation.op === "crash") handles.delete(operation.store);
    else if (operation.op === "corrupt") {
      for (const name of readdirSync(directory).filter((entry) => entry.endsWith(".idx"))) {
        const index = JSON.parse(readFileSync(join(directory, name), "utf8")) as { pack: string; records: [string, number, number, string][] };
        const record = index.records.find(([recordKey]) => recordKey === key(operation.key!));
        if (!record || record[2] === 0) continue;
        const packPath = join(directory, `${index.pack}.pack`), bytes = readFileSync(packPath);
        bytes[record[1]] = bytes[record[1]]! ^ 0xff;
        writeFileSync(packPath, bytes);
      }
    }
  }
  for (const store of handles.values()) store.close();
}

/** 📇️ Every published index validates against the schema and the packs on disk are exactly the indexed ones. */
function assertPublishedLayout(directory: string): void {
  const indexes = readdirSync(directory).filter((entry) => entry.endsWith(".idx"));
  for (const name of indexes) expect(validate(JSON.parse(readFileSync(join(directory, name), "utf8"))), `${name} ${JSON.stringify(validate.errors)}`).toBe(true);
  expect(fg.sync("*.pack", { cwd: directory }).sort()).toEqual(indexes.map((name) => `${name.slice(0, -4)}.pack`).sort());
}

test("compaction merges every pack of a store into one and keeps every record", () => {
  const directory = mkdtempSync(join(artifacts, "record-store-compact-"));
  try {
    for (let writer = 0; writer < 6; writer++) {
      const store = openRecordStore(directory);
      store.put(recordKey("record-store-law", String(writer)), Buffer.from(`v${writer}`, "utf8"));
      store.close();
    }
    expect(readdirSync(directory).filter((entry) => entry.endsWith(".idx")).length).toBe(6);
    const store = openRecordStore(directory);
    expect(store.stats.packs).toBeGreaterThan(RECORD_STORE_PACK_LIMIT);
    store.compact();
    expect(store.stats.packs).toBe(1);
    store.close();
    expect(readdirSync(directory).filter((entry) => entry.endsWith(".idx")).length).toBe(1);
    const reopened = openRecordStore(directory);
    for (let writer = 0; writer < 6; writer++) expect(Buffer.from(reopened.get(recordKey("record-store-law", String(writer)))!).toString("utf8")).toBe(`v${writer}`);
    reopened.close();
    assertPublishedLayout(directory);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test("fixture scenarios replay exactly", () => {
  expect(fixture.schemaVersion).toBe(1);
  for (const scenario of fixture.scenarios) {
    const directory = mkdtempSync(join(artifacts, "record-store-"));
    try {
      replay(directory, scenario.operations, (operation, actual) => expect(actual, `${scenario.id} ${operation.key}`).toBe(operation.expect ?? null));
      assertPublishedLayout(directory);
    } finally { rmSync(directory, { recursive: true, force: true }); }
  }
});

test("random operation sequences over two handles agree with an in-memory model of pack snapshots", () => {
  const next = random(31), keys = ["a", "b", "c", "d", "e", "f"], valueOf = (name: string): string => ["", "x", "yy", "zzz", "😀", "a\u0000b"][keys.indexOf(name)]!;
  const key = (name: string): string => recordKey("record-store-law", name);
  let checked = 0;
  for (let round = 0; round < 80; round++) {
    const directory = mkdtempSync(join(artifacts, "record-store-random-"));
    try {
      type Model = { store: RecordStore; known: Set<number>; believed: Set<string>; pending: Set<string>; accessed: Set<string>; uncertain: boolean };
      const packs = new Map<number, Set<string>>();
      let packId = 0;
      const open = (): Model => ({ store: openRecordStore(directory), known: new Set(packs.keys()), believed: new Set([...packs.values()].flatMap((pack) => [...pack])), pending: new Set(), accessed: new Set(), uncertain: false });
      const visible = (model: Model, item: string): boolean => model.pending.has(item) || [...model.known].some((id) => packs.get(id)?.has(item) ?? false);
      const publish = (model: Model, items: Iterable<string>): void => { const pack = new Set(items); if (pack.size === 0) return; packs.set(++packId, pack); model.known.add(packId); };
      const models = new Map<string, Model>([["a", open()], ["b", open()]]), trace: string[] = [];
      for (let step = 0; step < 50; step++) {
        const name = next() < 0.5 ? "a" : "b", model = models.get(name)!, other = models.get(name === "a" ? "b" : "a")!, roll = next(), item = keys[Math.floor(next() * keys.length)]!;
        trace.push(`${name}:${roll < 0.35 ? "put" : roll < 0.7 ? "get" : roll < 0.82 || roll < 0.9 && model.uncertain ? "flush" : roll < 0.9 ? "reopen" : roll < 0.95 || model.uncertain ? "crash" : "retain"}:${item}`);
        if (roll < 0.35) {
          model.store.put(key(item), Buffer.from(valueOf(item), "utf8"));
          model.accessed.add(item);
          if (!model.believed.has(item)) { model.believed.add(item); model.pending.add(item); }
        } else if (roll < 0.7) {
          const value = model.store.get(key(item)), actual = value === undefined ? null : Buffer.from(value).toString("utf8"), expected = model.believed.has(item) ? valueOf(item) : null;
          if (model.uncertain && !model.pending.has(item) && expected !== null) expect([null, expected], `round ${round} step ${step}`).toContain(actual);
          else expect(actual, `round ${round} step ${step} ${name} ${item} ${trace.join(" ")}`).toBe(expected);
          if (actual !== null) model.accessed.add(item);
          checked++;
        } else if (roll < 0.82 || roll < 0.9 && model.uncertain) {
          model.store.flush();
          publish(model, model.pending);
          model.pending.clear();
        } else if (roll < 0.9) {
          model.store.close();
          publish(model, model.pending);
          models.set(name, open());
        } else if (roll < 0.95 || model.uncertain) {
          models.set(name, open());
        } else {
          model.store.retainAccessed();
          publish(model, model.pending);
          model.pending.clear();
          const live = [...model.accessed].filter((accessedItem) => visible(model, accessedItem));
          for (const id of model.known) packs.delete(id);
          model.known.clear();
          packs.set(++packId, new Set(live));
          model.known.add(packId);
          model.believed = new Set(live);
          other.uncertain = true;
        }
      }
      for (const model of models.values()) model.store.close();
      assertPublishedLayout(directory);
    } finally { rmSync(directory, { recursive: true, force: true }); }
  }
  expect(checked).toBeGreaterThan(500);
});

test("record keys separate inputs by length and closure digests track every imported byte", () => {
  expect(recordKey("n", "ab", "c")).not.toBe(recordKey("n", "a", "bc"));
  expect(recordKey("n", "a")).toBe(recordKey("n", Buffer.from("a")));
  const directory = mkdtempSync(join(artifacts, "record-closure-"));
  try {
    mkdirSync(join(directory, "lib"));
    writeFileSync(join(directory, "entry.ts"), `import { a } from "./lib/a.ts";\nexport const b = await import("./lib/b.ts");\n`);
    writeFileSync(join(directory, "lib", "a.ts"), "export const a = 1;\n");
    writeFileSync(join(directory, "lib", "b.ts"), "export const b = 1;\n");
    const before = moduleSourceClosureDigest(join(directory, "entry.ts"));
    writeFileSync(join(directory, "lib", "b.ts"), "export const b = 2;\n");
    expect(moduleSourceClosureDigest(join(directory, "entry.ts"))).not.toBe(before);
    writeFileSync(join(directory, "lib", "b.ts"), "export const b = 1;\n");
    expect(moduleSourceClosureDigest(join(directory, "entry.ts"))).toBe(before);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});


test("stale compaction preserves independently projected immutable events",()=>{
  const oracle=new Database(":memory:");oracle.exec("CREATE TABLE events(sequence INTEGER, name TEXT, value TEXT)");
  try{for(const scenario of fixture.scenarios.filter(row=>row.id.startsWith("stale-"))){
    const directory=mkdtempSync(join(artifacts,"record-event-oracle-"));oracle.exec("DELETE FROM events");
    try{scenario.operations.forEach((row,index)=>{if(row.op==="put")oracle.query("INSERT INTO events VALUES(?,?,?)").run(index,row.key!,row.value!);});replay(directory,scenario.operations,(row,actual)=>expect(actual).toBe(row.expect??null));
      const expected=oracle.query("SELECT name,value FROM (SELECT name,value,row_number() OVER(PARTITION BY name ORDER BY sequence) position FROM events) WHERE position=1 ORDER BY name").all() as {name:string;value:string}[],store=openRecordStore(directory);
      try{expect(expected.map(row=>({name:row.name,value:Buffer.from(store.get(recordKey("record-store-law",row.name))!).toString("utf8")}))).toEqual(expected);}finally{store.close();}
    }finally{rmSync(directory,{recursive:true,force:true});}
  }}finally{oracle.close();}
});

test("each same-pack reader occupies its own bounded descriptor slot",()=>{
  const directory=mkdtempSync(join(artifacts,"record-shared-descriptors-")),readers:RecordStore[]=[],key=recordKey("shared-pack"),baseline=openRecordStoreDescriptorCount();
  try{const writer=openRecordStore(directory);writer.put(key,Buffer.from("one"));writer.close();
    for(let index=0;index<RECORD_STORE_DESCRIPTOR_LIMIT+8;index++){const reader=openRecordStore(directory);readers.push(reader);expect(Buffer.from(reader.get(key)!).toString()).toBe("one");expect(openRecordStoreDescriptorCount()).toBe(Math.min(baseline+index+1,RECORD_STORE_DESCRIPTOR_LIMIT));if(existsSync("/proc/self/fd")){const physical=readdirSync("/proc/self/fd").filter(name=>{try{return readlinkSync("/proc/self/fd/"+name).startsWith(directory+"/");}catch{return false;}}).length;expect(physical).toBe(Math.min(index+1,RECORD_STORE_DESCRIPTOR_LIMIT));}}
  }finally{for(const reader of readers)reader.close();rmSync(directory,{recursive:true,force:true});}
  expect(openRecordStoreDescriptorCount()).toBe(0);
});


test("retirement reconciles before publication and charges only physically committed changes",()=>{
  for(const row of fixture.retirementCases){
    const directory=mkdtempSync(join(artifacts,"record-retirement-")),key=recordKey("retirement-law");let late:RecordStore|undefined,peer:RecordStore|undefined;
    try{
      const initial=openRecordStore(directory);initial.put(key,Buffer.from("alpha"));initial.close();
      const names=fg.sync("*.{idx,pack}",{cwd:directory}),originals=names.map(name=>({name,bytes:readFileSync(join(directory,name))})),size=()=>fg.sync("*.{idx,pack}",{cwd:directory}).reduce((sum,name)=>sum+statSync(join(directory,name)).size,0);let attempts=0,commits=0,physicalBefore=0,delta=0;
      peer=openRecordStore(directory);
      late=openRecordStore(directory,{publication:(plan,write)=>{
        attempts++;
        if(row.id==="decline")return false;
        if(row.id==="peer-compact"&&attempts===1)peer!.compact();
        if(row.id==="continuous")for(const file of originals){const path=join(directory,file.name);if(attempts%2)rmSync(path,{force:true});else writeFileSync(path,file.bytes);}
        physicalBefore=size();write();commits++;delta+=plan.retainedDeltaBytes;expect(size()-physicalBefore).toBe(plan.retainedDeltaBytes);return true;
      }});
      if(row.id==="nonfile"){const path=join(directory,names.find(name=>name.endsWith(".idx"))!);rmSync(path);mkdirSync(path);expect(()=>late!.compact()).toThrow("Ordinary finite record retirement leaf required");expect(statSync(path).isDirectory()).toBe(true);}
      else if(row.outcome==="unstable")expect(()=>late!.compact()).toThrow("Record retirement snapshot did not stabilize");
      else{late.compact();expect(Buffer.from(late.get(key)!).toString()).toBe("alpha");}
      expect(attempts,row.id).toBe(row.attempts);expect(commits,row.id).toBe(row.commits);if(!commits)expect(delta).toBe(0);
      if(row.id==="continuous")expect(fg.sync("*.{idx,pack}",{cwd:directory})).toEqual([]);
      if(row.id==="decline")expect(fg.sync("*.{idx,pack}",{cwd:directory}).sort()).toEqual(names.sort());
    }finally{late?.close();peer?.close();rmSync(directory,{recursive:true,force:true});}
  }
});
