# Heating and Cooling Finger Exercise

Course-owned bilingual assignment, compiled with the shared `assignment` LaTeX class.

The assignment fixes exactly two things: the geometry of the tiny home and the structure of the submitted CSV file. It prescribes no concrete energy values. Climate data, U-values, air change, gains, heat pump performance, electricity price and all cooling design values are for the students to find and justify. The recommended route is to do everything directly in Grasshopper.

- The cover shows `\EnergyCoverAxonometry`, passed to the assignment class as `cover-content`: the same tiny home from the same south-east view as an illustration with staffage and without dimensions, labels or hidden edges. The dark clad box stands on a lawn with the shadows of house and canopy, a wooden step in front of the sliding door and a sofa behind the south window.
- The staffage is taken from the staffage catalogue `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog` (`person-drinking-coffee`, `three-seat-sofa`, `folding-deck-chair`, `japanese-maple`, `lavender-bush`, `boxwood-ball`, `domestic-cat`), exported with its command line as isometric light-line drawings and stored as the `\EnergyStaffage…` macros in metres on the picture plane, so a figure is placed with `staffage={x,y,z}` in building coordinates.
- `model.tex` holds the language-neutral geometry: the technical axonometry `\EnergyAxonometry` of the geometry page with dimensions, origin and coordinate triad, the cover axonometry, coordinate table and the numbered listing of the CSV rows.
- `Fingerübung_Heizen+Kühlen.csv` is the template handed out on Stud.IP: 60 rows, UTF-8 with BOM, CRLF, labels with empty values. Submissions keep its labels and order, put every field in double quotes, and are uploaded as `Fingerübung_Heizen+Kühlen_NAME.csv` by Friday 10:00.
- The sheet points to the Stud.IP template and to the reference solution `energiebilanz.csv` with its definition `entwurf.gh`. The reference is a different design; its numbers do not apply to the tiny home.
- `de.tex` and `en.tex` select the language explicitly; the CSV labels stay German in both.

The geometry is the tiny home of the lecture's finger exercise slides: a 3.0 m × 9.0 m × 2.6 m box with its long axis pointing north, one large south window, a glazed sliding door in the east facade and a canopy above that door. The lecture gives no dimensions, so the coordinates are set by this assignment. The German text addresses students with Du.

Build with `repo run @teaching/architecture-course:build-heating-cooling-en` or `repo run @teaching/architecture-course:build-heating-cooling-de`. Both producers publish through the course assignment transport and are dependencies of course development, site build and end-to-end targets.

`🧪️tests` contains a language-neutral feature contract, schema and geometry fixtures checked using existing test-only Ajv and mathjs. The test also compares the printed CSV listing against the template row by row and fails when a prescribed energy value appears. It is included in `repo run @teaching/architecture-course:test-assignment`.
