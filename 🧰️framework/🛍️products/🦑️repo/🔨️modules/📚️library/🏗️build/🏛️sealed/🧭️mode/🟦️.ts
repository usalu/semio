import policy from"../🔣️.json"with{type:"json"};

/** 🧭️ Tells the front-door script, before it loads anything else, whether this start runs the live tree: it reads the policy table only, so a live start never loads the sealed store, its trampoline or the promoter; every other value, a malformed one included, is answered by the trampoline, which owns the refusal text. A law holds it to the verdict of `implementationModeV1`.
 * @see ../🟦️.ts the policy and the mode rule, ../🚦️trampoline/🟦️.ts the start of a sealed run */
export function liveImplementationStartV1(environment:Readonly<Record<string,string|undefined>>):boolean{
 const raw=environment[policy.modeVariable];
 return raw===undefined||raw===""?policy.defaultMode==="live":raw.toLowerCase()==="live";
}
