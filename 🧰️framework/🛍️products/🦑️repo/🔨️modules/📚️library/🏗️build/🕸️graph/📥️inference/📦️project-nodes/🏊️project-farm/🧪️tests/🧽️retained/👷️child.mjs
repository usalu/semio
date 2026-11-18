const [serviceUrl, observationUrl, rowsText, ownersText, budgetText] = process.argv.slice(2);
const service = await import(serviceUrl), api = await import(observationUrl);
const rows = Number(rowsText), owners = Number(ownersText), budget = Number(budgetText);
const held = () => { const row = process.memoryUsage(); return row.heapUsed + row.external; };
Bun.gc(true);
const base = held(), log = [];
for (let owner = 0; owner < owners; owner++) {
  await api.captureObservedSourceInputsV1(async () => { for (let at = 0; at < rows; at++) api.recordRegistryOwnerV1("/w", "owner-" + owner + "-" + at, { root: "r/" + owner + "/" + at, manifest: "r/" + owner + "/" + at + "/project.json" }); return 1; }, undefined, false);
  Bun.gc(true);
  const trimmed = service.trimRetainedV1(budget);
  if (trimmed) Bun.gc(true);
  log.push({ owner, trimmed, interned: api.inferenceCacheSizesV1().interned, retainedMegabytes: Math.round((held() - base) / 1048576) });
}
console.log(JSON.stringify(log));
