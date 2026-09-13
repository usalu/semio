export const REPO_TEST_DOMAIN_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";

export const REPO_TEST_RUST_PACKAGE_REL = `${REPO_TEST_DOMAIN_REL}/📦️packages/🦀️rust`;

export const REPO_TEST_GO_PACKAGE_REL = `${REPO_TEST_DOMAIN_REL}/📦️packages/🐹️go`;

export const REPO_TEST_PYTHON_HOST_REL = `${REPO_TEST_DOMAIN_REL}/🖥️host`;

export const REPO_TEST_DOTNET_PACKAGE_REL = `${REPO_TEST_DOMAIN_REL}/📦️packages/🔷️dotnet`;

/** 🏗️ One build step whose emitted executable becomes the native host command. */
export type HostPreparation = Readonly<{ command: string; args: readonly string[]; executableFromStdout: (stdout: string) => string | null }>;

/** 🏗️ One materialized native entrypoint: where it lives and how it is launched. */
export type MaterializedHost = Readonly<{ command: string; args: readonly string[]; cwd: string; env: NodeJS.ProcessEnv; hostDir: string | null; problems: readonly string[]; preparation?: HostPreparation }>;
