"""🔬 Independently measure actual semantic union without adding overlapping owner quantities."""
import copy,importlib,json,math
from pathlib import Path
from jsonschema import Draft202012Validator
B=Path(__file__).parent;CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));ORACLE=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🧪️tests.🔬️scipy')


def test_independent_union_matches_neutral_real_curve_height_and_two_nanometre_controls():
    """⚖️ Compare independently declared real union quantities, preserving finite height and tiny overlap."""
    schema=json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'));schema['$defs']=json.loads((B.parent/'🧬️.schema.json').read_text(encoding='utf8'))['$defs'];Draft202012Validator(schema).validate(CASE)
    for row in CASE['cases']:
        value,error=ORACLE.union_volume(row['operands']);expected=row.get('expected_volume',row.get('expected_pi_coefficient',0)*math.pi)
        assert value>0 and error>=0 and abs(value-expected)<=8*error+64*math.ulp(expected),(row['id'],value,expected,error)
        if row['id']=='two-nanometre-overlap':assert value<sum(ORACLE.volume(part)[0] for part in row['operands'])


def test_independent_union_progress_cancellation_and_isolation():
    """🚦 Cancel real section integration after progress and preserve every original operand."""
    operands=CASE['cases'][-1]['operands'];before=copy.deepcopy(operands);events=[]
    try:ORACLE.union_volume(operands,progress=events.append,cancel=lambda:bool(events))
    except InterruptedError:pass
    else:raise AssertionError('Independent union ignored cancellation after progress')
    assert events and operands==before
    assert ORACLE.union_volume([])==(0.,0.)
