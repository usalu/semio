import { readFileSync } from "node:fs";
import { join } from "node:path";

type NativeCredentialSourcePopulation = Readonly<{ entrypoint: string; credential: string; runner: string; launch: string }>;
type McpCredentialSourcePopulation = Readonly<{ entrypoint: string; workspace: string; remote: string; directory: string; runner: string; launch: string }>;

function balancedBody(source: string, opening: number): string | undefined {
  let depth = 0;
  let quote: "'" | '"' | "`" | undefined;
  let rawTerminator: string | undefined;
  let lineComment = false;
  let blockComment = 0;
  for (let index = opening; index < source.length; index += 1) {
    const character = source[index]!;
    const next = source[index + 1];
    if (rawTerminator) {
      if (source.startsWith(rawTerminator, index)) {
        index += rawTerminator.length - 1;
        rawTerminator = undefined;
      }
      continue;
    }
    if (lineComment) {
      if (character === "\n") lineComment = false;
      continue;
    }
    if (blockComment > 0) {
      if (character === "/" && next === "*") {
        blockComment += 1;
        index += 1;
      } else if (character === "*" && next === "/") {
        blockComment -= 1;
        index += 1;
      }
      continue;
    }
    if (quote) {
      if (character === "\\") index += 1;
      else if (character === quote) quote = undefined;
      continue;
    }
    const rawPrefix = character === "r" ? index + 1 : character === "b" && next === "r" ? index + 2 : -1;
    if (rawPrefix >= 0) {
      let quoteIndex = rawPrefix;
      while (source[quoteIndex] === "#") quoteIndex += 1;
      if (source[quoteIndex] === '"') {
        rawTerminator = `"${"#".repeat(quoteIndex - rawPrefix)}`;
        index = quoteIndex;
        continue;
      }
    }
    if (character === "/" && next === "/") {
      lineComment = true;
      index += 1;
    } else if (character === "/" && next === "*") {
      blockComment = 1;
      index += 1;
    } else if (character === '"' || character === "'" || character === "`") quote = character;
    else if (character === "{") depth += 1;
    else if (character === "}" && --depth === 0) return source.slice(opening + 1, index);
  }
  return undefined;
}

/** 🧭 Extracts definition bodies with lexical brace balancing rather than global token ordering. */
export function sourceDefinitionBodies(source: string, selector: RegExp): readonly string[] {
  const flags = selector.flags.includes("g") ? selector.flags : `${selector.flags}g`;
  const expression = new RegExp(selector.source, flags);
  const bodies: string[] = [];
  for (let match = expression.exec(source); match; match = expression.exec(source)) {
    let opening = match.index + match[0].length;
    while (opening < source.length && source[opening] !== "{") opening += 1;
    if (opening >= source.length) continue;
    const body = balancedBody(source, opening);
    if (body !== undefined) bodies.push(body);
    expression.lastIndex = opening + (body?.length ?? 0) + 2;
  }
  return bodies;
}

function ordered(body: string, steps: readonly string[]): boolean {
  let previous = -1;
  return steps.every((step) => {
    const index = body.indexOf(step);
    if (index <= previous) return false;
    previous = index;
    return true;
  });
}

/** 🖥️ Checks the current WGPU entrypoint and its direct TypeScript process-owner chain. */
export function nativeCredentialSourceOrderConforms(source: NativeCredentialSourcePopulation): boolean {
  const mains = sourceDefinitionBodies(source.entrypoint, /\bfn\s+main\s*\([^)]*\)\s*/gu);
  const main = mains.filter((body) => body.includes('claim_inherited_local_hub_credential("native")'));
  const classes = sourceDefinitionBodies(source.runner, /\bclass\s+RunScript\b[^\{]*/gu);
  const run = classes.flatMap((body) => sourceDefinitionBodies(body, /\basync\s+run\s*\([^)]*\)\s*(?::[^\{]+)?/gu));
  const session = sourceDefinitionBodies(source.runner, /\basync\s+function\s+runNativeSession\s*\([^)]*\)\s*(?::[^\{]+)?/gu);
  const binary = sourceDefinitionBodies(source.runner, /\basync\s+function\s+runNativeBinary\s*\([^)]*\)\s*(?::[^\{]+)?/gu);
  return (
    main.length === 1 &&
    ordered(main[0]!, ['claim_inherited_local_hub_credential("native")', 'arg_value("--plugin")', "run_native("]) &&
    main[0]!.includes('return value == "3";') &&
    source.credential.includes("FD_CLOEXEC") &&
    source.credential.includes("_close(3)") &&
    main[0]!.includes("--assert-no-local-credential-state") &&
    main[0]!.includes("protected_credential_environment_is_absent") &&
    run.length === 1 &&
    run[0]!.includes("await runNativeSession(") &&
    !run[0]!.includes('runCmd("cargo"') &&
    session.length === 1 &&
    session[0]!.includes("await runNativeBinary(") &&
    binary.length === 1 &&
    binary[0]!.includes("await runTool(") &&
    source.launch.includes("os-hub:dev-secure-native")
  );
}

function mcpSchemasPreflight(main: string, claim: number): boolean {
  const preflight = sourceDefinitionBodies(main.slice(0, claim), /\bif\s+std::env::args\(\)\.nth\(1\)\.as_deref\(\)\s*==\s*Some\("schemas"\)\s*/gu);
  const prefix = main.slice(0, claim).replace(/\/\/[^\n]*/gu, "");
  return preflight.length === 1 && preflight[0]!.includes("schema_mirror_json()") && preflight[0]!.includes("return;") && (prefix.match(/\breturn\s*;/gu) ?? []).length === 1;
}

/** 🌉️ Checks the one current MCP runner and permits only its pure schemas-return preflight. */
export function mcpCredentialSourceOrderConforms(source: McpCredentialSourcePopulation): boolean {
  const mains = sourceDefinitionBodies(source.entrypoint, /\bfn\s+main\s*\([^)]*\)\s*/gu);
  const main = mains.filter((body) => body.includes('claim_inherited_local_hub_credential("mcp")'));
  if (main.length !== 1) return false;
  const claim = main[0]!.indexOf('claim_inherited_local_hub_credential("mcp")');
  const parsed = main[0]!.indexOf("parse_args()", claim);
  const activate = main[0]!.indexOf("run_stdio(options)", parsed);
  const environmentSeal = sourceDefinitionBodies(source.entrypoint, /\bfn\s+protected_credential_environment_is_absent\s*\([^)]*\)\s*(?:->[^\{]+)?/gu);
  const openHub = source.workspace.indexOf("pub fn open_hub(");
  const injectCredential = source.workspace.indexOf("set_local_hub_credential(credential)", openHub);
  const injectGrantSource = source.workspace.indexOf("set_hub_socket_grant_source(grant_source)", openHub);
  const returnWorkspace = source.workspace.indexOf("Ok(workspace)", openHub);
  return (
    claim >= 0 &&
    mcpSchemasPreflight(main[0]!, claim) &&
    parsed > claim &&
    activate > parsed &&
    main[0]!.includes("protected_credential_environment_is_absent") &&
    environmentSeal.length === 1 &&
    environmentSeal[0]!.includes('return value == "3";') &&
    !source.entrypoint.includes('"--token" => hub.') &&
    !source.remote.includes("set_token(") &&
    !source.remote.includes("DirectoryClient::new(transport, base_url)") &&
    openHub >= 0 &&
    injectCredential > openHub &&
    injectGrantSource > injectCredential &&
    returnWorkspace > injectGrantSource &&
    /pub\s+const\s+PROBE_PACK_SCHEMA_HASH:\s*&str\s*=\s*"[0-9a-f]{64}"/u.test(source.workspace) &&
    source.workspace.includes("authenticated_probe_document_is_known") &&
    source.workspace.includes("Some(probe_record_spec())") &&
    !source.workspace.includes("probe_document_socket_surface") &&
    !source.workspace.includes("set_document_execution_target_lease(") &&
    source.workspace.includes("artifact_document_key(artifact_id)") &&
    source.workspace.includes("surface: Some(PROBE_SURFACE_ID.to_string())") &&
    source.directory.includes('"/directory/socket-grants"') &&
    source.directory.includes("/directory/socket/v1?since=") &&
    source.directory.includes("directory_socket_hello_v1()") &&
    !source.runner.includes('runCmd("cargo", ["run"') &&
    source.runner.includes("runCmd(requireMcpBinary") &&
    source.launch.includes("os-hub:dev-secure-mcp")
  );
}

/** 🔎 Reads the exact current WGPU source population and rejects stale supervisors. */
export function proveNativeCredentialSourceOrder(repoRoot: string): void {
  const population: NativeCredentialSourcePopulation = {
    entrypoint: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs"), "utf8"),
    credential: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8"),
    runner: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts"), "utf8"),
    launch: readFileSync(join(repoRoot, ".vscode/🧩️launch.seed.jsonc"), "utf8"),
  };
  if (!nativeCredentialSourceOrderConforms(population)) throw new Error("WGPU credential claim or current native process-owner chain drift");
}

/** 🔎 Reads the one current MCP source population and rejects credential-flow drift. */
export function proveMcpCredentialSourceOrder(repoRoot: string): void {
  const population: McpCredentialSourcePopulation = {
    entrypoint: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs"), "utf8"),
    workspace: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"), "utf8"),
    remote: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs"), "utf8"),
    directory: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8"),
    runner: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts"), "utf8"),
    launch: readFileSync(join(repoRoot, ".vscode/🧩️launch.seed.jsonc"), "utf8"),
  };
  if (!mcpCredentialSourceOrderConforms(population)) throw new Error("MCP credential claim or current direct-binary process owner drift");
}
