/** 🫧️ The content-addressed blob endpoint of the dev server: the blob database and the Vite plugin that serves it. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { BLOB_ENDPOINT_PATH } from "@semio-tech/framework-os";
import { blake3Hex } from "@semio-tech/framework-hash";
import { REPO_ROOT } from "../🧭️repo-root/🟦️.ts";
import { backboneDatabaseCtorLazy } from "../🗃️sqlite-constructor/🟦️.ts";
import { SqliteDatabaseV1 } from "../../../../../../🔨️modules/⚙️runtime/🗃️sqlite/🟦️.ts";

//#region BlobVitePlugin
let blobDatabaseSingleton: InstanceType<typeof SqliteDatabaseV1> | undefined;

/** 🗄️ Lazily opens the dev-session-wide content-addressed blob store at `<repoRoot>/.🧬semio/🔗space/blobs.db` —
 * unlike backbone documents, blobs aren't scoped to a per-uri folder (there's no folder in the
 * `write-blob`/`read-blob` WIT signature), so this is one shared table for the whole dev server. */
async function blobDatabase(): Promise<InstanceType<typeof SqliteDatabaseV1>> {
  if (!blobDatabaseSingleton) {
    const Database = await backboneDatabaseCtorLazy();
    const dbPath = join(REPO_ROOT, ".🧬semio", "🔗space", "blobs.db");
    mkdirSync(dirname(dbPath), { recursive: true });
    blobDatabaseSingleton = new Database(dbPath);
    blobDatabaseSingleton.run("CREATE TABLE IF NOT EXISTS blob (hash TEXT PRIMARY KEY, media_type TEXT NOT NULL, size INTEGER NOT NULL, bytes BLOB NOT NULL)");
  }
  return blobDatabaseSingleton;
}

type BlobServerRequest = { method?: string; url?: string; on: (event: string, handler: (chunk?: unknown) => void) => void };
type BlobServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; end: (body?: string | Buffer) => void };

/** 📦️ Vite middleware for the dev-only content-addressed blob store: `PUT ${BLOB_ENDPOINT_PATH}?mediaType=`
 * (raw bytes body, BLAKE3-hashed above, returns `{"hash":...}`, idempotent via `INSERT OR IGNORE`) and
 * `GET ${BLOB_ENDPOINT_PATH}/:hash` (raw bytes response, 404 if absent). The browser host-shim's
 * `writeBlob`/`readBlob` (see `hostShimSource`) and `🟦️backbone-🟦️worker.ts`'s IndexedDB cache both talk to
 * this. Mirrors `vcs::FolderSqliteStorage`'s `blobs(hash, media_type, size, bytes)` table/shape. */
export function semioBlobVitePlugin() {
  return {
    name: "semio-blob",
    configureServer(server: { middlewares: { use: (handler: (req: BlobServerRequest, res: BlobServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(BLOB_ENDPOINT_PATH)) return next();
        const requestUrl = new URL(req.url, "http://127.0.0.1");
        if (req.method === "PUT") {
          const chunks: Buffer[] = [];
          req.on("data", (chunk) => {
            chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk as ArrayBuffer));
          });
          req.on("end", () => {
            void (async () => {
              const bytes = Buffer.concat(chunks);
              const mediaType = requestUrl.searchParams.get("mediaType") ?? "application/octet-stream";
              const hash = blake3Hex(new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength));
              const db = await blobDatabase();
              db.run("INSERT OR IGNORE INTO blob (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)", [hash, mediaType, bytes.length, bytes]);
              res.statusCode = 200;
              res.setHeader("content-type", "application/json");
              res.end(JSON.stringify({ hash }));
            })().catch((error) => {
              res.statusCode = 500;
              res.end(String(error));
            });
          });
          return;
        }
        if (req.method === "GET") {
          const hash = requestUrl.pathname.slice(`${BLOB_ENDPOINT_PATH}/`.length);
          if (!hash) {
            res.statusCode = 400;
            res.end("missing hash");
            return;
          }
          void (async () => {
            const db = await blobDatabase();
            const row = db.query("SELECT media_type, bytes FROM blob WHERE hash = ?1").get(hash) as { media_type?: string; bytes?: Uint8Array } | null;
            if (!row) {
              res.statusCode = 404;
              res.end("");
              return;
            }
            res.statusCode = 200;
            res.setHeader("content-type", row.media_type ?? "application/octet-stream");
            res.end(Buffer.from(row.bytes ?? new Uint8Array()));
          })().catch((error) => {
            res.statusCode = 500;
            res.end(String(error));
          });
          return;
        }
        res.statusCode = 405;
        res.end("method not allowed");
      });
    },
  };
}
//#endregion BlobVitePlugin
