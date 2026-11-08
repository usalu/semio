import{Database}from"bun:sqlite";
export function independentSqliteExtent(bytes:Uint8Array){
 const db=Database.deserialize(bytes,{safeIntegers:true});try{
  if((db.query("PRAGMA integrity_check").get() as {integrity_check:string}).integrity_check!=="ok"||db.query("PRAGMA foreign_key_check").all().length)throw Error("Independent database integrity");
  const tables=db.query("SELECT name,sql FROM sqlite_schema WHERE type='table' ORDER BY name").all() as {name:string,sql:string}[];
  let rows=0,valueBytes=0,schemaBytes=0;const tableWidths:Record<string,number>={};
  for(const table of tables){const name='"'+table.name.replaceAll('"','""')+'"',columns=db.query("PRAGMA table_info("+name+")").all() as {name:string}[];tableWidths[table.name]=columns.length;schemaBytes+=Buffer.byteLength(table.name)+Buffer.byteLength(table.sql);rows+=Number((db.query("SELECT count(*) AS count FROM "+name).get() as {count:bigint}).count);
   for(const column of columns){const id='"'+column.name.replaceAll('"','""')+'"';valueBytes+=Number((db.query("SELECT coalesce(sum(CASE typeof("+id+") WHEN 'null' THEN 0 WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+id+" AS BLOB)) WHEN 'blob' THEN length("+id+") END),0) AS bytes FROM "+name).get() as {bytes:bigint}).bytes);}
  }return{rows,valueBytes,schemaBytes,tableWidths};
 }finally{db.close();}
}


function ieeeFixture(value:unknown,schema:unknown,word:bigint|undefined,construct:boolean):{value:unknown,count:number}{
 const root=schema as Record<string,unknown>;let count=0;const bytes=new DataView(new ArrayBuffer(8));
 if(word!==undefined&&(word<0n||word>18446744073709551615n))throw Error("fixture word exceeds binary64");
 function node(value:unknown,shape:unknown):unknown{
  if(shape===null||typeof shape!=="object")throw Error("fixture has no authored schema");
  let current=shape as Record<string,any>;
  while(typeof current.$ref==="string"){if(!current.$ref.startsWith("#/"))throw Error("fixture schema reference is not local");let selected:any=root;for(const key of current.$ref.slice(2).split("/"))selected=selected[key.replaceAll("~1","/").replaceAll("~0","~")];if(selected===null||typeof selected!=="object")throw Error("fixture schema reference is absent");current=selected;}
  if(value===null)return null;
  const alternatives=current.anyOf??current.oneOf;if(Array.isArray(alternatives)){const selected=alternatives.filter((s:any)=>s.type!=="null");if(selected.length!==1)throw Error("fixture choice is not a single nullable owner");return node(value,selected[0])}
  const types=Array.isArray(current.type)?current.type:[current.type];
  if(types.includes("number")){if(typeof value!=="number")throw Error("authored scalar fixture is not numeric");count++;if(!construct)return value;bytes.setFloat64(0,value);return{bits:word??bytes.getBigUint64(0)}}
  if(Array.isArray(value)){if(!types.includes("array"))throw Error("fixture array has no authored items");if(construct)return value.map(v=>node(v,current.items));for(const child of value)node(child,current.items);return value}
  if(typeof value==="object"){const result:Record<string,unknown>={};for(const key of Object.keys(value)){if(!Object.hasOwn(current.properties??{},key))throw Error("fixture field has no authored schema: "+key);const child=node((value as Record<string,unknown>)[key],current.properties[key]);if(construct)result[key]=child}return construct?result:value}
  return value;
 }
 const result=node(value,root);return{value:result,count};
}
/** 🧫️ Own every authored binary scalar in a validated semantic test fixture. */
export function ownIeee754Fixture(value:unknown,schema:unknown,word?:bigint):unknown{return ieeeFixture(value,schema,word,true).value}
/** 🔢️ Count actual present scalar occurrences independently of their numeric payload values. */
export function countIeee754FixtureScalars(value:unknown,schema:unknown):number{return ieeeFixture(value,schema,undefined,false).count}
