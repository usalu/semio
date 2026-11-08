import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./custody.json";
import schema from "./custody-schema.json";
test("typed record custody prices original fields, destination and frontier headers",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.layouts)expect(db.query("select ?+?+? placement,?+?+1+?+1 publication,?+1 releaseCopy").get(row.field,row.fieldOption,row.word,row.value,row.output,row.word,row.word)).toEqual({placement:row.placement,publication:row.publication,releaseCopy:row.releaseCopy});expect(db.query("select 1+1+2+1+1 work").get()).toEqual({work:law.flow.work});expect(law.denied).toEqual({work:0,retainsOriginal:true})}finally{db.close()}});

test("caller initialization flags remain in the original paid field publication",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.layouts)expect(db.query("select ?+? bytes,1+1+1+1+1 work").get(row.placement,law.callerFlag.copyBytes)).toEqual({bytes:row.placement+1,work:law.callerFlag.flowWork});expect(law.callerFlag.deniedChanged).toBe(false)}finally{db.close()}});
