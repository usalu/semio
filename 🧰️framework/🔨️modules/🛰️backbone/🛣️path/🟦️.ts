/** 🛣️ Native filesystem paths carried opaquely after backbone wire tags. */
export function absoluteBackbonePathV1(path: string): string {
  return path.startsWith("/") || path.startsWith("\\") || /^[A-Za-z]:[\\/]/u.test(path) ? path : `/${path}`;
}

/** 📁️ Native parent path without interpreting percent sequences or URI authorities. */
export function backbonePathDirectoryV1(source: string): string {
  const path = absoluteBackbonePathV1(source);
  const root = /^(?:[A-Za-z]:[\\/]|[\\/]{2}[^\\/]+[\\/][^\\/]+(?:[\\/]|$))/u.exec(path)?.[0] ?? (path.startsWith("\\") ? "\\" : "/");
  const windows = root !== "/";
  let trailing = true;
  for (let index = path.length - 1; index >= root.length; index--) {
    const separator = path[index] === "/" || windows && path[index] === "\\";
    if (separator && !trailing) return path.slice(0, index);
    if (!separator) trailing = false;
  }
  return root;
}
