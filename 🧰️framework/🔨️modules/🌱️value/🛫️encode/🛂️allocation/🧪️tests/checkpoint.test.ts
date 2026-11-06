import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./checkpoint.schema.json";
import fixture from "./checkpoint.json";
/** 🚦️ The actual checkpoint decision belongs to one producer and progress snapshot. */
test("checkpoint recovery preserves original boolean and source",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE action(authority INTEGER, turn INTEGER, completed INTEGER, total INTEGER, owned INTEGER)");db.exec("INSERT INTO action VALUES(1,1,4,8,16)");for(const row of fixture.cases){const found=db.query("SELECT 1 FROM action WHERE authority=? AND turn=? AND completed=? AND total=? AND owned=?").get(row.authority,row.turn,row.completed,row.total,row.owned);const actual=found===null?"foreign":!row.entered&&row.reply!=="none"?"invalid":row.reply!=="none"?row.reply:row.entered?"pending":"invoke";expect(actual).toBe(row.expected)}}finally{db.close()}});
