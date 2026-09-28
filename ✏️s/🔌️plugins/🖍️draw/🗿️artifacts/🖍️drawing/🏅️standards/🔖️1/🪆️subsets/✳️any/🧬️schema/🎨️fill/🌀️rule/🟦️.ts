/** 🌀️ The authored rule for determining the inside of a compound path. */
export type FillRule = "evenodd" | "nonzero";
export function parseFillRule(value:unknown):FillRule {
  if(value!=="evenodd" && value!=="nonzero")throw new Error("Invalid fill rule");
  return value;
}
