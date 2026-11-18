Feature: Search, filter and sort

  Scenario: Titles are found in either language
    Given the archive rows
    When the reader types "Stieleiche" or "pendler zu FUSS"
    Then the oak and the commuter are found regardless of case and accents

  Scenario: Tags and summaries need loaded assets
    Given the commuter is loaded and the oak is not
    When the reader searches for the tag "commuter"
    Then only the commuter is found

  Scenario: Filters narrow together
    Given categories, analytics, a height range and a completion
    Then a row must satisfy all of them

  Scenario: Sorting is stable and honest about unknown values
    Given a sort by mass
    Then rows whose mass is not loaded yet come last in both directions

  Scenario: Power is shown, filtered and sorted without loading anything
    Given every row carries the total power of all its elements in watts, zero for an asset without a power-drawing element
    When the reader sorts by power or limits it to a range in watts
    Then the rows are ordered and narrowed by that power, using the loaded asset when there is one
