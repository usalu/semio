import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";

test("sealed Energy authored strings match independent SQLite TEXT custody",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE original_field(rank INTEGER PRIMARY KEY,value TEXT NOT NULL)");
  const values=[...law.strings,law.largeText.pattern.repeat(law.largeText.repetitions)];
  for(const [rank,value]of values.entries())database.query("INSERT INTO original_field VALUES (?,?)").run(rank,value);
  const rows=database.query("SELECT value FROM original_field ORDER BY rank").all()as {value:string}[];
  expect(rows.map(row=>row.value)).toEqual(values);
  for(const [rank,row]of rows.entries())expect(Array.from(new TextEncoder().encode(row.value))).toEqual(Array.from(new TextEncoder().encode(values[rank])));
 }finally{database.close();}
});
