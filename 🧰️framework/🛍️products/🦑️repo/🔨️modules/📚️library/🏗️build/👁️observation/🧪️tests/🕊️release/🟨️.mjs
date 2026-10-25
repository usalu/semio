const api=await import(process.argv[2]);
const rows=Number(process.argv[3]);
await api.captureObservedSourceInputsV1(async()=>{for(let at=0;at<rows;at++)api.recordRegistryOwnerV1("/w","owner-"+at,{root:"r/"+at,manifest:"r/"+at+"/project.json"});return 1;},undefined,false);
const held=api.inferenceCacheSizesV1();
api.releaseInferenceMemoryV1();
console.log(JSON.stringify({held,released:api.inferenceCacheSizesV1()}));
