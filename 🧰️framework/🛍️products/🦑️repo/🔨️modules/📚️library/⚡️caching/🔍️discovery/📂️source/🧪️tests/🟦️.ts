import{Database}from"bun:sqlite";
import{CACHE_POLICY,sourceFiles}from"../🟦️.ts";
import opacityFixture from"./🧫️fixtures/cases.json";
import{test,expect}from"bun:test";
import{readFileSync,writeFileSync,mkdirSync,existsSync}from"node:fs";
import{join,resolve,dirname,relative}from"node:path";
import{createHash}from"node:crypto";
import{pathToFileURL}from"node:url";
import Ajv from"ajv";
import{filter,sortBy}from"lodash";
import fixture from"./🔣️.json";
import schema from"./📋️schema.json";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import{captureNxAuthorizerSourcesV1}from"../../../🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{snapshotNxAuthorizerSourcesV1,compileNxAuthorizerSnapshotV1}from"../../../🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts";
declareBuildTestEnvelopeV1({owner:"source-discovery-bundle",maximumElapsedMilliseconds:120000,capabilities:{artifacts:true}});
test("relocated captured discovery preserves policy identity and source selection",()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const root=process.cwd(),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔍️discovery/📂️source/🟦️.ts",control=createIssuedResourceControlV1(original,"fixture:discovery-bundle",120000),captured=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal),snapshot=snapshotNxAuthorizerSourcesV1(captured),compiled=await compileNxAuthorizerSnapshotV1(root,snapshot,control,"runtime","module"),output=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"discovery-relocated","module.mjs");
 mkdirSync(dirname(output),{recursive:true});writeFileSync(output,compiled.bytes);expect(existsSync(join(dirname(output),"../../🔣️policy.json"))).toBe(false);
 const actual=await import(pathToFileURL(output).href),policyPath=resolve(import.meta.dir,"../../../🔣️policy.json"),policy=JSON.parse(readFileSync(policyPath,"utf8"));
 const policyRelative=relative(root,policyPath).replaceAll("\\","/");expect(captured.manifest.sources.find(row=>row.path===policyRelative)?.sha256).toBe(createHash("sha256").update(readFileSync(policyPath)).digest("hex"));expect(snapshot.files.some(row=>row.path===policyRelative)).toBe(true);
 expect(actual.CACHE_POLICY_PATH).toBe(policyPath);expect(actual.CACHE_POLICY).toEqual(policy);
 const operations={readDirectory:(directory:string)=>fixture.entries.filter(row=>dirname(row.path)===((directory.slice(root.length+1).replaceAll("\\","/"))||".")).map(row=>({name:row.path.split("/").at(-1)!,isDirectory:()=>row.kind==="directory",isFile:()=>row.kind==="file",isSymbolicLink:()=>row.kind==="symlink"}))},selected=actual.sourceFiles(root,operations),oracle=sortBy(filter(fixture.entries,row=>row.kind==="file"&&!row.path.split("/").some(part=>policy.generatedDirectories.includes(part))).map(row=>row.path));
 expect(selected).toEqual(fixture.expected);expect(selected).toEqual(oracle);
}),120000);

test("source coordinates exclude generated descendants while preserving clone ancestors",()=>{
 const database=new Database(":memory:");
 try{for(const row of opacityFixture.coordinateCases){
  const relative=row.path.replaceAll("\\","/"),parts=relative.split("/"),root="/source/"+row.root.replaceAll("\\","/");
  const oracle=database.query("SELECT count(*) count FROM json_each(?1) p WHERE p.value IN (SELECT value FROM json_each(?2)) OR (length(p.value)=10 AND substr(p.value,1,2) IN ('.p','.d') AND substr(p.value,3) NOT GLOB '*[^0-9a-f]*')").get(JSON.stringify(parts),JSON.stringify(CACHE_POLICY.generatedDirectories))as{count:number};
  expect(oracle.count>0).toBe(row.opaque);
  const directory=(name:string)=>({name,isDirectory:()=>true,isFile:()=>false,isSymbolicLink:()=>false}),file=(name:string)=>({name,isDirectory:()=>false,isFile:()=>true,isSymbolicLink:()=>false});
  const files=sourceFiles(root,{readDirectory:path=>{path=path.replaceAll("\\","/");const depth=path===root?0:path.slice(root.length+1).split("/").length;return depth===parts.length-1?[file(parts[depth]!)]:[directory(parts[depth]!)];}});
  expect(files).toEqual(row.opaque?[]:[relative]);
 }}finally{database.close();}
});
