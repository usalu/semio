import{expect,test}from"bun:test";
import*as api from"../../🟦️.ts";
import{beginAdmissionWaitV1}from"../../🐕️watchdog/🟦️.ts";

const environment=(value:api.ScriptProcessEnvelope)=>({[api.SCRIPT_PROCESS_INVOCATION_ENV]:JSON.stringify(value)});

/** 🪆️ Pins that an in-process nested receive of the parent's own deadline projection (`scriptInvocationEnvironment`) reuses the live parent, while a foreign envelope stays refused.
 * @see ../../🟦️.ts */
test("a nested receive of the parent's projected deadline envelope, credited by an admission wait, reuses the live parent",async()=>{
 const parent=api.createScriptProcessEnvelope({version:1,owner:"fixture:parent",maximumElapsedMilliseconds:600000},{fixture:"parent"},Date.now());
 await api.withScriptProcessEnvelope(parent,async owner=>{
  const end=beginAdmissionWaitV1();await new Promise<void>(done=>setTimeout(done,50));end();
  const projected=api.scriptInvocationEnvironment(owner,{});
  expect(projected[api.SCRIPT_PROCESS_INVOCATION_ENV]).not.toBe(JSON.stringify(parent));
  expect(await api.receiveScriptProcessInvocation(projected,async nested=>nested===owner)).toBe(true);
  const foreign=api.createScriptProcessEnvelope({version:1,owner:"fixture:foreign",maximumElapsedMilliseconds:600000},{fixture:"foreign"},Date.now());
  await expect(api.receiveScriptProcessInvocation(environment(foreign),async()=>undefined)).rejects.toThrow("Nested Original process envelope must preserve its actual parent");
 });
});
