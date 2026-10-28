import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework/io/sqlite-snapshot";
import {stepSnapshotToSqliteDatabase as project,stepSnapshotFromSqliteDatabase as restore} from "../🟦️.ts";
import type {StepSnapshot,StepValue} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
function fixture(values:StepValue[]):StepSnapshot{return{schema:"stdio.step",header:{fileDescription:{description:[""],implementationLevel:"2;1"},fileName:{name:"",timestamp:"",author:[""],organization:[""],preprocessorVersion:"",originatingSystem:"",authorization:""},fileSchema:{schemas:["AUTOMOTIVE_DESIGN"]}},entities:[{id:1n,name:"BINARY_EXTENSION",args:values,complex:[]}]};}
test("step shared Part21 binary values remain exact in independent SQLite",async()=>{
 const contents=["0","0F","17","23","31","00ABCDEF"],input=fixture(contents.map(content=>({binary:content})));const db=Database.deserialize(await exportSqliteDatabase(await project(input)));
 try{expect(db.query("SELECT string_value FROM step_value WHERE kind='binary' ORDER BY id LIMIT 6").all()).toEqual(contents.map(string_value=>({string_value})));expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(input);db.exec("UPDATE step_value SET string_value='18' WHERE kind='binary'");await expect(restore(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}
 for(const content of["","4","1","18","24","32","0f","0G"]){await expect(project(fixture([{binary:content}]))).rejects.toThrow();}
});
