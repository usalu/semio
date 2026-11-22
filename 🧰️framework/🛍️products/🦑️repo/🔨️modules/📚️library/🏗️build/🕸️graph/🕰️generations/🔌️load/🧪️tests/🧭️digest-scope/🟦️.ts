import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import {implementationDigestV1} from "../../🟨️.mjs";
import {sectionalDigestV1} from "../../../🌍️environment/🟨️.mjs";
import sections from "../../../../📥️inference/♻️revision/🧬️schema/🔣️.json";

/** 🧭️ Contract: a landing in a data file of the build library that nothing of the graph reads (capacity numbers, worker and remote policies) moves neither the implementation digest of a generation nor the fact of any owner, while an edit of a section the graph reads moves both; the digest equals an independent sha256 of the canonical remainder.
 * @see ../../🟨️.mjs */
const library=fileURLToPath(new URL("../../../../../../",import.meta.url)),declared=sections.definitions.Sections.const.files as Record<string,string[]>,file="🏗️build/🔣️policy.json";
const text=readFileSync(join(library,file),"utf8"),ignore=declared[file]!;
const rows=(sha:string)=>[{kind:"digest",path:file,fact:{sha256:sha,size:1,mtimeMs:1}},{kind:"stamp",path:"inferrer:project-nodes",fact:"semio.graph-inferrer.project-nodes/1"}];
const digest=(body:string)=>implementationDigestV1(rows(sectionalDigestV1(body,ignore)));
const edit=(section:string)=>{const value=JSON.parse(text);value[section]={...(typeof value[section]==="object"?value[section]:{}),landing:Date.now()};return JSON.stringify(value);};

test("the ignored sections of the build policy include every capacity, worker and remote section",()=>{
 expect([...ignore].sort()).toEqual(["hostCapacity","remoteCache","remoteExecution","workers"]);
});

test("an edit of an ignored section keeps the digest, an edit of a section the graph reads moves it",()=>{
 const base=digest(text);
 for(const section of ignore)expect(digest(edit(section)),section).toBe(base);
 for(const section of ["graphMembers","namedInputs"])expect(digest(edit(section)),section).not.toBe(base);
});

test("the sectional digest equals an independent sha256 of the canonical remainder",()=>{
 const value=JSON.parse(text),kept=Object.fromEntries(Object.entries(value).filter(([key])=>!ignore.includes(key)));
 const canonical=(item:unknown):string=>Array.isArray(item)?"["+item.map(canonical).join(",")+"]":item!==null&&typeof item==="object"?"{"+Object.keys(item).sort().map(key=>JSON.stringify(key)+":"+canonical((item as Record<string,unknown>)[key])).join(",")+"}":JSON.stringify(item);
 expect(sectionalDigestV1(text,ignore)).toBe(createHash("sha256").update(canonical(kept)).digest("hex"));
});
