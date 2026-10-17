/** 🔎️ Receipt conflict: 1 test covering qualified publication conflicts preserve refusal and report exact bounded receipt differences. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import ts from "typescript";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

test("qualified publication conflicts preserve refusal and report exact bounded receipt differences",async()=>{
 const source=readFileSync(new URL("../../🎛️owner/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("publisher.ts",source,ts.ScriptTarget.Latest,true),fn=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="finishNativeImagePublicationCandidateV1");if(!fn)throw Error("Actual publication completion definition required");
 const code=ts.transpileModule(fn.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,admit=new Ajv({strict:true}).compile(schema.definitions.ReceiptConflict),db=new Database(":memory:");
 try{for(const row of fixture.cases){
  const token=Object.freeze({model:"receipt-conflict"}),receipt=Object.freeze({version:1,filesystemBytes:33554432,maximumPhysicalBytes:34603008,dev:1,ino:2,size:33554432,mountDirectory:{dev:"1",ino:"3",mode:16877}}),expected=Buffer.from(JSON.stringify(receipt)+"\n"),bytes=row.bytesChanged?Buffer.from(JSON.stringify({...receipt,ino:4})+"\n"):expected,fields=["1","2","33152","1",String(expected.length),"1000","1000"],expectedStamp=fields.join(":");if(row.stampIndex!==null)fields[row.stampIndex]=String(Number(fields[row.stampIndex])+1);const actualStamp=fields.join(":"),state={token,receipt,recordStamp:expectedStamp},scope={qualified:true,options:{directory:"/receipt-conflict-source-model"}},sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");let reads=0,fault:unknown;
  const finish=new Function("requireCandidate","requireNativeImagePublicationV1","readNativeImagePublicationRecordV1","join","Buffer","sha",code+";return finishNativeImagePublicationCandidateV1;")((actual:unknown)=>{expect(actual).toBe(token);return state;},(actual:unknown)=>{expect(actual).toBe(token);return scope;},async(actual:unknown,path:string)=>{expect(actual).toBe(token);expect(path).toBe(join(scope.options.directory,"image.json"));reads++;return{bytes,stamp:actualStamp,sha256:sha(bytes)};},join,Buffer,sha)as(token:unknown)=>Promise<void>;
  try{await finish(token);}catch(error){fault=error;}expect(fault).toBeInstanceOf(Error);expect((fault as Error).message).toBe("Qualified publication receipt changed");expect(reads).toBe(1);
  const cause=(fault as Error).cause as Record<string,unknown>;expect(admit(cause),row.id).toBe(true);expect(Object.isFrozen(cause)).toBe(true);const independent=db.query("select ?1=?2 stampEqual,?3=?4 bytesEqual,?5 expectedBytes,?6 actualBytes").get(expectedStamp,actualStamp,expected.toString("hex"),bytes.toString("hex"),expected.length,bytes.length)as{stampEqual:number;bytesEqual:number;expectedBytes:number;actualBytes:number};
  expect(cause).toEqual({version:1,kind:"qualified-receipt-conflict",stampEqual:Boolean(independent.stampEqual),bytesEqual:Boolean(independent.bytesEqual),expectedStamp,actualStamp,expectedSha256:sha(expected),actualSha256:sha(bytes),expectedBytes:independent.expectedBytes,actualBytes:independent.actualBytes});
 }
 }finally{db.close();}
});
