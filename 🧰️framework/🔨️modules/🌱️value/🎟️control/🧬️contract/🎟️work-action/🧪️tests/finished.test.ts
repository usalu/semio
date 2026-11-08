import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./finished.schema.json";
import cases from "./finished.json";
/** 🧾️ A finished original Work proof still names its actual scope, issuer and observed work. */
test("finished original Work custody",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ? AND ?) AS valid");for(const c of cases){const r=q.get(+c.sameIssuer,+c.sameScope,+c.sameActual,+c.finished) as {valid:number};expect(Boolean(r.valid)).toBe(c.valid)}}finally{db.close()}});
