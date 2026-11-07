import { existsSync, mkdirSync, realpathSync } from "node:fs";
import { join, resolve, sep } from "node:path";

export interface TicketOutputOperations {
  realpath(path: string): string;
  exists(path: string): boolean;
  makeDirectory(path: string): void;
}

/** 🎫️ Keeps every diagnostic artifact within an explicitly selected repository ticket. */
export function ticketOutput(
  root: string,
  args: string[],
  environment: Readonly<Record<string, string | undefined>> = process.env,
  operations: TicketOutputOperations = {
    realpath: realpathSync,
    exists: existsSync,
    makeDirectory: (path) => mkdirSync(path, { recursive: true }),
  },
): string {
  const index = args.indexOf("--ticket");
  const ticket = args.find((arg) => arg.startsWith("--ticket="))?.slice(9) ?? (index >= 0 ? args[index + 1] : environment.SEMIO_TICKET_DIR);
  if (!ticket) throw new Error("Select the active ticket with --ticket <directory> or SEMIO_TICKET_DIR");
  const directory = operations.realpath(resolve(root, ticket));
  const tickets = operations.realpath(join(root, ".🧬semio/🦑️repo/🎫️tickets"));
  if (!directory.startsWith(`${tickets}${sep}`) || !operations.exists(join(directory, "🎫️ticket.json"))) throw new Error("Diagnostic output requires an existing repository ticket");
  const generated = join(directory, "🗑️generated");
  operations.makeDirectory(generated);
  if (operations.realpath(generated) !== generated) throw new Error("Diagnostic output requires a physical generated directory within the ticket");
  const output = join(generated, "nx");
  operations.makeDirectory(output);
  if (operations.realpath(output) !== output) throw new Error("Diagnostic output requires a physical output directory within the ticket");
  return output;
}
