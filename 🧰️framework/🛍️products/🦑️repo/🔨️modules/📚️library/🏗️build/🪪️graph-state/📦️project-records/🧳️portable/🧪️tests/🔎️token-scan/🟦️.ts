/** 🔎️ Contract: a machine that takes the rows of a record or of a closure entry looks for the generation token in each row without writing the row out as JSON, and answers what the JSON search answered. @see ../../🟦️.ts */
import{expect,spyOn,test}from"bun:test";
import{hostOfRootV1,localRowsV1,localizeRowsV1,portableRowsPolicyV1 as policy,type ObservationRowV1}from"../../🟦️.ts";

const host=hostOfRootV1("/work/semio"),put="g".repeat(16);
const rows:ObservationRowV1[]=Array.from({length:400},(_,at)=>at%7===0?{kind:"file",path:`${policy.rootToken}/lib/${policy.token}/${at}.ts`,fact:{sha256:"a".repeat(64),size:at,mtimeMs:0,ctimeMs:0,ino:0}}:at%11===0?{kind:"file",path:`${policy.rootToken}/lib/${at}.ts`,fact:{sha256:"b".repeat(64),note:[`x ${policy.token} y`,{deep:[`${policy.token}`]}],size:at}}:{kind:"file",path:`${policy.rootToken}/lib/${at}.ts`,fact:{sha256:"c".repeat(64),size:at,mtimeMs:0,ctimeMs:0,ino:0}});
const oracle=(row:ObservationRowV1)=>JSON.stringify([row.path,row.fact??null]).includes(policy.token)?(([path,fact])=>({...row,path,...row.fact===undefined?{}:{fact}}))(JSON.parse(JSON.stringify([row.path,row.fact]).replaceAll(policy.token,put))):row;
const counted=<T>(work:()=>T):{value:T;stringified:number}=>{const spy=spyOn(JSON,"stringify");try{const value=work();return{value,stringified:spy.mock.calls.length};}finally{spy.mockRestore();}};

test("the rows of a record take the machine's generation without a row being written out as JSON",()=>{
 const{value,stringified}=counted(()=>localRowsV1(rows,put));
 expect(value).toEqual(rows.map(oracle));expect(rows.filter(row=>JSON.stringify(row).includes(policy.token)).length).toBeGreaterThan(50);
 expect(JSON.stringify(value).includes(policy.token)).toBe(false);
 expect(stringified).toBe(0);
});

test("the rows of a closure entry take the machine's generation without a row being written out as JSON",()=>{
 const{value,stringified}=counted(()=>localizeRowsV1(rows,host,{generation:put}));
 expect(value.map(row=>row.fact)).toEqual(rows.map(row=>oracle(row).fact));expect(value.map(row=>row.path)).toEqual(rows.map(row=>oracle(row).path.replace(policy.rootToken,host.root)));
 expect(JSON.stringify(value).includes(policy.token)).toBe(false);
 expect(stringified).toBe(0);
});

test("a row converted once is not converted again for the same memo",()=>{
 const memo=new WeakMap<object,ObservationRowV1>(),first=localizeRowsV1(rows,host,{generation:put},policy,memo),second=localizeRowsV1(rows,host,{generation:put},policy,memo);
 expect(second.every((row,at)=>row===first[at])).toBe(true);
});
