import {expect,test} from "bun:test";
import cases from "./control-cases.json" with {type:"json"};
import {parseIfc4x3Step,writeIfc4x3Step,writeIfc4x3StepChunks} from "../step.ts";

const name="FILE_NAME('a','',(''),(''),'','','');",description="FILE_DESCRIPTION(('test'),'2;1');",schema="FILE_SCHEMA(('IFC4X3_ADD2'));";
const envelope=(data="")=>`ISO-10303-21;HEADER;${description}${name}${schema}ENDSEC;DATA;${data}ENDSEC;END-ISO-10303-21;`;

test("neutral mandatory header forms and order are admitted before publication",async()=>{
 for(const row of cases.headers){let source=envelope();if(row.replace)source=source.replace(name,row.replace);if(row.description)source=source.replace(description,row.description);if(row.schema)source=source.replace(schema,row.schema);await expect(parseIfc4x3Step(source)).rejects.toHaveProperty("code",row.code);}
 await expect(parseIfc4x3Step(envelope().replace(description+name,name+description))).rejects.toHaveProperty("code","step.header");
 const base=await parseIfc4x3Step(envelope());base.header.fileName=[];await expect(writeIfc4x3Step(base)).rejects.toHaveProperty("code","step.header");
});
test("owned export isolates caller changes after capture",async()=>{
 const base=await parseIfc4x3Step(envelope("#1=IFCLABEL('before');")),expected=structuredClone(base);let changed=false;const output=await writeIfc4x3Step(base,{progress:progress=>{if(progress.phase==="write"&&!changed){changed=true;base.entities[0]!.args[0]={kind:"string",value:"after"};base.entities.push({id:2n,name:"IFCLABEL",args:[],complex:[]});base.header.fileName=[];}}});expect(changed).toBe(true);expect(await parseIfc4x3Step(output)).toEqual(expected);
});
test("one large aggregate streams bounded chunks and preserves exact graph",async()=>{
 const base=await parseIfc4x3Step(envelope(`#1=IFCLABEL((${Array.from({length:cases.largeItems},(_,index)=>String(index)).join(",")}));`));let count=0,output="";for await(const chunk of writeIfc4x3StepChunks(base)){expect(chunk.length).toBeLessThanOrEqual(cases.chunkCodeUnits);output+=chunk;count++;}expect(count).toBeGreaterThan(10);expect(await parseIfc4x3Step(output)).toEqual(base);
});
test.each(cases.cancelPhases)("cancellation reaches interior %s work",async phase=>{
 const abort=new AbortController();let seen=false;const control={signal:abort.signal,batchSize:1,progress:(progress:{phase:string;completed:number;total:number})=>{if(progress.phase===phase&&progress.completed>4096){seen=true;abort.abort();}}};
 if(phase==="write-text"){const base=await parseIfc4x3Step(envelope(`#1=IFCLABEL('${"x".repeat(32768)}');`));await expect(writeIfc4x3Step(base,control)).rejects.toHaveProperty("code","io.cancelled");}
 else {const data=phase==="parse-trivia"?`/*${" ".repeat(32768)}*/#1=IFCLABEL();`:phase==="parse-unicode"?`#1=IFCLABEL('\\X2\\${"00E4".repeat(16384)}\\X0\\');`:`#1=IFCLABEL("0${"AB".repeat(16384)}");`;await expect(parseIfc4x3Step(envelope(data),control)).rejects.toHaveProperty("code","io.cancelled");}expect(seen).toBe(true);
});
test("captured controls retain the original signal when caller code replaces its options",async()=>{
 const original=new AbortController(),replacement=new AbortController();let seen=false;const control={signal:original.signal,progress:(progress:{phase:string})=>{if(progress.phase==="parse-unicode"){seen=true;control.signal=replacement.signal;original.abort();}}};await expect(parseIfc4x3Step(envelope(`#1=IFCLABEL('\\X2\\${"00E4".repeat(4096)}\\X0\\');`),control)).rejects.toHaveProperty("code","io.cancelled");expect(seen).toBe(true);
});
test("depth admission agrees for parsed values, owned graph and stream encoding",async()=>{
 const base=await parseIfc4x3Step(envelope("#1=IFCA((1),IFCLABEL('x'));"),{maxDepth:1});expect(await parseIfc4x3Step(await writeIfc4x3Step(base,{maxDepth:1}),{maxDepth:1})).toEqual(base);await expect(parseIfc4x3Step(envelope("#1=IFCA((IFCLABEL('x')));"),{maxDepth:1})).rejects.toHaveProperty("code","step.depth");
 for(const control of [{batchSize:NaN},{maxDepth:257},{maxNumericCodeUnits:16385}])await expect(parseIfc4x3Step(envelope(),control)).rejects.toHaveProperty("code","io.control");await expect(parseIfc4x3Step(envelope(`#1=IFCA(${'1'.repeat(4097)}.);`))).rejects.toHaveProperty("code","step.numericLimit");
 await expect(parseIfc4x3Step(envelope("#1=IFCA(1E5);"))).rejects.toHaveProperty("code","step.number");
});
test("stream preflight refuses malformed Unicode before returning a first chunk",async()=>{
 const base=await parseIfc4x3Step(envelope("#1=IFCLABEL('x');"));base.entities[0]!.args[0]={kind:"string",value:"\uD800"};let chunks=0;const encode=async()=>{for await(const chunk of writeIfc4x3StepChunks(base))chunks+=chunk.length;};await expect(encode()).rejects.toHaveProperty("code","step.string");expect(chunks).toBe(0);
});
test("neutral Part21 ignored characters and print directives retain semantic text",async()=>{
 for(const row of cases.lexical){const graph=await parseIfc4x3Step(envelope(row.data));expect(graph.entities[0]!.name).toBe(row.nameAfter);expect(graph.entities[0]!.args[0]).toEqual(row.binary?{kind:"binary",value:row.binary}:{kind:"string",value:row.value!});expect(await parseIfc4x3Step(await writeIfc4x3Step(graph))).toEqual(graph);}
 const damaged=envelope("#1=IFCLABEL(\r\n?\t);");try{await parseIfc4x3Step(damaged);throw new Error("refusal expected");}catch(error){expect(error).toHaveProperty("offset",damaged.indexOf("?"));}
 await expect(parseIfc4x3Step(envelope("#1=IFCLABEL('\uD800\n\uDC00');"))).rejects.toHaveProperty("code","step.string");
});
