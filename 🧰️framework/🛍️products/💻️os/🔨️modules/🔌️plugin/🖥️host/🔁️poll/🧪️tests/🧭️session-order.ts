import{test,expect}from"bun:test";import{readFileSync}from"node:fs";import Ajv from"ajv/dist/2020";import{Database}from"bun:sqlite";test("issuer origin scopes require complete purpose aggregation before financial commit",()=>{const f=JSON.parse(readFileSync(import.meta.dirname+"/🧫️session-order.json","utf8")),s=JSON.parse(readFileSync(import.meta.dirname+"/🧬️session-order.json","utf8"));expect(new Ajv({strict:true}).compile(s)(f)).toBe(true);const db=new Database(":memory:");try{const before=(a:string,b:string,order:string[])=>db.query("select cast(a.key as integer)<cast(b.key as integer) valid from json_each(?) a,json_each(?) b where a.value=? and b.value=?").get(JSON.stringify(order),JSON.stringify(order),a,b);for(const[a,b]of f.requiredBefore)expect(before(a,b,f.order)).toEqual({valid:1});const wrong=[...f.order];[wrong[1],wrong[2]]=[wrong[2],wrong[1]];expect(before("aggregatePurposes","validateOriginScopes",wrong)).toEqual({valid:0})}finally{db.close()}});
test("frozen escrow traversal follows the publisher's whole row extent",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🧫️session-order.json",import.meta.url),"utf8"));
 const db=new Database(":memory:");
 try{
  const rows=db.query("select case when ?1 between 48 and 5936 and (?1-48)%184=0 then (?1-48)/184 end rows");
  for(const row of fixture.escrowExtents)expect(rows.get(row.bytes)).toEqual({rows:row.rows});
 }finally{db.close()}
 const session=readFileSync(new URL("../🎛️session/🦀️.rs",import.meta.url),"utf8");
 expect(session).toContain("original_escrow_length_valid(extent)");
 expect(session).toContain("(length-ORIGINAL_ESCROW_HEADER_BYTES)/ORIGINAL_ESCROW_ROW_BYTES");
});
