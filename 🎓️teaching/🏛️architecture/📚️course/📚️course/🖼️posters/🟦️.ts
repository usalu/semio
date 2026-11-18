/** 🖼️ The posters of the learning clips as the site's build emits them: a still frame of each clip's video (640 × 360), copied
 * into the content-hashed assets and keyed by the file name the clip's `poster` names.
 * @see ../../🔣️.json — the clips and their poster files
 * @see ../../🧪️tests/🎬️clips/🟦️.ts — holds this list to the clips on disk */
import poster0 from "../../../⚡️energy/🧲️physics/🎬️clip/🖼️posters/PhysicalFundamentals_FullSection.png?url";
import poster1 from "../../../⚡️energy/🌬️ventilation/🎬️clip/🖼️posters/Ventilation_01_WarumLueften.png?url";
import poster2 from "../../../⚡️energy/🔥️heating/🎬️clip/🖼️posters/Heating_01_Introduction.png?url";
import poster3 from "../../../⚡️energy/❄️cooling/🎬️clip/🖼️posters/Cooling_01_HeatingVsCooling.png?url";
import poster4 from "../../../⚡️energy/📊️demand/🎬️clip/🖼️posters/EnergyBalance_01_Energiebilanz.png?url";
import poster5 from "../../../🦗️grasshopper/🦴️anatomy/🎬️clip/🖼️posters/Grasshopper_01_Anatomy.png?url";
import poster6 from "../../../🦗️grasshopper/💊️capsule/🎬️clip/🖼️posters/Grasshopper_02_Capsule.png?url";
import poster7 from "../../../🦗️grasshopper/🗼️tower/🎬️clip/🖼️posters/Grasshopper_03_Tower.png?url";
import poster8 from "../../../🦗️grasshopper/🫗️casting/🎬️clip/🖼️posters/Grasshopper_04_Casting.png?url";
import poster9 from "../../../🦗️grasshopper/🌳️trees/🎬️clip/🖼️posters/Grasshopper_05_Trees.png?url";
import poster10 from "../../../📊️excel/🦴️anatomy/🎬️clip/🖼️posters/Excel_01_Anatomy.png?url";
import poster11 from "../../../📊️excel/📍️references/🎬️clip/🖼️posters/Excel_02_References.png?url";
import poster12 from "../../../📊️excel/📋️program/🎬️clip/🖼️posters/Excel_03_Program.png?url";
import poster13 from "../../../📊️excel/⚡️energy/🎬️clip/🖼️posters/Excel_04_Energy.png?url";
import poster14 from "../../../🏗️bim/📊️information/🎬️clip/🖼️posters/BIM_01_Information.png?url";
import poster15 from "../../../🏗️bim/🧬️objects/🎬️clip/🖼️posters/BIM_02_Objects.png?url";
import poster16 from "../../../🏗️bim/👁️representations/🎬️clip/🖼️posters/BIM_03_Representations.png?url";
import poster17 from "../../../🏗️bim/🧩️models/🎬️clip/🖼️posters/BIM_04_Models.png?url";
import poster18 from "../../../🏗️bim/🔌️exchange/🎬️clip/🖼️posters/BIM_05_Exchange.png?url";
import poster19 from "../../../🏗️bim/🔗️consistency/🎬️clip/🖼️posters/BIM_06_Consistency.png?url";
import poster20 from "../../../🎯️optimization/📉️extrema/🎬️clip/🖼️posters/Optimization_Extrema.png?url";
import poster21 from "../../../🎯️optimization/🎲️sampling/🎬️clip/🖼️posters/Optimization_Sampling.png?url";
import poster22 from "../../../🎯️optimization/🧪️experiments/🎬️clip/🖼️posters/Optimization_Experiments.png?url";
import poster23 from "../../../🎯️optimization/⚖️index/🎬️clip/🖼️posters/Optimization_Index.png?url";
import poster24 from "../../../🎯️optimization/🏆️pareto/🎬️clip/🖼️posters/Optimization_Pareto.png?url";
import poster25 from "../../../🎯️optimization/🧬️algorithms/🎬️clip/🖼️posters/Optimization_Algorithms.png?url";

/** 🎞️ The built URL of every poster the site ships, by poster file name. */
export const ARCHITECTURE_CLIP_POSTERS: Readonly<Record<string, string>> = {
  "PhysicalFundamentals_FullSection.png": poster0,
  "Ventilation_01_WarumLueften.png": poster1,
  "Heating_01_Introduction.png": poster2,
  "Cooling_01_HeatingVsCooling.png": poster3,
  "EnergyBalance_01_Energiebilanz.png": poster4,
  "Grasshopper_01_Anatomy.png": poster5,
  "Grasshopper_02_Capsule.png": poster6,
  "Grasshopper_03_Tower.png": poster7,
  "Grasshopper_04_Casting.png": poster8,
  "Grasshopper_05_Trees.png": poster9,
  "Excel_01_Anatomy.png": poster10,
  "Excel_02_References.png": poster11,
  "Excel_03_Program.png": poster12,
  "Excel_04_Energy.png": poster13,
  "BIM_01_Information.png": poster14,
  "BIM_02_Objects.png": poster15,
  "BIM_03_Representations.png": poster16,
  "BIM_04_Models.png": poster17,
  "BIM_05_Exchange.png": poster18,
  "BIM_06_Consistency.png": poster19,
  "Optimization_Extrema.png": poster20,
  "Optimization_Sampling.png": poster21,
  "Optimization_Experiments.png": poster22,
  "Optimization_Index.png": poster23,
  "Optimization_Pareto.png": poster24,
  "Optimization_Algorithms.png": poster25,
};
