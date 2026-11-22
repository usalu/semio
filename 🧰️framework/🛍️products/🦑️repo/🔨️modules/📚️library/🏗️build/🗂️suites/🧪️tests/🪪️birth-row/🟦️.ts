import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{readBuildTestTableV1,buildTestRootV1}from"../../🟦️.ts";
import{routeProcessTestFilesV1,discoverProcessTestFilesV1}from"../../../../../../../../🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-route-table"});
/** 🪪️ A birth a test file declares agrees with the suite row that runs it: the library envelope names its own row and scratch, a direct declaration carries `artifacts` exactly when the row does. */
test("every declared test birth agrees with the capabilities, scratch and id of its suite row",()=>{
 const table=readBuildTestTableV1(),files=discoverProcessTestFilesV1(buildTestRootV1),routes=routeProcessTestFilesV1(files,table),problems:string[]=[];
 for(const file of files){
  const rows=routes.get(file);if(!rows?.length)continue;
  const text=readFileSync(join(buildTestRootV1,file),"utf8"),row=rows[0]!;
  if(!/\b(?:test|registerBuildTestLawV1|registerIsolatedBuildTestV1)\(/u.test(text))continue;
  for(const match of text.matchAll(/declareLibraryBuildTestEnvelopeV1\(\s*\{\s*suite:\s*"([^"]+)"([^}]*)\}\s*\)/gu)){
   const scratch=/scratch:\s*"(sandbox|checkout-test|checkout)"/u.exec(match[2]!)?.[1]??"sandbox";
   if(!rows.some(candidate=>candidate.id===match[1]))problems.push(file+": suite "+match[1]+" is not its row "+rows.map(candidate=>candidate.id).join(","));
   else if(!rows.some(candidate=>candidate.id===match[1]&&(candidate.scratch??"sandbox")===scratch))problems.push(file+": scratch "+scratch+" differs from row "+match[1]);
  }
  for(const match of text.matchAll(/declareBuildTestEnvelopeV1\(\s*\{[^\n]*?capabilities:\s*\{([^}]*)\}/gu)){
   const artifacts=/artifacts:\s*true/u.test(match[1]!),rowArtifacts=(row.capabilities as{artifacts?:boolean}|undefined)?.artifacts===true;
   if(artifacts!==rowArtifacts)problems.push(file+": declares artifacts="+artifacts+" but row "+row.id+" has "+rowArtifacts);
  }
 }
 expect(problems).toEqual([]);
});
