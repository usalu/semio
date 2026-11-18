@capability-proctor-chaos
@comparison-quiz-lifecycle-reference
Feature: A hundred learners play every quiz through dropped connections, retried commands and one restart of the proctor
  The proctor is one native binary over one SQLite file, built with CQRS and event sourcing. Whatever the network and the
  process do to it, its events must stay the only truth: no learner loses an event, none gets one twice, every submitted
  run is scored exactly once and every read model equals what the quiz core computes from those events.

  The clients are protocol-level, not browsers: each speaks the framework envelopes over plain HTTP and WebSocket and keeps
  a journal of every command it sent, the verdict it got and the durable records it was answered with. About a hundred
  of them arrive within a few seconds from one network or from many. Each registers as an anonymous learner, under a
  pseudonym of its own, under a name of its own, or under one of a few handles that several learners claim at the same
  time and spell in different cases and with different white space. Each then plays every quiz of the course, the
  challenge changing from quiz to quiz and from learner to learner: an easy, medium, hard or expert run is started, an
  expert task is opened before it is answered, tasks are answered one to three times, easy runs are read again for their
  hints, and some runs are left open and voided by starting the quiz at another challenge. Every run is submitted and the
  result, the crowd of the quiz, the leaderboards and the learner are read. Each learner holds a presence socket in the
  roster room and in the room of its page, and follows its own event stream over a socket.

  While they play, a chaos schedule runs beside them. It drops and reopens presence sockets one by one and in Wi-Fi
  storms of a seventh of all sockets; it drops event streams, which resume after the last record they received; it
  aborts requests after a few milliseconds, so that the proctor may or may not have decided them; it sends a command
  twice at once and sends it again after its verdict; and it kills or stops the proctor process once, at a third of
  the run, starts it again over the same data and leaves it to the clients to retry with the same command ids.

  THE REFERENCE is the quiz core in TypeScript (`@semio-tech/course`: `decideLearner`, `evolveLearner`, `transcript`,
  `leaderboard`, `crowdView`, `learnerView`, `runView`, `normalizeHandle`), which shares no code with the Rust proctor
  that answers. It is fed the journals alone — the commands each learner sent, in order, at the instants it sent them —
  and the quiz files, whose revision is the SHA-256 of their bytes. Everything is compared as JSON with sorted keys.

  Run from the repository root with the proctor binary built by `repo cargo --store=shared --configuration=release -- build
  --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --bin proctor --release`:
  `bun 🎓️teaching/🛂️proctor/🧪️tests/🌪️chaos/🟦️.ts --executable <proctor> [--learners 100] [--addresses 100|1] [--restart kill|term|none] [--port 8792] [--report file]`.
  `--addresses 1` puts every learner behind one address under the production limits.

  Background:
    Given the proctor serves the architecture course, with its ten quizzes, in production mode behind a trusted proxy
    And every client sends the address a proxy would forward it with, one address per learner or all learners behind one

  Scenario: No request fails with a server fault
    When the learners play and the chaos runs
    Then no response is a 5xx
    And a request that got no answer, was aborted or was refused with 429 or 503 is sent again until it has a verdict
    And no command is refused or rejected other than by the rejections a learner can cause: a handle claimed by another, a second open run, an answer after its task's time

  Scenario: Every accepted command is one event and no event is lost or doubled
    When every learner has finished
    Then the event stream of each learner, read over HTTP, holds seq 1 to n without a gap or a repeat
    And its events, in order and byte for byte, are the events of the accepted commands of the journal, the registration of a handle first
    And the same stream followed over a socket that was dropped and resumed holds every seq once, and what it received twice it kept once
    And a command that was sent twice at once, aborted and sent again, or sent again after its verdict, got the same verdict each time and made one event

  Scenario: Every submitted run is scored exactly once, as the quiz core scores it
    When every learner has finished
    Then for each learner the quiz core, deciding the journal's commands in order, accepts the commands the proctor accepted with the same events apart from the instants, and rejects those it rejected with the same rejection
    And each accepted submission has exactly one run-submitted event, no run is submitted twice and the number of run-submitted events is the number of accepted submit-run commands
    And every run of the journal, on all four challenges and over all ten quizzes, was submitted or voided

  Scenario: The read models equal their recomputation
    When every learner has finished
    Then the learner view of each learner equals the view the quiz core folds from its journal
    And the run view of each of its runs, sheet and answers and result, equals the view the quiz core builds
    And the daily, weekly, monthly and all-time leaderboards of every quiz and of the whole course equal the leaderboards the quiz core ranks from the transcripts of the journals
    And the own row of a sample of the learners on the all-time board equals the quiz core's
    And the crowd of every quiz equals the crowd the quiz core computes from the results of the journals

  Scenario: A handle has one holder, however many learners claim it
    Given several learners claim the same handle at the same moment, spelled with different case and white space
    When every learner has finished
    Then exactly one learner of each handle had its claim accepted and the others were rejected as claimed
    And the handle query answers that learner as the holder, and a rejected learner carried on anonymously

  Scenario: Presence converges after every reconnect
    When every learner has finished and every roster socket has shared its learner's latest state
    Then a fresh member of the roster room is sent exactly one session per learner, each showing the learner's latest state
    And after every learner has left, the room holds nobody else

  Scenario: Nothing is lost by the restart
    When the proctor is stopped and started once more after the audit
    Then the same audit of the streams, the views and the boards finds the same events and no difference

  Scenario: The latency is reported
    When every learner has finished
    Then the count, p50, p95, p99 and maximum of the commands and of the queries are reported, as each request took and as the learner waited with its retries
