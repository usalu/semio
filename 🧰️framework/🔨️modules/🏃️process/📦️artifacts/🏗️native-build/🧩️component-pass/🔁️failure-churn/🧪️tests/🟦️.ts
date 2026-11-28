import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {basename,join,resolve} from "node:path";
import {mkdtemp,rm,writeFile} from "node:fs/promises";
import {spawn} from "node:child_process";
import {EventEmitter,once} from "node:events";
import {blake3} from "@noble/hashes/blake3.js";
import {sha256} from "@noble/hashes/sha2.js";
import {admitWarmCargoRetryGraphV1,classifyCargoGraphObservationFailureV1,failedCompilerMessageFilesV1,failedDependencyDriftV1,failedPassConcurrentEditV1,retryableWarmComponentPassV1,runCargoPassAttemptsV1,watchWarmCargoAttemptV1} from "../🟦️.ts";
import { concurrentEditOfV1, churnProtocolV1, retryOnConcurrentEditV1 } from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {terminateOwnedChildTree} from "../../../../../🪓️termination/🟦️.ts";
import {watchCargoSourceDriftV1} from "../../../📥️source/👁️drift/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const utf8 = (text:string) => new TextEncoder().encode(text);
const oracle = (text:string) => Buffer.from(blake3(utf8(text))).toString("hex");
const pkg = (name:string) => "path+file:///workspace/" + name + "#0.1.0";
const unit = (name:string,indexes:readonly number[],packageName=name) => ({
  identity:name,pkg_id:pkg(packageName),dependencies:indexes.map(index=>({index})),
});
const units = [unit("failed",[1]),unit("direct",[2]),unit("transitive",[]),unit("unrelated",[]),unit("host",[],"direct")];

for (const row of fixture.earlyDrift) test(row.name, async () => {
  const drift = {path:resolve("workspace","source.rs"),expected:oracle(fixture.consumed),observed:oracle(fixture.replacement)};
  const controller = new AbortController();
  if (row.cancelled) controller.abort();
  const events:string[] = [];
  const watcher = watchWarmCargoAttemptV1({
    sources:new Map([[drift.path,drift.expected]]), startedMs:1, signal:controller.signal,
    mayRetry:row.remaining ? () => !row.repeated : undefined,
    active:() => row.active && !row.linked,
    stopChild:() => events.push("stop"),
  }, (_sources,_started,onDrift) => {
    onDrift(drift);
    return async () => { events.push("drain"); };
  });
  await watcher.close();
  const db = new Database(":memory:");
  try {
    const expected = db.query("SELECT ?1 AND NOT ?2 AND NOT ?3 AND ?4 AND NOT ?5 AS stopped")
      .get(Number(row.active),Number(row.linked),Number(row.cancelled),Number(row.remaining),Number(row.repeated)) as {stopped:number};
    expect(Boolean(expected.stopped)).toBe(row.stopped);
    expect(watcher.drift()).toEqual(row.stopped ? drift : undefined);
    expect(events).toEqual(row.remaining ? [...(row.stopped ? ["stop"] : []),"drain"] : []);
  } finally { db.close(); }
});

test("admitted source watcher retires an active attempt after a proved byte change",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"drift-"));
  const path=join(root,"source.rs"), hash=(text:string)=>Buffer.from(sha256(utf8(text))).toString("hex");
  const controller=new AbortController(), deadline=AbortSignal.timeout(4000);
  const child=spawn(process.execPath,["-e","console.log('ready'); setTimeout(() => process.exit(0), 4000);"],{detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"]});
  const terminal=once(child,"close");
  try{
    await once(child.stdout!,"data",{signal:deadline});
    await writeFile(path,fixture.consumed);
    const watcher=watchWarmCargoAttemptV1({
      sources:new Map([[path,hash(fixture.consumed)]]),startedMs:Date.now(),signal:controller.signal,
      mayRetry:()=>true,active:()=>child.exitCode===null&&child.signalCode===null,
      stopChild:()=>terminateOwnedChildTree(child),
    });
    try{
      await writeFile(path,fixture.replacement);
      const [exit]=await terminal;
      expect(exit).not.toBe(0);
      expect(watcher.drift()).toEqual({path,expected:hash(fixture.consumed),observed:hash(fixture.replacement)});
      expect(controller.signal.aborted).toBe(false);
    }finally{await watcher.close();}
  }finally{
    terminateOwnedChildTree(child);
    await terminal.catch(()=>{});
    await rm(root,{recursive:true,force:true});
  }
},5000);

for (const mode of ["warm","standalone"] as const) test(`${mode} source watcher closes only on its accepted mismatch`,async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"drift-"));
  const row=fixture.watchLifecycle;
  const [first,second,third]=row.changes.map(file=>join(root,file));
  const expected=Buffer.from(sha256(utf8(fixture.consumed))).toString("hex");
  const observed=Buffer.from(sha256(utf8(fixture.replacement))).toString("hex");
  const sources=row.changes.map(file=>({path:join(root,file),digest:expected}));
  const events=new EventEmitter(),offered:string[]=[];
  const controller=new AbortController();
  const db=new Database(":memory:");
  let close=async()=>{},accepted:string|undefined;
  try{
    const oracle=db.query("SELECT value AS path FROM json_each(?1) WHERE ?3 = 'standalone' OR value NOT IN (SELECT value FROM json_each(?2)) ORDER BY key LIMIT 1")
      .get(JSON.stringify(row.changes),JSON.stringify(row.handled),mode) as {path:string};
    expect(oracle.path).toBe(mode==="warm"?row.warmAccepted:row.standaloneAccepted);
    await Promise.all(sources.map(({path})=>writeFile(path,fixture.consumed)));
    const startedMs=Date.now();
    if(mode==="warm"){
      let stops=0;
      const watcher=watchWarmCargoAttemptV1({
        sources:new Map(sources.map(({path,digest})=>[path,digest])),
        startedMs,signal:controller.signal,active:()=>true,
        mayRetry:({path})=>{
          offered.push(basename(path));
          events.emit("offered");
          return !row.handled.includes(basename(path));
        },
        stopChild:()=>{stops++;events.emit("accepted");},
      });
      close=watcher.close;
      const firstOffer=once(events,"offered",{signal:AbortSignal.timeout(1800)});
      await writeFile(first!,fixture.replacement);
      await firstOffer;
      expect(watcher.drift()).toBeUndefined();
      const secondOffer=once(events,"accepted",{signal:AbortSignal.timeout(1800)});
      await writeFile(second!,fixture.replacement);
      await secondOffer;
      expect(watcher.drift()).toEqual({path:second,expected,observed});
      accepted=basename(watcher.drift()!.path);
      await writeFile(third!,fixture.replacement);
      await new Promise(resolve=>setTimeout(resolve,750));
      expect(stops).toBe(1);
      expect([...new Set(offered)]).toEqual(row.changes.slice(0,2));
    }else{
      close=watchCargoSourceDriftV1(sources,startedMs,({path})=>{
        offered.push(basename(path));
        accepted=basename(path);
        events.emit("accepted");
      });
      const firstOffer=once(events,"accepted",{signal:AbortSignal.timeout(1800)});
      await writeFile(first!,fixture.replacement);
      await firstOffer;
      await writeFile(second!,fixture.replacement);
      await new Promise(resolve=>setTimeout(resolve,750));
      expect(offered).toEqual(row.changes.slice(0,1));
    }
    expect(accepted).toBe(oracle.path);
  }finally{
    await close();
    db.close();
    await rm(root,{recursive:true,force:true});
  }
},5000);

for (const row of fixture.cases) test(row.name,async () => {
  const workspace=resolve("workspace"),build=resolve("build"),target=resolve("target");
  const artifacts:{identity:string;filenames:string[]}[]=[],files=new Map<string,Uint8Array>(),events:string[]=[];
  const checksum=oracle(fixture.consumed);
  expect(utf8(fixture.replacement).length).toBe(utf8(fixture.consumed).length);
  for (const identity of ["direct","transitive","unrelated","host"]) {
    const source=resolve(workspace,identity+".rs"),stem=resolve(build,"out",identity+"-abc");
    artifacts.push({identity,filenames:[resolve(build,"out","lib"+identity+"-abc.rmeta")]});
    files.set(source,utf8(identity===row.changed ? fixture.replacement : fixture.consumed));
    files.set(stem+".d",utf8("# checksum:blake3="+checksum+" file_len:"+utf8(fixture.consumed).length+" "+source+"\n"));
  }

  const result=runCargoPassAttemptsV1(async suffix=>{
    events.push(suffix ? "retry" : "initial");
    return suffix ? row.retryExit : row.initialExit;
  },async exit=>{
    if (exit===0) return undefined;
    const drift=await failedDependencyDriftV1({
      workspace,build,target,graph:{units},artifacts,failed:new Map([[pkg("failed"),new Set([row.code])]]),
      read:async path=>files.get(path),
    });
    if (!drift) return undefined;
    expect(drift).toEqual({path:resolve(workspace,row.changed+".rs"),expected:checksum,observed:oracle(fixture.replacement)});
    return drift;
  },async () => {
    events.push("replan");
    if (!row.graphStable) throw Error("graph changed");
  });
  if (row.exit===null) await expect(result).rejects.toThrow("graph changed");
  else expect(await result).toBe(row.exit);
  expect(events).toEqual(row.attempts===2 ? ["initial","replan","retry"] : row.graphStable ? ["initial"] : ["initial","replan"]);
});

for (const row of fixture.successiveChanges) test(row.name, async () => {
  const calls:string[] = [];
  const source = (attempt:number) => row.sources[attempt];
  const result = await runCargoPassAttemptsV1(async (suffix, mayRetry) => {
    calls.push(suffix);
    const path = source(calls.length - 1);
    if (path) expect(mayRetry?.({path,expected:oracle(path),observed:oracle(path+" changed")}) ?? false)
      .toBe(calls.length < 4 && !row.sources.slice(0,calls.length - 1).includes(path));
    return source(calls.length - 1) ? 101 : 0;
  }, async exit => {
    const path = source(calls.length - 1);
    return exit && path ? {path, expected:oracle(path), observed:oracle(path+" changed")} : undefined;
  }, async () => {});
  expect(calls).toEqual(row.suffixes);
  expect(result).toBe(row.exit);
  const db=new Database(":memory:");
  try {
    const facts=db.query("select min(4, count(distinct value)+1) as attempts from json_each(?1)").get(JSON.stringify(row.sources)) as {attempts:number};
    expect(calls.length).toBe(facts.attempts);
  } finally {db.close();}
});

for (const [index, row] of fixture.retryEligibility.entries()) test(`component retry eligibility ${index + 1}`, () => {
  const passes = row.passes.map(pass => ({ profile: pass.profile, packages: Array(pass.components).fill(null) }));
  expect(retryableWarmComponentPassV1(row.generation, passes)).toBe(row.enabled);
  const database = new Database(":memory:");
  try {
    const query = database.query("select ?1 = 'warm' and count(*) = 1 and sum(json_extract(value, '$.components')) > 0 as enabled from json_each(?2)");
    const expected = query.get(row.generation, JSON.stringify(row.passes)) as { enabled: number };
    expect(Boolean(expected.enabled)).toBe(row.enabled);
  } finally {
    database.close();
  }
});

for (const row of fixture.warmGraph) test(row.name,async () => {
  const database=new Database(":memory:");
  try {
    const expected=database.query("select iif(?2 = ?3 and (?1 = ?3 or ?4 <= min(?5,?6)),?3,null) as admitted")
      .get(row.previous,row.planned,row.admitted,row.replacementBytes,row.heldBytes,row.freeBytes) as {admitted:string|null};
    expect(expected.admitted).toBe(row.result);
    const events:string[]=[];
    const result=admitWarmCargoRetryGraphV1({
      suffix:"-retry",
      manifest:resolve("workspace","Cargo.toml"),
      previous:{unit:row.previous},
      observe:async suffix=>{events.push(suffix);return {unit:suffix==="-retry-plan"?row.planned:row.admitted};},
      admit:async()=>{events.push("source");},
      replacementFits:async()=>{events.push("budget");return row.replacementBytes<=Math.min(row.heldBytes,row.freeBytes);},
    });
    if (row.result===null) await expect(result).rejects.toThrow("Cargo unit graph");
    else expect(await result).toEqual({unit:row.result});
    expect(events).toEqual(["-retry-plan","source","-retry",...(row.checkReplacement?["budget"]:[])]);
  } finally {
    database.close();
  }
});


test("component build retry drops an incompatible held graph before admitting a new plan", async () => {
  const row = fixture.warmGraph.find(row => row.name === "replacement exceeds held footprint")!;
  const database = new Database(":memory:");
  try {
    const expected = database.query("SELECT iif(?1 > ?2 AND ?1 <= ?3, ?4, null) AS unit")
      .get(row.replacementBytes, row.heldBytes, row.freeBytes, row.admitted);
    const cancellation = new AbortController();
    const events: string[] = [];
    let held = false;
    const graph = await retryOnConcurrentEditV1("build", async attempt => {
      expect(held).toBe(false);
      held = true;
      events.push(`acquire:${attempt}`);
      try {
        const previous = attempt === 1 ? row.previous : row.admitted;
        return await admitWarmCargoRetryGraphV1({
          manifest: resolve("workspace", "Cargo.toml"),
          suffix: "-retry",
          previous: { unit: previous },
          observe: async () => ({ unit: row.admitted }),
          admit: async () => { events.push(`admit:${attempt}`); },
          replacementFits: async () => row.replacementBytes <= row.heldBytes,
        });
      } finally {
        held = false;
        events.push(`release:${attempt}`);
      }
    }, {
      signal: cancellation.signal,
      onRetry: event => events.push(`retry:${event.attempt}`),
      sleep: async (_milliseconds, signal) => { expect(signal).toBe(cancellation.signal); },
    });
    expect<unknown>(graph).toEqual(expected);
    expect(events).toEqual(["acquire:1", "admit:1", "release:1", "retry:2", "acquire:2", "admit:2", "release:2"]);
  } finally { database.close(); }
});


for (const outcome of fixture.uncodedDiagnostic.outcomes) test(outcome.name, async () => {
  const row = fixture.uncodedDiagnostic, repository = resolve("workspace");
  const paths = failedCompilerMessageFilesV1({
    manifest_path: resolve(repository, row.manifest),
    target: {src_path:resolve(repository, row.target)},
    message: {spans:[{file_name:row.span}]},
  }, repository, resolve(repository, row.cargoWorkspace));
  const database = new Database(":memory:");
  try {
    const available = database.prepare<{modified:number},[string,string]>(
      "select 1 as modified from json_each(?1) where value = ?2",
    );
    const present = JSON.stringify(row.expected.filter(path=>path!==outcome.vanished).map(path=>resolve(repository,path)));
    const edit = await failedPassConcurrentEditV1({
      startedMs: 2, files: paths, recheck: async () => {},
      modifiedMs: path=>available.get(present,path)?.modified,
    });
    expect(paths).toEqual(row.expected.map(path=>resolve(repository,path)));
    if (outcome.vanished) {
      expect(edit?.reason).toBe("vanished");
      expect(edit?.path).toBe(resolve(repository,outcome.vanished));
    } else expect(edit).toBeUndefined();
  } finally { database.close(); }
});


test("only a stale locked Cargo unit graph requests fresh build admission", () => {
  const manifest = resolve("workspace", "Cargo.toml");
  const database = new Database(":memory:");
  try {
    const classify = database.query("SELECT instr(?1, 'cannot update the lock file') > 0 AS replan");
    for (const row of fixture.graphObservations) {
      const { replan } = classify.get(row.stderr) as { replan: number };
      expect(Boolean(replan)).toBe(row.replan);
      const failure = new Error(row.stderr);
      const classified = classifyCargoGraphObservationFailureV1(failure, manifest);
      if (row.replan) {
        const buildRank = churnProtocolV1().profiles.build.rank;
        expect(concurrentEditOfV1(classified, buildRank)).toEqual({ path: manifest, reason: "input", rank: buildRank - 1 });
        expect(concurrentEditOfV1(classified, buildRank - 1)).toBeUndefined();
        expect((classified as Error).cause).toBe(failure);
      } else {
        expect(classified).toBe(failure);
      }
    }
  } finally { database.close(); }
});
