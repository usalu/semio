import{expect,test}from"bun:test";
import{graphPublicationWithWorkspaceRootV1}from"../../🟦️.ts";

const publication=(environment:Record<string,string|undefined>)=>({directory:"/d",toolingIdentity:"0".repeat(64),files:[],environment,generationHolds:()=>true}) as never;

/** 🧭️ Contract: a forwarded publication always names its workspace root, from the client environment when it carries one and from the authenticated caller otherwise, and never alters the original. */
test("a forwarded publication without NX_WORKSPACE_ROOT_PATH takes the authenticated caller's repository root",()=>{
 const bare=publication({SEMIO_NX_GRAPH_STORAGE:"/s"}),named=publication({NX_WORKSPACE_ROOT_PATH:"/client"});
 const filled=graphPublicationWithWorkspaceRootV1(bare,"/repo") as unknown as{environment:Record<string,string>};
 expect(filled.environment).toEqual({SEMIO_NX_GRAPH_STORAGE:"/s",NX_WORKSPACE_ROOT_PATH:"/repo"});expect(Object.isFrozen(filled.environment)).toBe(true);
 expect((bare as unknown as{environment:Record<string,string>}).environment.NX_WORKSPACE_ROOT_PATH).toBeUndefined();
 expect(graphPublicationWithWorkspaceRootV1(named,"/repo")).toBe(named);
});
