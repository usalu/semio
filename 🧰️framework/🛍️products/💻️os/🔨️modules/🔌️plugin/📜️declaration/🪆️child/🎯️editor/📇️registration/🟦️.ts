import type {ChildEditorCapability} from '../🧭️source/🟦️.ts';
import type {EmbeddedChildAdmitterV1,EmbeddedChildVectorSourceV1} from '../../../../../../../../🔨️modules/🚪️io/🔌️ports/🧩️admission/🟦️.ts';
/** 📇️ A child editor registration binds metadata to the exact native owner admission capability. */
export interface ChildEditorRegistration<S>{readonly identity:ChildEditorCapability;readonly admission:EmbeddedChildAdmitterV1;readonly vector:EmbeddedChildVectorSourceV1;readonly snapshotType?:S;}
