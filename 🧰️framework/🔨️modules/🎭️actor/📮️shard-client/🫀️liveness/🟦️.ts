/** 🫀️ Owns host watchdog, guest progress cadence, and plugin loading deadlines. */
export const SHARD_LIVENESS_POLICY = Object.freeze({
  heartbeatTimeoutMs: 5000,
  missedLimit: 3,
  progressIntervalMs: 1000,
  firstTurnTimeoutMs: 30_000,
  pluginLoadIdleTimeoutMs: 30_000,
  pluginLoadCeilingMs: 300_000,
});
