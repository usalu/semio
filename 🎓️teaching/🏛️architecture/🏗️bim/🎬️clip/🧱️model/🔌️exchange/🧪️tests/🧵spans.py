"""🧵 Native quoted STEP text always annotates real serialized characters."""
import importlib,json,unittest
from pathlib import Path
native=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

class NativeStepSpanContract(unittest.TestCase):
    """🔖 Empty, ordinary and escaped text preserve lexical bytes and visible span extents."""
    def check(self,parts,serialized,annotated):
        """🪡 Validate the actual native writer against one portable lexical example."""
        writer=native._Writer(1,[])
        writer.raw(parts)
        self.assertEqual(writer.lines,[serialized])
        self.assertTrue(all(0<=s['start']<s['end']<=len(serialized) for s in writer.spans))
        self.assertEqual([serialized[s['start']:s['end']] for s in writer.spans if s['role']=='value'],[annotated])

    def test_quoted_empty_normal_and_escaped_strings_have_real_token_spans(self):
        for case in CASE['cases']:
            with self.subTest(case=case['id']):
                self.check(native._str(case['value'],'value'),case['quoted'],case['annotated'])

    def test_typed_text_and_label_reuse_the_same_quoted_token_contract(self):
        for kind in CASE['typed']:
            for case in CASE['cases']:
                with self.subTest(kind=kind,case=case['id']):
                    self.check(native._value({'value':case['value'],'ifc_type':kind}),kind.upper()+'('+case['quoted']+')',case['annotated'])
