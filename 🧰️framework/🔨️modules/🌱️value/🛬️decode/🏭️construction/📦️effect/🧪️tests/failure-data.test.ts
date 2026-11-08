import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{semioSchemaAjvV1}from"@semio-tech/framework/schema/oracles/validator";
import law from"./failure-data.json";
import schema from"./failure-data-schema.json";
test("full native failure data stays independent of its metadata acknowledgement",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("create table custody(domain text,message text,semantic_refusal integer,metadata_refusal integer)");for(const domain of law.domains)db.run("insert into custody values(?,?,1,1)",domain,law.message);expect(db.query("select count(*) domains,min(message) original,sum(semantic_refusal) semantic,sum(metadata_refusal) metadata from custody").get()).toEqual({domains:3,original:law.message,semantic:3,metadata:3});expect(db.query("select ?+?+? work,? denied").get(law.work.semantic,law.work.wholeOutcome,law.work.failureData,law.work.denied)).toEqual({work:3,denied:0});expect(Object.values(law.independent).every(Boolean)).toBe(true)}finally{db.close()}});
