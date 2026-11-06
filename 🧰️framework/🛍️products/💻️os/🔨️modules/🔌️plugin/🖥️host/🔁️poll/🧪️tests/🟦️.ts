import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"../🔣️.json";
import schema from"../🧬️.json";
test("each Poll crossing requires both real scopes to cover its Core reserve",()=>{const db=new Database(":memory:"),valid=new Ajv({strict:true}).compile({type:"array",minItems:6,maxItems:6,items:{type:"integer",minimum:0}});try{for(const header of [144,152,144]){const copy=4400+header+1024,work=2948;for(const [row,expected]of [[[21,copy,work,21,copy,work],1],[[21,copy-1,work,21,copy,work],0],[[21,copy,work,21,copy-1,work],0],[[21,copy,work-1,21,copy,work],0],[[21,copy,work,21,copy,work-1],0]] as const){expect(valid(row)).toBe(true);expect(db.query("SELECT ?>=21 AND ?>=? AND ?>=? AND ?>=21 AND ?>=? AND ?>=? AS ok").get(row[0],row[1],copy,row[2],work,row[3],row[4],copy,row[5],work)).toEqual({ok:expected})}}}finally{db.close()}});
test("poll transport preserves typed capture and both guest output owners before callbacks and Source return",()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 db.exec("create table events(name text primary key,position integer not null)");
 const at=(name:string)=>`(select position from events where name='${name}')`;
 const has=(name:string)=>`exists(select 1 from events where name='${name}')`;
 const before=(left:string,right:string)=>`coalesce(${at(left)}<${at(right)},0)`;
 for(const row of corpus.cases){expect(Boolean(validate(row)),row.name).toBe(true);db.exec("delete from events");for(const[index,event]of row.events.entries())db.query("insert into events values(?,?)").run(event,index);
  const terms=[`${at("sourceAdmitted")}=0`,`${at("decoderReturned")}=(select max(position)from events)`,before("inputAdmitted","pollExecuted"),before("pollExecuted","serializedHandback"),before("serializedHandback","residentRetired"),before("residentRetired","decoderReturned"),`(not ${has("callbacks")} or (${before("typedCapture","callbacks")} and ${before("residentRetired","callbacks")} and not ${has("cancelled")}))`,`(not ${has("typedCapture")} or ${before("captureRetired","decoderReturned")})`,`(not ${has("decodeRefused")} or (${before("parserRetired","serializedHandback")} and ${before("failureRetired","decoderReturned")}))`];
  const result=db.query(`select (${terms.join(" and ")}) valid`).get()as{valid:number};expect(Boolean(result.valid),row.name).toBe(row.valid);
 }db.close();
});
