/** 👷️ The child of the model: answers one verdict per line it reads, like a farm child that holds no custody and verifies the stored record it is asked about. @see ../🟦️.ts */
const observation=await import("../../../🟨️.mjs") as any;
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{},verify:async()=>true});
for await(const line of console){
 if(line.trim()==="")continue;
 const request=JSON.parse(line);
 const said=await observation.verifyStoredProjectRecordV1(request);
 process.stdout.write(JSON.stringify(said)+"\n");
}
