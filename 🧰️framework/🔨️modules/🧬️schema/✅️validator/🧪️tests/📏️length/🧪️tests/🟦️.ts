import{test,expect}from"bun:test";
import{schemaStringLawsV1}from"../🟦️.ts";
for(const law of schemaStringLawsV1())test(law.name,()=>{expect(law.prove()).toBeGreaterThan(0);});
