/** 🏷️ Contract: a failure of one owner's inference leaves the read with the owner it belongs to and the text of the original cause, whatever the failure is (an error that cannot be extended, a plain value), and tagging itself touches no observation. */
import{expect,test}from"bun:test";
import{tagProjectFailureV1}from"../../🟨️.mjs";

test("a failure carries its owner and stays the same error",async()=>{
 const error=new Error("boom");
 await expect(tagProjectFailureV1("libs/alpha",async()=>{throw error;})).rejects.toBe(error);
 expect((error as Error&{semioProject?:string}).semioProject).toBe("libs/alpha");
});

test("a failure that cannot be extended is wrapped with its owner, its code and the original text and cause",async()=>{
 const frozen=Object.freeze(Object.assign(new Error("Original project inference observations required"),{code:"BUILD_X"}));
 const got=await tagProjectFailureV1("libs/alpha",async()=>{throw frozen;}).catch(error=>error) as Error&{semioProject?:string;code?:string};
 expect(got).not.toBe(frozen);expect(got.semioProject).toBe("libs/alpha");expect(got.code).toBe("BUILD_X");
 expect(got.message).toContain("Original project inference observations required");expect(got.cause).toBe(frozen);
});

test("a thrown plain value becomes an error naming the owner and the value",async()=>{
 const got=await tagProjectFailureV1("libs/beta",async()=>{throw "plain";}).catch(error=>error) as Error&{semioProject?:string};
 expect(got).toBeInstanceOf(Error);expect(got.semioProject).toBe("libs/beta");expect(got.message).toContain("plain");
});

test("the tag reads no observation authority and a value passes through",async()=>{
 expect(await tagProjectFailureV1("libs/alpha",async()=>7)).toBe(7);
 const text=tagProjectFailureV1.toString();
 expect(text).not.toMatch(/observe|Observation|requireCurrent/u);
});
