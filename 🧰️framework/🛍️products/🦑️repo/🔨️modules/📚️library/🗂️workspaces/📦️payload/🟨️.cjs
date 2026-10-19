const { relative } = require("node:path");

/** 📦️ Enumerates concrete authored exports across authored subpaths and conditions. */
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

/** 🔗️ Binds nested payload identity to its nearest Source owner through its declared export namespace. */
function ownsPayload(owner, payload) {
  if (!owner.name || owner.name !== payload.name) return false;
  const prefix = relative(owner.absDir, payload.absDir).replaceAll("\\", "/") + "/";
  for (const target of exportTargets(owner.exports)) {
    if (!target.startsWith("./") || /[\\:*?%#\u0000]/u.test(target)) continue;
    const segments = target.slice(2).split("/");
    if (segments.some(segment => !segment || segment === "." || segment === ".." || segment === "node_modules") || !segments.join("/").startsWith(prefix)) continue;
    return true;
  }
  return false;
}
module.exports = { ownsPayload };
