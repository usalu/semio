const { join, relative } = require("node:path");

/** 📦️ Enumerates concrete physical exports across authored subpaths and conditions. */
function exportTargets(value, subpaths = true) {
  if (typeof value === "string") return [value];
  if (Array.isArray(value)) return value.flatMap((entry) => exportTargets(entry, false));
  if (!value || typeof value !== "object") return [];
  const entries = Object.entries(value);
  if (entries.some(([key]) => key.startsWith("."))) {
    if (!subpaths || entries.some(([key]) => key !== "." && (!key.startsWith(".") || key.includes("*")))) return [];
    return entries.flatMap(([, entry]) => exportTargets(entry, false));
  }
  if (entries.some(([key]) => !key || /^\d+$/u.test(key))) return [];
  const targets = [];
  for (const [condition, entry] of entries) {
    targets.push(...exportTargets(entry, false));
    if (condition === "default") break;
  }
  return targets;
}

/** 🔗️ Binds nested payload identity to its nearest concrete physical export owner. */
function* ownsPayloadPlan(owner, payload) {
  if (!owner.name || owner.name !== payload.name) return false;
  const prefix = relative(owner.absDir, payload.absDir).replaceAll("\\", "/") + "/";
  for (const target of exportTargets(owner.exports)) {
    if (!target.startsWith(".") || /[\\:*?%#\u0000]/u.test(target)) continue;
    const segments = target.slice(2).split("/");
    if (segments.some(segment => !segment || segment === "." || segment === ".." || segment === "node_modules") || !segments.join("/").startsWith(prefix)) continue;
    let path = owner.absDir, owned = true;
    for (const [index, segment] of segments.entries()) {
      path = join(path, segment);
      if ((yield { operation: "state", path }) !== (index === segments.length - 1 ? "file" : "directory")) { owned = false; break; }
    }
    if (owned) return true;
  }
  return false;
}
function ownsPayload(owner, payload, operations) {
  const plan = ownsPayloadPlan(owner, payload); let step = plan.next();
  try { while (!step.done) step = plan.next(operations.state(step.value.path)); return step.value; }
  finally { plan.return(); }
}
module.exports = { ownsPayload, ownsPayloadPlan };
