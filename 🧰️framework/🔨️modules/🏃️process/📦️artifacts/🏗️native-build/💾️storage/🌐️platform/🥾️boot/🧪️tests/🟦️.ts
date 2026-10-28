import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{mkdtempSync,mkdirSync,lstatSync,rmSync,realpathSync,renameSync,symlinkSync}from"node:fs";
import{join,dirname,resolve}from"node:path";
import Ajv from"ajv";
import schema from"../../../🧬️schema/🔣️.json";
import mountSchema from"../../📋️contract/📂️mount/🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
import readCorpus from"./🧫️fixtures/🔣️.json";
import{isOwnedChildLostErrorV1,ownedChildLostErrorV1}from"../../../../../../🎛️owned-execution/🏁️settlement/🟦️.ts";
import*as boot from"../🟦️.ts";

test("observation tolerates boot independent ancestry DATA with an exact SQLite oracle",()=>{const ajv=new Ajv({strict:true}).addSchema(mountSchema).addSchema(schema),validCase=ajv.compile({$ref:schema.$id+"#/definitions/AncestryBindingCase"}),validBinding=ajv.compile({$ref:schema.$id+"#/definitions/AncestryBinding"}),db=new Database(":memory:"),bind=(boot as unknown as Record<string,unknown>).requireNativeImageAncestryBindingV1;expect(typeof bind).toBe("function");try{for(const row of corpus){expect(Boolean(validCase(row)),row.name).toBe(true);const input=row.observation as {published?:unknown;current?:unknown},valid=Boolean(validBinding(input)),same=valid?Boolean((db.query("SELECT json_array_length(:published)=json_array_length(:current) AND NOT EXISTS(SELECT 1 FROM json_each(:published) AS p LEFT JOIN json_each(:current) AS c ON p.key=c.key WHERE p.type<>'text' OR c.type<>'text' OR p.value IS NOT c.value) AS same").get({":published":JSON.stringify(input.published),":current":JSON.stringify(input.current)})as{same:number}).same):false;expect(same,row.name).toBe(row.accepted);let accepted=true;try{(bind as Function)(input);}catch{accepted=false;}expect(accepted,row.name).toBe(row.accepted);}}finally{db.close();}});

test("observation tolerates stable ancestry while actual ancestor replacement and links refuse",()=>{const bind=(boot as unknown as Record<string,unknown>).requireNativeImageAncestryBindingV1;expect(typeof bind).toBe("function");const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original test artifact directory required");mkdirSync(base,{recursive:true});const area=realpathSync(mkdtempSync(join(resolve(base),"boot-"))),parent=join(area,"p"),directory=join(parent,"d","image");try{mkdirSync(join(parent,"d"),{recursive:true});const published=boot.readNativeImageAncestryV1(directory),expected:string[]=[];for(let path=dirname(directory);;path=dirname(path)){expected.push(String(lstatSync(path,{bigint:true}).ino));if(dirname(path)===path)break;}expect(published).toEqual(expected);expect(Object.isFrozen(published)).toBe(true);(bind as Function)({published,current:boot.readNativeImageAncestryV1(directory)});renameSync(parent,join(area,"old"));mkdirSync(join(parent,"d"),{recursive:true});expect(()=>(bind as Function)({published,current:boot.readNativeImageAncestryV1(directory)})).toThrow("ancestry");rmSync(parent,{recursive:true});symlinkSync(join(area,"old"),parent,process.platform==="win32"?"junction":"dir");expect(()=>boot.readNativeImageAncestryV1(directory)).toThrow("ancestry");}finally{rmSync(area,{recursive:true,force:true});}});

test("observation tolerates canonical boot identifiers without granting receipt mutation",()=>{expect(boot.parseNativeBootSessionV1("1e4a0cbe-3710-481f-9614-f8f1fcbd61d0\n")).toBe("1E4A0CBE-3710-481F-9614-F8F1FCBD61D0");expect(()=>boot.parseNativeBootSessionV1("not a boot session")).toThrow("Exact boot session");expect(()=>boot.parseNativeBootSessionV1(" ".repeat(129)+"1E4A0CBE-3710-481F-9614-F8F1FCBD61D0")).toThrow("Exact boot session");});

test("a boot session read repeats exactly after a lost child, at most the declared attempts, with an exact SQLite oracle",async()=>{
 const database=new Database(":memory:"),ajv=new Ajv({strict:true}).addSchema(schema),validate=ajv.compile({$ref:schema.$id+"#/definitions/BootSessionReadCase"}),policy=schema.definitions.BootSessionRead.const,control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>{},yieldContinuation:async()=>{}};
 try{for(const row of readCorpus){expect(validate(row)).toBe(true);let calls=0;const read=async():Promise<string>=>{const outcome=row.outcomes[calls++]!;if(outcome==="lost")throw ownedChildLostErrorV1("/usr/sbin/sysctl",4242);if(outcome==="fault")throw Error("fault");return"value";};
  const settled=await boot.retryNativeBootReadV1(control,read).then(()=>"value",error=>isOwnedChildLostErrorV1(error)?"lost":"fault"),
   expected=database.query<{attempts:number;last:string},[string,number]>("select coalesce((select min(key)+1 from json_each(?1) where key<?2 and value!='lost'),?2) attempts,(select value from json_each(?1) where key=coalesce((select min(key) from json_each(?1) where key<?2 and value!='lost'),?2-1)) last").get(JSON.stringify(row.outcomes),policy.maximumAttempts)!;
  expect(calls).toBe(row.attempts);expect(calls).toBe(expected.attempts);expect(settled).toBe(row.result);expect(settled).toBe(expected.last==="ok"?"value":expected.last);}}
 finally{database.close();}
});
