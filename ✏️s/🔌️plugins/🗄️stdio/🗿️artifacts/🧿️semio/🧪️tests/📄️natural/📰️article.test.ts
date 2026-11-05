import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import law from "./🧫️fixtures/📰️article.json";
import schema from "./🧫️fixtures/📰️article.schema.json";
function list(text:string):unknown[]{
 let index=0;
 const space=()=>{while(/\s/.test(text[index]??"")&&index<text.length)index++;};
 const value=():unknown=>{space();if(text[index]==="["){index++;const rows:unknown[]=[];space();while(text[index]!=="]"){rows.push(value());space();if(text[index]===","){index++;continue;}if(text[index]!=="]")throw new Error("invalid list separator");}index++;return rows;}const start=index;while(index<text.length&&!/[\],\s]/.test(text[index]))index++;return text.slice(start,index);};
 const result=value();space();if(index!==text.length||!Array.isArray(result))throw new Error("invalid document list");return result;
}
test("article retains all runs and marks under its current canonical schema",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const text=await Bun.file(law.dsl).text();expect(await Bun.file(law.pack).exists()).toBe(true);
 const lines=text.split("\n");const schemaLine=lines.find(line=>line.startsWith("schema="))!;
 const embedded=Buffer.from(schemaLine.slice(7),"hex").toString("utf8");
 const runs=list(lines.find(line=>line.startsWith("runs="))!.slice(5)) as unknown[][];
 const db=new Database(":memory:");db.run("CREATE TABLE runs(id INTEGER PRIMARY KEY, marks INTEGER NOT NULL)");
 runs.forEach((run,index)=>{expect(run.length).toBe(3);expect(Array.isArray(run[2])).toBe(true);db.run("INSERT INTO runs VALUES (?,?)",index,(run[2] as unknown[]).length);});
 expect(db.query("SELECT count(*) AS runs,SUM(marks) AS marks FROM runs").get()).toEqual({runs:law.runs,marks:law.marks});db.close();
 expect(lines[0]).toBe(law.preamble);expect(embedded).toBe(law.schema);
});
