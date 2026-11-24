/** 🖨️ Contract: admission's generated task prerequisites reach an authored consumer at once, a consumer the Print catalogue synthesizes is left to project assembly (it does not exist before assembly), and any other absent consumer is still refused; a SQLite oracle decides each verdict independently.
 * @see ../../🟨️.mjs
 * @see ../../../🧩️component-targets/🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";

const{libraryBootstrap}=await import("../../../🟨️.mjs") as any;
await libraryBootstrap;
const{withOwnerGeneratedDependenciesV1}=await import("../../🟨️.mjs") as any;
const workspaceRoot=mkdtempSync(join(tmpdir(),"print-generated-"));
afterAll(()=>rmSync(workspaceRoot,{recursive:true,force:true}));
mkdirSync(join(workspaceRoot,"p/cat"),{recursive:true});
writeFileSync(join(workspaceRoot,"p/cat/catalog.json"),JSON.stringify({version:1,librarySources:["lib"],documents:fixture.catalog.map(id=>({id,collection:"templates",texPath:`docs/${id}.tex`,sources:[]}))}));

test("generated prerequisites apply to authored consumers, defer catalogue consumers and refuse every other absent consumer",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.run("create table authored(name text primary key)");db.run("create table document(id text primary key)");
  for(const name of fixture.authored)db.run("insert into authored values(?)",[name]);
  for(const id of fixture.catalog)db.run("insert into document values(?)",[id]);
  const oracle=db.query<{verdict:string},[string,number]>(`select case when exists(select 1 from authored where name=?1) then 'applied'
   when ?2 and (?1 in ('build','build-viz') or exists(select 1 from document where 'build-'||id=?1 or 'watch-'||id=?1)) then 'deferred' else 'refused' end verdict`);
  for(const row of fixture.cases){
   const expected=oracle.get(row.consumer,Number(row.printCatalog))!.verdict;
   expect(expected,row.id).toBe(row.expected);
   const json={name:"p",root:"p",...row.printCatalog?{metadata:{printCatalog:"p/cat/catalog.json"}}:{},targets:Object.fromEntries(fixture.authored.map(name=>[name,{executor:"x"}]))};
   let verdict:string;
   try{
    const result=withOwnerGeneratedDependenciesV1(json,{[row.consumer]:[row.producer]},workspaceRoot);
    verdict=result.targets[row.consumer]?.dependsOn?.includes(row.producer)?"applied":JSON.stringify(result)===JSON.stringify(json)?"deferred":"changed";
   }catch(error){verdict=String((error as Error).message).startsWith("Generated consumer task absent")?"refused":String(error);}
   expect(verdict,row.id).toBe(expected);
  }
 }finally{db.close();}
});
