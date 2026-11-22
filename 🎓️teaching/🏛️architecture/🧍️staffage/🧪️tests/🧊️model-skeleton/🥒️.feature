@capability-staffage-model-skeleton
Feature: The rig, its poses and its clips become a neutral node tree through the single evaluation
  Every bone of the rig is one node of the neutral model in its rest placement, parents first. The staffage joint becomes the mechanism: a revolute joint turns about the node's own +Z
  (the staffage lateral axis) within its angle limits in radians; a bone that a pose or a clip key spreads or twists, or whose joint limits a twist, is spherical about its bone axis; a
  prismatic joint slides along its axis in the parent frame; a fixed joint is fixed; the rig root is free. No kinematics is repeated: every pose and every clip frame is the bone frames
  of `evaluate`, expressed relative to the parent. A clip is sampled at its own fps with the closing key on the absolute timeline (root motion), rotation keys continuous.

  THE REFERENCES are `evaluate` (frames composed independently with three.js) for poses and clips, the kernel's own pose replay for the mechanism, and three's FBXLoader with AnimationMixer for the FBX.

  Scenario: One node per bone with the hand-computed mechanism
    Given the sample figure
    When its model is built
    Then the nodes are the bones parents first, the pelvis is free and every other joint is revolute about +Z with its limits in radians

  Scenario: Spread, twisted, sliding and fixed bones get their own joint
    Given the sample figure with a spread arm, a head with twist limits, a sliding foot and a fixed head
    When the model is built and every pose is replayed by the kernel
    Then each joint is spherical, spherical, prismatic and fixed as hand-computed, no joint limit is clamped and the replayed world matrices equal the evaluated frames

  Scenario: Every pose is the evaluated pose
    Given every named and key pose of the sample figure
    When the kernel replays the pose of the model
    Then every bone world matrix equals the frame of `evaluate` within 1 nm

  Scenario: Every clip is sampled at its fps
    Given the walk, idle and bag-swing clips at 30 fps
    When their animations are built
    Then each has ceil(duration x fps) + 1 keys from 0 to the duration, its events are the clip events, and the frames at the keys equal `evaluate`

  Scenario: Between the keys the animation stays within tolerance
    Given arbitrary times inside every clip
    When the kernel samples the animation there
    Then the bone origins are within 6 mm and the axes within 0.02 of the evaluated frames

  Scenario: Root motion and rotation continuity
    Given the planted walk clip
    When its closing key is read
    Then the root has advanced by the authored advance and no rotation key flips hemisphere

  Scenario: The FBX replays like the evaluation
    Given the FBX of each clip loaded by three's FBXLoader
    When an AnimationMixer plays it to arbitrary times
    Then every bone world position equals the evaluated one within 6 mm

  Scenario: The doe carries its paired glands under gravity
    Given the doe in standing, climbing and browsing poses
    Then the fixture angles are measured to vertical and both glands and teats have exposed skin at every scale whose feature admits them, and the suspension keeps exposed bare skin at all four scales

  Scenario: A canine lumbar column encloses an open spinal canal
    Given the Labrador's structural model at each of the four scales
    Then seven distinct vertebrae enclose a clear axial passage and each has solid bone ventral to the passage at every scale whose feature admits all seven
    And the lumbar bone keeps a member solid at all four scales

  Scenario: Four wheels support the skateboard at every digital scale whose feature admits them
    Given the 11.5 mm concave maple deck on its two trucks
    Then the physical model, rolling members and thermal wheel surfaces share four 54 mm wheels on a 171 mm track at every scale whose feature admits the wheels, and the deck keeps its gauge where the feature admits it
    And the deck and wheel thermal surfaces remain at all four scales

  Scenario: The tower crane trolley runs on four flanged wheels
    Given a trolley on two jib rails and two wire-rope sheaves
    Then all four treads meet the rail tops and both sheaves have a 12 mm groove at every scale whose feature admits them

  Scenario: A red squirrel articulates its tapered brush from the rump
    Given the seated, running and climbing squirrel across the four digital scales
    Then the six joined groups carry 23 caudal centra at every scale whose feature admits them, keep a member solid at all four scales, conserve tail mass and carry the guard locks on the same thermal surface

  Scenario: Display scale preserves an individual's long-bone shaft
    Given the eight adults in the anatomical fixture at all four scales
    Then the left and right humeri, femora, tibiae, radii and ulnae retain their own fine-scale cross section

  Scenario: Three road vehicles retain distinct wheel centres and heat surfaces
    Given the five-spoke city wheel, pressed delivery-van wheel and twelve-spoke period wheel
    Then the four tyres have open beads, coaxial hubs, separate brake and metal surfaces and physical wheel members at every scale whose feature admits them, and the tyres at every scale

  Scenario: Lifting-arm pivots and excavator rollers keep their physical contact geometry
    Given the scissor pin and crawler idler clearance dimensions in the fixture
    Then every link has three open bores and both tracks have eight supported rollers at every scale whose feature admits them

  Scenario: The industrial running gear carries each load to the ground
    Given a forklift, four-axle concrete mixer and motor scooter at every scale
    Then all their tyres are present in the structural load path and their thermal tyre surface

  Scenario: Cycle spokes meet their actual seats and the scooter axle passes through its cartridges
    Given the authored front and rear wheels at each digital scale
    Then each bicycle wheel carries thirty-six spokes and an independently ray-tested bore in each nipple
    And the scooter rear wheel has five windows and two 14 by 32 by 9 millimetre annular bearings at every scale whose feature admits them
