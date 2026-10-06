import {Database} from "bun:sqlite";
export function sqliteInventory(caseRow:any):unknown{
 const db=new Database(":memory:");
 try{
  const scope=caseRow.coordinate.owner,wire=db.query(`WITH leaves AS(SELECT CAST(s.key AS INTEGER) AS sourceIndex,CAST(l.key AS INTEGER) AS leafIndex,json_extract(l.value,'$.owner') AS owner,json_extract(l.value,'$.id') AS id,json_extract(l.value,'$.variant') AS variant,json_extract(l.value,'$.outcomes') AS outcomes FROM json_each(?,'$.sources') s,json_each(s.value) l),eligible AS(SELECT * FROM leaves WHERE substr(owner,1,length(?)+1)=?||'/' AND length(owner)>length(?)+1 AND NOT EXISTS(SELECT 1 FROM json_each(?) omitted WHERE instr('/'||substr(owner,length(?)+2)||'/','/'||omitted.value||'/')>0)),ranked AS(SELECT *,row_number() OVER(PARTITION BY id ORDER BY sourceIndex,leafIndex) AS ordinal FROM eligible) SELECT json_object('id',id,'variant',variant,'outcomes',json(outcomes)) AS wire FROM ranked WHERE ordinal=1 ORDER BY sourceIndex,leafIndex`).all(JSON.stringify(caseRow),scope,scope,scope,JSON.stringify(caseRow.coordinate.excludedOwnerSegments),scope) as {wire:string}[];
  return {schema:"semio.repository-test.runtime-inventory/v2",artifact:caseRow.coordinate.artifact,standard:caseRow.coordinate.standard,subset:caseRow.coordinate.subset,...(caseRow.coordinate.surface===null?{}:{surface:caseRow.coordinate.surface}),bridgeVersion:1,producedBy:"sample-inventory",mutations:wire.map(row=>JSON.parse(row.wire))};
 }finally{db.close();}
}
