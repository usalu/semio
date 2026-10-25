import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {nxChildLaunchesV1,nxLaunchWithoutParallelismV1} from "../🟦️.ts";
import fixture from "./🔣️.json";

const oracle=`with t as (select key,value,lag(value) over (order by key) prev from json_each(?1)),b as (select coalesce(min(key),1000000) k from json_each(?1) where value='--')
select json_group_array(value) value from (select value from t,b where key>=b.k or (value!='--parallel' and substr(value,1,11)!='--parallel=' and coalesce(prev,'')!='--parallel') order by key)`;

test("a launch without its parallelism grant equals the authored vector and an independent SQLite computation",()=>{
 const db=new Database(":memory:");
 try{for(const row of fixture.launchCases){const actual=nxLaunchWithoutParallelismV1(row.launch);expect(actual).toEqual(row.bare);expect(JSON.parse((db.query(oracle).get(JSON.stringify(row.launch)) as {value:string}).value)).toEqual(actual);}}finally{db.close();}
});

test("a selected Nx child is carried only when declared exactly or up to its parallelism grant",()=>{
 for(const row of fixture.childCases){
  const carried=nxChildLaunchesV1(fixture.declared,row.selection);
  if(row.carries==="declared")expect(carried).toEqual(fixture.declared);
  else if(row.carries==="declared-and-selection")expect(carried).toEqual([...fixture.declared,row.selection]);
  else expect(carried).toBeUndefined();
 }
});
