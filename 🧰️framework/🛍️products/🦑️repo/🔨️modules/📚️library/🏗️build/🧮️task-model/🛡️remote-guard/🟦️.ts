import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🛡️ The authored remote-cache guard rows.
 * @see ./🧬️schema/🔣️.json */
export const REMOTE_GUARD_POLICY_V1=schema.definitions.Policy.const;
const rows=REMOTE_GUARD_POLICY_V1.rows;

/** 🔎️ The stated reason why a target (by its name, any emoji prefix ignored) must not use the remote cache, or `undefined`. */
export function remoteGuardV1(targetName:string):Readonly<{en:string;de:string}>|undefined{
 const bare=targetName.replace(/^[^A-Za-z0-9]+/u,"");
 return rows.find(row=>(row.targets as readonly string[]).includes(bare))?.reason;
}

/** 🧬️ The extra Nx input that makes the hash of a guarded target unique to its machine, none for every other target. */
export function remoteGuardInputsV1(targetName:string):readonly {runtime:string}[]{
 return remoteGuardV1(targetName)?[{runtime:REMOTE_GUARD_POLICY_V1.machineRuntime}]:[];
}
