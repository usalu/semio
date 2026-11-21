const filesystemRoute = "/@fs/";

/** 🧭️ Resolves the HTML host's browser entry to a portable Vite optimizer path. */
export function playgroundOptimizerEntryV1(root: string, browserEntry: string): string {
  if (browserEntry.startsWith(filesystemRoute)) return browserEntry.slice(filesystemRoute.length);
  return root.replaceAll("\\", "/").replace(/\/$/u, "") + browserEntry;
}
