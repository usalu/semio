import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

/** 🤝️ A read of a selected scope shares its owner records with the fleet like a full read: the fleet port (look, claim, publish) is opened for it too, and only the children and pools that cannot work under an admission stay off. */
const text=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
const portStart=text.indexOf("let fleetPort;"),portEnd=text.indexOf("useProjectFleetV1(fleetPort);",portStart);

test("the fleet port of the owner loop is opened whatever the admission of the read",()=>{
 expect(portStart).toBeGreaterThan(0);expect(portEnd).toBeGreaterThan(portStart);
 const block=text.slice(portStart,portEnd);
 expect(block).toContain("createProjectFleetPortV1(");
 expect(block).not.toContain("ownerAdmission");
 expect(block).toContain("takeScope={run:records.runTakeLookV1}");
});

test("the farm stays off for an admission, the owner pool serves it: the wave carries the admission of every owner",()=>{
 expect(text).toContain("const farm=timedFarmV1(options?.ownerAdmission?undefined:openProjectFarmV1(");
 expect(text).toContain("let ownerPool;{try{");
 expect(text).toContain("...options?.ownerAdmission?{admission:ownerAdmissionOfV1(options.ownerAdmission,job)}:{}");
});
