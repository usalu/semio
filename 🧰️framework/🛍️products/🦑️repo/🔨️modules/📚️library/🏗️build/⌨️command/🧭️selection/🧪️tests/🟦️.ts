import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{repositoryCommandKernelV1,repositoryCommandMemoryCappedV1}from"../🟦️.ts";
import fixture from"./🔣️.json";

test("the memory cap of a command process follows the fixture table and an independent SQLite model of the declared kinds and free verbs",()=>{
 const db=new Database(":memory:");
 try{
  for(const row of fixture.cases){
   const kind=repositoryCommandKernelV1(row.args,{}).kind,verb=row.args[0]!,model=db.query("select ((?1 in ('runtime','task') and ?2 not in ('tools','reset','remote-execution','leases','doctor','capacity','fleet','scheduler','test-file','typecheck')) or ?1 in ('cargo','cache','cache-ledger')) as capped").get(kind,verb)as{capped:number};
   expect(repositoryCommandMemoryCappedV1(row.args,{}),row.args.join(" ")).toBe(row.capped);
   expect(Boolean(model.capped),row.args.join(" ")).toBe(row.capped);
  }
 }finally{db.close();}
});
