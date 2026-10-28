import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import law from "./🔣️.json";

test("original UI merge agrees with independent JavaScript and SQLite ownership results",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE merges(id INTEGER PRIMARY KEY,value TEXT)");for(const[index,row]of law.cases.entries()){const a=row.args,b=row.input;const object=(v:unknown)=>v!==null&&typeof v==="object"&&!Array.isArray(v);const output=b===null?a:a===null?b:object(a)&&object(b)?Object.assign({},a,b):b;expect(output).toEqual(row.expected);db.query("INSERT INTO merges VALUES (?,json(?))").run(index,JSON.stringify(output));expect(JSON.parse((db.query("SELECT value FROM merges WHERE id=?").get(index)as{value:string}).value)).toEqual(row.expected)}db.close();
});
