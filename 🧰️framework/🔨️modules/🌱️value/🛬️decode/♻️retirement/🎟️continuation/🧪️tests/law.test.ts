import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./schema.json";
import law from "./🔣️.json";
/** 🧹️ The exact native header is funded before its original continuation closes. */
test("decoder continuation physical law",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? >= ? AND ? >= ? AND ? >= ?) AS admitted");for(const [items,copy,work,expected] of [[0,32,1,false],[1,31,1,false],[1,32,0,false],[1,32,1,true]] as const){const row=q.get(items,law.items,copy,32,work,law.work) as {admitted:number};expect(Boolean(row.admitted)).toBe(expected)}}finally{db.close()}});
