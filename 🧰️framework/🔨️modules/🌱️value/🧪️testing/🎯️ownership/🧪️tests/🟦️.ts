import{findWorkspaceRoot}from"../../../../🏃️process/🧭️routing/🟦️.ts";
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{dirname,join,resolve,relative}from"node:path";
import{createHash}from"node:crypto";
import{Database}from"bun:sqlite";
import ts from"typescript";
import Ajv from"ajv";
import * as TOML from"@iarna/toml";
import{sha256}from"@noble/hashes/sha2.js";
import{readRustSourceClosureV1}from"../../../../🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import law from"./🔣️.json";
import schema from"../🧬️schema/🔣️.json";
const root=process.cwd(),value=resolve(import.meta.dir,"../../.."),manifestPath=join(value,law.manifest);
const canonical=(source:string)=>source.split("\n").filter(line=>!/^\s*use\b.*;\s*$/u.test(line)&&!/^\s*#\[path\s*=.*\]\s*mod\s+\w+;\s*$/u.test(line)).join("\n").replaceAll("crate::","semio_framework_value::").replaceAll("super::super::PagedBytes","semio_framework_value::paged::PagedBytes").replaceAll("pub(super)","pub(crate)").trim();
const names=(source:string)=>[...source.matchAll(/#\[test\]\s*fn\s+([A-Za-z0-9_]+)/gu)].map(match=>match[1]!);
test("every original value assertion and fixture has one bounded Cargo test owner",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const manifest=TOML.parse(readFileSync(manifestPath,"utf8")) as any,targets=manifest.test??[],caps=manifest.package.metadata.semio["rust-source-boundary"];
 expect(caps).toEqual(law.caps);expect(manifest.lib.test??true).toBe(true);expect(targets).toHaveLength(law.expected.integrationTargets);
 const db=new Database(":memory:");db.run("CREATE TABLE actual(test TEXT PRIMARY KEY,owner TEXT NOT NULL)");db.run("CREATE TABLE expected(test TEXT PRIMARY KEY,owner TEXT NOT NULL)");
 let libraryFiles=0;
 for(const owner of law.owners){
  const expectedEntry=join(value,owner.entry),entry=owner.id==="lib"?join(dirname(manifestPath),manifest.lib.path):join(dirname(manifestPath),targets.find((target:any)=>target.name===owner.id)?.path??"missing");
  expect(entry,owner.id).toBe(expectedEntry);
  const seen=new Map<string,Uint8Array>();let bytes=0;
  const files=await readRustSourceClosureV1([entry],root,law.cfg,async path=>{const prior=seen.get(path);if(prior)return prior;let data;try{data=readFileSync(path)}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error}if(seen.size>=96||bytes+data.byteLength>1048576)throw Error("Bounded test ownership observation exceeded");seen.set(path,data);bytes+=data.byteLength;return data},{manifestDirectory:dirname(manifestPath),generated:()=>undefined});
  expect(files.length,owner.id).toBeLessThanOrEqual(caps.maxSourceFiles);expect(files.reduce((n,file)=>n+file.bytes,0),owner.id).toBeLessThanOrEqual(caps.maxSourceBytes);expect(files.reduce((n,file)=>n+file.activeLines,0),owner.id).toBeLessThanOrEqual(caps.maxActiveLines);
  if(owner.id==="lib")libraryFiles=files.length;
  for(const file of files.filter(file=>file.kind==="source"))for(const name of names(readFileSync(resolve(root,file.path),"utf8")))db.run("INSERT INTO actual VALUES(?,?)",[name,owner.id]);
  for(const original of owner.sources){
   expect(files.some(file=>resolve(root,file.path)===join(value,original.path)),original.path).toBe(true);
   const source=readFileSync(join(value,original.path),"utf8"),body=canonical(source),systemHash=createHash("sha256").update(body).digest("hex"),oracleHash=Buffer.from(sha256(Buffer.from(body))).toString("hex");
   expect(names(source)).toEqual(original.tests);expect((source.match(/\bassert(?:_eq|_ne)?!\s*/gu)??[]).length).toBe(original.assertionTokens);expect(systemHash,original.path).toBe(original.bodySha256);expect(systemHash).toBe(oracleHash);expect([...source.matchAll(/(?:include_str|fixture_text|repo_text)!\("([^"]+)"\)/gu)].map(match=>match[1]!)).toEqual(original.fixtureLiterals);
   for(const name of original.tests)db.run("INSERT INTO expected VALUES(?,?)",[name,owner.id]);
  }
 }
 expect(libraryFiles).toBe(law.expected.libraryTestFiles);
 const mismatch=db.query("SELECT * FROM (SELECT * FROM actual EXCEPT SELECT * FROM expected) UNION ALL SELECT * FROM (SELECT * FROM expected EXCEPT SELECT * FROM actual)").all();expect(mismatch).toEqual([]);
 const counts=db.query("SELECT owner,COUNT(*) AS count FROM actual GROUP BY owner ORDER BY owner").all();expect(counts).toEqual(law.owners.map(owner=>({owner:owner.id,count:owner.sources.flatMap(source=>source.tests).length})).sort((a,b)=>a.owner.localeCompare(b.owner)));
 expect((db.query("SELECT COUNT(*) AS count FROM actual").get() as {count:number}).count).toBe(law.expected.individualTests);db.close();
});

test("ordinary value validation and exact UTF8 laws select their actual owners",()=>{
 const tree=ts.createSourceFile("value-script.ts",readFileSync(join(value,law.validation.source),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
 const owned=(name:string)=>{const owner=tree.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text===name);if(!owner)throw Error("Actual value test owner required: "+name);return owner;};
 const field=(owner:ts.Node,name:string)=>{let value:ts.Expression|undefined;const visit=(node:ts.Node)=>{if(ts.isPropertyAssignment(node)&&(ts.isIdentifier(node.name)||ts.isStringLiteral(node.name))&&node.name.text===name){if(value)throw Error("Ambiguous selected package contract");value=node.initializer;}ts.forEachChild(node,visit);};visit(owner);if(!value)throw Error("Actual selected "+name+" required");return value;};
 const packages=field(owned("TestScript"),"packages");expect(ts.isArrayLiteralExpression(packages)).toBe(true);if(!ts.isArrayLiteralExpression(packages))throw Error("Literal selected test packages required");expect(packages.elements.map(item=>{if(!ts.isStringLiteral(item))throw Error("Static package identity required");return item.text;})).toEqual(law.validation.packages);
 expect(JSON.parse(field(owned("RetainedUtf8CheckScript"),"groups").getText(tree))).toEqual([law.validation.utf8Group]);
});

test("the unchanged sort fixture resolves from facade and companion manifest ownership",()=>{
 for(const manifest of law.fixtureCoordinate.manifests){const directory=dirname(join(value,manifest)),actual=join(findWorkspaceRoot(directory),law.fixtureCoordinate.literal),expected=join(value,law.fixtureCoordinate.path);expect(actual).toBe(expected);const bytes=readFileSync(actual);expect(createHash("sha256").update(bytes).digest("hex")).toBe(law.fixtureCoordinate.sha256);expect(Buffer.from(sha256(bytes)).toString("hex")).toBe(law.fixtureCoordinate.sha256);}
});
