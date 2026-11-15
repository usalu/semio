# Heating and Cooling Finger Exercise

Course-owned bilingual assignment, compiled with the shared `assignment` LaTeX class.

The supplied lecture slides provide the tasks and fixed system assumptions. The document explicitly adds fictional teaching geometry and thermal inputs so it is self-contained. It calculates a simplified annual heating balance and a solar-only sensible cooling design case, not a regulatory energy assessment or full HVAC design. Historical slide deadlines and platform links are excluded.

Build with `repo run @teaching/architecture-course:build-heating-cooling-en` or `repo run @teaching/architecture-course:build-heating-cooling-de`. Both producers publish through the course assignment transport and are dependencies of course development, site build and end-to-end targets.

`de.tex` and `en.tex` select language explicitly. `model.tex` supplies shared geometry and inputs. `🧪️tests` contains a language-neutral feature contract, schema and numerical fixtures checked using existing test-only Ajv and mathjs. They are included in `repo run @teaching/architecture-course:test-assignment`. Student PDFs contain no worked numerical solutions.
