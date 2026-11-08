import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{semioSchemaAjvV1}from"@semio-tech/framework/schema/oracles/validator";
import law from"./owner.json";
import schema from"./owner-schema.json";
test("retained outcome retirement preserves its selected result and does not treat receipt work as fuel",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("create table custody(variant text,value integer,receipt text,changed integer)");for(const variant of law.variants)db.run("insert into custody values(?,?,?,?)",variant,law.value,law.workMetadata,Number(law.deniedMutation));expect(db.query("select count(*) variants,min(value) value,min(receipt) receipt,sum(changed) changes from custody").get()).toEqual({variants:2,value:73,receipt:law.workMetadata,changes:0});expect(law.birthCopy).toBe("optionOutcomePlusResultCursor");expect(law.denials).toEqual(["work","copy","capacity"])}finally{db.close()}});
