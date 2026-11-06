import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import schema from "./🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("issued reference retention agrees with independent original row selection",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const database=new Database(":memory:");
 try{database.exec("CREATE TABLE original_row(rank INTEGER PRIMARY KEY,value TEXT NOT NULL)");for(const[rank,value]of law.rows.entries())database.query("INSERT INTO original_row VALUES (?,?)").run(rank,value);const retained=database.query("SELECT value FROM original_row WHERE rank=?").get(law.selected)as{value:string};database.exec("DROP TABLE original_row");expect(retained.value).toBe(law.retained);}finally{database.close();}
});

test("fallible original child selection admits its accessor before retaining an existing row",()=>{
 const database=new Database(":memory:");
 try{database.exec("CREATE TABLE original_row(rank INTEGER PRIMARY KEY,value TEXT NOT NULL)");for(const[rank,value]of law.rows.entries())database.query("INSERT INTO original_row VALUES (?,?)").run(rank,value);for(const row of law.childCases){const actual=database.query("SELECT CASE WHEN ?>0 AND ?>0 THEN 1 ELSE 0 END AS calls,CASE WHEN ?>0 AND ?>0 AND EXISTS(SELECT 1 FROM original_row WHERE rank=?) THEN 1 ELSE 0 END AS retained").get(row.items,row.depth,row.items,row.depth,row.selected)as{calls:number,retained:number};expect(actual.calls).toBe(row.accessorCalls);expect(Boolean(actual.retained)).toBe(row.retained);}}finally{database.close();}
});
