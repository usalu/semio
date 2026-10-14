/** 🎚️ Declares explicit target workflow compiler mode selection. */
export function fixedTargetBuildMode(name:string):"prototype"|"release"|undefined;
/** 🧩️ Projects mode inheritance into an owned target declaration. */
export function targetBuildModes<T extends Record<string,unknown>>(name:string,target:T):T&{defaultConfiguration:"prototype"|"release";configurations:Record<string,unknown>;metadata:Record<string,unknown>};
