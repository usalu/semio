import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework/io/sqlite-snapshot";
import {ifc2x3SnapshotToSqliteDatabase as project,ifc2x3SnapshotFromSqliteDatabase as restore} from "../🟦️.ts";
import type {Ifc2x3Snapshot,Part21Value} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
function fixture(values:Part21Value[]):Ifc2x3Snapshot{return{schema:"stdio.ifc2x3",document:{header:{fileDescription:[],fileName:[],fileSchema:[]},instances:[{id:1n,entities:[{typeName:"BINARY_EXTENSION",arguments:[{kind:"list",values},{kind:"typed",typeName:"BINARY",values}]}]}]}};}
test("ifc2x3 shared Part21 binary values remain exact in independent SQLite",async()=>{
 const contents=["0","0F","17","23","31","00ABCDEF"],input=fixture(contents.map(content=>({kind:"binary",value:content})));const db=Database.deserialize(await exportSqliteDatabase(await project(input)));
 try{expect(db.query("SELECT string_value FROM ifc2x3_value WHERE kind='binary' ORDER BY id LIMIT 6").all()).toEqual(contents.map(string_value=>({string_value})));expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(input);db.exec("UPDATE ifc2x3_value SET string_value='18' WHERE kind='binary'");await expect(restore(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}
 for(const content of["","4","1","18","24","32","0f","0G"]){await expect(project(fixture([{kind:"binary",value:content}]))).rejects.toThrow();}
});
