/** ⏲️ The limits of the build command runtime, read from the sibling schema document (`🔣️.json`, definition `Limits`). */
import schema from "./🔣️.json" with {type: "json"};

/** 🧱️ The custody session limits and the stop grace of the build command runtime; the schema document is their single source. */
export const COMMAND_RUNTIME_LIMITS_V1 = Object.freeze({...schema.definitions.Limits.const});
