import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("native original entity ranks match independent SQLite indexed keys",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE entity(id INTEGER PRIMARY KEY,rank INTEGER NOT NULL)");for(let rank=0;rank<law.rowCount;rank++)db.query("INSERT INTO entity VALUES (?,?)").run(rank+1,rank);for(const row of law.cases){const result=db.query("SELECT rank FROM entity WHERE id=?").get(row.id)as{rank:number}|null;expect(result?.rank??null).toBe(row.rank);}}finally{db.close();}});
