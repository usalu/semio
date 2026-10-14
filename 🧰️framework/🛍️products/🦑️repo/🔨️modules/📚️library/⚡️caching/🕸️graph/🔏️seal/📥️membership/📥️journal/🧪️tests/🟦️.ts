import { expect, test } from "bun:test";
import { mkdir, mkdtemp, writeFile, rm, symlink } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { createRepositoryFilesystemJournalV1 } from "../🟦️.ts";
import { nxGraphSourceMembershipV2 } from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import type { FilesystemJournalChangeV1, FilesystemJournalControlV1 } from "../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts";

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
  console.log("[DEBUG] Native watcher/Git journal preserved unrelated source, witnessed new owner and refused symlink");
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
  console.log("[DEBUG] wholly ignored structural owner admitted alongside native effective census");
 }finally{await journal?.close();await rm(root,{recursive:true,force:true});}
},35000);
