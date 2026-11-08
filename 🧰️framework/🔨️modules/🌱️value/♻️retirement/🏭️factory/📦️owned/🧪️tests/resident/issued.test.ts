import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import {expect,test} from "bun:test";
import {isDeepStrictEqual} from "node:util";
import law from "./issued.json";
import schema from "./issued.schema.json";
test("issued shared snapshot election matches independent retained value oracle",()=>{for(const[k,v]of Object.entries(law))expect(isDeepStrictEqual(schema.properties[k as keyof typeof schema.properties].const,v)).toBe(true);const rows=Array.from({length:law.aliases},(_,index)=>({remaining:law.aliases-index-1,retire:index+1===law.aliases,value:JSON.parse(JSON.stringify(law.body))}));expect(rows.map(r=>r.retire)).toEqual([law.firstAliasRetiresPayload,law.lastAliasRetiresExactPayload]);expect(rows.at(-1)?.value).toBe(law.body);});

test("captured issuer custody uses actual issued shared originals",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ? captured").get(Number(law.capturedFactory))).toEqual({captured:1})}finally{db.close()}});
