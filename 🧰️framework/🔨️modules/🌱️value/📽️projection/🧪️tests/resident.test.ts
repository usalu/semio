import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./resident.schema.json";
import fixture from "./resident.json";
test("snapshot reply remains resident until paid publication",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE state(phase TEXT PRIMARY KEY, child INTEGER, output INTEGER, reply INTEGER, source TEXT)");for(const state of fixture.states)db.run("INSERT INTO state VALUES(?,?,?,?,?)",[state.phase,Number(state.child),Number(state.output),Number(state.reply),state.source]);const rows=db.query<{phase:string,child:number,output:number,reply:number,source:string},[]>("SELECT * FROM state ORDER BY CASE phase WHEN \"pending\" THEN 0 WHEN \"recorded\" THEN 1 WHEN \"denied\" THEN 2 ELSE 3 END").all();expect(rows.map(row=>[row.child,row.output,row.reply])).toEqual([[1,0,0],[1,0,1],[1,0,1],[1,1,0]]);expect(new Set(rows.map(row=>row.source))).toEqual(new Set(["original"]));expect(fixture.failure.copied).toBe(7)}finally{db.close()}});
