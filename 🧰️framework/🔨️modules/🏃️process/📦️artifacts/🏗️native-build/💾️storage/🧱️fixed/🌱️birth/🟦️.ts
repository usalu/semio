import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🌱️ Admits an interrupted creation envelope without deriving authority from its payload. */
export function nativeFixedIncompleteBirthV1(entries:readonly string[],attached:boolean):boolean{const expected=schema.definitions.IncompleteBirthRetention.const.entries;return !attached&&entries.length===expected.length&&entries.every((name,index)=>name===expected[index]);}
