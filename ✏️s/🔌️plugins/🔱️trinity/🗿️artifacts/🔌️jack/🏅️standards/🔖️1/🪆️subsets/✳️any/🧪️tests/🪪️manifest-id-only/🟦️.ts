/** 🪪️ Committed Jack snapshot and diff fixtures name their manifest by id only, checked by independent Ajv against the actual JSON schemas. */
import {expect,test} from "bun:test";
import {readFileSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import {artifactReferenceSchema} from "@semio-tech/framework/schema/artifact-reference";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import {readFrameworkSourceJson} from "@semio-tech/framework/test/source-text";
import {parseJackDiff} from "../../🧬️schema/🔺️diff/🟦️.ts";
import {parseJackJsonValue} from "../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts";
const here=(relative:string)=>new URL(relative,import.meta.url);
const read=(url:URL)=>JSON.parse(readFileSync(url,"utf8"));
const REGISTERED=["nakagin","test-retained","mini","gizmo"];
const ajv=semioSchemaAjvV1({allErrors:true}).addSchema(artifactReferenceSchema);
for(const path of["🔨️modules/🚪️io/🧬️schema/🔣️.json","🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔣️.json","🔨️modules/🌱️value/🏷️type/🧬️schema/🔣️.json","🔨️modules/🌱️value/🧬️schema/🔣️.json"])ajv.addSchema(readFrameworkSourceJson(path));
ajv.addSchema(read(here("../../🧬️schema/🔣️.json")));
const validateSnapshot=ajv.compile(read(here("../../🧬️schema/📸️snapshot/🔣️.json")));
const validateDiff=ajv.compile(read(here("../../🧬️schema/🔺️diff/🔣️.json")));
const fixtures=here("../🧫️fixtures/🧬️mutations/");
function collect(directory:string,side:string):string[]{const found:string[]=[];for(const entry of readdirSync(directory)){const path=join(directory,entry);if(statSync(path).isDirectory())found.push(...collect(path,side));else if(entry==="🔣️.json"&&path.includes(side))found.push(path);}return found;}
const root=decodeURIComponent(fixtures.pathname).replace(/^\/([A-Za-z]:)/,"$1");
const snapshots=collect(root,"📸️snapshot");
const diffs=collect(root,"🔺️diff");
test("Jack committed fixtures exist for snapshots and diffs",()=>{expect(snapshots.length).toBeGreaterThan(0);expect(diffs.length).toBeGreaterThan(0);});
for(const path of snapshots)test("Jack snapshot fixture carries only a registered manifestId "+path.slice(root.length),()=>{const value=read(new URL("file:///"+path.replaceAll("\\","/")));expect(validateSnapshot(value),JSON.stringify(validateSnapshot.errors)).toBe(true);expect("manifest"in value).toBe(false);expect(REGISTERED).toContain(value.manifestId);expect(parseJackJsonValue(value).manifestId).toBe(value.manifestId);});
for(const path of diffs)test("Jack diff fixture carries a nullable manifestId and no manifest "+path.slice(root.length),()=>{const value=read(new URL("file:///"+path.replaceAll("\\","/")));expect(validateDiff(value),JSON.stringify(validateDiff.errors)).toBe(true);expect("manifest"in value).toBe(false);expect(Object.hasOwn(value,"manifestId")).toBe(true);expect(parseJackDiff(value).manifestId).toBe(value.manifestId);});
test("Jack snapshot schema refuses an inline manifest and a missing manifestId",()=>{const value=read(new URL("file:///"+snapshots[0]!.replaceAll("\\","/")));expect(validateSnapshot({...value,manifest:{nodeKinds:[],edgeKinds:[],portKinds:[]}})).toBe(false);const{manifestId,...rest}=value;expect(manifestId).toBeString();expect(validateSnapshot(rest)).toBe(false);});
