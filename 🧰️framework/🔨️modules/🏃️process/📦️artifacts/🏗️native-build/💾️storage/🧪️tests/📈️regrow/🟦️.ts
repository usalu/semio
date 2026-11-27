/** 📈️ Tests for the journal event that grows one committed envelope after an in-place regrow of its image. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{nativeStorageEventV1,nativeStorageStateV1,type NativeStorageEventV1,type NativeStorageAllocationV1}from"../../📜️journal/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
const base:NativeStorageAllocationV1={id:"a".repeat(32),owner:"repository:graph-store",kind:"artifact-publication",bytes:100,phase:"committed"};
const rows=[
 {name:"grow",sequence:2,change:null,bytes:150,accepted:true},
 {name:"stale-sequence",sequence:1,change:null,bytes:150,accepted:false},
 {name:"foreign-owner",sequence:2,change:{owner:"other"},bytes:150,accepted:false},
 {name:"wrong-amount",sequence:2,change:{bytes:99},bytes:150,accepted:false},
 {name:"wrong-phase",sequence:2,change:{phase:"reserved"},bytes:150,accepted:false},
 {name:"shrink",sequence:2,change:null,bytes:60,accepted:false},
 {name:"unchanged",sequence:2,change:null,bytes:100,accepted:false}
]as const;
test("regrow CAS agrees with SQLite: only a committed artifact envelope of the exact owner and sequence may grow",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);const validate=ajv.getSchema(schema.$id+"#/definitions/Event")!;
 for(const row of rows){
  const state={version:1 as const,sequence:2,allocations:[base]},event={type:"regrown" as const,expectedSequence:row.sequence,allocation:{...base,...(row.change??{})},bytes:row.bytes}as NativeStorageEventV1,db=new Database(":memory:");
  try{
   db.exec("create table held(id text primary key,owner text,kind text,bytes integer,phase text);create table journal(sequence integer);insert into journal values(2)");db.query("insert into held values(?,?,?,?,?)").run(base.id,base.owner,base.kind,base.bytes,base.phase);
   expect(validate(event)).toBe(true);const old=(event as Extract<NativeStorageEventV1,{type:"regrown"}>).allocation;
   const changed=db.query("update held set bytes=? where id=? and owner=? and kind=? and bytes=? and phase=? and phase='committed' and kind='artifact-publication' and ?>bytes and (select sequence from journal)=?").run(row.bytes,old.id,old.owner,old.kind,old.bytes,old.phase,row.bytes,row.sequence).changes;
   expect(Boolean(changed),row.name).toBe(row.accepted);
   if(!row.accepted){expect(()=>nativeStorageEventV1(state,event,200),row.name).toThrow();continue;}
   const next=nativeStorageEventV1(state,event,200);expect(next.sequence).toBe(3);expect([...next.allocations]).toEqual(db.query<NativeStorageAllocationV1,[]>("select id,owner,kind,bytes,phase from held").all());
   expect(nativeStorageEventV1(nativeStorageStateV1(),{type:"checkpoint",state:next},200)).toEqual(next);
  }finally{db.close();}
 }
});
test("a regrow beyond the capacity left is refused and a rebudget still cannot grow",()=>{
 const state={version:1 as const,sequence:2,allocations:[base,{...base,id:"b".repeat(32),owner:"other",bytes:80}]};
 expect(()=>nativeStorageEventV1(state,{type:"regrown",expectedSequence:2,allocation:base,bytes:150},200)).toThrow("regrow");
 expect(nativeStorageEventV1(state,{type:"regrown",expectedSequence:2,allocation:base,bytes:120},200).allocations[0]!.bytes).toBe(120);
 expect(()=>nativeStorageEventV1({...state,allocations:[base]},{type:"rebudgeted",expectedSequence:2,allocation:base,bytes:150},200)).toThrow("rebudget");
});
