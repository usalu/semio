import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
import fixture from"../🧫️fixtures/🔣️.json" with{type:"json"};
import{nativeWindowsAllocationUnitFromV1}from"../🟦️.ts";

test("allocation-unit facts reject invalid or unbounded volume reports",()=>{
 const valid=new Ajv({strict:true}).compile(schema.definitions.Case),db=new Database(":memory:"),policy=schema.definitions.Policy.const;try{for(const row of fixture.cases){expect(valid(row)).toBe(true);const actual=nativeWindowsAllocationUnitFromV1(row.sectors,row.sectorBytes),oracle=db.query("select ?1>0 and ?2>0 and (?1 & (?1-1))=0 and (?2 & (?2-1))=0 and ?1<=?3 and ?2<=?4 and ?1*?2<=?5 accepted").get(row.sectors,row.sectorBytes,policy.maximumSectorsPerCluster,policy.maximumSectorBytes,policy.maximumAllocationBytes) as{accepted:number};expect(Boolean(oracle.accepted),row.name).toBe(row.accepted);expect(actual!==null,row.name).toBe(row.accepted);if(actual!==null)expect(actual).toBe(row.sectors*row.sectorBytes);}}finally{db.close();}
});
