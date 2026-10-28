/** 🔤 Decode a STEP PAGE payload using the pinned owned ISO 8859 alphabet. */
import data from "./data.json" with {type:"json"};

for(const page of data.pages){Object.freeze(page.scalars);Object.freeze(page);}Object.freeze(data.pages);Object.freeze(data);

/** 🗺️ Undefined pages, payloads and scalar positions return no character. */
export function part21AlphabetCharacter(selector:string,payload:number):string|undefined {
 const page=selector.length===1?selector.charCodeAt(0)-65:-1;if(page<0||page>=9||!Number.isInteger(payload)||payload<32||payload>126)return;
 const scalar=data.pages[page]!.scalars[payload-32];return scalar===null||scalar===undefined?undefined:String.fromCodePoint(scalar);
}
