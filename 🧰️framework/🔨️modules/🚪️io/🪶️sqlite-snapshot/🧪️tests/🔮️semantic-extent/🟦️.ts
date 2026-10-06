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
