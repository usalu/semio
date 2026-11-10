import{refuseBuildV1}from"../../../../📖️texts/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import table from"./🔣️.json"with{type:"json"};

if(validateJsonSchemaSubset(schema,table).length)throw refuseBuildV1("invalid","closure.walk-views-required");

/** 🧠️ The declared guard of the reach sets. */
export const REACH_SETS_POLICY_V1=Object.freeze(structuredClone(table));

const hashOf=words=>{let hash=2166136261;for(let at=0;at<words.length;at++)hash=Math.imul(hash^words[at],16777619)>>>0;return hash;};
const sameWords=(left,right)=>{if(left.length!==right.length)return false;for(let at=0;at<left.length;at++)if(left[at]!==right[at])return false;return true;};
const trimmed=words=>{let end=words.length;while(end>0&&words[end-1]===0)end--;return end===words.length?words:words.slice(0,end);};

/**
 * 🧠️ Creates the store of reach sets: dense bitsets of 32 bit words over module numbers, equal sets one shared buffer (hash-consing), the union of sets a word-wise OR. `retain(live)` drops every set that no memo entry holds any more.
 * @param {number} [guardBytes]
 */
export function createReachSetsV1(guardBytes=REACH_SETS_POLICY_V1.guardBytes){
 let buckets=new Map(),bytes=0;
 const intern=raw=>{
  const words=trimmed(raw),hash=hashOf(words),held=buckets.get(hash);
  if(held!==undefined)for(const candidate of held)if(sameWords(candidate,words))return candidate;
  bytes+=words.byteLength+16;
  if(bytes>guardBytes)throw Object.assign(Error("Reach sets exceed their guard of "+guardBytes+" bytes"),{code:"BUILD_REACH_SETS_GUARD"});
  if(held===undefined)buckets.set(hash,[words]);else held.push(words);
  return words;
 };
 return Object.freeze({
  of(numbers){let top=-1;for(const number of numbers)if(number>top)top=number;const words=new Uint32Array((top>>5)+1);for(const number of numbers)words[number>>5]|=1<<(number&31);return intern(words);},
  union(sets){let length=0;for(const set of sets)if(set.length>length)length=set.length;const words=new Uint32Array(length);for(const set of sets)for(let at=0;at<set.length;at++)words[at]|=set[at];return intern(words);},
  has:(set,number)=>(number>>5)<set.length&&(set[number>>5]>>>(number&31)&1)===1,
  *members(set){for(let at=0;at<set.length;at++){let word=set[at];while(word!==0){const bit=31-Math.clz32(word&-word);yield at*32+bit;word&=word-1;}}},
  retain(live){const next=new Map();bytes=0;for(const set of live){const hash=hashOf(set),held=next.get(hash);if(held===undefined){next.set(hash,[set]);bytes+=set.byteLength+16;}else if(!held.includes(set)){held.push(set);bytes+=set.byteLength+16;}}buckets=next;},
  stats:()=>({sets:[...buckets.values()].reduce((sum,list)=>sum+list.length,0),bytes,guardBytes}),
 });
}
