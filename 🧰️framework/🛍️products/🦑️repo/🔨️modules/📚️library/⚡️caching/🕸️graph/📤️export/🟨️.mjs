/** 📤️ Shares exact native hash instructions across the task graph's debug projection. */
export function compactTaskPlans(plans) {
  const instructions = [...new Set(Object.values(plans).flat())].sort();
  if (instructions.some(instruction => typeof instruction !== "string")) throw Error("Native task plan instructions required");
  const indexes = new Map(instructions.map((instruction, index) => [instruction, index]));
  return { version: 1, instructions, tasks: Object.fromEntries(Object.entries(plans).sort(([left], [right]) => left.localeCompare(right)).map(([id, plan]) => [id, [...new Set(plan.map(instruction => indexes.get(instruction)))].sort((left, right) => left - right)])) };
}
