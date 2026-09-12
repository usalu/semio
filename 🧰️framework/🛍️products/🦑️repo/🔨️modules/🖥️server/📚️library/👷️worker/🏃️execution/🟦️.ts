// #region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Repo server library: PostgreSQL, auth, events, parsing (Next.js API routes).
// #endregion 🧲️Header


import { createOwnedServerJobQueue, type OwnedServerJob } from "../../../🎛️coordinator/🔌️ports/🟦️.ts";
import { DATABASE_URL, markDiscordDeliveryFailed, markDiscordDeliverySent } from "../../🗄️persistence/🟦️.ts";
import { sendDiscordMessage } from "../../📡️event/🟦️.ts";

// #region 🔖️worker
// #region 🌊️Jobs
// Job handler definitions.

interface DiscordSendJob {
  deliveryId: string;
  title: string;
  body: string;
  attempt: number;
}

async function handleDiscordSend(jobs: OwnedServerJob<DiscordSendJob>[]) {
  for (const job of jobs) {
    const { deliveryId, title, body, attempt } = job.data;
    const success = await sendDiscordMessage(title, body);
    if (success) {
      await markDiscordDeliverySent(deliveryId);
    } else {
      await markDiscordDeliveryFailed(deliveryId, "delivery failed", attempt + 1);
    }
  }
}

interface ReindexJob {
  repoRoot: string;
}

async function handleReindex(jobs: OwnedServerJob<ReindexJob>[]) {
  for (const job of jobs) {
    console.log(`[worker] reindex job for ${job.data.repoRoot}`);
  }
}
// #endregion 🌊️Jobs

// #region 🌩️Main
/** @emoji 🌩️ Starts pg-boss workers (separate process entry via `🟦️worker.ts`). */
export async function runRepoServerWorker(): Promise<void> {
  const boss = createOwnedServerJobQueue(DATABASE_URL);

  boss.on("error", (error) => console.error("[pg-boss error]", error));

  await boss.start();
  console.log("[worker] pg-boss started");

  await boss.work<DiscordSendJob>("discord.send", handleDiscordSend);
  await boss.work<ReindexJob>("repo.reindex", handleReindex);

  console.log("[worker] listening for jobs");

  process.on("SIGINT", async () => {
    console.log("[worker] shutting down...");
    await boss.stop();
    process.exit(0);
  });

  process.on("SIGTERM", async () => {
    console.log("[worker] shutting down...");
    await boss.stop();
    process.exit(0);
  });
}
// #endregion 🌩️Main
// #endregion 🔖️worker
