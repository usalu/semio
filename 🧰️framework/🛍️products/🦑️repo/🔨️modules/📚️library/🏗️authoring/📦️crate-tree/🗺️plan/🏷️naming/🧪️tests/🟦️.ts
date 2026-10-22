/** 🏷️ Contract: crate naming derives the mutation family of a slice path, the `mut-…` crate label of a group, emoji-free facet slugs, Rust identifiers of crate names and the leading facet emoji; the corpus is language-agnostic and `🐍️oracle.py` re-derives every name with its own emoji ranges. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {crateIdent,crateIdentPlaceholder,facetEmoji,facetSlug,isMutationCrateLabel,mutationCrateLabel,mutationFamilyOf} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.family)test("mutation family of "+JSON.stringify(row.eff),()=>{expect(mutationFamilyOf(row.eff)).toBe(row.expected);});
for(const row of corpus.label)test("crate label of "+JSON.stringify(row.families),()=>{expect(mutationCrateLabel(row.families)).toBe(row.expected);});
for(const row of corpus.mutationLabels)test("mutation label "+JSON.stringify(row.label),()=>{expect(isMutationCrateLabel(row.label)).toBe(row.expected);});
for(const row of corpus.slug)test("facet slug of "+JSON.stringify(row.name),()=>{expect(facetSlug(row.name)).toBe(row.expected);});
for(const row of corpus.ident)test("crate identifier of "+JSON.stringify(row.name),()=>{expect(crateIdent(row.name)).toBe(row.expected);});
for(const row of corpus.emoji)test("facet emoji of "+JSON.stringify(row.name),()=>{expect(facetEmoji(row.name)).toBe(row.expected);});
for(const row of corpus.placeholder)test("placeholder "+row.index,()=>{expect(crateIdentPlaceholder(row.index)).toBe(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
