import {Database} from "bun:sqlite";
import {createIssuedResourceControlV1,receiveScriptProcessInvocation,requireIssuedResourceControlV1,withIssuedResourceControlV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1} from "../../../🟦️.ts";

type Case={owner:"issued"|"requested";failure:"deadline"|"cancellation"|"undefined"|"error"};

/** 📤️ Observes one genuine failed child Original as bounded result data. */
export async function observeSettlementCaseV1(row:Case):Promise<void>{
 const db=new Database(":memory:"),sentinel=Error("accepted callback failure"),cancel=new AbortController();let failed=false,failure:unknown,positive=false,closed=false,phase:Parameters<typeof requireIssuedResourceControlV1>[0]|undefined;
 db.exec("create table events(position integer primary key,name text unique)");
 try{
  try{await receiveScriptProcessInvocation(process.env,async invocation=>{
   const body=async(control:Parameters<typeof requireIssuedResourceControlV1>[0])=>{phase=control;positive=(control.remainingMilliseconds()??0)>0;db.query("insert into events values(?,?)").run(1,"callback");void withIssuedResourceOperationV1(control,async()=>{await new Promise<void>(done=>setTimeout(done,row.failure==="cancellation"?80:300));db.query("insert into events values(?,?)").run(2,"child");});if(row.failure==="cancellation")setTimeout(()=>cancel.abort(Error("accepted cancellation")),40);if(row.failure==="undefined")throw undefined;if(row.failure==="error")throw sentinel;return "callback returned";};
   const budget=row.failure==="cancellation"?2000:200;
   if(row.owner==="issued")await withIssuedResourceControlV1(invocation,"test:post-drain",budget,body,cancel.signal);
   else await withRequestedIssuedResourceControlV1(createIssuedResourceControlV1(invocation,"test:post-drain-parent",0),budget,body,cancel.signal);
  });}catch(error){failed=true;failure=error;}
  try{requireIssuedResourceControlV1(phase!);}catch{closed=true;}
  db.query("insert into events values(?,?)").run(3,"observed");
  console.log(JSON.stringify({failed,positive,closed,preserved:row.failure==="undefined"?failure===undefined:row.failure==="error"?failure===sentinel:true,message:failure instanceof Error?failure.message:String(failure),order:db.query("select name from events order by position").all()}));
 }finally{db.close();}
}

