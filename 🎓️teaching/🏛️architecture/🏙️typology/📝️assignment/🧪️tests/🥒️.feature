@capability-typology-assignment
@oracle-ajv
@oracle-mathjs
@oracle-d3-array
@comparison-ordered-json-v1
Feature: One bilingual brief specifies the parametric design of a modular façade typology
  The language-neutral v1 fixture specifies a group of four that develops eight module types on a
  4.5 by 4.5 metre grid, four parts split between dMA and NGS, an example naming scheme and the
  individual submission files. ajv validates the versioned schema, mathjs independently evaluates
  counts and areas, and d3-array independently enumerates names and variant files.
  None of these references reads the document implementation.

  @id-composition
  @level-fundamental
  @mode-differential
  Scenario: Four members with one corner and one middle module each develop eight module types
    Given a group of four members and one corner and one middle module per member
    Then the group develops four corner and four middle module types
    And the façade of all eight module types has twelve sides
    And every member draws twenty axonometries: variants zero to four of both modules as visualization and as simulation model
    And the base room of a module measures 4.5 by 4.5 metres

  @id-naming
  @level-fundamental
  @mode-differential
  Scenario: Example names decode position, kind, window count and window format
    Given the codes M middle, E corner, W wall, B balcony, H portrait and Q landscape
    Then M-B-1-Q is a middle balcony module with one landscape window
    And M-W-2-H is a middle wall module with two portrait windows
    And E-B-2-QH is a corner balcony module with two windows in landscape and portrait format
    And the split decoder agrees with an independent enumeration of every scheme name

  @id-submission
  @level-fundamental
  @mode-differential
  Scenario: The individual archive holds twenty-six files with one image per model, module and variant
    Given the visualization and simulation model, the middle and corner module and variants zero to four
    Then the archive contains twenty images, one Grasshopper model, four GLB models and one CSV file
    And the image names run from Variante0 to Variante4 for Mittelmodul and Eckmodul, the simulation images with the Simulation suffix

  Scenario: Simulation model axonometries show U-values and summer airflow
    Given the planar surfaces of the simulation model and their surface types from Part C
    Then the simulation model, its calculations and its axonometries belong to Part D of NGS
    And Part B of dMA keeps only the visualization model and its axonometries
    And the simulation axonometries repeat the visualization axonometries from the same viewpoint
    And each surface is coloured by its U-value on the fixed colour scale of the layout template
    And that colour is the material of the surface's mesh in the simulation GLB
    And the airflow estimated for summer with full solar load is drawn as arrows

  @id-languages
  @level-fundamental
  @mode-conformance
  Scenario: English and German documents share one structure
    Given separate English and German documents using the assignment class
    Then both open with the assignment cover: the cover image, four facts, the dMA and NGS partner windows and the LUH logo
    And both use the same eight headings, four titled tables and table row counts
    And the workflow table lists parts A to D with their subjects
    And every submission file name and example module name is set in monospace
    And the three example axonometries appear in the overview table in both languages
    And the submission date appears in each language's notation

  @id-courseOwnership
  @level-fundamental
  @mode-differential
  Scenario: The course publishes the typology assignment from its authored leaf
    Given the Architecture course Parametric topic
    Then the assignment leaf holds both language mains, contents, the bounded JPEG cover, the light and dark LUH logos and three bounded PNG axonometries
    And each localized download names its course-owned producer and publication
    And the dev, dev-site and build targets depend on both producers
