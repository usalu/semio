import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {projectDocumentBufferV1,type BufferSettlementV1} from "../🟦️.ts";
test("document buffer settlement preserves uncertain paths and dirty drafts with independent SQLite",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const row of fixture.cases){const affected=db.query("select exists(select 1 from json_each(?1) where ?2=value or substr(?2,1,length(value)+1)=value||'/') matched").get(JSON.stringify(row.roots),row.path)as{matched:number};const changed=!row.dirty&&Boolean(affected.matched)&&(row.operation==="rename"||row.recovery!==null);const oracle={path:changed&&row.operation==="rename"?row.destination+row.path.slice(row.roots[0]!.length):row.path,readOnly:changed&&!(row.operation==="rename"&&row.loaded&&row.path===row.roots[0]),recovery:changed&&row.operation==="uncertain"?row.recovery:null,changed};expect(oracle,row.id).toEqual({path:row.expectedPath,readOnly:row.readOnly,recovery:row.expectedRecovery,changed:row.changed});expect(projectDocumentBufferV1(row as BufferSettlementV1),row.id).toEqual(oracle);}}finally{db.close();}
});
