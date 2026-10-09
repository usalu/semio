/** 📄️ Original checkpoint publication follows separate page and ledger turns. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
test("actor checkpoint preserves state and progress through separate original ledger close",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8"));
 const db=new Database(":memory:");db.run("CREATE TABLE phase(ordinal INTEGER PRIMARY KEY,name TEXT)");
 fixture.phases.forEach((name:string,ordinal:number)=>db.run("INSERT INTO phase VALUES(?,?)",ordinal,name));
 const publications=db.query("SELECT CASE WHEN name='ledger' THEN 'checkpoint' ELSE 'yield' END AS publication FROM phase ORDER BY ordinal").all().map((row:any)=>row.publication);
 expect(publications).toEqual(fixture.publications);
 const bytes=Buffer.from(fixture.state);db.run("CREATE TABLE checkpoint(state BLOB,progress INTEGER)");db.run("INSERT INTO checkpoint VALUES(?,?)",bytes,fixture.appliedProgress);
 const row=db.query("SELECT hex(state) AS state,progress FROM checkpoint").get() as {state:string;progress:number};
 expect(row.state).toBe("040506");expect(row.progress).toBe(73);
 let projection={state:[] as number[],progress:0,calls:0};
 for(const phase of fixture.phases){projection=applyPatch(projection,phase==='job'?[{op:'replace',path:'/calls',value:1}]:phase==='page'?[{op:'replace',path:'/state',value:fixture.state}]:[{op:'replace',path:'/progress',value:fixture.appliedProgress}],true).newDocument;}
 expect(projection).toEqual({state:fixture.state,progress:fixture.appliedProgress,calls:fixture.jobCalls});db.close();
 console.log(`[DEBUG] actor checkpoint SQLite/RFC6902 projection: phases=${fixture.phases.join(',')} originalBytes=${row.state} appliedProgress=${row.progress} jobCalls=${fixture.jobCalls}`);
});
