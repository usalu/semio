/** 🎫️ Ticket outcomes expose exact caller-owned paths and match independent SQLite construction. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
test("ticket lifecycle outcomes retain exact dynamic paths",()=>{const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");try{for(const row of fixture.cases){const path=[row.year,row.month,row.day].map(x=>String(x).padStart(2,"0")).join("/")+"/"+row.slug,actual={path,ticket:row.ticket},oracle=db.query("SELECT json_object('path',printf('%02d/%02d/%02d/%s',?,?,?,?),'ticket',json(?)) AS result").get(row.year,row.month,row.day,row.slug,JSON.stringify(row.ticket))as{result:string};expect(validate(actual)).toBe(true);expect(actual).toEqual(row.expected);expect(JSON.parse(oracle.result)).toEqual(actual);expect(validate({...actual,path:"../foreign"})).toBe(false);expect(validate({...actual,foreign:true})).toBe(false);}}finally{db.close();}});

 test("local ticket options refuse nonboolean authority flags",()=>{const validate=new Ajv({strict:true}).compile(schema.$defs.LocalManagementOptions),db=new Database(":memory:");try{for(const row of fixture.argumentCases){const own=Object.values(row.arguments).every(value=>typeof value==="boolean"),independent=db.query("SELECT NOT EXISTS(SELECT 1 FROM json_each(?) WHERE type NOT IN ('true','false')) accepted").get(JSON.stringify(row.arguments))as{accepted:number};expect(own).toBe(row.accepted);expect(validate(row.arguments)).toBe(row.accepted);expect(Boolean(independent.accepted)).toBe(row.accepted);}}finally{db.close();}});
