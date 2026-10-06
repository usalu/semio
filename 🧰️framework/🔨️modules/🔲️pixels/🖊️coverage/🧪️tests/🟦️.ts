/** 🧫️ Exact polygon coverage checked against shared vectors and independent SVG alpha. */
import {expect,test,spyOn} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import polygonClipping from "polygon-clipping";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import retirementCases from "../🧫️fixtures/🧹️retirement/🔣️.json";
import {CoverageJob,polygonCoverage,type CoverageInput} from "../🟦️.ts";

const validate=new Ajv({strict:true}).compile(schema);
function complete(input:CoverageInput,budget:number) {
  const job=new CoverageJob(input);let previous=0;
  for(let step=0;step<100000;step++) {
    const progress=job.advance(budget);
    expect(progress.work-previous).toBeLessThanOrEqual(budget);previous=progress.work;
    if(progress.done) return job.result();
  }
  throw Error("Coverage job did not terminate");
}
async function svgCoverage(input:CoverageInput):Promise<number[]> {
  let contours=input.contours,transform=input.transform;
  if(contours.length===1&&input.rule==="nonzero"&&contours[0]!.some(p=>p.some(value=>Math.abs(value)>100000))) {
    const m=transform,ring=contours[0]!.map(p=>[m[0]!*p[0]!+m[2]!*p[1]!+m[4]!,m[1]!*p[0]!+m[3]!*p[1]!+m[5]!] as [number,number]);
    contours=polygonClipping.intersection([ring],[[[0,0],[input.width,0],[input.width,input.height],[0,input.height]]]).flat();transform=[1,0,0,1,0,0];
  }
  const d=contours.map(points=>points.length?`M${points.map(point=>point.join(",")).join("L")}Z`:"").join("");
  const scale=128,svg=`<svg xmlns="http://www.w3.org/2000/svg" width="${input.width*scale}" height="${input.height*scale}" viewBox="0 0 ${input.width} ${input.height}"><path transform="matrix(${transform.join(" ")})" d="${d}" fill="red" fill-rule="${input.rule}"/></svg>`;
  const pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
  const coverage:number[]=[];
  for(let at=0;at<input.width*input.height;at++) {
    const x=at%input.width,y=Math.floor(at/input.width);let alpha=0;
    for(let dy=0;dy<scale;dy++)for(let dx=0;dx<scale;dx++)alpha+=pixels[((y*scale+dy)*input.width*scale+x*scale+dx)*4+3]!;
    coverage.push(Math.round(alpha/(scale*scale)));
  }
  return coverage;
}
for(const row of fixture)test(row.name,async()=>{
  expect(validate(row.input)).toBe(true);
  const input=row.input as CoverageInput,before=structuredClone(input);
  for(const budget of [1,7,4096])expect([...complete(input,budget).coverage]).toEqual(row.expected);
  expect(input).toEqual(before);
  const reference=await svgCoverage(input);
  for(let at=0;at<row.expected.length;at++)expect(Math.abs(row.expected[at]!-reference[at]!)).toBeLessThanOrEqual(2);
});
test("seeded crossing contours and affine transforms match independent SVG area",async()=>{
  let seed=0xabcde;
  const random=()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
  for(let sample=0;sample<96;sample++) {
    const input:CoverageInput={width:3,height:3,rule:sample%2?"evenodd":"nonzero",transform:[.75+random()/2,random()-.5,random()-.5,.75+random()/2,random()-.5,random()-.5],contours:Array.from({length:1+sample%3},()=>Array.from({length:3+Math.floor(random()*5)},()=>[random()*4-.5,random()*4-.5]))};
    const actual=complete(input,sample%2?1:4096).coverage,reference=await svgCoverage(input);
    const delta=Math.max(...actual.map((value,at)=>Math.abs(value-reference[at]!)));
    if(delta>2)throw Error(JSON.stringify({sample,input,actual:[...actual],reference,delta}));
    expect(delta).toBeLessThanOrEqual(2);
  }
});
test("coverage preparation and sweep can be cancelled without publishing a partial mask",()=>{
  for(const steps of [0,1,20,60]) {
    const job=new CoverageJob(fixture[3]!.input as CoverageInput);
    expect(()=>job.result()).toThrow();
    for(let step=0;step<steps;step++)job.advance(1);
    job.cancel();expect(()=>job.advance(1)).toThrow(/cancelled/);expect(()=>job.result()).toThrow(/cancelled/);
  }
});
test("coverage refuses malformed geometry, dimensions, rules and work grants",()=>{
  const input=fixture[0]!.input as CoverageInput;
  for(const patch of [{width:0},{width:16384,height:16384},{rule:"inverse"},{transform:[1,0,0,1,Infinity,0]}])expect(()=>new CoverageJob({...input,...patch} as CoverageInput)).toThrow();
  const job=new CoverageJob({...input,contours:[[[NaN,0],[1,0],[1,1]]]});
  expect(()=>job.advance(100)).toThrow();
  for(const grant of [0,-1,NaN,Infinity,1.5])expect(()=>new CoverageJob(input).advance(grant)).toThrow();
});
test("async coverage yields, reports progress and responds to cancellation",async()=>{
  const input=fixture[3]!.input as CoverageInput,events:string[]=[];
  const timer=setTimeout(()=>events.push("timer"),0);
  const mask=await polygonCoverage(input,{workBudget:7,onProgress:progress=>events.push(progress.phase)});
  clearTimeout(timer);expect([...mask.coverage]).toEqual(fixture[3]!.expected);
  expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
  const controller=new AbortController();
  await expect(polygonCoverage(input,{signal:controller.signal,workBudget:1,onProgress:()=>controller.abort()})).rejects.toThrow(/cancelled/);
  let observed=false;await expect(polygonCoverage(input,{signal:controller.signal,onProgress:()=>{observed=true;}})).rejects.toThrow(/cancelled/);expect(observed).toBe(false);
});
test("cancellation leaves already published coverage and caller geometry intact",()=>{
  const input=fixture[0]!.input as CoverageInput,before=structuredClone(input),job=new CoverageJob(input);
  while(!job.advance(4096).done){}const mask=job.result();job.cancel();
  expect([...mask.coverage]).toEqual(fixture[0]!.expected);expect(input).toEqual(before);
});

test("coverage retirement transfers actual masks and drains private sweep owners",async()=>{
 const progressOracle=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/retirementProgress"});
 const stages:Record<string,string>={preparing:"prepare",cancelled:"events"};
 for(const row of retirementCases)for(const grant of [1,7,4096]){
  const source=fixture.find(c=>c.name===row.source)!;expect(source).toBeDefined();const input=structuredClone(source.input) as CoverageInput;if(row.stop==="failure")(input.contours[0]![0]! as number[])[0]=1000000001;
  const before=structuredClone(input),job=new CoverageJob(input),state=job as any;
  if(row.stop==="failure")expect(()=>job.advance(4096)).toThrow(/point/);
  else if(row.stop==="preparing")job.advance(1);
  else if(row.stop!=="fresh") {const stop=stages[row.stop]??row.stop;for(let at=0;at<200000;at++){if(state.stage===stop)break;job.advance(1);}expect(state.stage).toBe(stop);}
  const published=row.publishedBeforeCancel?job.result():null;
  if(row.stop==="cancelled"||row.cancelBeforeTransfer||published)job.cancel();
  const mask=row.output?job.result():null,expected=14+state.edges.length+state.events.length;expect(expected).toBe(row.work.typescript);
  const retired=(job as any).intoRetirement();expect(retired.output).toBe(mask);expect(()=>job.advance(1)).toThrow(/cancel/i);expect(()=>job.result()).toThrow(/cancel/i);expect(()=>(job as any).intoRetirement()).toThrow(/transferred/i);
  for(const bad of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>retired.job.advance(bad)).toThrow(/grant/);
  let work=0;
  for(let at=0;at<300000;at++){const owners=state.edges.length+state.events.length,p=retired.job.advance(grant);expect(progressOracle(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);expect(owners-state.edges.length-state.events.length).toBeLessThanOrEqual(grant);work=p.work;if(p.done)break;}
  expect(work).toBe(row.work.typescript);expect(retired.job.terminalIsEmpty()).toBe(true);expect(retired.job.advance(1)).toEqual({phase:"complete",work,done:true});expect(state.input.contours).toEqual([]);expect(state.input.transform).toEqual([]);
  for(const key of ["edges","events","positions","active","eventIds","sorted","copied"])expect(state[key]).toEqual([]);expect(state.areas.length).toBe(0);expect(state.mask.coverage.length).toBe(0);expect(state.sorter).toBeNull();expect(state.first).toBeNull();expect(state.previous).toBeNull();expect(input).toEqual(before);
  const output=retired.output??published;
  if(output){expect([...output.coverage]).toEqual(source.expected);const reference=await svgCoverage(input);for(let at=0;at<reference.length;at++)expect(Math.abs(output.coverage[at]-reference[at]!)).toBeLessThanOrEqual(2);}
 }
 process.stderr.write("[DEBUG] Actual coverage retirement: eighteen interruption cases at grants 1/7/4096; transferred/published masks match independent SVG\n");
});

test("async coverage retires unpublished completed masks on final abort and callback failure",async()=>{
 const input=fixture.find(row=>row.name==="integer rectangle")!.input as CoverageInput,original=CoverageJob.prototype.intoRetirement;
 for(const failure of ["abort","callback"]){const controller=new AbortController(),error=new Error("injected coverage callback failure"),observed:{owner:ReturnType<CoverageJob["intoRetirement"]>;pixels:number;work:number;maximumGrant:number}[]=[];
  const spy=spyOn(CoverageJob.prototype,"intoRetirement").mockImplementation(function(this:CoverageJob){const pixels=(this as any).mask.coverage.length,owner=original.call(this),record={owner,pixels,work:0,maximumGrant:0};observed.push(record);const advance=owner.job.advance.bind(owner.job);owner.job.advance=grant=>{const progress=advance(grant);record.work=progress.work;record.maximumGrant=Math.max(record.maximumGrant,grant);return progress;};return owner;});
  try{const result=polygonCoverage(input,{signal:controller.signal,workBudget:1,onProgress:progress=>{if(progress.done){if(failure==="abort")controller.abort();else throw error;}}});if(failure==="abort")await expect(result).rejects.toThrow(/cancelled/i);else await expect(result).rejects.toBe(error);}finally{spy.mockRestore();}
  expect(observed).toHaveLength(1);const record=observed[0]!;expect(record.pixels).toBe(input.width*input.height);expect(record.owner.output).toBeNull();expect(record.owner.job.terminalIsEmpty()).toBe(true);expect(record.work).toBe(20);expect(record.maximumGrant).toBe(1);process.stderr.write(`[DEBUG] Actual async coverage ${failure} after completion retired ${record.pixels} unpublished mask bytes through ${record.work} one-unit structural cleanup steps\n`);
 }
});
