import type {JournalFeedV1,JournalRowV1} from "../../../🗞️journal-clean/🟦️.ts";
import cases from "../../../💾️owner-checkpoint/🧪️tests/🔣️.json";

export const CHECKPOINT_SCOPE=cases.scope,CHECKPOINT_FORMAT=cases.format,ownerQuestions=cases.questions;
export const feedOf=(rows:readonly (readonly [string,0|1,string])[]):JournalFeedV1=>({excluder:()=>()=>false,cursor:async()=>({epoch:"e1",seq:10}),since:async()=>({exact:true,cursor:{epoch:"e1",seq:12},rows})});
export const rowsOf=(root:string,index:number):JournalRowV1[]=>[{kind:"exists",path:root+"/pkg"+index+"/manifest.json",fact:true},{kind:"file",path:root+"/pkg"+index+"/source.ts",fact:"sha"+index}];
