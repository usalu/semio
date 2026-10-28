import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import law from "./🔣️.json";

test("original UI conversion preserves independent JSON and SQLite values",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE values_by_case (id INTEGER PRIMARY KEY, value TEXT NOT NULL)");const cases=[...law.cases,law.wideText.repeat.repeat(law.wideText.count)];let deep:unknown=true;for(let i=0;i<law.deepListDepth;i++)deep=[deep];cases.push(deep as never);for(const[index,value]of cases.entries()){const json=JSON.stringify(value);db.query("INSERT INTO values_by_case VALUES (?,json(?))").run(index,json);expect(JSON.parse((db.query("SELECT value FROM values_by_case WHERE id=?").get(index)as{value:string}).value)).toEqual(value);expect(JSON.parse(Buffer.from(json).toString("utf8"))).toEqual(value)}db.close();
});
