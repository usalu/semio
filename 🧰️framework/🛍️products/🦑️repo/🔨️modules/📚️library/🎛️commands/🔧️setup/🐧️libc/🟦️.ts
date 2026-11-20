import { spawnSync } from "node:child_process";

/** 🐧 Linux is glibc unless `ldd --version` reports musl. A missing glibc version field is not musl. */
export function linuxNativeLibcV1(lddVersion: string): "gnu" | "musl" {
  return /musl/i.test(lddVersion) ? "musl" : "gnu";
}

/** 🐧 The running host is glibc when its runtime reports one, otherwise {@link linuxNativeLibcV1}. */
export function linuxHostLibcV1(): "gnu" | "musl" {
  const glibc = (process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } | undefined)?.header?.glibcVersionRuntime;
  if (glibc) return "gnu";
  const ldd = spawnSync("ldd", ["--version"], { encoding: "utf8" });
  return linuxNativeLibcV1(`${ldd.stdout ?? ""}\n${ldd.stderr ?? ""}`);
}
