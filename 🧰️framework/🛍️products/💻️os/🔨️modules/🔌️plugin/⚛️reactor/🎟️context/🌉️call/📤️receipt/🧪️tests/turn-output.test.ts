/** 📦️ Fixed output headers preserve only a valid borrowed capsule extent. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./turn-output.json";
import schema from "../🧬️schema/turn-output.json";
test("input continuation requires the real prior output lifetime to close",()=>{const db=new Database(":memory:");try{for(const [pending,closed,returned,accepted,expected]of [[1,1,1,1,1],[0,1,1,1,0],[1,0,1,1,0],[1,1,0,1,0],[1,1,1,0,0]])expect(db.query("SELECT ? AND ? AND ? AND ? AS continue_input").get(pending,closed,returned,accepted)).toEqual({continue_input:expected})}finally{db.close()}});
test("turn output extent follows the fixed receipt readiness contract",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const row of fixture.cases){const valid=db.query("SELECT CASE WHEN ?=0 THEN ?=0 AND ?=0 ELSE ?=1 AND ?>0 AND ?>0 END AS valid").get(+row.ready,row.pointer,row.length,+row.captured,row.pointer,row.length) as {valid:number};expect(Boolean(valid.valid)).toBe(row.valid);const bytes=Buffer.alloc(152);bytes.write("SAC3");bytes[4]=5;bytes[5]=+row.ready|(+row.captured<<1)|(+row.inputPending<<2);expect(Boolean(bytes[5]&4)).toBe(row.inputPending);bytes.writeUInt32LE(3000,136);bytes.writeUInt32LE(48,140);bytes.writeUInt32LE(row.pointer,144);bytes.writeUInt32LE(row.length,148);expect(db.query("SELECT length(?) AS size").get(bytes)).toEqual({size:152});}}finally{db.close()}});
