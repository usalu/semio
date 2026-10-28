import type {DrawingArtifact} from '../../🧬️schema/🟦️.ts';
import {DRAWING_VECTOR_SOURCE} from '../../🧬️schema/🧵️vector/🟦️.ts';
import {DRAWING_ADMITTER} from '../../🧬️schema/✅️validation/🟦️.ts';
import {DRAWING_EDITOR_DIALECT} from '../🟦️.ts';
import {surfaceAppId} from '../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🟦️.ts';
import type {ChildEditorRegistration} from '../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪆️child/🎯️editor/📇️registration/🟦️.ts';
const editorId=surfaceAppId(DRAWING_EDITOR_DIALECT,'editor');
const registration=(commandId:string):ChildEditorRegistration<DrawingArtifact>=>Object.freeze({identity:Object.freeze({...DRAWING_EDITOR_DIALECT,editorId,commandId}),admission:DRAWING_ADMITTER,vector:DRAWING_VECTOR_SOURCE});
export const DRAWING_CHILD_EDIT_PATH=registration('editPath');
export const DRAWING_CHILD_EDIT_FILL=registration('editFill');
export const DRAWING_CHILD_PATCH_LAYER=registration('patchLayer');

/** 📖️ Actual vector reads bind the typed Drawing owner and its mandatory producers without an edit verb. */
export const DRAWING_CHILD_VECTOR_READ:import('../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪆️child/📖️vector/🟦️.ts').ChildVectorReadRegistration<DrawingArtifact>=Object.freeze({identity:Object.freeze({...DRAWING_EDITOR_DIALECT}),admission:DRAWING_ADMITTER,vector:DRAWING_VECTOR_SOURCE});

export const DRAWING_CHILD_SET_ARTBOARD=registration('setArtboard');
