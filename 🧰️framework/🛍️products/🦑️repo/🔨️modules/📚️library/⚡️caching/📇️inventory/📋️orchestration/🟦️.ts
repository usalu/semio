import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { inventory } from "../🧮️composition/🟦️.ts";
import { ticketOutput } from "../../🎫️output/🟦️.ts";


export class AuditScript extends BundleScript {
  run(args: string[]): void {
    const output = ticketOutput(this.repoRoot, args);
    const result = inventory(this.repoRoot);
    for (const [name, value] of Object.entries(result)) writeFileSync(join(output, `${name}.json`), JSON.stringify(value, null, 2) + "\n");
    writeFileSync(join(dirname(dirname(output)), "📓️nx-inventory.md"), `# Nx Inventory\n\n${result.projects.length} projects from every native Nx provider; ${result.commands.length} commands; ${result.artifacts.length} declared artifacts; ${result.violations.length} unresolved contract findings.\n\nResolved target settings and configuration provenance come from the graph constructed by the outer Nx invocation. Generated machine-readable inventories are in 🗑️generated/nx while the ticket is active.\n`);
    console.log(`[nx-audit] projects=${result.projects.length} commands=${result.commands.length} artifacts=${result.artifacts.length} violations=${result.violations.length}`);
  }
}

export class PolicyScript extends BundleScript {
  run(): void {
    const result = inventory(this.repoRoot);
    for (const finding of result.violations) console.error(`${finding.rule} ${finding.path}:${finding.line} ${finding.entry_point}: ${finding.evidence}`);
    if (result.violations.length) throw new Error(`${result.violations.length} Nx contract violations`);
    console.log("[nx-policy] all target and command contracts passed");
  }
}
