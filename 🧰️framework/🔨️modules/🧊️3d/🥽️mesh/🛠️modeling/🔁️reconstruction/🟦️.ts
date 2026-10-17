/** 🔑️ Quantizes the retained producer's f32 grid with Rust's signed half-away rounding. */
export function meshWeldKeyV1(point:readonly number[],precision:number):readonly[string,string,string] {
 const actual=Math.fround(precision);if(point.length!==3||!Number.isFinite(actual)||actual<=0)throw Error("Positive finite precision and three coordinates required");
 const scale=Math.fround(1/Math.max(actual,Math.fround(1e-9)));
 const keys=point.map(value=>{const scaled=Math.fround(value)*scale,rounded=scaled<0?-Math.floor(-scaled+0.5):Math.floor(scaled+0.5);if(!Number.isFinite(rounded)||rounded< -(2**63)||rounded>=2**63)throw Error("Weld coordinate exceeds precision grid");return BigInt(rounded).toString();});
 const[a,b,c]=keys;if(a===undefined||b===undefined||c===undefined)throw Error("Three keys required");return Object.freeze([a,b,c]);
}
