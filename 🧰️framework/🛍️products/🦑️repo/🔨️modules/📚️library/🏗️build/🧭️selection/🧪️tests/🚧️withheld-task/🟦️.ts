/** 🚧️ Contract: a task of a project the graph withholds is refused with the reason the project is withheld (the source that does not parse and its digest) in the language of the caller, English and German; a task of a project that does not exist is still an unknown task. */
import{expect,test}from"bun:test";
import{unknownBuildTaskV1}from"../../🟦️.ts";

const graph={projects:{},unavailable:[{root:"p/root",project:"p",source:"p/root/a.ts",sha256:"abcdef0123456789",reason:"Cannot find module"}]} as never;
const messages={en:{locale:"en",withheld:"is withheld: p/root/a.ts does not parse (sha256 abcdef012345)"},de:{locale:"de",withheld:"wird zurückgehalten: p/root/a.ts lässt sich nicht lesen (sha256 abcdef012345)"}};

for(const row of Object.values(messages))test("a withheld project names its source in "+row.locale,async()=>{
 const saved={...process.env};Object.assign(process.env,{SEMIO_LANGUAGE:row.locale,LANG:row.locale,LC_ALL:row.locale});
 try{const error=await unknownBuildTaskV1(graph,"p","p:build");expect(error.message).toContain(row.withheld);expect(error.message).not.toContain("Unknown build task");}
 finally{process.env=saved;}
});

test("a project nobody knows is an unknown task",async()=>{expect((await unknownBuildTaskV1(graph,"ghost","ghost:build")).message).toBe("Unknown build task: ghost:build");});
