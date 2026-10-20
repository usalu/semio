import"../../../../../🧪️tests/📐️phases/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "@semio-tech/framework-filesystem/observation/journal/schema.json";
import { mkdir, mkdtemp, writeFile, rm, symlink, rename, readdir } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { createRepositoryFilesystemJournalV1 } from "../🟦️.ts";
import { createLiveFilesystemJournalV1 } from "@semio-tech/framework-filesystem/observation/journal";
import { nxGraphSourceMembershipV2 } from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import type { FilesystemJournalChangeV1, FilesystemJournalControlV1 } from "@semio-tech/framework-filesystem/observation/journal";

test("repository journal joins installed native watcher, Git census and no-follow scope", async () => {
 const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR; if (!artifact) throw Error("Ticket artifact directory required");
 await mkdir(dirname(artifact), { recursive: true });
 const root = await mkdtemp(join(dirname(artifact), "j-")), abort = new AbortController(), start = performance.now(), seen: FilesystemJournalChangeV1[] = [];
 const control: FilesystemJournalControlV1 = { signal: abort.signal, remainingMilliseconds: () => 30_000 - (performance.now() - start), publish: async () => {}, yieldContinuation: async () => { await delay(0); } };
 const require = createRequire(join(process.cwd(), "package.json")), { Watcher } = require(join(dirname(require.resolve("nx/package.json")), "dist/src/native/index.js"));
 let receive: ((error: unknown, events: readonly FilesystemJournalChangeV1[]) => void) | undefined, watcher: any, journal: Awaited<ReturnType<typeof createRepositoryFilesystemJournalV1>> | undefined;
 try {
  await mkdir(join(root, ".git/objects"), { recursive: true }); await mkdir(join(root, ".git/refs/heads"), { recursive: true });
  await writeFile(join(root, ".git/HEAD"), "ref: refs/heads/main\n"); await writeFile(join(root, ".git/config"), "[core]\nrepositoryformatversion = 0\nbare = false\n");
  for (const owner of ["a", "b"]) { await mkdir(join(root, owner)); await writeFile(join(root, owner, "package.json"), JSON.stringify({ name: owner })); await writeFile(join(root, owner, "main.ts"), "export const answer = 1;\n"); }
  journal = await createRepositoryFilesystemJournalV1(root, {
   watch(callback) { receive = callback; watcher = new Watcher(root, [], false); watcher.watch((error: unknown, events: FilesystemJournalChangeV1[]) => { if(events) seen.push(...events); callback(error, events ?? []); }); return async () => { await watcher.stop(); }; },
   flush() { const events = watcher.forceFlushPending(); seen.push(...events); return events; },
   async census(current) { return (await nxGraphSourceMembershipV2(root, current.signal)).files; },
   async admittedPaths(paths, current) { const files = new Set((await nxGraphSourceMembershipV2(root, current.signal)).files); return paths.filter(path => files.has(path)); },
   isGlobalIdentity: path => path.endsWith("/package.json")
  }, control);
  const scope = { roots: ["a"], files: [] }, original = await journal.capture(scope, control);
  expect(original.globalIdentity.paths).toEqual(["a/package.json", "b/package.json"]);
  await writeFile(join(root, "b/main.ts"), "export const answer = 2;\n");
  for(let attempt = 0; attempt < 50 && !seen.some(event => event.path === "b/main.ts"); attempt++) { await delay(20); await journal.capture(scope, control); }
  expect(seen.some(event => event.path === "b/main.ts")).toBe(true);
  expect(await journal.verify(original, scope, control)).toBe(true);
  await mkdir(join(root, "c")); await writeFile(join(root, "c/package.json"), JSON.stringify({ name: "c" }));
  for(let attempt = 0; attempt < 50 && !seen.some(event => event.path === "c/package.json" || event.path === "c"); attempt++) { await delay(20); await journal.capture(scope, control); }
  expect(seen.some(event => event.path === "c/package.json" || event.path === "c")).toBe(true);
  expect(await journal.verify(original, scope, control)).toBe(false);
  await symlink(join(root, "a"), join(root, "alias"), process.platform === "win32" ? "junction" : "dir");
  await expect(journal.capture({ roots: ["alias"], files: [] }, control)).rejects.toThrow("no-follow");
 } finally { await journal?.close(); if(!journal && watcher) await watcher.stop(); await rm(root, { recursive: true, force: true }); }
}, 35_000);

test("cold journal discovers a wholly ignored structural owner absent from native census",async()=>{
 const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Ticket artifact directory required");await mkdir(dirname(artifact),{recursive:true});const root=await mkdtemp(join(dirname(artifact),"ji-")),signal=new AbortController().signal;
 const control:FilesystemJournalControlV1={signal,remainingMilliseconds:()=>30000,publish:async()=>{},yieldContinuation:async()=>{}};
 let journal:Awaited<ReturnType<typeof createRepositoryFilesystemJournalV1>>|undefined;
 try{
  await mkdir(join(root,".git/objects"),{recursive:true});await mkdir(join(root,".git/refs/heads"),{recursive:true});await writeFile(join(root,".git/HEAD"),"ref: refs/heads/main\n");await writeFile(join(root,".git/config"),"[core]\nrepositoryformatversion = 0\nbare = false\n");
  await writeFile(join(root,".gitignore"),fixture.ignoredOwner.ignore);await mkdir(dirname(join(root,fixture.ignoredOwner.path)),{recursive:true});await writeFile(join(root,fixture.ignoredOwner.path),fixture.ignoredOwner.contents);
  const ordinary=(await nxGraphSourceMembershipV2(root,signal)).files;expect(ordinary).not.toContain(fixture.ignoredOwner.path);
  journal=await createRepositoryFilesystemJournalV1(root,{watch(){return async()=>{};},flush(){return [];},async census(){return ordinary;},async admittedPaths(paths){return paths.filter(path=>ordinary.includes(path));},isGlobalIdentity:path=>path.endsWith("/package.json")},control);
  expect((await journal.capture({roots:[],files:[]},control)).globalIdentity.paths).toEqual(fixture.expectedGlobal);
 }finally{await journal?.close();await rm(root,{recursive:true,force:true});}
},35000);


test("directory deletion alone admits the actual renamed parent membership and its selected witness",async()=>{
 const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Ticket artifact directory required");const root=await mkdtemp(join(dirname(artifact),"jr-")),law=fixture.structuralRename;
 const control:FilesystemJournalControlV1={signal:new AbortController().signal,remainingMilliseconds:()=>30000,publish:async()=>{},yieldContinuation:async()=>{}};
 let receive:((error:unknown,events:readonly FilesystemJournalChangeV1[])=>void)|undefined,journal:Awaited<ReturnType<typeof createRepositoryFilesystemJournalV1>>|undefined;
 try{
  await mkdir(join(root,law.source),{recursive:true});for(const file of law.files)await writeFile(join(root,law.source,file),file==="package.json"?'{"name":"branch"}':'export const branch = 1;');
  journal=await createRepositoryFilesystemJournalV1(root,{watch(callback){receive=callback;return()=>{}},flush(){return[]},async census(){return law.files.map(file=>law.source+"/"+file)},async admittedPaths(paths){return paths},isGlobalIdentity:path=>path.endsWith("/package.json")},control);
  const scope={roots:["owners"],files:[]},before=await journal.capture(scope,control);await rename(join(root,law.source),join(root,law.destination));receive!(null,law.nativeEvents as readonly FilesystemJournalChangeV1[]);
  const observed=await journal.capture(scope,control),physical=(await readdir(join(root,law.destination))).map(file=>law.destination+"/"+file).sort();expect(physical).toEqual(law.expectedMembership);expect(observed.selectedMembership.paths).toEqual(physical);expect(observed.globalIdentity.paths).toEqual(law.expectedGlobal);expect(await journal.verify(before,scope,control)).toBe(false);
  expect(new Ajv({strict:true}).compile(schema)(observed)).toBe(true);
  const selected=await journal.capture({roots:[law.destination],files:[]},control);expect(selected.sequence).toBeGreaterThan(0);expect(selected.changes.some(row=>row.path===law.destination+"/main.ts"&&row.type==="create")).toBe(true);
 }finally{await journal?.close();await rm(root,{recursive:true,force:true})}
},35000);

/** 🕰️ Journal transport preserves the initiating absolute deadline beyond one second. */
test("journal RPC retains the caller deadline through delayed reconciliation",async()=>{
 const rpc=await import("../🌐️rpc/🟦️.ts"),law=await Bun.file(new URL("../../../../../🧪️tests/🕸️daemon/📥️journal/🧫️fixtures/🔣️.json",import.meta.url)).json();
 const request={...law.request,deadlineUnixMilliseconds:law.deadline.deadlineUnixMilliseconds};
 const schema=await Bun.file(new URL("../🌐️rpc/🧬️schema/🔣️.json",import.meta.url)).json(),neutral=await Bun.file(new URL("../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🧬️schema/🔣️.json",import.meta.url)).json();
 const ajv=new Ajv({strict:true}).addSchema(neutral),validate=ajv.compile(schema);expect(validate(request)).toBe(true);expect(rpc.readFilesystemJournalRpcRequestV1(request)).toEqual(request);const{deadlineUnixMilliseconds,...missing}=request;expect(validate(missing)).toBe(false);expect(()=>rpc.readFilesystemJournalRpcRequestV1(missing)).toThrow();
 const{Database}=await import("bun:sqlite"),db=new Database(":memory:");try{expect(rpc.filesystemJournalRpcRemainingMillisecondsV1(request,law.deadline.later)).toBe((db.query("SELECT ?1-?2 AS remaining").get(law.deadline.deadlineUnixMilliseconds,law.deadline.later)as{remaining:number}).remaining);expect(rpc.filesystemJournalRpcRemainingMillisecondsV1({...request,deadlineUnixMilliseconds:null},law.deadline.later)).toBeNull()}finally{db.close()}
 const abort=new AbortController(),started=Date.now(),deadline=started+3000,control:FilesystemJournalControlV1={signal:abort.signal,remainingMilliseconds:()=>rpc.filesystemJournalRpcRemainingMillisecondsV1({...request,deadlineUnixMilliseconds:deadline},Date.now()),publish:async()=>{},yieldContinuation:async()=>{}};
 let receive!: (error:unknown,events:readonly FilesystemJournalChangeV1[])=>void;const journal=await createLiveFilesystemJournalV1({source:{watch(callback){receive=callback;return()=>{}},async census(){return[]},flush:()=>[],async reconcile(event){await delay(law.deadline.workMilliseconds);return[event.path]},async admitScope(){}},excluded:()=>false,isGlobalIdentity:()=>false,maximumPaths:10,maximumEvents:10},control);
 try{receive(null,[{path:"sample/main.ts",type:"create"}]);expect((await journal.capture({roots:["sample"],files:[]},control)).selectedMembership.paths).toEqual(["sample/main.ts"]);expect(Date.now()-started).toBeGreaterThan(1000);expect(rpc.filesystemJournalRpcRemainingMillisecondsV1(request,4000)).toBe(0)}finally{await journal.close()}
});
