/** 🎟️ Native fixture dispatch renews the authored turn on the same retained Source after a spent archive close turn. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import cases from "./🔣️.json" with {type:"json"};

const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
type Grant=readonly [items:number,copyBytes:number];

const grantFor=(driver:string,ceiling:Grant,left:Grant):Grant=>{
 const body=source.slice(source.indexOf("pub async fn "+driver+"<"));
 const selector=body.match(/let \(fresh_fuel,fresh_grant\)=\(OriginalFixtureCaller::turn_fuel\(\),(cx\.[^;]+)\);/u)?.[1];
 const grants={"cx.retained_grant()":left,"cx.original_turn_ceiling().0":ceiling};
 if(selector===undefined||!Object.hasOwn(grants,selector))throw Error("Original fixture renewal selector required");
 return grants[selector as keyof typeof grants];
};

for(const row of cases)test(row.name,()=>{
 const db=new Database(":memory:");
 try{
  const issued=row.issued as unknown as Grant,spent=row.spent as unknown as Grant,prior=row.prior as unknown as Grant,sourceLeft=row.source as unknown as Grant,demand=row.demand as unknown as Grant;
  const left:Grant=[prior[0]-spent[0],prior[1]-spent[1]];
  const oracle=db.query("select min(?1,?3) as items, min(?2,?4) as copy, min(?1,?3)>=?5 and min(?2,?4)>=?6 as enters").get(...issued,...sourceLeft,...demand);
  for(const driver of ["dispatch_fixture_action","dispatch_fixture_command"]){
   const desired=grantFor(driver,issued,left),grant=[Math.min(desired[0],sourceLeft[0]),Math.min(desired[1],sourceLeft[1])];
   const actual={items:grant[0],copy:grant[1],enters:Number(grant[0]!>=demand[0]&&grant[1]!>=demand[1])};
   expect(actual,driver).toEqual(oracle);
   expect(Boolean(actual.enters),driver).toBe(row.enters);
  }
 }finally{db.close();}
});
