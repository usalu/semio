import{test,expect}from"bun:test";
import{mkdtempSync,mkdirSync,writeFileSync,rmSync}from"node:fs";
import{join,dirname,resolve}from"node:path";
import{Database}from"bun:sqlite";
import corpus from"./🔣️.json";
const inference=await import("../../🟨️.mjs"),scripts=await import("../../🧭️script-inputs/🟨️.mjs");await inference.libraryBootstrap;
test("resource overlays preserve closure answers without duplicating module memo observations",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned test artifacts required");mkdirSync(artifacts,{recursive:true});const root=mkdtempSync(join(artifacts,"mm-")),cache=inference.createScriptInputCache(),db=new Database(":memory:");
 try{
  for(const[file,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,file)),{recursive:true});writeFileSync(join(root,file),text);}
  db.run("CREATE TABLE edge(parent TEXT,child TEXT)");db.run("INSERT INTO edge VALUES(?,?)",["entry.ts","target.ts"]);db.run("INSERT INTO edge VALUES(?,?)",["target.ts","nested.ts"]);db.run("CREATE TABLE opaque(path TEXT)");
  for(const row of corpus.cases){
   db.run("DELETE FROM opaque");for(const file of row.resources)db.run("INSERT INTO opaque VALUES(?)",[file]);
   const oracle=db.query("WITH RECURSIVE reach(path) AS (VALUES (\'entry.ts\') UNION SELECT edge.child FROM edge JOIN reach ON edge.parent=reach.path WHERE NOT EXISTS(SELECT 1 FROM opaque WHERE opaque.path=reach.path)) SELECT path FROM reach ORDER BY path").all().map((value:any)=>value.path);
   expect(oracle,row.id+" SQLite").toEqual(row.expected);
   const closure=await scripts.relativeScriptInputs([join(root,"entry.ts")],root,cache as never,false,row.resources.map(file=>resolve(root,file)));
   expect(closure.map((file:string)=>file.replace("{workspaceRoot}/","")).sort(),row.id).toEqual(row.expected);
   expect(cache.files.size,row.id+" memo variants").toBe(row.memoFiles);
  }
 }finally{db.close();rmSync(root,{recursive:true,force:true});}
},60000);
