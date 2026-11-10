import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "./birth-chain.schema.json";
import law from "./birth-chain.json";
/** 🌱️ A birth chain turn is atomic: it is born exactly when its Work budget covers the whole chain, and spends at most that. */
test("return birth chain cut at every turn budget",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 for(const cut of law.cuts){expect(cut.born).toBe(cut.fuel>=law.needed)}
 expect(law.spent).toBeLessThanOrEqual(law.needed);
});
