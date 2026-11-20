const fs = require("node:fs"), { createHash } = require("node:crypto"), { spawnSync } = require("node:child_process"), Module = require("node:module"), { dirname, join } = require("node:path");
const root = "/vroot", groups = Number(process.env.PACKED_GROUPS ?? 50), subgroups = 20, middles = 10, leaves = Number(process.env.PACKED_LEAVES ?? 48), dev = 16777232n, stamp = 1700000000000000000n;
const packagePaths = []; for (let g = 0; g < groups; g += 5) for (let s = 0; s < subgroups; s += 4) packagePaths.push("members/g" + g + "/s" + s + "/t0");
packagePaths.sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
const manifest = JSON.stringify({ name: "synthetic", private: true, workspaces: packagePaths, semio: { workspace: { schemaVersion: 1, members: ["members/**"], owners: [] } } });
const parse = path => path === root ? [] : path.startsWith(root + "/") ? path.slice(root.length + 1).split("/") : undefined;
const hashOf = text => { let a = 0x811c9dc5, b = 0x9747b28c; for (let index = 0; index < text.length; index++) { const c = text.charCodeAt(index); a = Math.imul(a ^ c, 0x01000193) >>> 0; b = Math.imul(b ^ c, 0x85ebca6b) >>> 0; } return BigInt(a) * 4294967296n + BigInt(b); };
const packageText = parts => JSON.stringify({ name: "p-" + parts.slice(1).join("-"), version: "1.0.0" });
const hasPackage = parts => parts.length === 4 && parts[3] === "t0" && Number(parts[1].slice(1)) % 5 === 0 && Number(parts[2].slice(1)) % 4 === 0;
const bound = [undefined, 1, groups, subgroups, middles, leaves], letter = ["", "m", "g", "s", "t", "u"];
const level = (parts, depth) => { const name = parts[depth - 1]; return depth === 1 ? name === "members" : name[0] === letter[depth] && /^[a-z](?:0|[1-9][0-9]*)$/u.test(name) && Number(name.slice(1)) < bound[depth]; };
function nodeKind(path) {
  const parts = parse(path); if (!parts) return undefined;
  if (parts.length === 0) return "d";
  if (parts.length === 1 && parts[0] === "package.json") return "f";
  if (parts[0] !== "members") return undefined;
  const last = parts[parts.length - 1];
  if (parts.length === 5 && last === "package.json") return hasPackage(parts.slice(0, -1)) && [1, 2, 3, 4].every(depth => level(parts, depth)) ? "f" : undefined;
  if (parts.length > 5) return undefined;
  for (let depth = 1; depth <= parts.length; depth++) if (!level(parts, depth)) return undefined;
  return "d";
}
function children(parts) {
  if (parts.length === 0) return [["members", "d"], ["package.json", "f"]];
  const depth = parts.length, names = [];
  if (depth > 4) return depth === 5 ? names : undefined;
  for (let index = 0; index < bound[depth + 1]; index++) names.push([letter[depth + 1] + index, "d"]);
  if (hasPackage(parts)) names.push(["package.json", "f"]);
  return names;
}
function stats(path) {
  const kind = nodeKind(path); if (!kind) return undefined;
  const parts = parse(path), text = kind === "f" ? (parts.length === 1 ? manifest : packageText(parts.slice(0, -1))) : "";
  return { kind, text, dev, ino: hashOf(path) + 2n, mode: kind === "d" ? 16877n : 33188n, nlink: 1n, size: BigInt(text.length), mtimeNs: stamp, ctimeNs: stamp, birthtimeNs: stamp };
}
const asStats = row => ({ ...row, isDirectory: () => row.kind === "d", isFile: () => row.kind === "f", isSymbolicLink: () => false });
const expectedRow = (path, listed) => { const row = stats(path); if (!row) return undefined; const state = [row.kind === "d" ? "directory" : "file", String(row.dev), String(row.ino), Number(row.mode)]; const value = { path, state }; if (listed) value.list = children(parse(path)).filter(([name, kind]) => name === "package.json" || kind === "d").map(([name, kind]) => ({ name, kind: kind === "d" ? "directory" : "file" })).sort((a, b) => Buffer.from(a.name).compare(Buffer.from(b.name))); return value; };
function install() {
  const real = {}, table = new Map(); let next = 1000000;
  for (const name of ["lstatSync", "opendirSync", "openSync", "fstatSync", "readSync", "closeSync"]) real[name] = fs[name];
  const virtual = path => typeof path === "string" && (path === root || path.startsWith(root + "/"));
  fs.lstatSync = (path, options) => { if (!virtual(path)) return real.lstatSync(path, options); const row = stats(path); if (!row) throw Object.assign(Error("ENOENT " + path), { code: "ENOENT" }); return asStats(row); };
  fs.opendirSync = (path, options) => { if (!virtual(path)) return real.opendirSync(path, options); const list = children(parse(path)); if (!list) throw Object.assign(Error("ENOTDIR " + path), { code: "ENOTDIR" }); let index = 0; return { readSync: () => index >= list.length ? null : (([name, kind]) => ({ name, isSymbolicLink: () => false, isDirectory: () => kind === "d", isFile: () => kind === "f" }))(list[index++]), closeSync() {} }; };
  fs.openSync = (path, flags, mode) => { if (!virtual(path)) return real.openSync(path, flags, mode); const row = stats(path); if (!row || row.kind !== "f") throw Object.assign(Error("ENOENT " + path), { code: "ENOENT" }); const fd = next++; table.set(fd, row); return fd; };
  fs.fstatSync = (fd, options) => table.has(fd) ? asStats(table.get(fd)) : real.fstatSync(fd, options);
  fs.readSync = (fd, buffer, offset, length, position) => { if (!table.has(fd)) return real.readSync(fd, buffer, offset, length, position); const bytes = Buffer.from(table.get(fd).text), count = Math.max(0, Math.min(length, bytes.length - position)); bytes.copy(buffer, offset, position, position + count); return count; };
  fs.closeSync = fd => table.has(fd) ? void table.delete(fd) : real.closeSync(fd);
}
const footprint = () => { if (process.platform !== "darwin") return process.memoryUsage().rss / 1048576; const match = spawnSync("/usr/bin/footprint", [String(process.pid)], { encoding: "utf8" }).stdout.match(/Footprint:\s*([\d.]+)\s*(KB|MB|GB)/u); return match ? Number(match[1]) * { KB: 1 / 1024, MB: 1, GB: 1024 }[match[2]] : NaN; };
async function main() {
  const source = process.env.PACKED_SOURCE ?? join(__dirname, "../../🟨️.cjs"), origin = process.env.PACKED_ORIGIN ?? join(__dirname, "../../🟨️.cjs");
  install();
  const loaded = new Module(origin, null); loaded.filename = origin; loaded.paths = Module._nodeModulePaths(dirname(origin)); loaded._compile(fs.readFileSync(source, "utf8"), origin);
  const ownership = loaded.exports.createBunWorkspaceOwnershipV1({ ownershipPath: origin, ownershipSource: fs.readFileSync(origin, "utf8") });
  const started = performance.now(); let peak = footprint(), steps = 0; const trace = [];
  const sample = label => { const value = footprint(); peak = Math.max(peak, value); trace.push([label, Math.round((performance.now() - started) / 1000), Math.round(value)]); };
  const control = { signal: new AbortController().signal, frontierBatchPaths: 64, advance: async () => { if (++steps % 150000 === 0) sample("step " + steps); } };
  let result;
  await ownership.withCurrentBunWorkspaceOwnershipV1(root, { kind: "all" }, control, async custody => {
    sample("custody");
    const observed = custody.observed(), hash = createHash("sha256"); let rows = 0, listed = 0, mismatches = 0, first;
    let foreign = 0;
    for (const row of observed.paths) {
      if (!parse(row.path)) { foreign++; continue; }
      hash.update(JSON.stringify(row) + "\n"); rows++; if (row.list) listed++;
      const expected = expectedRow(row.path, row.list !== undefined);
      if (!expected || JSON.stringify(expected) !== JSON.stringify(row)) { mismatches++; first ??= row.path; }
    }
    sample("rows checked");
    await custody.recheck(); sample("recheck");
    result = { rows, foreign, listed, mismatches, first, rowsSha: hash.digest("hex"), scopesSha: createHash("sha256").update(JSON.stringify(custody.scopes)).digest("hex"), packages: custody.scopes.reduce((sum, scope) => sum + scope.packages.length, 0), textsSha: createHash("sha256").update(JSON.stringify(observed.texts)).digest("hex") };
  });
  sample("closed");
  console.log("RESULT " + JSON.stringify({ ...result, steps, peakFootprintMegabytes: Math.round(peak), seconds: Math.round((performance.now() - started) / 100) / 10, trace }));
}
if (require.main === module) main().catch(error => { console.log("FAILED " + error.stack); process.exit(1); });
module.exports = { expectedRow, children, parse, stats, packagePaths };
