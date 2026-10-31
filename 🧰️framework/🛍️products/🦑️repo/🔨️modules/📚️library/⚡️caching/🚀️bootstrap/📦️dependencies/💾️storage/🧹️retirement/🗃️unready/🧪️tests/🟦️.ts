import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import corpus from "./🔣️.json" with {type:"json"};
import corpusSchema from "./🧬️schema.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

test("retained unready capsule neutral fixtures carry closed non-authoritative output",()=>{
 const ajv=new Ajv({strict:true});expect(ajv.validate(corpusSchema,corpus)).toBe(true);
 for(const row of corpus.cases)if(row.accepted){expect(ajv.validate(schema,row.expected)).toBe(true);expect(row.expected?.intentSha256).toBe(createHash("sha256").update(row.intentText).digest("hex"));expect(row.expected?.receiptSha256).toBe(createHash("sha256").update(row.receiptText).digest("hex"));}
});

test("actual retained unready capsule reader matches independent SQLite projection and refuses hostile wire",async()=>{
 const api=await import("../🟦️.ts"),read=(api as Record<string,unknown>).readJavascriptDependencyRetainedUnreadyCapsuleDataV1;expect(typeof read).toBe("function");if(typeof read!=="function")throw Error("Actual retained unready capsule reader required");
 const db=new Database(":memory:");try{for(const row of corpus.cases){let accepted=false,result:unknown;try{result=read(row.workspace,row.root,row.capsule,row.intentText,row.receiptText);accepted=true;}catch{}expect(accepted,row.id).toBe(row.accepted);if(accepted){const projection=db.query("select json_object('version',1,'purpose','retained-unready-dependency-generation-data','authority',json('false'),'currentReady',json('false'),'capsule',?1,'generation',json_extract(?2,'$.generation'),'scope',json_extract(?2,'$.scope'),'originalDirectory',json_extract(?2,'$.directory'),'recipeSha256',json_extract(?2,'$.recipeSha256'),'intentSha256',?3,'receiptSha256',?4) text").get(row.capsule,row.intentText,createHash("sha256").update(row.intentText).digest("hex"),createHash("sha256").update(row.receiptText).digest("hex")) as {text:string};expect(result).toEqual(JSON.parse(projection.text));expect(result).toEqual(row.expected);expect(Object.isFrozen(result)).toBe(true);expect(new Ajv({strict:true}).validate(schema,result)).toBe(true);}}}finally{db.close();}
 let hooks=0;const hostile=new Proxy({},{get(){hooks++;throw Error("Getter");},ownKeys(){hooks++;throw Error("Keys");}}),base=corpus.cases[0]!;for(const index of [0,1,2,3,4]){const args:unknown[]=[base.workspace,base.root,base.capsule,base.intentText,base.receiptText];args[index]=hostile;expect(()=>read(...args)).toThrow();}expect(hooks).toBe(0);
});
