import policy from "../🔣️.json" with { type: "json" };

/** 🚀️ Names for every declared birth the bun child arguments that run one test file beneath that birth: the launcher table a route table hands the shared suite runner. @see ../🔣️.json */
export function genuineBirthLaunchersV1(script: string, root: string): Readonly<Record<string, (file: string) => readonly string[]>> {
  return Object.fromEntries(policy.births.map(birth => [birth.id, (file: string) => [script, "--file=" + file, "--birth=" + birth.id, "--root=" + root]]));
}
