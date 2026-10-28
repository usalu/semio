import assert from'node:assert/strict';
import{OriginalPostRead,type OriginalPostIssuer,type OriginalPostAuthority,type PostDemand,type PostGrant,type PostProgress,type OriginalPostSlot}from'../🟦️.ts';
import type{PostCustodyOracle}from'./oracle.mjs';
import fixture from'./🔣️.json';
const zero=():PostProgress=>({items:0,copyBytes:0,capacityBytes:0,releaseBytes:0});
export function checkOriginalPostCustody(oracle:PostCustodyOracle){let turns=0,denials=0,aliases=0,retired=0;
 for(const row of fixture.cases){const original=row.value,expected=oracle.snapshot(original);let aliasCount=0,admissions=0;const identity={};
  class Source implements OriginalPostAuthority<typeof original>{private owner:typeof original|undefined=original;private lease=true;
   borrow(){assert(this.owner!==undefined&&this.lease);return{value:this.owner,identity,projection:0};}
   takeDemand():PostDemand{if(aliasCount>0)throw Error('Original post aliases remain');return this.lease?{copyBytes:8,capacityBytes:0,releaseBytes:8,depth:1}:{copyBytes:16,capacityBytes:0,releaseBytes:32,depth:1};}
   take(_grant:PostGrant){this.takeDemand();if(this.lease){this.lease=false;return{kind:'pending' as const,progress:{items:1,copyBytes:8,capacityBytes:0,releaseBytes:8}};}const value=this.owner!;this.owner=undefined;return{kind:'ready' as const,value,progress:{items:1,copyBytes:16,capacityBytes:0,releaseBytes:32}};}
   terminalIsEmpty(){return this.owner===undefined;}
  }
  const issuer:OriginalPostIssuer<typeof original>={openingDemand(value){assert.equal(value,original);return{copyBytes:16,capacityBytes:40,releaseBytes:0,depth:1};},admit(value,_grant){assert.equal(value,original);admissions++;return{source:new Source(),progress:{items:1,copyBytes:16,capacityBytes:40,releaseBytes:0}};}};
  const owner=new OriginalPostRead(issuer),post:OriginalPostSlot<typeof original>={value:original};
  const prepare=(mode:'original'|'mutable')=>{for(let step=0;step<32;step++){const d=owner.demand(post,mode,4096);if(d===undefined)return;const grant={items:1,...d};const denied=[{...grant,items:0},...(['copyBytes','capacityBytes','releaseBytes','depth']as const).filter(key=>d[key]>0).map(key=>({...grant,[key]:d[key]-1}))];for(const cut of denied){const beforeAdmissions=admissions,beforeValue=post.value;assert.deepEqual(owner.prepare(post,mode,4096,cut),zero());assert.equal(admissions,beforeAdmissions);assert.equal(post.value,beforeValue);assert.deepEqual(owner.demand(post,mode,4096),d);denials++;}const progress=owner.prepare(post,mode,4096,grant)!;for(const key of ['copyBytes','capacityBytes','releaseBytes']as const)assert(progress[key]<=grant[key]);assert(progress.items>0);turns++;}throw Error('Original post custody stalled');};
  if(row.reads>0){prepare('original');for(let i=0;i<row.reads;i++){const ref=owner.original(post);assert.equal(ref.value,original);assert.equal(ref.identity,identity);aliasCount++;aliases++;}assert.throws(()=>owner.mutable(post));assert.throws(()=>owner.demand(post,'mutable',4096));while(aliasCount>0)aliasCount--;}
  prepare('mutable');assert.equal(owner.mutable(post),original);assert(owner.terminalIsEmpty());assert(oracle.equal(original,expected));oracle.retire(post.value,original);retired++;assert.equal(admissions,row.reads>0?1:0);
 }
 return{cases:fixture.cases.length,turns,denials,aliases,retired};
}
