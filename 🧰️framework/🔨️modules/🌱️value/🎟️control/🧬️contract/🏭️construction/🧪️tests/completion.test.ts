import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./completion.schema.json";
import cases from "./completion.json";
/** 📬️ Only the actual recorded completion moves under its original funded action. */
test("original construction completion custody",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ? AND ? AND ?) AS moved");for(const c of cases){const r=q.get(+c.recorded,+c.same,+c.free,+c.grant,+c.work) as {moved:number};expect(Boolean(r.moved)).toBe(c.moved);const published=db.query("SELECT (? AND ?) AS published").get(+c.moved,+c.postAccepted) as {published:number};expect(Boolean(published.published)).toBe(c.published);expect(c.originalRefusal).toBe("original");if(c.payoutRefusal!==null){expect(c.moved).toBe(true);expect(c.postAccepted).toBe(false);expect(c.payoutRefusal).not.toBe(c.originalRefusal)}}}finally{db.close()}});
