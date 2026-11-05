import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/lifetime.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
test("finite source lifetime remains fixed across bounded callback turns",()=>{
 const policy={...fixture.policy,maximumLifetimeMicroseconds:law.maximumLifetimeMicroseconds};expect(new Ajv({strict:true}).compile(schema)(policy)).toBe(true);
 const db=new Database(":memory:");try{for(const row of law.cases){expect(db.query("SELECT min(?+?,?) AS deadline").get(row.now,law.maximumTurnMicroseconds,law.absoluteDeadline)).toEqual({deadline:row.deadline});expect(law.initialNow+law.maximumLifetimeMicroseconds).toBe(law.absoluteDeadline);}}finally{db.close();}
});
