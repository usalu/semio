import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {join,posix,win32} from "node:path";
import {minimatch} from "minimatch";
import ts from "typescript";
import * as normalization from "../../🧹️normalization/🟦️.ts";
import cases from "../🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json";

test("sentinel examples have no production asset or runtime authority reader",()=>{
 expect(existsSync(join(import.meta.dir,"../../🖼️assets/🚨️transaction-sentinel-cases/🔣️.json"))).toBe(false);
 const source=readFileSync(join(import.meta.dir,"../../🧹️normalization/🟦️.ts"),"utf8"),ast=ts.createSourceFile("source.ts",source,ts.ScriptTarget.Latest,true);
 const forbidden=new Set(["serializedSentinelCases","planSerializedEvidenceRemovals","TRANSACTION_SENTINEL_CASES_CATALOG_PATH"]),found:string[]=[];
 function visit(node:ts.Node){if(ts.isIdentifier(node)&&forbidden.has(node.text))found.push(node.text);ts.forEachChild(node,visit);}visit(ast);expect(found).toEqual([]);
 expect(source.includes('kind: "serialized-path-sentinel"')).toBe(false);
 console.log("[DEBUG] Sentinel examples are plain test inputs; serialized trial authority is absent");
});
for(const row of cases.virtualPathPolicyCases)test("platform path rule "+row.id,()=>{
 const segments=row.inputPath.replaceAll("\\","/").split("/"),reserved=segments.some(segment=>minimatch(segment.toUpperCase(),"{CON,PRN,AUX,NUL,COM[1-9],LPT[1-9]}{,.*}")),trailing=segments.some(segment=>segment.endsWith(".")||segment.endsWith(" "));
 const independent=[...(reserved?["windows-reserved-name"]:[]),...(trailing?["trailing-dot-or-space"]:[])];expect(independent).toContain(row.expectedViolationCode);expect(normalization.taxonomyPlatformPathViolationCodes(row.inputPath)).toEqual(independent);expect(row.physicalSourcePath).toBeNull();console.log("[DEBUG] Platform sentinel "+row.id+" agrees with minimatch");
});
for(const row of cases.symlinkFlavorCases)test("repository local symlink rule "+row.id,()=>{
 const api=/^[A-Za-z]:|^\\\\/u.test(row.repositoryRoot)?win32:posix,relative=api.relative(row.repositoryRoot,row.target),independent=!api.isAbsolute(relative)&&relative!==".."&&!relative.startsWith(".."+api.sep);
 expect(independent).toBe(row.owned);expect(normalization.repositoryLocalSymlinkTargetPath(row.repositoryRoot,row.target)!==null).toBe(independent);console.log("[DEBUG] Symlink sentinel "+row.id+" agrees with Node path");
});
