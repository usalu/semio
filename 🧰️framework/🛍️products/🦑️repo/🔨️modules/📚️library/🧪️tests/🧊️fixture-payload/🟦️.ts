import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {join,resolve} from "node:path";
import {minimatch} from "minimatch";
import { taxonomyFixturePayloadPath } from "../../🧹️normalization/🧱️inventory-leaf-stamps/🟦️.ts";
import { inventoryTaxonomy } from "../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../🧹️normalization/🧠️planning/🟦️.ts";
import { parseTaxonomyPlan } from "../../🧹️normalization/📃️plan-document/🟦️.ts";
import { loadTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import input from "../\u{1f9eb}\uFE0Ffixtures/\u{1f9ca}\uFE0Ffixture-payload/\u{1f523}\uFE0F.json";
import privateInput from "./🧫️fixtures/🔣️.json";
const library=resolve(import.meta.dir,"../.."),root=resolve(library,"../../../../.."),taxonomy=loadTaxonomy();
for(const row of input.paths)test("fixture payload semantic boundary "+row.id,()=>{const actual=taxonomyFixturePayloadPath;expect(typeof actual).toBe("function");const oracle=minimatch(row.path,"**/"+taxonomy.testsDirName+"/**/"+taxonomy.testFixturesDirName+"/**/*");expect(oracle).toBe(row.opaque);expect(actual(row.path,taxonomy)).toBe(oracle);});
test("actual original inventory keeps fixture bytes physical and gives payloads no production reference authority",()=>{const path=input.observedPath,bytes=readFileSync(join(root,path)),actual=inventoryTaxonomy({repoRoot:root,scope:path,workers:1}),entry=actual.entries.find(entry=>entry.sourcePath===path)!;expect(entry.size).toBe(bytes.length);expect(entry.contentHash).toBe(new Bun.CryptoHasher("sha256").update(bytes).digest("hex"));expect(entry.referencesOut).toEqual([]);expect(entry.referencesIn).not.toContain(path);const plan=planTaxonomy(actual,{baselineCommit:execFileSync("git",["rev-parse","HEAD"],{cwd:root,encoding:"utf8"}).trim(),excludedTreeDigests:[]});expect(plan.edits.filter(edit=>edit.path===path)).toEqual([]);expect(plan.unresolved.filter(row=>row.code==="frozen-coordinate-evidence-invalid")).toEqual([]);expect(plan.regenerations).toEqual([]);expect(parseTaxonomyPlan(plan)).toEqual(plan);},30_000);

test("original host Source imports remain visible while private fixture payload references stay opaque",()=>{const scope=privateInput.modeledPath.split("/\u{1f9eb}\uFE0Ffixtures/")[0]!,actual=inventoryTaxonomy({repoRoot:root,scope,workers:1}),host=actual.entries.find(entry=>entry.sourcePath===scope+"/\u{1f7e6}\uFE0F.ts")!,data=actual.entries.find(entry=>entry.sourcePath===privateInput.modeledPath)!;expect(host.referencesOut).toContain(privateInput.modeledPath);expect(data.referencesIn).toContain(host.sourcePath);expect(data.referencesOut).toEqual([]);},30_000);
