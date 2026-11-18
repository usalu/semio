/** 👷️ The farm child of the model: resolves the closure references of the owner it is asked about and files the result like the observe job, answers one reference per line. @see ../🟦️.ts */
const B=process.env.R121_B!;
const inference=await import(B+"/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const SI=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any,OB=await import(B+"/👁️observation/🟨️.mjs") as any,PN=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs") as any;
for await(const line of console){
 if(line.trim()==="")continue;
 const{root,store,dir,job,headerDigest}=JSON.parse(line);
 SI.bindClosureStoreV1(root,store,false);
 const scripts={...SI.createScriptInputCache(),lateClosures:true};
 const refs=await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(()=>PN.resolveOwnerRefsV1({workspaceRoot:root,scripts},job,headerDigest)),{verdicts:dir});
 process.stdout.write(JSON.stringify({refs:refs??null,pid:process.pid})+"\n");
}
