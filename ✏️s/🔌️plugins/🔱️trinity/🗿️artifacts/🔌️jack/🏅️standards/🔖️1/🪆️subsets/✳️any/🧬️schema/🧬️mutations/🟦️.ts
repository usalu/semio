/** 🧩️ Jack direct-mutation discriminated union. */
import type { SetQuery } from "./🔎️set-query/🟦️.ts";
import type { SetRoot } from "./🎯️set-root/🟦️.ts";

export type JackMutation = ({ mutation: "setQuery" } & SetQuery) | ({ mutation: "setRoot" } & SetRoot);
