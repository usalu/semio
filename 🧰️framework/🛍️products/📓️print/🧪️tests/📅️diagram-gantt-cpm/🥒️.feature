@capability-diagram-critical-path-method
@no-oracle-critical-path-method
@comparison-viz-probe-v1
Feature: The Gantt family schedules a task network with the critical-path method
  `semio-viz-diagram-process.sty`'s `🔖️Cpm` region runs the textbook CPM over a task table of
  `id`, `duration` and a space-separated `deps` column, and every `pm-gantt` and `pm-network`
  rendering reads its result:

  - **forward pass** — `ES(t) = max ES over the predecessors' EF, 0 when there is none`, and
    `EF(t) = ES(t) + duration(t)`; the relaxation is repeated once per task, which is enough for
    any acyclic network whatever order the rows arrive in.
  - **project end** — the largest EF of the network.
  - **backward pass** — every task starts at `LF = project end`; then `LS(t) = LF(t) − duration(t)`
    and each predecessor's LF is lowered to the smallest LS of its successors, again relaxed once
    per task.
  - **total float** — `LS(t) − ES(t)`; a task with zero float lies on the critical path and the
    Gantt bar and the PERT node are drawn in the danger tone.

  **Why there is no third-party oracle.** The repository's registered oracles are the d3 packages
  plus dagre and ajv; none of them schedules an activity network, and no CPM package is available
  to the test platform. The reference is therefore the independent implementation in this adapter,
  written from the definitions above rather than from the LaTeX source, plus the schedule the
  scenario writes out. If `criticalpath` (or an equivalent npm package) is ever registered this
  case upgrades to `@mode-differential` without changing the probe. The probe emits
  `geometry/diagram-cpm` records of `ES, EF, LS, LF, float` in task-table order.

  @id-forward-and-backward-pass
  @level-quick
  @mode-conformance
  Scenario: A diamond network with one slack branch
    Given the project tasks
      | id | duration | deps | es | ef | ls | lf | float |
      | a  | 3        |      | 0  | 3  | 0  | 3  | 0     |
      | b  | 4        | a    | 3  | 7  | 3  | 7  | 0     |
      | c  | 6        | a    | 3  | 9  | 6  | 12 | 3     |
      | d  | 5        | b    | 7  | 12 | 7  | 12 | 0     |
      | e  | 2        | c d  | 12 | 14 | 12 | 14 | 0     |
      | f  | 1        | e    | 14 | 15 | 14 | 15 | 0     |
    Then the schedule matches the critical-path definitions

  @id-independent-cpm-reference
  @level-quick
  @mode-conformance
  Scenario: A chain with two independent branches agrees with the reference implementation
    Given the project tasks
      | id | duration | deps |
      | s  | 2        |      |
      | p  | 5        | s    |
      | q  | 1        | s    |
      | r  | 3        | q    |
      | t  | 2        | p r  |
    Then the schedule matches the critical-path definitions
