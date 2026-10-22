import "../../🎫️tickets/🧪️tests/🟦️.ts";
import "../📥️bindings/🧪️tests/🟦️.ts";
import "../🧭️routes/🧪️tests/🟦️.ts";
/** 📄️ Structured repository resources match the closed corpus and independent SQLite JSON output. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
const read=(p:string)=>JSON.parse(readFileSync(new URL(p,import.meta.url),"utf8"));
test("repository file and folder resources use closed JSON documents",()=>{
 const schema=read("../🧬️schema/🔣️.json"),fixture=read("🧫️fixtures/🔣️.json"),ajv=new Ajv({strict:true});ajv.addSchema(schema);
 expect(ajv.getSchema(schema.$id+"#/$defs/Fixture")!(fixture)).toBe(true);
 const validate=ajv.getSchema(schema.$id)!;
 for(const row of fixture.cases){
  expect(fixture.mimeType).toBe("application/json");expect(validate(row.document),row.name).toBe(true);
  const kind=row.uri==="repo://files"?"files":"folders",db=new Database(":memory:");
  db.run("CREATE TABLE entries(id TEXT,path TEXT,name TEXT,kind TEXT,extension TEXT)");
  for(const entry of row.document.repo[kind])db.run("INSERT INTO entries VALUES(?,?,?,?,?)",[entry.id,entry.path,entry.name,entry.kind,entry.extension??""]);
  const fields=kind==="files"?"'id',id,'path',path,'name',name,'kind',kind,'extension',extension":"'id',id,'path',path,'name',name,'kind',kind";
  const result=db.query("SELECT json_object('repo',json_object('"+kind+"',json_group_array(json_object("+fields+")))) AS document FROM entries").get()as{document:string};
  expect(JSON.parse(result.document),row.name).toEqual(row.document);
  expect(validate({...row.document,foreignRoot:"refused"})).toBe(false);db.close();
 }
});
