import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{semioSchemaAjvV1}from"@semio-tech/framework/schema/oracles/validator";
import fixture from"./reply.json";
import schema from"./schema.json";
test("original allocation cancellation recovers recorded replies without an invented physical post",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,fixture)).toBe(true);const db=new Database(":memory:");try{db.run("create table events(event text)");for(const event of fixture.events)db.run("insert into events values(?)",[event]);const row=db.query("select sum(event='quote') quotes,sum(event='cancel') cancels,sum(event='producer-recover') producerRecoveries,sum(event='post') posts from events").get()as{quotes:number,cancels:number,producerRecoveries:number,posts:number};expect(row.quotes).toBe(fixture.quotes);expect(row.cancels).toBe(fixture.cancels);expect(row.producerRecoveries).toBe(fixture.producerRecoveries);expect(row.posts).toBe(fixture.admittedBytes);expect(fixture.actual).toBeNull();expect(semioSchemaAjvV1({strict:true}).validate(schema,{...fixture,actual:0})).toBe(false);}finally{db.close()}});
