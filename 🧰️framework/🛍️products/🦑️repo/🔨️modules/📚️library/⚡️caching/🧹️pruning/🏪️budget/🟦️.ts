import schema from "./🧬️schema/🔣️.json" with {type:"json"};
export{retireInactiveCargoStoreV1,CARGO_RETIREMENT_POLICY_V1,planInactiveCargoRetirementV1,type CargoRetirementPolicyV1,type CargoRetirementPlanV1,type CargoRetirementResultV1}from"./⚙️execution/🟦️.ts";

/** 🎛️ Closes maintenance dispatch to authored operations without caller-selected paths or limits. */
export function readInactiveCargoRetirementOperationV1(args:readonly string[]):"preview"|"retire"|"preview-historical"|"retire-historical"{if(args.length!==1||!schema.definitions.Operation.enum.includes(args[0]!))throw Error("Expected cache-retire-inactive preview|retire|preview-historical|retire-historical");return args[0] as "preview"|"retire"|"preview-historical"|"retire-historical";}
