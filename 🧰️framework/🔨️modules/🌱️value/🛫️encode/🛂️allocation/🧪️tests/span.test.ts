import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./span.schema.json";
import fixture from "./span.json";
/** 🧭️ A resumed span belongs to its actual authority, dispatch, extent and physical snapshot. */
test("scoped span recovery keeps original identity and producer reply",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE action(authority INTEGER, turn INTEGER, units INTEGER, owned INTEGER, slot INTEGER)");db.exec("INSERT INTO action VALUES(1,1,3,8,1)");for(const row of fixture.cases){const found=db.query("SELECT 1 FROM action WHERE authority=? AND turn=? AND units=? AND owned=? AND slot=?").get(row.authority,row.turn,row.units,row.owned,row.slot);const actual=found===null?"foreign":!row.entered&&row.reply!=="none"?"invalid":row.reply!=="none"?row.reply:row.entered?"pending":"invoke";expect(actual).toBe(row.expected)}}finally{db.close()}});
