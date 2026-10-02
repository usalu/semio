/** 🛡️ Borrowed I-JSON admission retains the entire owned JSON syntax domain. */
import type {JsonSnapshot,JsonValue} from "../../../🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import {jsonNumberMeaning} from "../../../🧱️base/🧬️schema/📸️snapshot/🔢️number/🟦️.ts";
import type {ArtifactDialect} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type {SqliteDatabase} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,type ArtifactSqliteOptions} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
/** 🧭️ An owned named-profile diagnostic without foreign runtime types. */
export interface IJsonSqliteDiagnostic{readonly code:string;readonly severity:"error"|"warning";readonly message:string}
/** 🔬️ Three ordered, iterative native-domain passes with controlled work. */
export async function checkIJsonConformanceControlled(snapshot:JsonSnapshot,options:ArtifactSqliteOptions={}):Promise<readonly IJsonSqliteDiagnostic[]>{
 let steps=0;const diagnostics:IJsonSqliteDiagnostic[]=[];
 const tick=async():Promise<void>=>{if(++steps%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",steps,0);};
 const add=(code:string,severity:"error"|"warning",message:string):void=>{diagnostics.push({code:`stdio.json.i-json.${code}`,severity,message});};
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);
 if(snapshot.value.kind!=="object"&&snapshot.value.kind!=="array")add("top-level-scalar","warning","top-level value is neither an object nor an array -- RFC7493 §2.1");
 for(let pass=0;pass<3;pass++){
  let rows=0;
  const active=new Set<JsonValue>(),stack:{value:JsonValue;index:number;seen:Set<string>}[]=[{value:snapshot.value,index:-1,seen:new Set()}];
  while(stack.length){
   const frame=stack.at(-1)!;
   if(frame.index===-1){
    if(active.has(frame.value))throw Error("I-JSON typed JSON contains a cycle");active.add(frame.value);
    if(++rows>(options.maxRows??1_000_000))throw Error("I-JSON conformance row limit");await tick();frame.index=0;
    if(pass===1&&frame.value.kind==="number"){
     const lexeme=frame.value.lexeme,meaning=await jsonNumberMeaning(lexeme,options,"projectSnapshot",steps,0);
     if(!meaning.valid)add("invalid-number-lexeme","error","a number requires an RFC8259 lexeme");
     else if(meaning.numeric===null)add("number-not-binary64","error","a number exceeds finite IEEE754 binary64 representation -- RFC7493 §2.2");
     else if(!lexeme.includes(".")&&!lexeme.includes("e")&&!lexeme.includes("E")){const digits=lexeme.startsWith("-")?lexeme.slice(1):lexeme;if(digits.length>16||digits.length===16&&digits>"9007199254740991")add("unsafe-integer","error","integer exceeds ±(2^53-1) -- RFC7493 §2.2");}
    }
    if(pass===2&&frame.value.kind==="string"){
     let noncharacter=false,scanned=0;
     for(const character of frame.value.value){const code=character.codePointAt(0)!;noncharacter||=code>=0xfdd0&&code<=0xfdef||(code&0xffff)>=0xfffe;if(++scanned%65536===0)await artifactSqliteCheckpoint(options,"projectSnapshot",steps,0);}
     if(noncharacter)add("string-noncharacter","warning","string contains a Unicode noncharacter -- RFC7493 §2.3");
    }
   }
   let child:JsonValue|undefined;
   if(frame.value.kind==="array")child=frame.value.items[frame.index];
   else if(frame.value.kind==="object"){const member=frame.value.members[frame.index];if(member){if(pass===0){if(frame.seen.has(member.key))add("duplicate-member-name","error","object member name appears more than once -- RFC7493 §2.3");frame.seen.add(member.key);}child=member.value;}}
   if(child){frame.index++;stack.push({value:child,index:-1,seen:new Set()});}else{active.delete(frame.value);stack.pop();}
  }
 }
 await artifactSqliteCheckpoint(options,"projectSnapshot",steps,steps,false);return diagnostics;
}
/** 🧾️ Exact I-JSON coordinate and owned document identity before named admission. */
export async function validateIJsonSnapshotSqliteDialect(snapshot:JsonSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly IJsonSqliteDiagnostic[]>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);
 if(dialect.artifactKind!=="s.stdio.json"||dialect.standard!=="rfc8259"||dialect.subset!=="i-json")throw Error("I-JSON does not own this semantic subset");
 const table=database.tables.find(table=>table.name.toLowerCase()==="json_document");if(table?.rows.length!==1||table.rows[0]!.rowid!==1n||artifactSqliteInteger(table.rows[0]!,0)!==1n||artifactSqliteText(table.rows[0]!,1)!==snapshot.schema)throw Error("I-JSON document identity disagrees with its snapshot");
 return checkIJsonConformanceControlled(snapshot,options);
}
