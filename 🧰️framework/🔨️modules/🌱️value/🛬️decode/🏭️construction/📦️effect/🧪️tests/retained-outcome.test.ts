import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{semioSchemaAjvV1}from"@semio-tech/framework/schema/oracles/validator";
import law from"./retained-outcome.json";
import schema from"./retained-outcome-schema.json";
test("resident receiving outcomes transfer only with the original funded empty destination",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.cases)expect(db.query("select ? and ? and ? and ? and ? moves").get(Number(row.ready),Number(row.empty),Number(row.work),Number(row.copy),Number(row.sameOwner))).toEqual({moves:Number(row.moves)});expect(db.query("select ?+? actualWork").get(law.semanticWork,law.transferWork)).toEqual({actualWork:2});expect(db.query("select ?+?+?+? reportedWork").get(law.semanticWork,law.readyRepeatWork,law.transferredWork,law.transferWork)).toEqual({reportedWork:2})}finally{db.close()}});

test("successful receipt acknowledgement consumes fresh metadata Work without replaying semantic currencies",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ?+?+? work,? metadata_only,?+? wrongly_consumed,? calls").get(law.semanticWork,law.transferWork,law.acknowledgement.work,Number(law.acknowledgement.metadataOnly),Number(law.acknowledgement.unreportedAccepted),Number(law.acknowledgement.errorAccepted),law.acknowledgement.receiveCallsAfterRecovery)).toEqual({work:3,metadata_only:1,wrongly_consumed:0,calls:1})}finally{db.close()}});
