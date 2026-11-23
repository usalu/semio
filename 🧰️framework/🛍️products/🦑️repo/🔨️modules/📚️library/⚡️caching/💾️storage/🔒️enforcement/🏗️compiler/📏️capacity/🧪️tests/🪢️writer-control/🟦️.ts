import{test,expect}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createIssuedResourceControlV1,withRequestedIssuedResourceControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import{requireCompilerWriterCustodyV1,withCompilerSharedHeadroomV1}from"../../🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-compiler-processes"});

/** 🪢️ The headroom recovery of a pressed compile runs under the control of the live writer custody it escalated into. A control captured above a deadline (the store's reservation control) is no refinement of that custody: the custody's control carries the deadline in between, and the retained lifetime refuses with "Actual inherited control restriction required". Only an image that is short of headroom reaches this path, so a roomy image never showed it. */
test("the headroom recovery runs under the live writer's own control and a control captured above a deadline is refused",()=>withBuildTestInvocationV1(async original=>{
 const outer=createIssuedResourceControlV1(original,"rust:writer-control",280000),directory=mkdtempSync(join(tmpdir(),"writer-control-")),leases=join(directory,"leases"),root=join(directory,"image");
 try{
  await withRequestedIssuedResourceControlV1(outer,200000,deadline=>withRequestedIssuedResourceControlV1(deadline,0,async captured=>{
   await Bun.write(join(leases,".keep"),"");await Bun.write(join(root,".keep"),"");
   const image={root,maximumPhysicalBytes:2**30,recheck:async()=>{}},steps:string[]=[];let recovered=false;
   await withRequestedIssuedResourceControlV1(captured,5000,async restricted=>{
    await withCompilerSharedHeadroomV1(image,leases,restricted,1024,{version:1,wait:false},async()=>{steps.push("body");},async actual=>{
     expect(()=>requireCompilerWriterCustodyV1(root,captured)).toThrow("Actual inherited control restriction required");
     requireCompilerWriterCustodyV1(root,actual);recovered=true;steps.push("recover");
    },async()=>recovered?1<<20:0);
   });
   expect(steps).toEqual(["recover","body"]);
  }));
 }finally{rmSync(directory,{recursive:true,force:true});}
}),120000);
