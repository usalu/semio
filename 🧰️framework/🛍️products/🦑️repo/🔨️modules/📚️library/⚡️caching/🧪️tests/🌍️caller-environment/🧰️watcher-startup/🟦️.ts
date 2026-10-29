import ts from "typescript";
import { runInNewContext } from "node:vm";
import { createNxDaemonReadinessV1 } from "../../../🕸️graph/🏁️startup/🧬️lifetime/🟦️.ts";

/** 🚦️ Runs the actual SDK listen callback against a failed watcher and an already waiting service request. */
export async function probeDaemonWatcherStartupV1(source: string, row: Readonly<{ message: string; maximumMilliseconds: number }>): Promise<{ startupRejected: boolean; pendingRejected: boolean; reported: number }> {
  const file = ts.createSourceFile("server.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const start = file.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "startServer");
  let callback: ts.Node | undefined;
  const visit = (node: ts.Node): void => { if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.expression.getText(file) === "server" && node.expression.name.text === "listen") callback = node.arguments[1]; ts.forEachChild(node, visit); };
  if (!start) throw Error("Actual SDK daemon startup required");
  visit(start);
  if (!callback || !ts.isArrowFunction(callback)) throw Error("Actual SDK daemon listen callback required");
  const failure = Error(row.message), readiness = createNxDaemonReadinessV1();
  let reported = 0, invoke!: () => Promise<void>;
  const startup = new Promise<void>((resolve, reject) => {
    invoke = runInNewContext(`(${callback!.getText(file)})`, {
      resolve, reject, server: {}, socketPath: "/fixture/daemon.sock",
      logger_1: { serverLogger: { log() {} } }, socket_utils_1: { isWindows: true }, daemonIsOutdated() {},
      shutdown_utils_1: { getOutputWatcherInstance: () => undefined, storeOutputWatcherInstance() {} },
      watcher_1: { watchOutputFiles: async () => { throw failure; } }, handle_outputs_changes_1: { handleOutputsChanges() {} },
      handleWorkspaceChanges: async (error: unknown) => { if (error !== failure) throw Error("Original watcher failure required"); reported++; },
    }) as () => Promise<void>;
  });
  const settled = startup.then(() => { readiness.ready(); return false; }, error => { readiness.refuse(error); return error === failure; });
  const pending = readiness.admit().then(() => false, error => error === failure);
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    await invoke();
    const [startupRejected, pendingRejected] = await Promise.race([Promise.all([settled, pending]), new Promise<[boolean, boolean]>(accept => { timer = setTimeout(() => accept([false, false]), row.maximumMilliseconds); })]);
    return { startupRejected, pendingRejected, reported };
  } finally { clearTimeout(timer); readiness.refuse(Error("Watcher startup probe finished")); }
}
