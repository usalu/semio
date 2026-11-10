/** 🪆️ Contract: a derived question that needs a TypeScript program or a derived walk is asked in a child process, one at a time, and never in the process that is already that child; the policy is closed under its schema; a child that cannot answer fails with a named code instead of hanging. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import policy from"../🔣️.json";
import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,isHeavyQuestionV1}from"../🟨️.mjs";

test("the policy follows its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(schema,policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(HEAVY_QUESTION_POLICY_V1).toEqual(policy as never);
});

test("programs and derived walks are heavy, command walks are not, and the child itself never routes again",()=>{
 const row=(question:unknown)=>({kind:"derived",path:JSON.stringify(question),fact:"x"});
 expect(isHeavyQuestionV1(row({program:1,workspaceRoot:"/w",projectRoot:"p",projects:[]}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({derive:1,workspaceRoot:"/w",recipe:{}}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({walk:1,roots:[],resources:[],selectors:[]}),{})).toBe(false);
 expect(isHeavyQuestionV1({path:"not json"},{})).toBe(false);
 expect(isHeavyQuestionV1(row({program:1}),{[HEAVY_QUESTION_POLICY_V1.environment]:"2560"})).toBe(false);
});

test("a child that cannot answer fails with the named code, and questions are asked one at a time",async()=>{
 const row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:"/nonexistent-semio-root",recipe:{fn:"none"}}),fact:"x"};
 const outcomes=await Promise.allSettled([askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"}),askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"})]);
 for(const outcome of outcomes){expect(outcome.status).toBe("rejected");expect((outcome as PromiseRejectedResult).reason.code).toBe("BUILD_HEAVY_QUESTION");}
},120000);
