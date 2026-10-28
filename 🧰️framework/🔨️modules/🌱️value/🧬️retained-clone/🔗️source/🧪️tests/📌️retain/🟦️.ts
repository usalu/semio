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
