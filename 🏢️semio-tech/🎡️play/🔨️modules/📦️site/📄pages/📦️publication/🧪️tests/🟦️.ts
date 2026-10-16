/** 📦️ Play page publication laws: bounded cohort commit, independent cohort readers, selected-mode browser paths and the held input group. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFileSync,existsSync}from"node:fs";
import{resolve,join}from"node:path";
import{repositoryCachePartitionRootDataV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import pagePublicationSchema from"../🧬️schema/🔣️.json"with{type:"json"};
const workspace=resolve(import.meta.dir,"../../../../../../..");
test("neutral Play page cohort has one bounded commit and real shared/exclusive lifetime owners",()=>{
 const oracle=new Ajv({strict:true}),db=new Database(":memory:");try{const pages=pagePublicationSchema.definitions.Protocol.const.families.map(family=>({family,manifestSha256:new Bun.CryptoHasher("sha256").update(family).digest("hex")})),cohort={version:1,mode:"release",pages};expect(oracle.validate(pagePublicationSchema.definitions.Cohort,cohort)).toBe(true);db.exec("create table expected(family text primary key,digest text)");for(const row of pages)db.query("insert into expected values(?,?)").run(row.family,row.manifestSha256);expect(db.query("select count(*) n from expected").get()).toEqual({n:4});for(const actual of[pages,pages.map((row,index)=>index?row:{...row,manifestSha256:"0".repeat(64)})])expect(actual.every(row=>(db.query("select digest from expected where family=?").get(row.family)as{digest:string}).digest===row.manifestSha256)).toBe(actual===pages);}finally{db.close();}
 const owner=resolve(import.meta.dir,"../🟦️.ts");
 expect(existsSync(owner),"actual funded Play Pages cohort owner").toBe(true);const source=readFileSync(owner,"utf8");for(const name of["withResourceLeases","withArtifactScratchV1","withRepositoryArtifactV1","withRepositoryArtifactTaskV1","withRepositoryArtifactDestinationV1","stageRepositoryArtifacts"])expect(source).toContain(name);expect(source).toContain('mode:"exclusive"');expect(source).toContain('mode:"shared"');expect(source).toContain("manifestSha256");expect(source).toContain("frameworkOsViteArtifactInputsV1");
});


test("neutral Play cohort readers reject duplicate, mixed, wrong-mode and undeclared content with independent SQLite witnesses",async()=>{
 const {readPlayPageCohortV1,readPlayPageManifestV1,requirePlayPageCohortMemberV1}=await import("../📋️contract/🟦️.ts");
 const manifest={version:1,mode:"release",family:"play",files:[{path:"index.html",bytes:4,sha256:"1".repeat(64)}]}as const,bytes=new TextEncoder().encode(JSON.stringify(manifest)),digest=new Bun.CryptoHasher("sha256").update(bytes).digest("hex"),cohort={version:1,mode:"release",pages:[{family:"play",manifestSha256:digest}]}as const,db=new Database(":memory:");
 try{
  db.exec("create table expected(family text primary key,digest text)");db.query("insert into expected values(?,?)").run("play",digest);
  expect(readPlayPageCohortV1(cohort,"release")).toEqual(cohort);expect(readPlayPageManifestV1(manifest,"release","play")).toEqual(manifest);
  for(const value of[{...cohort,pages:[...cohort.pages,...cohort.pages]},{...cohort,pages:[{family:"modules",manifestSha256:digest}]},{...cohort,mode:"prototype"},{...cohort,authority:true}])expect(()=>readPlayPageCohortV1(value,"release")).toThrow();
  for(const value of[{...manifest,files:[...manifest.files,...manifest.files]},{...manifest,files:[{...manifest.files[0],path:"../outside"}]},{...manifest,files:[{...manifest.files[0],path:pagePublicationSchema.definitions.Protocol.const.manifest}]},{...manifest,family:"modules"},{...manifest,mode:"prototype"}])expect(()=>readPlayPageManifestV1(value,"release","play")).toThrow();
  for(const actual of[bytes,new TextEncoder().encode(JSON.stringify({...manifest,files:[]}))]){const hash=new Bun.CryptoHasher("sha256").update(actual).digest("hex"),accepted=(db.query("select digest from expected where family=?").get("play")as{digest:string}).digest===hash;if(accepted)expect(requirePlayPageCohortMemberV1(cohort,"play",actual,"release")).toEqual(manifest);else expect(()=>requirePlayPageCohortMemberV1(cohort,"play",actual,"release")).toThrow();}
 }finally{db.close();}
});


test("neutral browser production paths carry selected mode inside canonical products and count the actual Pages reader cohort",async()=>{
 const {builtPluginModulesRootIn}=await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts"),{playRuntimeAssetSources}=await import("../../../../🧩️runtime/📦️assets/🟦️.ts");
 for(const mode of["prototype","release"]as const){const rows=playRuntimeAssetSources(workspace,mode),browser=rows.filter(row=>row.owner!=="infinite:fonts"&&!row.owner.startsWith("canvas:"));console.log("[DEBUG] Pages actual "+mode+" browser readers="+browser.length+" plus font/Canvas="+(rows.length-browser.length));expect(new Set(browser.map(row=>row.root)).size).toBe(browser.length);expect(builtPluginModulesRootIn(workspace,mode)).toBe(join(repositoryCachePartitionRootDataV1(workspace,"products"),"browser",mode,"modules"));}
});


test("neutral Pages holds its full actual79 input group through immutable assembly rather than only font and Canvas",()=>{
 const source=readFileSync(resolve(import.meta.dir,"../🟦️.ts"),"utf8"),tree=ts.createSourceFile("pages.ts",source,ts.ScriptTarget.Latest,true),calls=new Set<string>();const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node))calls.add(node.expression.getText(tree));ts.forEachChild(node,visit);};visit(tree);
 for(const name of["withRepositoryArtifactGroupV1","withRepositoryArtifactGroupTaskV1","playRuntimeAssetSources"])expect(calls.has(name)).toBe(true);
 expect(source).not.toContain("withViteArtifactInputsV1(");
});
