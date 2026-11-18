import {expect,test} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {join,relative,sep} from "node:path";
import {fileURLToPath} from "node:url";

const framework=fileURLToPath(new URL("../../../../../../../../../",import.meta.url)),roots=["🛍️products/🦑️repo","🔨️modules"].map(name=>join(framework,name));
const skipped=new Set(["node_modules",".git",".cache","target","dist","🧪️tests","🧪️testing"]),extensions=[".ts",".mts",".mjs",".cjs",".js"];
const interfaceFile="🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs";

const left:Readonly<Record<string,string>>={
 "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🟨️.mjs":"the one request beside the manifest that pins the version: a captured implementation admits only declared literal requests, so this site is the installed package at the pinned version in every process shape; the interface checks version and API of the answer",
 "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/🧩️units/🔌️repository/🟦️.ts":"typecheck unit of one project, anchored at that project's directory: runs the compiler of that project, which a workspace member may declare in another version than the tooling",
 "🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🟦️.ts":"spawns the tsc binary of the workspace it builds, anchored at that workspace directory (two sites)",
 "🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🧬️declarations/🟦️.ts":"emits declarations with the compiler of the workspace it builds, anchored at that workspace directory (a fixture workspace carries its own)",
 "🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🧟️generated-corruption/🟦️.ts":"anchored at the explicit checkout root and its direct dependency path, not at the module location; the readiness stage dependencies checks that install",
 "🔨️modules/🏃️process/🛠️repository-tooling/🟦️.ts":"resolves the tsc binary from an explicit directory given by the caller, in a framework module that cannot import the repo product",
 "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🔌️providers/🔷️typescript/🟦️.ts":"spawns the tsc binary of the project it builds, not the tooling compiler",
 "🔨️modules/🏃️process/🏗️build/🟦️typescript/🟦️.ts":"spawns the tsc binary of the project it builds and records that project's version",
 "🛍️products/🦑️repo/🔨️modules/🧪️test/🗺️test-layout/🟦️.ts":"static import of a test-layout scan in the test module, which cannot import the repo library (types-heavy, 115 uses)",
 "🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📄️retirement-file/🧪️.ts":"test support of a framework module, which cannot import the repo product"
};
const bare=[
 /(?<!\bimport)\(\s*["'`]typescript(?:\/[^"'`]*)?["'`]\s*[,)]/u,
 /^\s*import\s+(?!type\b)[^;'"`]*\bfrom\s*["'`]typescript["'`]/mu,
 /^\s*import\s*["'`]typescript["'`]/mu,
 /\bawait\s+import\(\s*["'`]typescript["'`]/u,
 /\bimport\(\s*["'`]typescript["'`]\s*\)\s*\.(?:then|catch)\b/u
];
const loadsByBareName=(text:string):boolean=>text.includes("typescript")&&bare.some(pattern=>pattern.test(text.replace(/\btypeof\s+import\(\s*["'`]typescript["'`]\s*\)/gu,"")));
const walk=(directory:string,found:string[]):void=>{
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  if(skipped.has(entry.name))continue;
  const path=join(directory,entry.name);
  if(entry.isDirectory())walk(path,found);
  else if(entry.isFile()&&extensions.some(extension=>entry.name.endsWith(extension))&&loadsByBareName(readFileSync(path,"utf8")))found.push(relative(framework,path).split(sep).join("/"));
 }
};

test("no tooling file outside the interface loads the TypeScript compiler by its bare name",()=>{
 const found:string[]=[];for(const root of roots)walk(root,found);
 expect(found.filter(path=>left[path]===undefined&&path!==interfaceFile).sort()).toEqual([]);
},300000);

test("every file left on purpose still loads by bare name, so the list never goes stale, and each states why",()=>{
 const found:string[]=[];for(const root of roots)walk(root,found);
 expect(Object.keys(left).filter(path=>!found.includes(path)).sort()).toEqual([]);
 for(const reason of Object.values(left))expect(reason.length).toBeGreaterThan(20);
},300000);

test("the matcher sees the loaders it must and ignores types",()=>{
 for(const text of ['require("typescript")','createRequire(import.meta.url)("typescript")','own("typescript")','require.resolve("typescript/bin/tsc")','import ts from "typescript";','import * as ts from "typescript"','import "typescript";','const t=await import("typescript");','import("typescript").then(x=>x)'])expect(loadsByBareName(text)).toBe(true);
 for(const text of ['import type ts from "typescript";','import type * as ts from "typescript";','const t=0 as typeof import("typescript");','let n:import("typescript").Node;','const language="typescript";','f("typescript-compiler")'])expect(loadsByBareName(text)).toBe(false);
});
