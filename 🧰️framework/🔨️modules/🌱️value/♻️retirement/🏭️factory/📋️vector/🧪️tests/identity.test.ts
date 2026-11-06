import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("original vector issuer requires both live data and concrete implementation",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of[law.issuerSame,law.issuerAlias])expect(db.query("SELECT (? AND ?) AS admitted").get(Number(row.dataEqual),Number(row.implementationEqual))).toEqual({admitted:Number(row.admitted)});}finally{db.close();}
});
