import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import schema from"./🧬️.json";import fixture from"./🔣️.json";test("typed kernel output projection has one shared JSON wire contract",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const db=new Database(":memory:");for(const row of fixture.cases){expect(JSON.stringify(wire(row))).toBe(row.expected);expect((db.query("select json(?) as value").get(row.expected)as{value:string}).value).toBe(row.expected);}db.close();});
function wire(row:{kind:string,input:unknown}){if(row.kind==="turn")return{Ok:row.input};const input=row.input;if((row.kind==="numberBound"||row.kind==="rowAction")&&input&&typeof input==="object"){const value={...input}as Record<string,unknown>;const key=row.kind==="numberBound"?"exclusive":"disabled";if(value[key]===false)delete value[key];return value}return input}
test("default boolean omission agrees with independent SQLite object projection",()=>{const db=new Database(":memory:");for(const row of fixture.cases.filter(r=>r.kind==="numberBound"||r.kind==="rowAction")){const path=row.kind==="numberBound"?"$.exclusive":"$.disabled";const found=db.query("select case when json_extract(?1,?2)=0 then json_remove(?1,?2) else json(?1) end as value").get(JSON.stringify(row.input),path)as{value:string};expect(found.value).toBe(row.expected);}db.close();});
test("the Poll output decoder requires the single authored success envelope",()=>{
 const turn=fixture.cases.find(row=>row.kind==="turn")!;
 const validate=new Ajv({strict:true}).compile(schema.$defs.turnOutput);
 const db=new Database(":memory:");
 try{
  const accept=db.query("select json_type(?1, '$.Ok')='object' and (select count(*) from json_each(?1))=1 as accepted");
  for(const [value,expected] of [[JSON.parse(turn.expected),true],[turn.input,false],[{Err:turn.input},false],[{Ok:turn.input,Err:turn.input},false]] as const){
   expect(validate(value)).toBe(expected);
   expect(Boolean((accept.get(JSON.stringify(value)) as {accepted:number}).accepted)).toBe(expected);
  }
 }finally{db.close();}
});
