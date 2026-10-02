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
function ownsPayload(owner, payload, operations) {
  if (!owner.name || owner.name !== payload.name) return false;
  const prefix = relative(owner.absDir, payload.absDir).replaceAll("\\", "/") + "/";
  return exportTargets(owner.exports).some((target) => {
    if (!target.startsWith(".") || /[\\:*?%#\u0000]/u.test(target)) return false;
    const segments = target.slice(2).split("/");
    if (segments.some((segment) => !segment || segment === "." || segment === ".." || segment === "node_modules")) return false;
    if (!segments.join("/").startsWith(prefix)) return false;
    let path = owner.absDir;
    return segments.every((segment, index) => {
      path = join(path, segment);
      return operations.state(path) === (index === segments.length - 1 ? "file" : "directory");
    });
  });
}
module.exports = { ownsPayload };
