/** 🟠️ The mutations phase of toolJobLatestWinsSelfTests. */
import { toolJobMountedDispatchOneTurnExact } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";
import type { buildRawCloseState } from "../🔸️raw-close/🟦️.ts";

/** 🟡 Builds mutations of toolJobLatestWinsSelfTests. */
export function buildMutations({ mutateFunction, dispatchFixture, pipelineGuard, production, validDispatch }: Pick<ReturnType<typeof buildRawCloseState>, "mutateFunction" | "dispatchFixture" | "pipelineGuard" | "production" | "validDispatch">) {
  const mutations: Record<string, (text: string) => string> = {
    none: (text) => text,
    "wrong-helper": (text) => text.replace("self.as_dispatch().start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await", "self.as_dispatch().other_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await"),
    "missing-helper": (text) => text.replace("async fn start_typed_command_operation(", "async fn unrelated_command_operation("),
    "duplicate-helper": (text) => `${text}\nasync fn start_typed_command_operation() {}`,
    "missing-pipeline-guard": (text) => mutateFunction(text, dispatchFixture.dispatcher, pipelineGuard, "self.media.accept_incomplete_pipeline(&admission)?"),
    "duplicate-session": (text) => mutateFunction(text, dispatchFixture.helper, "RefusedWorkerSource::pending(dispatch.job,", "RefusedWorkerSource::pending(extra.job,identity)); let _again=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,"),
    "duplicate-pump": (text) => mutateFunction(text, "drive_worker_step", "|same|session.pump_one_for_interactive_turn(same)", "|same|{session.pump_one_for_interactive_turn(same);session.pump_one_for_interactive_turn(same)}"),
    "direct-reducer": (text) => mutateFunction(text, dispatchFixture.dispatcher, pipelineGuard, `A::handle(&command).await; ${pipelineGuard}`),
    "direct-dispatch": (text) => mutateFunction(text, dispatchFixture.dispatcher, pipelineGuard, `self.tools.tool_jobs.dispatch(operation_spec); ${pipelineGuard}`),
    "run-to-completion": (text) => mutateFunction(text, "drive_worker_step", "let step_fuel=", "session.run_to_completion();let step_fuel="),
  };
  if (new Set(dispatchFixture.cases.map((law: { mutation: string }) => law.mutation)).size !== Object.keys(mutations).length) throw new Error("mounted dispatch fixture omits an exact hostile case");
  for (const law of dispatchFixture.cases) {
    const changed = mutations[law.mutation]!(production);
    if (validDispatch(law.mutation) !== law.admitted || (law.mutation !== "none" && changed === production)) throw new Error(`mounted dispatch fixture oracle: ${law.mutation}`);
    if (toolJobMountedDispatchOneTurnExact(changed) !== law.admitted) throw new Error(`mounted dispatch exact helper law: ${law.mutation}`);
  }
}
