/** 🧩️ Jack direct-mutation discriminated union. */
import type { SetQuery } from "./🔎️set-query/🟦️.ts";

export type JackMutation = { mutation: "setQuery" } & SetQuery;
