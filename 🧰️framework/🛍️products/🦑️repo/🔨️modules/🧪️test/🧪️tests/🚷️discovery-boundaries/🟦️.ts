import { expect, test } from "bun:test";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import glob from "fast-glob";
import layoutVectors from "../../🧫️fixtures/📐️test-layout/🔣️.json";

type Vector = Readonly<{ schemaVersion: number; features: readonly string[]; links: readonly { path: string; target: string; file?: boolean }[]; expected: readonly string[]; candidateCases: readonly {id:string;inputs:readonly string[];expected:readonly string[];mutation?:{path:string;operation:"add"|"remove"}}[]; authorityCases:readonly {id:string;path:string;authority:"root"|"taxonomy"|"policy"}[] }>;
const owner = resolve(import.meta.dir, "../.."), repo = resolve(owner, "../../../../..");
const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🚷️discovery-boundaries/🔣️.json"), "utf8")) as Vector;

const taxonomyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const vocabulary = JSON.parse(readFileSync(join(repo, taxonomyPath), "utf8"));
const plugin = await import(pathToFileURL(join(owner, "🟨️.mjs")).href) as { discoverCaseDirs: (root: string) => string[]; default: {createNodesV2: readonly [string,(files:readonly string[],options:object,context:{workspaceRoot:string})=>Promise<readonly [string,{projects:Record<string,{root:string}>}][]>]} };
const kind = vocabulary.fileKinds[vocabulary.testFeatureFileKindId];
const feature = `${kind.emoji}${kind.extensionChains[0]}`;

test("portable no-follow discovery fixtures satisfy their schema", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);
});

test("Nx test discovery rejects loop, case and feature links and skips generated/opaque trees", () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "case-discovery-"));
  try {
    mkdirSync(dirname(join(root, taxonomyPath)), { recursive: true });
    writeFileSync(join(root, taxonomyPath), JSON.stringify(vocabulary));
    for (const path of fixture.features) {
      mkdirSync(join(root, path), { recursive: true });
      writeFileSync(join(root, path, feature), "Feature: Fixture\n");
    }
    for (const link of fixture.links) {
      mkdirSync(dirname(join(root, link.path)), { recursive: true });
      const junction = process.platform === "win32";
      const target = junction && link.file ? dirname(join(root, link.target)) : join(root, link.target);
      symlinkSync(target, join(root, link.path), junction ? "junction" : link.file ? "file" : "dir");
    }
    const ignored = [
      ...vocabulary.implementationLeafPolicy.ignoredPathPatterns.map((pattern: string) => `${pattern}/**`),
      ...vocabulary.pathEmojiPolicy.reservedSubtreeDirectoryNames.map((segment: string) => `**/${segment}/**`),
      ...Object.values(vocabulary.pathExclusions).map((entry) => `${(entry as { path: string }).path}**`),
    ];
    const oracle = glob.sync(`**/${vocabulary.testsDirName}/*/${feature}`, { cwd: root, dot: true, followSymbolicLinks: false, ignore: ignored }).filter((path) => !lstatSync(join(root, path)).isSymbolicLink()).map((path) => dirname(path).replaceAll("\\", "/")).sort();
    expect(oracle).toEqual([...fixture.expected]);
    expect(plugin.discoverCaseDirs(root)).toEqual(oracle);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("Nx uses the current supplied candidate inventory with independent no-follow glob membership",async()=>{
 const output=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir());mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"case-candidates-"));
 const put=(path:string,content:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);};
 try{
  put(taxonomyPath,JSON.stringify(vocabulary));put(`${dirname(taxonomyPath)}/⚡️caching/🔣️policy.json`,readFileSync(join(repo,dirname(taxonomyPath),"⚡️caching/🔣️policy.json"),"utf8"));put(`${vocabulary.testDomainPath}/📜️script.ts`,"export {};\n");
  for(const path of fixture.features)put(`${path}/${feature}`,"Feature: Candidate\n");
  for(const link of fixture.links){mkdirSync(dirname(join(root,link.path)),{recursive:true});const junction=process.platform==="win32";symlinkSync(junction&&link.file?dirname(join(root,link.target)):join(root,link.target),join(root,link.path),junction?"junction":link.file?"file":"dir");}
  expect(new Set(fixture.candidateCases.map(row=>row.id)).size).toBe(fixture.candidateCases.length);
  for(const row of fixture.candidateCases){
   if(row.mutation){if(row.mutation.operation==="add")put(row.mutation.path,"Feature: Added\n");else rmSync(join(root,row.mutation.path));}
   const ignored=[...vocabulary.implementationLeafPolicy.ignoredPathPatterns.map((pattern:string)=>`${pattern}/**`),...vocabulary.pathEmojiPolicy.reservedSubtreeDirectoryNames.map((segment:string)=>`**/${segment}/**`),...Object.values(vocabulary.pathExclusions).map(entry=>`${(entry as {path:string}).path}**`)];
   const inventory=glob.sync(`**/${vocabulary.testsDirName}/*/${feature}`,{cwd:root,dot:true,followSymbolicLinks:false,ignore:ignored}).filter(path=>!lstatSync(join(root,path)).isSymbolicLink());
   const oracle=[...new Set(row.inputs.map(path=>path.replaceAll("\\","/")).filter(path=>!path.split("/").some(part=>!part||part==="."||part==="..")&&inventory.includes(path)))].sort();expect(oracle,row.id).toEqual([...row.expected].sort());
   const actual=await plugin.default.createNodesV2[1](row.inputs,{}, {workspaceRoot:root});expect(actual.map(entry=>entry[0]).sort(),row.id).toEqual(oracle);expect(actual.flatMap(entry=>Object.values(entry[1].projects)).map(project=>project.root).sort()).toEqual(oracle.map(dirname).sort());
  }
 }finally{rmSync(root,{recursive:true,force:true});}
},30000);

test("fresh authority links and resident Node and Bun source mutations fail closed",async()=>{
 const output=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir());mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"candidate-authority-"));const put=(path:string,content:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);};
 const library=dirname(taxonomyPath),featurePath=`${fixture.expected[0]}/${feature}`,policyPath=`${library}/⚡️caching/🔣️policy.json`;
 try{
  put(taxonomyPath,JSON.stringify(vocabulary));put(policyPath,readFileSync(join(repo,policyPath),"utf8"));put(featurePath,"Feature: Authority\n");put(`${vocabulary.testDomainPath}/📜️script.ts`,"export {};\n");
  expect(new Set(fixture.authorityCases.map(row=>row.id)).size).toBe(fixture.authorityCases.length);
  const oracle=String.raw`import{mkdirSync,writeFileSync,symlinkSync,lstatSync,rmSync}from"node:fs";import{join,dirname}from"node:path";const base=process.argv[1],rows=JSON.parse(process.argv[2]),result=[];mkdirSync(base,{recursive:true});for(const row of rows){const path=join(base,row.path),target=row.authority==="root"?base:join(base,row.id+".json");mkdirSync(dirname(path),{recursive:true});if(row.authority!=="root")writeFileSync(target,"{}");symlinkSync(process.platform==="win32"&&row.authority!=="root"?dirname(target):target,path,process.platform==="win32"?"junction":row.authority==="root"?"dir":"file");result.push({id:row.id,linked:lstatSync(path).isSymbolicLink()});rmSync(path);}process.stdout.write(JSON.stringify(result));`;
  const child=spawnSync("node",["--input-type=module","--eval",oracle,join(root,"physical-oracle"),JSON.stringify(fixture.authorityCases)],{encoding:"utf8",timeout:3000});
  const nodeRows=child.status===0?JSON.parse(child.stdout):[];
  for(const row of fixture.authorityCases){
   const path=join(root,row.path),saved=row.authority==="root"?null:readFileSync(path,"utf8"),target=row.authority==="root"?root:join(root,`authority-${row.id}.json`);if(saved!==null){writeFileSync(target,saved);rmSync(path);}symlinkSync(process.platform==="win32"&&row.authority!=="root"?dirname(target):target,path,process.platform==="win32"?"junction":row.authority==="root"?"dir":"file");
   expect(child.status,row.id).toBe(0);expect(nodeRows.find((observed:{id:string;linked:boolean})=>observed.id===row.id)?.linked).toBe(true);expect(lstatSync(path).isSymbolicLink()).toBe(true);
   await expect(plugin.default.createNodesV2[1]([featurePath],{}, {workspaceRoot:row.authority==="root"?path:root})).rejects.toThrow(row.authority==="root"?/real workspace ancestry/:row.authority==="taxonomy"?/no-follow taxonomy/:/no-follow cache policy/);
   rmSync(path);if(saved!==null)writeFileSync(path,saved);
  }
  const changed=structuredClone(vocabulary);changed.pathExclusions.fixture={path:fixture.expected[0]};put(taxonomyPath,JSON.stringify(changed));expect(await plugin.default.createNodesV2[1]([featurePath],{}, {workspaceRoot:root})).toEqual([]);put(taxonomyPath,JSON.stringify(vocabulary));
  const helper=readFileSync(join(owner,"🕸️dependencies/🟨️.mjs"),"utf8");
  put("plugin/🟨️.mjs",readFileSync(join(owner,"🟨️.mjs"),"utf8"));put("plugin/🕸️dependencies/🟨️.mjs",helper);put("📚️library/🟨️.mjs",`export {cacheInternals} from ${JSON.stringify(pathToFileURL(join(repo,library,"🟨️.mjs")).href)};\n`);
  const code=`import assert from "node:assert/strict";import{readFileSync,writeFileSync}from"node:fs";import{join}from"node:path";import{pathToFileURL}from"node:url";const root=${JSON.stringify(root)},feature=${JSON.stringify(featurePath)},path=join(root,"plugin/🕸️dependencies/🟨️.mjs"),source=readFileSync(path,"utf8"),resident=(await import(pathToFileURL(join(root,"plugin/🟨️.mjs")).href)).default;assert.equal((await resident.createNodesV2[1]([feature],{}, {workspaceRoot:root})).length,1);writeFileSync(path,source+"\\nexport const invalid = ;\\n");await assert.rejects(()=>resident.createNodesV2[1]([feature],{}, {workspaceRoot:root}),/Unexpected|Expected|Parse|Syntax|stale ESM/i);writeFileSync(path,source);assert.equal((await resident.createNodesV2[1]([feature],{}, {workspaceRoot:root})).length,1);console.log("fresh resident source accepted/refused/accepted");`;
  for(const runtime of ["node",process.execPath]){const child=spawnSync(runtime,[...(runtime==="node"?["--input-type=module"]:[]),"--eval",code],{encoding:"utf8",timeout:10000});expect(child.status,child.stdout+child.stderr).toBe(0);expect(child.stdout.trim()).toBe("fresh resident source accepted/refused/accepted");}
 }finally{rmSync(root,{recursive:true,force:true});}
},30000);

test("Nx discovers the same canonical names and semantic owners as the layout vectors", async () => {
    const current = plugin.default;
    const cases = new Map<string, boolean>();
    for (const vector of layoutVectors.cases) for (const source of vector.sources) {
      if (!/^.+\/🧪️tests\/[^/]+\/🟦️\.ts$/u.test(source.path)) continue;
      if (source.path.split("/").at(-2) === vocabulary.testRunnerConfigurationCaseName) continue;
      const accepted = !vector.expected.some(finding => finding.path === source.path && ["test-case-name", "test-owner-delivery-scope", "test-layout-depth"].includes(finding.code));
      cases.set(source.path.replace(/🟦️\.ts$/u, "🥒️.feature"), accepted);
    }
    const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
    mkdirSync(output, { recursive: true });
    const root = mkdtempSync(join(output, "layout-nx-candidates-"));
    const put = (path: string, content: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); };
    const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
    try {
      put(`${library}/🔣️taxonomy.json`, JSON.stringify(vocabulary));
      put(`${library}/⚡️caching/🔣️policy.json`, readFileSync(join(repo, library, "⚡️caching/🔣️policy.json"), "utf8"));
      put(`${vocabulary.testDomainPath}/📜️script.ts`, "export {};\n");
      for (const path of cases.keys()) put(path, "Feature: Actual layout input\n");
      for (const [path, accepted] of cases) {
        const discovered = await current.createNodesV2[1]([path], {}, { workspaceRoot: root });
        expect(discovered.map(entry => entry[0]), path).toEqual(accepted ? [path] : []);
      }
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
