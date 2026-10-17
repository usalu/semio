/** 🗄️ Declares the per-statement `safeIntegers` switch that `bun:sqlite` implements but does not type.
 *
 * Bun reads it on every `Statement` (`db.query(sql).safeIntegers(true).get()` returns `bigint` for INTEGER columns) and on
 * `Database` options; its bundled declarations only cover the options. Independent SQLite oracles read 64-bit words
 * through it without reopening the database in integer-safe mode.
 * @see https://bun.sh/docs/api/sqlite#safe-integers */
import type { SQLQueryBindings } from "bun:sqlite";

declare module "bun:sqlite" {
  interface Statement<ReturnType, ParamsType extends SQLQueryBindings[]> {
    safeIntegers(enabled?: boolean): this;
  }
}
