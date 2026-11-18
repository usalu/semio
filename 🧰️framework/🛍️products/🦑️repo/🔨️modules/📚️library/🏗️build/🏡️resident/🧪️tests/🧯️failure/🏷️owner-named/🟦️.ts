/** 🏷️ Contract: the one-line message of an unexpected failure that belongs to an owner names the owner, so the refusal a remote machine reads says which owner failed and why without the service log. */
import{expect,test}from"bun:test";
import{failureMessageV1}from"../../../🏛️service/🧯️failure/🟨️.mjs";

test("the message names the owner of the failure",()=>{
 const error=Object.assign(new Error("Original project inference observations required"),{semioProject:"libs/alpha"});
 const text=failureMessageV1(error);
 expect(text).toContain("libs/alpha");expect(text).toContain("Original project inference observations required");expect(text.length).toBeLessThanOrEqual(300);
});

test("a failure without an owner reads as before",()=>{
 expect(failureMessageV1(new Error("a\n  b"))).toBe("Error: a b");
});
