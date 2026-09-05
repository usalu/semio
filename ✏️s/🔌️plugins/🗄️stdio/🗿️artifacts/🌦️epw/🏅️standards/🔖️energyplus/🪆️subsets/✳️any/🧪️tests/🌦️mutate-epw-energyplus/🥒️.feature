@capability-epw-energyplus-mutate
@oracle-csv-epw-energyplus-mutate
@comparison-semantic-epw-v1
@mutations-epw-energyplus-any
Feature: Apply every typed EnergyPlus EPW mutation to the only EPW this repository has
  NO REAL ENERGYPLUS WEATHER FILE EXISTS IN THIS REPOSITORY. The repository was searched
  (`*.epw`/`*.tmy`/`*.tmy3`, and every weather-shaped asset under `♻️mit-bestand`) and the only two
  `.epw` files tracked anywhere are byte-identical copies of the same 32-line handcrafted stub whose
  own COMMENTS 1 line says so directly: "semio W0 handcrafted EPW fixture -- structurally valid,
  plausible values, not a real station record." The input this feature uses,
  asset://📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw, IS that
  stub: a real, structurally valid LOCATION for Hannover (52.37N, 9.74E) with a real 2009 ASHRAE
  Handbook DESIGN CONDITIONS block, but only 24 hourly records (one day, hour-ending 1..24), not the
  8,760 a genuine annual TMY file carries. Fixing this properly needs a genuine TMY3 export for
  Hannover whose own DESIGN CONDITIONS header happens to match this stub's real ASHRAE 2009 block
  (or the stub's header regenerated from whichever real station is sourced) — sourcing that file is
  out of this ticket's scope, and no larger fixture was fabricated to paper over its absence: padding
  this file with invented hourly values and presenting it as real would be worse than the honest
  32-line stub it already is.

  `csv` 1 (already linked, no new dependency) is the reference for this format's RECORD half only:
  an EPW file is 8 fixed header lines followed by N comma-separated 35-column hourly records, and
  `csv` (`has_headers(false)`, `flexible(true)`) genuinely reads AND writes that record grid, exactly
  as the sibling `csv-rfc4180-mutate` entry does for its own format. It carries zero knowledge of
  EnergyPlus header GRAMMAR — LOCATION's 10-field shape, DESIGN CONDITIONS'/TYPICAL EXTREME PERIODS'/
  GROUND TEMPERATURES'/HOLIDAYS-DST's/both COMMENTS lines' retained-verbatim convention, DATA
  PERIODS' `N,recordsPerHour,(name,day,start,end)×N` shape — because no third-party crate on
  crates.io validates that grammar. `epw-rs` was surveyed and NOT registered: it is alpha-quality
  and read-only (confirmed: no writer in its public API), which the fleet brief's §6 rules out as a
  differential producer regardless.

  This is why the 13 declared kinds split into two evidence tiers rather than one:
  `no-mutation`/`insert-record`/`remove-record`/`set-record-field` touch ONLY the record grid, so
  `csv` is a genuine second PRODUCER for them and they are typed `@mode-differential`. The remaining
  nine (`set-snapshot`/`set-location`/`set-design-conditions`/`set-typical-extreme-periods`/
  `set-ground-temperatures`/`set-holidays-dst`/`set-comments1`/`set-comments2`/`set-data-periods`)
  touch the header, where this subset's oracle module writes the bytes itself (hand-rolled,
  independent of the subject crate — the oracle role never links it) with no third-party validation
  of EPW grammar to lean on, so they are typed `@mode-property`: only this module's own
  self-consistency (a mutation followed by its own inverse restores the original) is asserted, read
  back through `csv`'s generic comma-grid reader for structural evidence, never claimed as agreement
  between two independent producers it does not have.

  Every scenario copies the fixture into the case work directory before touching it; the committed
  stub is never written to.

  Honest limits: the Rust SUBJECT phase does not compile (a concurrent session's in-flight
  `ManuallyDrop<Option<RetainedJobPayload>>` migration in `semio-framework-job`, which the whole
  subject host depends on); every scenario below is written and `sut`-gated so it compiles into the
  subject role the moment that lands, and only the oracle side is verified here.
  A further, format-specific limit for the eventual subject comparison: this subset's own schema
  stores every record column as `String` specifically to avoid float-reformatting drift (see
  `../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`'s own module
  doc), and its own `codec_retention_law` unit test already proves `encode_epw(decode_epw(fixture))`
  is BYTE-IDENTICAL to this exact fixture. That means the identity-round-trip scenario below is
  expected to reproduce the input byte-for-byte once the subject compiles — correctly, not as a
  smuggled-bytes defect: EPW carries none of the writer freedom (object layout, whitespace, line
  terminator choice) that makes byte-identity suspicious for the other formats in this wave, so this
  scenario's genuineness rests on `identity_round_trip` calling `decode_epw`/`encode_epw` (a real
  `Result`-returning parse into `EpwSnapshot`, then a reserialize from that struct alone) rather than
  on an artificial "must differ" assertion this format cannot honestly satisfy.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real weather file's record grid
    Given the real input weather file asset://📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id               | params |
      | insert-record      | {"index": 5, "fields": ["2026","1","15","99","0","?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9","2.5","-1.0","70","101100","500","1200","280","300","200","100","30000","25000","5000","1500","250","3.0","5","3","20.0","22000","0","999999999","14","0.081","0","88","0.2","0","0"]} |
      | remove-record      | {"index": 10} |
      | set-record-field   | {"recordIndex": 3, "fieldIndex": 6, "value": "12.3"} |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real weather file's record grid
    Given the real input weather file asset://📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection

  @id-mutate
  @level-exhaustive
  @mode-property
  Scenario Outline: Apply <id> to the real weather file's header
    Given the real input weather file asset://📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                           | params |
      | set-snapshot                  | {"snapshot": {"location": {"city":"Berlin","stateProvince":"Berlin","country":"DEU","source":"semio-fixture","wmo":"10382","latitude":"52.52","longitude":"13.405","timeZone":"1.0","elevation":"34.0"},"designConditions":"DESIGN CONDITIONS,0","typicalExtremePeriods":"TYPICAL/EXTREME PERIODS,0","groundTemperatures":"GROUND TEMPERATURES,0","holidaysDst":"HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0","comments1":"COMMENTS 1,wave 7 set-snapshot replacement","comments2":"COMMENTS 2,wave 7 set-snapshot replacement","dataPeriods":{"recordsPerHour":1,"periods":[{"name":"Data","startDayOfWeek":"Monday","startDate":" 1/ 2","endDate":" 1/ 2"}]},"records":[["2026","1","16","1","0","?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9","-5.0","-9.0","85","101100","0","0","280","0","0","0","0","0","0","0","200","2.0","3","2","20.0","22000","0","999999999","14","0.081","0","88","0.2","0","0"],["2026","1","16","2","0","?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9","-4.5","-8.5","84","101100","0","0","280","0","0","0","0","0","0","0","205","2.1","3","2","20.0","22000","0","999999999","14","0.081","0","88","0.2","0","0"]]}} |
      | set-location                  | {"location": {"city":"Berlin","stateProvince":"Berlin","country":"DEU","source":"semio-fixture","wmo":"10382","latitude":"52.52","longitude":"13.405","timeZone":"1.0","elevation":"34.0"}} |
      | set-design-conditions         | {"value": "DESIGN CONDITIONS,1,Wave 7 mutation test value"} |
      | set-typical-extreme-periods   | {"value": "TYPICAL/EXTREME PERIODS,1,Wave 7 mutation test period"} |
      | set-ground-temperatures       | {"value": "GROUND TEMPERATURES,1,Wave 7 mutation test depth"} |
      | set-holidays-dst              | {"value": "HOLIDAYS/DAYLIGHT SAVINGS,Yes,1,1,1"} |
      | set-comments1                | {"value": "COMMENTS 1,Wave 7 mutation test comment"} |
      | set-comments2                | {"value": "COMMENTS 2,Wave 7 mutation test comment"} |
      | set-data-periods              | {"dataPeriods": {"recordsPerHour":1,"periods":[{"name":"Data","startDayOfWeek":"Monday","startDate":" 1/ 2","endDate":" 1/ 2"}]}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real weather file
    Given the real input weather file asset://📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original weather file
    Examples:
      | id                           | params |
      | insert-record                  | {"index": 5, "fields": ["2026","1","15","99","0","?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9","2.5","-1.0","70","101100","500","1200","280","300","200","100","30000","25000","5000","1500","250","3.0","5","3","20.0","22000","0","999999999","14","0.081","0","88","0.2","0","0"]} |
      | remove-record                  | {"index": 10} |
      | set-record-field               | {"recordIndex": 3, "fieldIndex": 6, "value": "12.3"} |
      | set-snapshot                   | {"snapshot": {"location": {"city":"Berlin","stateProvince":"Berlin","country":"DEU","source":"semio-fixture","wmo":"10382","latitude":"52.52","longitude":"13.405","timeZone":"1.0","elevation":"34.0"},"designConditions":"DESIGN CONDITIONS,0","typicalExtremePeriods":"TYPICAL/EXTREME PERIODS,0","groundTemperatures":"GROUND TEMPERATURES,0","holidaysDst":"HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0","comments1":"COMMENTS 1,wave 7 set-snapshot replacement","comments2":"COMMENTS 2,wave 7 set-snapshot replacement","dataPeriods":{"recordsPerHour":1,"periods":[{"name":"Data","startDayOfWeek":"Monday","startDate":" 1/ 2","endDate":" 1/ 2"}]},"records":[["2026","1","16","1","0","?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9","-5.0","-9.0","85","101100","0","0","280","0","0","0","0","0","0","0","200","2.0","3","2","20.0","22000","0","999999999","14","0.081","0","88","0.2","0","0"],["2026","1","16","2","0","?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9","-4.5","-8.5","84","101100","0","0","280","0","0","0","0","0","0","0","205","2.1","3","2","20.0","22000","0","999999999","14","0.081","0","88","0.2","0","0"]]}} |
      | set-location                   | {"location": {"city":"Berlin","stateProvince":"Berlin","country":"DEU","source":"semio-fixture","wmo":"10382","latitude":"52.52","longitude":"13.405","timeZone":"1.0","elevation":"34.0"}} |
      | set-design-conditions          | {"value": "DESIGN CONDITIONS,1,Wave 7 mutation test value"} |
      | set-typical-extreme-periods    | {"value": "TYPICAL/EXTREME PERIODS,1,Wave 7 mutation test period"} |
      | set-ground-temperatures        | {"value": "GROUND TEMPERATURES,1,Wave 7 mutation test depth"} |
      | set-holidays-dst               | {"value": "HOLIDAYS/DAYLIGHT SAVINGS,Yes,1,1,1"} |
      | set-comments1                 | {"value": "COMMENTS 1,Wave 7 mutation test comment"} |
      | set-comments2                 | {"value": "COMMENTS 2,Wave 7 mutation test comment"} |
      | set-data-periods               | {"dataPeriods": {"recordsPerHour":1,"periods":[{"name":"Data","startDayOfWeek":"Monday","startDate":" 1/ 2","endDate":" 1/ 2"}]}} |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real weather file
    Given the real input weather file asset://📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original weather file

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real weather file, where byte identity IS the correct answer
    Given the real input weather file asset://📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw
    When the weather file is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are bit-identical to the input, which is EPW's absence of writer freedom working correctly rather than a byte pass-through
