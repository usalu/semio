@capability-repo.tickets.issue-sync
@oracle-ticket-issue-sync-typescript
@comparison-ordered-json-v1
Feature: What a ticket owes its issue tracker is a fixed sequence of calls, and no failure is fatal
  A ticket never calls `gh`: it calls the `IssueTracker` port that `🧩️providers` declares, and what
  the lifecycle owes that port is exactly which calls it makes, in which order, with which arguments.
  The vectors script the port's answers — a created URL, an issue state, a milestone number, and the
  methods that fail — and the projection is the ordered interaction list plus the link the ticket
  ends up carrying. A management failure is never fatal to the ticket: it comes back as a warning and
  the ticket is still written. The reference is a second reader of the same scripts, written in
  TypeScript from the port's contract rather than from the Rust source.

  @id-a-new-ticket-creates-one-issue
  @level-fundamental
  @mode-differential
  Scenario: A ticket with no issue asks for its goal's milestone and creates exactly one issue
    Given the issue tracker scripts local://🐙️transcripts.json
    When a ticket with no management link is synchronised
    Then every implementation projects the same ordered calls and the same resulting issue link

  @id-an-existing-open-issue-is-left-alone
  @level-fundamental
  @mode-differential
  Scenario: A ticket that already carries an open issue creates nothing
    Given the issue tracker scripts local://🐙️transcripts.json
    When a ticket carrying an open issue is synchronised with reopening enabled
    Then every implementation projects a read and no reopen, and the same unchanged link

  @id-a-closed-issue-is-reopened
  @level-fundamental
  @mode-differential
  Scenario: A ticket whose issue is closed is reopened rather than duplicated
    Given the issue tracker scripts local://🐙️transcripts.json
    When a ticket carrying a closed issue is synchronised with reopening enabled
    Then every implementation projects a read followed by a reopen, and the same unchanged link

  @id-a-close-comments-labels-and-closes
  @level-fundamental
  @mode-differential
  Scenario: Closing a ticket comments the summary, applies the labels and closes the issue, in that order
    Given the issue tracker scripts local://🐙️transcripts.json
    When a ticket is closed both normally and in bulk
    Then every implementation projects the same ordered calls for each, with the bulk close skipping the comment and the labels

  @id-every-failure-becomes-a-warning
  @level-fundamental
  @mode-error
  Scenario: A tracker that refuses every call yields warnings, never a failed ticket
    Given the issue tracker scripts local://🐙️transcripts.json
    When every synchronisation is run against a tracker that fails
    Then every implementation projects the same warnings and no error
