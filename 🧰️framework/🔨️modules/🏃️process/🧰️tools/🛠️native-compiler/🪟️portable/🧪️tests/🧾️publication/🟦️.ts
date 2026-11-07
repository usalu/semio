import{expect,test}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../../🧬️schema/🔣️.json" with{type:"json"};
import fixture from"../../🧫️fixtures/🧾️publication.json" with{type:"json"};
import intentFixture from"../../🧫️fixtures/🧾️intent.json" with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{portableMsvcReceiptMatchesV1,portableMsvcRecoveryV1,portableMsvcProvisionDecisionV1}from"../../🧾️publication/🟦️.ts";
import recovery from"../../🧫️fixtures/♻️recovery.json" with{type:"json"};
import provision from"../../🧫️fixtures/🚦️provision.json" with{type:"json"};

test("publication metadata matches only exact allocation, source pins and directory identity",()=>{
 const db=new Database(":memory:"),valid=new Ajv({strict:true}).compile(schema.definitions.PublicationReceipt),expected={allocation:"allocation-1",recipeSha256:"a".repeat(64),inventorySha256:"b".repeat(64),directory:{device:"1",inode:"2"}},base={version:1,owner:schema.definitions.Protocol.const.owner,catalogSha256:schema.definitions.Protocol.const.manifestSha256,...expected};try{for(const row of fixture.cases){const value:any=structuredClone(base);if(row.mutation==="allocation")value.allocation="foreign";if(row.mutation==="catalog")value.catalogSha256="c".repeat(64);if(row.mutation==="recipe")value.recipeSha256="c".repeat(64);if(row.mutation==="inventory")value.inventorySha256="c".repeat(64);if(row.mutation==="device")value.directory.device="3";if(row.mutation==="inode")value.directory.inode="3";if(row.mutation==="extra")value.extra=true;const oracle=db.query("select ?1=?2 and ?3=?4 and ?5=?6 and ?7=?8 and ?9=?10 and ?11=?12 and ?13=7 matches").get(value.allocation,expected.allocation,value.catalogSha256,base.catalogSha256,value.recipeSha256,expected.recipeSha256,value.inventorySha256,expected.inventorySha256,value.directory.device,expected.directory.device,value.directory.inode,expected.directory.inode,Object.keys(value).length) as{matches:number};expect(Boolean(oracle.matches),row.name).toBe(row.matches);expect(valid(value)&&portableMsvcReceiptMatchesV1(value,expected),row.name).toBe(row.matches);}}finally{db.close();}
});

test("pending publication intent is closed",()=>{
 const ajv=new Ajv({strict:true}).compile({...schema, $ref:"#/definitions/PublicationIntent"}),receipt={version:1,owner:schema.definitions.Protocol.const.owner,allocation:"allocation-1",catalogSha256:schema.definitions.Protocol.const.manifestSha256,recipeSha256:"a".repeat(64),inventorySha256:"b".repeat(64),directory:{device:"1",inode:"2"}};
 for(const row of intentFixture.cases){const value:any={version:1,scratch:"D:\\stage-1234567890abcdef",directory:{device:"1",inode:"3"},receipt:structuredClone(receipt)};if(row.mutation==="extra")value.extra=true;if(row.mutation==="identity")delete value.directory.inode;if(row.mutation==="receipt")value.receipt.extra=true;expect(ajv(value),row.name).toBe(row.valid);expect(validateJsonSchemaSubset(schema.definitions.PublicationIntent,value,schema).length===0,row.name).toBe(row.valid);}
});

test("interrupted publication observations select only a safe next step",()=>{
 const db=new Database(":memory:");try{for(const row of recovery.cases){const oracle=db.query("select case when ?1 and ?3 and not ?4 and not ?2 then 'resume' when ?1 and not ?3 and ?4 then 'finish' when not ?1 and not ?3 and not ?4 and not ?2 then 'create' when not ?1 and not ?3 and ?4 and ?2 then 'ready' else 'refuse' end result").get(row.intent,row.receipt,row.stage,row.published) as{result:string};expect(oracle.result).toBe(row.result);expect(portableMsvcRecoveryV1(row)).toBe(row.result);}}finally{db.close();}
});

test("receipt observation only selects the reader path",()=>{
 const db=new Database(":memory:");try{for(const row of provision.cases){const oracle=db.query("select case when ?1='ordinary' and not ?2 then 'reader' else 'exclusive' end decision").get(row.receipt,row.pending) as{decision:string};expect(oracle.decision).toBe(row.decision);expect(portableMsvcProvisionDecisionV1({receipt:row.receipt,pending:row.pending})).toBe(row.decision);}}finally{db.close();}
});
