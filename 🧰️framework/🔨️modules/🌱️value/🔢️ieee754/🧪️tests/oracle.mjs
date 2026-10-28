/** 🧪️ Independent test-only byte identity oracle for native IEEE scalar values. */
import lodash from 'lodash';
export function verifyIeeeIdentity(corpus) {
  for(const vector of corpus.cases) {
    const left=Uint8Array.from(Buffer.from(vector.left,'hex')),right=Uint8Array.from(Buffer.from(vector.right,'hex'));
    if(left.length!==vector.width/8||right.length!==vector.width/8)throw Error(vector.name+': word width');
    if(lodash.isEqual(left,right)!==vector.equal)throw Error(vector.name+': independent byte identity');
  }
  return {cases:corpus.cases.length};
}
