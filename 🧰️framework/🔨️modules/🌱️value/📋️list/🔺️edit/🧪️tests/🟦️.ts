import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import Ajv2020 from "ajv/dist/2020";
import {IndexedEditPlan,IndexedOutputCursor,type IndexedEditFault} from "../🟦️.ts";

test("indexed edits preserve strict base/final coordinate admission and independent SQLite order",()=>{
 const database=new Database(":memory:");
 const validate=new Ajv2020({strict:true}).compile(schema);
 try{
  database.exec("CREATE TABLE edits(kind INTEGER,ordinal INTEGER,idx INTEGER,PRIMARY KEY(kind,ordinal))");
  const insert=database.query("INSERT INTO edits VALUES(?,?,?)"),ordered=database.query<{idx:number;ordinal:number},[number]>("SELECT idx,ordinal FROM edits WHERE kind=? ORDER BY idx,ordinal");
  for(const law of fixture.cases){
   expect(validate({base:law.base,removed:law.removed,modified:law.modified,added:law.added})).toBe(true);
   database.exec("DELETE FROM edits");
   for(const [kind,rows]of [law.removed,law.modified,law.added].entries())for(const [ordinal,index]of rows.entries())insert.run(kind,ordinal,index);
   const rows=[0,1,2].map(kind=>ordered.all(kind));
   let fault:IndexedEditFault|null=null;
   for(const [kind,entries]of rows.entries()){
    for(const [rank,row]of entries.entries()){
     const invalid=kind<2?row.idx>=law.base||rank>0&&entries[rank-1]!.idx===row.idx||kind===1&&law.removed.includes(row.idx):rank>0&&entries[rank-1]!.idx===row.idx||row.idx>law.base-law.removed.length+rank;
     if(invalid){fault=(["remove","modify","add"]as const)[kind]!;break;}
    }
    if(fault)break;
   }
   expect<string|null>(fault).toBe(law.fault);
   const actual=new IndexedEditPlan(law.base,law.removed,law.modified,law.added);
   let turns=0;while(!actual.settled){actual.step();expect(++turns).toBeLessThan(2000);}
   expect(actual.fault).toBe(fault);
   if(fault===null){
    for(const kind of [0,1,2]as const)expect(actual.entries(kind)).toEqual(rows[kind]!);
    const expected=Array.from({length:law.base},(_,index)=>`b${index}`).filter((_,index)=>!law.removed.includes(index));
    for(const row of rows[2]!)expected.splice(row.idx,0,`a${row.ordinal}`);
    expect(expected).toEqual(law.output!);
    const cursor=new IndexedOutputCursor(),output:string[]=[];let steps=0;
    for(;;){const next=cursor.next(actual);if(next.kind==="complete")break;if(next.kind==="base")output.push(`b${next.index}`);if(next.kind==="added")output.push(`a${next.ordinal}`);cursor.advance(actual);expect(++steps).toBeLessThan(law.base+law.added.length+1);}
    expect(output).toEqual(expected);
   }
  }
  console.log(`[DEBUG] ${fixture.cases.length} strict indexed admission/stream outputs agree with independent SQLite ordering and array splice semantics`);
 }finally{database.close();}
});
