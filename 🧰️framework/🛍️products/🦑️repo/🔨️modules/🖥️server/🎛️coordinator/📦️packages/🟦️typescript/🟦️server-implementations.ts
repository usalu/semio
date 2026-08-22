//#region 🔌️Adapters
import { createRequire } from "node:module";
import { NextResponse } from "next/server";
import PgBoss from "pg-boss";
//#endregion 🔌️Adapters

//#region 🔖️OwnedServerContract
export type OwnedDatabaseQueryResult = { readonly rows: unknown[] };
export type OwnedDatabasePool = {
  query(statement: string, values?: readonly unknown[]): Promise<OwnedDatabaseQueryResult>;
  end(): Promise<void>;
};
type OwnedPostgresPool = {
  query(statement: string, values?: readonly unknown[]): Promise<{ readonly rows: unknown[] }>;
  end(): Promise<void>;
};
type OwnedPostgresPoolConstructor = new (options: { readonly connectionString: string; readonly max: number }) => OwnedPostgresPool;
export type OwnedServerRequest = { readonly headers: { get(name: string): string | null } };
export type OwnedServerResponse = Response;
export type OwnedServerJob<T> = { readonly data: T };
export type OwnedServerJobQueue = {
  on(event: "error", listener: (error: Error) => void): void;
  start(): Promise<void>;
  work<T>(name: string, handler: (jobs: OwnedServerJob<T>[]) => Promise<void>): Promise<void>;
  stop(): Promise<void>;
};
//#endregion 🔖️OwnedServerContract

//#region 🔗️ExternalImplementations
const { Pool } = createRequire(import.meta.url)("pg") as { readonly Pool: OwnedPostgresPoolConstructor };
//#endregion 🔗️ExternalImplementations

//#region 🏭️Factories
/** @emoji 🗄️ Creates the repository-owned SQL pool behind the coordinator's declaring manifest. */
export function createOwnedDatabasePool(connectionString: string, max: number): OwnedDatabasePool {
  const pool = new Pool({ connectionString, max });
  return {
    async query(statement, values) {
      const result = await pool.query(statement, values ? [...values] : undefined);
      return { rows: result.rows };
    },
    async end() {
      await pool.end();
    },
  };
}

/** @emoji 📨️ Creates a JSON response without exposing the Next.js response implementation. */
export function createOwnedJsonResponse(body: unknown, status: number): OwnedServerResponse {
  return NextResponse.json(body, { status });
}

/** @emoji 🧪️ Recognizes responses created by the coordinator's Next.js implementation. */
export function isOwnedServerResponse(value: unknown): value is OwnedServerResponse {
  return value instanceof NextResponse;
}

/** @emoji 🌊️ Creates the repository-owned durable job queue behind the coordinator's declaring manifest. */
export function createOwnedServerJobQueue(connectionString: string): OwnedServerJobQueue {
  const boss = new PgBoss(connectionString);
  return {
    on(event, listener) {
      boss.on(event, listener);
    },
    async start() {
      await boss.start();
    },
    async work<T>(name: string, handler: (jobs: OwnedServerJob<T>[]) => Promise<void>) {
      await boss.work<T>(name, async (jobs) => handler(jobs.map((job) => ({ data: job.data }))));
    },
    async stop() {
      await boss.stop();
    },
  };
}
//#endregion 🏭️Factories
