/** ♻️ Contract: the refinement of a fact index merges rows with the same consumers into one class exactly like the naive per-row set model, with counting arrays instead of one Set per row (526,000 Sets were 4.5 GB of transient memory for one patch of the real index). */
import{expect,test}from"bun:test";
import{refineFactIndexV1}from"../../🟨️.mjs";

type Index={version:1;owners:string[];rows:{kind:string;path:string;fact:number}[];classes:{r:number[];o:number[]}[]};

/** 🧪️ The independent model: a Set of consumers per row, groups by the sorted consumer list in order of the first row. */
function naive(index:Index):Index{
 const consumers=index.rows.map(()=>new Set<number>()),groups=new Map<string,{r:number[];o:number[]}>(),classes:{r:number[];o:number[]}[]=[];
 for(const cls of index.classes)for(const owner of cls.o)for(const id of cls.r)consumers[id]!.add(owner);
 consumers.forEach((set,id)=>{const list=[...set].sort((a,b)=>a-b),key=list.join(",");let group=groups.get(key);if(!group){group={r:[],o:list};groups.set(key,group);classes.push(group);}group.r.push(id);});
 const covered=new Set(classes.flatMap(cls=>cls.o));
 for(const cls of index.classes)for(const owner of cls.o)if(!covered.has(owner)){covered.add(owner);classes.push({r:[],o:[owner]});}
 return{version:1,owners:index.owners,rows:index.rows,classes};
}

test("random indexes with duplicate consumers, unused rows and owners without rows refine exactly like the naive model",()=>{
 let seed=20261024;const next=(n:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%n;};
 for(let round=0;round<300;round++){
  const owners=Array.from({length:1+next(12)},(_,at)=>"o"+at),rows=Array.from({length:next(40)},(_,at)=>({kind:"file",path:"p"+at,fact:at}));
  const classes=Array.from({length:next(10)},()=>({r:rows.length?Array.from({length:next(6)},()=>next(rows.length)):[],o:Array.from({length:next(5)},()=>next(owners.length))}));
  const index:Index={version:1,owners,rows,classes};
  expect(refineFactIndexV1(index),"round "+round).toEqual(naive(index));
 }
});
