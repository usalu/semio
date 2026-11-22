import {test,expect} from "bun:test";
import {dirname,resolve} from "node:path";
import {loadProductFunctionsSyncV1} from "../../../../🧪️testing/🧩️extracted-function/🟦️.ts";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json";
import schema from "./🧬️.json";
import Ajv from "ajv";
test("retirement descriptor reads stay finite when a record grows after stat",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of corpus.cases){let read=0,closed=0,opened=0;const data=Buffer.from(row.actual),file={dev:1,ino:2,nlink:1,size:row.declared,mtimeMs:1,ctimeMs:1,isFile:()=>true,isSymbolicLink:()=>false},ports={Buffer:{alloc:(size:number)=>{expect(size).toBe(row.declared+1);return Buffer.alloc(size);}},dirname,resolve,constants:{O_RDONLY:0,O_NOFOLLOW:1},lstatSync:(path:string)=>path==="/record"?file:{isDirectory:()=>true,isSymbolicLink:()=>false},openSync:()=>{opened++;return 7;},closeSync:()=>{closed++;},fstatSync:()=>file,readFileSync:()=>{throw Error("Unbounded descriptor read forbidden");},readSync:(_fd:number,buffer:Buffer,offset:number,length:number,position:number|null)=>{expect(length).toBeLessThanOrEqual(row.maximum+1-read);expect(position).toBe(null);const count=Math.min(length,1,data.length-read);if(count>0)data.copy(buffer,offset,read,read+count);read+=count;return count;}},run=loadProductFunctionsSyncV1(new URL("./🟦️.ts",import.meta.url),["readNativeImageRetirementFileV1"],ports).readNativeImageRetirementFileV1;let accepted=false;try{const result=run("/record",row.maximum);accepted=true;expect(result.bytes.toString()).toBe(row.actual);expect(result.stamp).toBe("1:2:1:"+row.declared+":1:1");}catch(error){if(row.accepted)throw error;}expect(accepted,row.id).toBe(row.accepted);expect(closed).toBe(opened);expect(read).toBeLessThanOrEqual(row.maximum+1);const oracle=db.query("SELECT (?1 > 0 AND ?1 < 9007199254740991 AND ?1=CAST(?1 AS INTEGER) AND ?2=length(?3) AND ?2<=?1) accepted").get(row.maximum,row.declared,row.actual) as {accepted:number};expect(accepted,row.id+" SQLite byte-length oracle").toBe(Boolean(oracle.accepted));}}finally{db.close();}
});
