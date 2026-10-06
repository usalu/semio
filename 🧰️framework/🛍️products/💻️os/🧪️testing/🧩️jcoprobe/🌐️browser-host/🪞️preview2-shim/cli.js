import { inputStreamCreate, outputStreamCreate, pollableCreate, } from "./io.js";
export { _setEnv, _setArgs, environment } from "./environment.js";
export { _setCwd } from "./config.js";
const symbolDispose = Symbol.dispose ?? Symbol.for("dispose");
class ComponentExit extends Error {
    exitError = true;
    code;
    constructor(code) {
        super(`Component exited ${code === 0 ? "successfully" : "with error"}`);
        this.code = code;
    }
}
export const exit = {
    exit(status) {
        throw new ComponentExit(status.tag === "err" ? 1 : 0);
    },
    // @ts-expect-error - Available only wasi-cli v0.2.12
    exitWithCode(code) {
        throw new ComponentExit(code);
    },
};
export function _setStdin(handler) {
    stdinStream.handler = handler;
}
export function _setStderr(handler) {
    stderrStream.handler = handler;
}
export function _setStdout(handler) {
    stdoutStream.handler = handler;
}
const stdinStream = inputStreamCreate({
    blockingRead() {
        throw { tag: "closed" };
    },
    subscribe() {
        return pollableCreate();
    },
    [symbolDispose]() { },
});
function consoleStream(writeLine) {
    const decoder = new TextDecoder();
    let pending = "";
    const emitCompleteLines = () => {
        const lines = pending.split("\n");
        pending = lines.pop();
        for (const line of lines) {
            writeLine(line.endsWith("\r") ? line.slice(0, -1) : line);
        }
    };
    return {
        write(contents) {
            pending += decoder.decode(contents, { stream: true });
            emitCompleteLines();
        },
        flush() {
            pending += decoder.decode();
            if (pending) {
                writeLine(pending);
            }
            pending = "";
        },
        blockingFlush() {
            this.flush?.();
        },
        drop() {
            this.flush?.();
        },
    };
}
const stdoutStream = outputStreamCreate(consoleStream((line) => console.log(line)));
const stderrStream = outputStreamCreate(consoleStream((line) => console.error(line)));
export const stdin = {
    getStdin() {
        return stdinStream;
    },
};
export const stdout = {
    getStdout() {
        return stdoutStream;
    },
};
export const stderr = {
    getStderr() {
        return stderrStream;
    },
};
class TerminalInput {
}
class TerminalOutput {
}
export const terminalInput = {
    TerminalInput,
};
export const terminalOutput = {
    TerminalOutput,
};
export const terminalStderr = {
    getTerminalStderr() {
        return undefined;
    },
};
export const terminalStdin = {
    getTerminalStdin() {
        return undefined;
    },
};
export const terminalStdout = {
    getTerminalStdout() {
        return undefined;
    },
};
/** Create isolated browser CLI interfaces without changing compatibility globals. */
export function createCli(config = {}) {
    const stdinInstance = inputStreamCreate(config.stdin ?? {
        blockingRead() {
            throw { tag: "closed" };
        },
        subscribe: () => pollableCreate(),
    });
    const stdoutInstance = outputStreamCreate(config.stdout ?? consoleStream((line) => console.log(line)));
    const stderrInstance = outputStreamCreate(config.stderr ?? consoleStream((line) => console.error(line)));
    const env = Object.entries(config.environment ?? {});
    const args = [...(config.arguments ?? [])];
    const cwd = config.initialCwd ?? "/";
    return {
        environment: {
            getEnvironment: () => env.map(([key, value]) => [key, value]),
            getArguments: () => [...args],
            initialCwd: () => cwd,
        },
        exit,
        stdin: { getStdin: () => stdinInstance },
        stdout: { getStdout: () => stdoutInstance },
        stderr: { getStderr: () => stderrInstance },
        terminalInput,
        terminalOutput,
        terminalStdin,
        terminalStdout,
        terminalStderr,
    };
}
