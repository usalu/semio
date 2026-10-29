"""🧪 Native-independent chapter source contracts for the remodelled geometry interface."""

import ast
import builtins
import symtable
import json
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
CLIP = HERE.parents[1]
CASES = json.loads((HERE / "🔣️.json").read_text(encoding="utf-8"))


class BimChapterSourceContract(unittest.TestCase):
    """🧩 Reject stale provider keys and duplicated geometry before native rendering."""

    def test_native_class_matrix_and_chimney_meanings_include_every_discipline(self):
        """🧬 Execute the actual matrix prefix and bilingual class rows with newer native families."""
        from types import SimpleNamespace
        case=CASES['native_class_matrix']
        tree=ast.parse((CLIP/'🧩️models'/'🐍️.py').read_text())
        scene=next(node for node in tree.body if isinstance(node,ast.ClassDef))
        schema=next(node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name=='_schema')
        stop=next(i for i,node in enumerate(schema.body) if isinstance(node,ast.Assign) and any(isinstance(target,ast.Name) and target.id=='schema' for target in node.targets))
        schema.body=schema.body[:stop]+[ast.Return(value=ast.Name(id='classes',ctx=ast.Load()))]
        rows=next(node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name=='_discipline_rows')
        space={'DISCIPLINES':tuple(case['counts'])}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[schema,rows],type_ignores=[])),'native-class-callback','exec'),space)
        subject=type('Matrix',(),{'_schema':space['_schema'],'_discipline_rows':space['_discipline_rows']})()
        subject.info={name:{'counts_by_class':counts} for name,counts in case['counts'].items()}
        subject.language=lambda:'en'
        subject._piloti_label=lambda:('columns','Stützen')
        self.assertEqual(set(subject._schema()),set().union(*case['counts'].values()))
        for discipline in case['counts']:
            if 'IfcChimney' in case['counts'][discipline]:
                self.assertEqual(dict(subject._discipline_rows(discipline))['IfcChimney'],(case['chimney']['en'],case['chimney']['de']))

    def test_service_placement_oracle_checks_native_parts_without_requiring_an_axis(self):
        """🧱 Execute the real generic placement loop on a prism-only native chimney."""
        import math
        from types import SimpleNamespace
        case=CASES['part_only_service']
        tree=ast.parse((CLIP/'🧪️tests'/'🐍️.py').read_text())
        loop=next(node for node in ast.walk(tree) if isinstance(node,ast.For) and isinstance(node.target,ast.Name) and node.target.id=='service' and ast.unparse(node.iter)=='services.values()')
        subject=unittest.TestCase()
        subject.api=SimpleNamespace(part_vertices=lambda part:part['vertices'])
        space={'self':subject,'math':math,'services':{case['tag']:case},'box':{'height':case['height']}}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[loop],type_ignores=[])),'actual-service-placement-loop','exec'),space)

    def test_standard_hook_registers_native_contracts_exactly_once(self):
        """🧩 Execute the real test hook with observable module and function dispatch."""
        from collections import Counter
        from types import SimpleNamespace
        prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
        imports,classes,called=[],[],[]
        cases={prefix+case['module']:case for case in CASES['standard_registration']}
        def imported(name):
            imports.append(name)
            path=CLIP/Path(*name.removeprefix(prefix).split('.'))
            tree=ast.parse(path.with_suffix('.py').read_text())
            names=[node.name for node in tree.body if isinstance(node,ast.FunctionDef) and node.name.startswith('test_')]
            def probe(key):
                def check():
                    called.append((name,key))
                return check
            return SimpleNamespace(**{key:probe(key) for key in names},run=lambda root:None)
        class Loader:
            def loadTestsFromName(self,name):
                classes.append(name)
                return unittest.TestSuite()
            def discover(self,*args,**kwargs):
                return unittest.TestSuite()
        tree=ast.parse((CLIP/'🧪️tests'/'🐍️.py').read_text())
        hook=next(node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name=='load_tests')
        space={'unittest':unittest,'importlib':SimpleNamespace(import_module=imported),'CLIP':CLIP,'ROOT':CLIP.parents[4]}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[hook],type_ignores=[])),'standard-hook','exec'),space)
        suite=space['load_tests'](Loader(),unittest.TestSuite(),None)
        result=unittest.TestResult()
        suite.run(result)
        self.assertEqual(result.errors,[])
        for name,case in cases.items():
            with self.subTest(module=name):
                scheduled=classes if case['kind']=='class' else imports
                self.assertEqual(Counter(scheduled)[name],1)
                path=CLIP/Path(*case['module'].split('.'))
                tree=ast.parse(path.with_suffix('.py').read_text())
                declarations=[node.name for node in ast.walk(tree) if isinstance(node,ast.FunctionDef) and node.name.startswith('test_')]
                self.assertEqual(set(declarations),set(case['tests']))
                if case['kind']=='function':
                    self.assertEqual(Counter(key for module,key in called if module==name),Counter(case['tests']))

    def test_takeoff_glass_hatches_use_native_filling_hosts(self):
        import numpy as np
        from types import SimpleNamespace
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        case=CASES["glass_hatching"]
        recorded=[]
        class Group(list):
            def add(self,*parts):
                self.extend(parts)
                return self
        def hatch(rings,color,*args):
            if color=="glass":
                recorded.extend(Polygon(np.asarray(ring)[:,:2]) for ring in rings)
            return None
        tree=ast.parse((CLIP/'👁️representations'/'🐍️.py').read_text())
        scene=next(n for n in tree.body if isinstance(n,ast.ClassDef))
        method=next(n for n in scene.body if isinstance(n,ast.FunctionDef) and n.name=='_takeoff_elevation')
        space={'np':np,'_drafting':SimpleNamespace(region=lambda *args:None),'BACKGROUND':'background','GOOD':'wall','DATA':'glass'}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),'actual-takeoff-callback','exec'),space)
        callback=type('Takeoff',(),{method.name:space[method.name]})()
        callback._board_elevation=lambda *args:Group([SimpleNamespace(at=lambda s,z:np.array([s,z,0]))])
        callback._state=lambda k:case
        callback._wall_of=lambda state:(state['wall'],state['opening'])
        callback._geo=lambda state:state
        callback.hatch=hatch
        callback._takeoff_elevation(0,None)
        self.assertEqual(len(recorded),len(case['expected']))
        actual=unary_union(recorded)
        expected=unary_union([Polygon(ring) for ring in case['expected']])
        self.assertLess(actual.symmetric_difference(expected).area,1e-12)
        self.assertAlmostEqual(actual.area,case['area'],places=10)

    def test_entrance_cut_includes_only_material_at_its_native_height(self):
        import numpy as np
        from types import SimpleNamespace
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        case=CASES["entrance_section"]
        recorded=[]
        class Group:
            def __init__(self,*args,**kwargs):
                self.items=list(args)
            def add(self,*parts):
                self.items.extend(parts)
                return self
            def move_to(self,*args):
                return self
        def polygon(ring,*args):
            recorded.append(Polygon(np.asarray(ring)[:,:2]))
            return Group()
        tree=ast.parse((CLIP/'📊️information'/'🐍️.py').read_text())
        scene=next(n for n in tree.body if isinstance(n,ast.ClassDef))
        method=next(n for n in scene.body if isinstance(n,ast.FunctionDef) and n.name=='_wall_detail')
        provider=SimpleNamespace(filling_geometry=lambda tag,model:case['door'],part_regions=lambda part,z:[part['outline']])
        draft=SimpleNamespace(View=lambda *args,**kwargs:lambda x,y,z:(x,y,0),polygon=polygon)
        space={'np':np,'_model':provider,'_draft':draft,'_engine':SimpleNamespace(VGroup=Group),'_panels':SimpleNamespace(frame_box=lambda *args,**kwargs:Group()),'PX':0,'PW':6,'MUTED':'muted','INK':'ink','GLASS':'glass','LABEL':12,'SMALL':10}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),'actual-entrance-cut','exec'),space)
        callback=type('Entrance',(),{method.name:space[method.name]})()
        callback._g={'curtain_wall':case['curtain_wall'],'storeys':{'ground':case['cut_z']-1.2}}
        callback._m={}
        callback._say=lambda *args,**kwargs:Group()
        callback.tr=lambda en,de:en
        callback._wall_detail()
        actual=unary_union(recorded)
        expected=unary_union([Polygon(ring) for ring in case['expected']])
        self.assertLess(actual.symmetric_difference(expected).area,1e-12)
        self.assertAlmostEqual(actual.area,case['area'],places=10)

    def test_provenance_helper_distinguishes_all_native_bases_in_both_languages(self):
        tree=ast.parse((CLIP/'🔌️exchange'/'🐍️.py').read_text())
        scene=next(n for n in tree.body if isinstance(n,ast.ClassDef))
        method=next(n for n in scene.body if isinstance(n,ast.FunctionDef) and n.name=='_basis')
        space={}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),'actual-provenance-label','exec'),space)
        callback=type('Basis',(),{method.name:space[method.name]})()
        for language in ('en','de'):
            callback.tr=lambda en,de,language=language:en if language=='en' else de
            for case in CASES['provenance_labels']['cases']:
                self.assertEqual(callback._basis({'basis':case['basis']}),case[language])
            for value in CASES['provenance_labels']['invalid']:
                with self.subTest(language=language,basis=value),self.assertRaises(ValueError):
                    callback._basis({'basis':value})

    def test_exchange_count_card_reports_registered_products_without_historic_range(self):
        from types import SimpleNamespace
        case=CASES['column_count_card']
        class Group:
            def __init__(self,*args,**kwargs):
                self.items=list(args)
        tree=ast.parse((CLIP/'🔌️exchange'/'🐍️.py').read_text())
        scene=next(n for n in tree.body if isinstance(n,ast.ClassDef))
        methods=[n for n in scene.body if isinstance(n,ast.FunctionDef) and n.name in ('_beat_interface','_count_legend')]
        method=next(n for n in methods if n.name=='_beat_interface')
        stop=next(i for i,n in enumerate(method.body) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='arrows' for t in n.targets))
        method.body=method.body[:stop]
        space={'_engine':SimpleNamespace(VGroup=Group),'_pick':lambda *args:None,'TAG':'19','STAR':'*','S_SMALL':10,'ARCH':'architecture','STRUCT':'structure','MUTED':'muted','GOOD':'good','WARN':'warn'}
        exec(compile(ast.fix_missing_locations(ast.Module(body=methods,type_ignores=[])),'actual-count-card','exec'),space)
        callback=type('Counts',(),{name:space[name] for name in ('_beat_interface','_count_legend')})()
        facts=SimpleNamespace(m={'schema_id':case['schema_id']},objects=case['objects'],g={'pilotis':[{}]*case['columns']},excerpt={})
        for language in ('en','de'):
            labels=[]
            def label(en,de,*args,**kwargs):
                labels.append(en if language=='en' else de)
                return Group()
            callback.label=label
            callback.tr=lambda en,de:en if language=='en' else de
            callback.villa_axon=callback.badge=callback.text=callback.step_panel=lambda *args,**kwargs:Group()
            callback._beat_interface(facts)
            text=' '.join(labels)
            for required in case[language]:
                self.assertIn(required,text)
            for forbidden in case['forbidden']:
                self.assertNotIn(forbidden,text)

    def test_members_card_uses_actual_clipped_mesh_counts_and_selected_supports(self):
        from copy import deepcopy
        from types import SimpleNamespace
        case=CASES['irregular_members']
        class Group:
            def __init__(self,*args,**kwargs):
                self.submobjects=list(args)
            def add(self,*args):
                self.submobjects.extend(args)
                return self
            def move_to(self,*args):
                return self
            def get_center(self):
                return [0,0,0]
        tree=ast.parse((CLIP/'🧩️models'/'🐍️.py').read_text())
        scene=next(n for n in tree.body if isinstance(n,ast.ClassDef))
        method=next(n for n in scene.body if isinstance(n,ast.FunctionDef) and n.name=='_members')
        stop=next(i for i,n in enumerate(method.body) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Tuple) and any(isinstance(p,ast.Name) and p.id=='rows' for p in t.elts) for t in n.targets))
        method.body=method.body[:stop]+[ast.Return(value=ast.Name(id='spec',ctx=ast.Load()))]
        space={'VGroup':Group,'Circle':Group,'LEFT_PLAN':[-3,0,0],'STRUCT':'structure','GOOD':'good','DATA':'data'}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),'actual-irregular-member-card','exec'),space)
        callback=type('Members',(),{method.name:space[method.name]})()
        callback._chains=callback.scale_bar=callback.villa_plan=lambda *args,**kwargs:Group()
        callback.analysis_view=lambda *args,**kwargs:SimpleNamespace(members=Group(Group(),Group()),shells=Group(Group(),Group(),Group()),nodes=Group(*[Group() for _ in range(5)]),outline=Group(),remove=lambda *args:None)
        for language in ('en','de'):
            callback.analysis=deepcopy(case['analysis'])
            callback.language=lambda:language
            callback.tr=lambda en,de:en if language=='en' else de
            rows=callback._members()
            self.assertEqual([(row[0],row[1]) for row in rows],[(row['name'],row['count']) for row in case['expected']])
            self.assertIn(case['member_scope'][language],rows[0][2][0 if language=='en' else 1])
        callback.analysis['counts']['shells']+=1
        with self.assertRaises(ValueError):
            callback._members()

    def test_analysis_narration_preserves_selected_births_and_unsolved_scope(self):
        for case in CASES['selected_analysis_scope']:
            tree=ast.parse((CLIP/case['chapter']/'🐍️.py').read_text())
            scene=next(n for n in tree.body if isinstance(n,ast.ClassDef))
            field=next(n for n in scene.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='NARRATION' for t in n.targets))
            row=next(row for row in ast.literal_eval(field.value) if row[0]==case['clause'])
            for index,language in enumerate(('en','de'),1):
                with self.subTest(chapter=case['chapter'],clause=case['clause'],language=language):
                    self.assertIn(case[language],row[index])

    def test_roundtrip_units_accept_native_dimensionless_station_fractions(self):
        tree=ast.parse((CLIP/'🔌️exchange'/'🐍️.py').read_text())
        helper=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='_units')
        policy=next(n for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='ALLOWED_UNITS' for t in n.targets))
        space={}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[helper,policy],type_ignores=[])),'actual-roundtrip-unit-policy','exec'),space)
        case=CASES['roundtrip_units']
        actual=space['_units'](case['document'])
        self.assertEqual(actual,set(case['expected']))
        self.assertTrue(actual<=space['ALLOWED_UNITS'])
        self.assertFalse(space['_units'](case['unknown'])<=space['ALLOWED_UNITS'])

    def test_chapters_use_the_current_geometry_contract(self):
        for chapter in CASES["chapters"]:
            source = (CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8")
            tree = ast.parse(source)
            scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
            for rule in chapter["methods"]:
                with self.subTest(chapter=chapter["folder"], method=rule["name"]):
                    method = next(node for node in scene.body if isinstance(node, ast.FunctionDef) and node.name == rule["name"])
                    text = ast.get_source_segment(source, method)
                    keys = {node.slice.value for node in ast.walk(method) if isinstance(node, ast.Subscript) and isinstance(node.slice, ast.Constant)}
                    keys |= {node.args[0].value for node in ast.walk(method) if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == "get" and node.args and isinstance(node.args[0], ast.Constant)}
                    calls = {node.func.attr if isinstance(node.func, ast.Attribute) else node.func.id for node in ast.walk(method) if isinstance(node, ast.Call) and isinstance(node.func, (ast.Attribute, ast.Name))}
                    self.assertTrue(set(rule.get("required_keys", [])) <= keys)
                    self.assertFalse(set(rule.get("forbidden_keys", [])) & keys)
                    self.assertTrue(set(rule.get("required_calls", [])) <= calls)
                    self.assertFalse(set(rule.get("forbidden_calls", [])) & calls)
                    for term in rule.get("required_text", []):
                        self.assertIn(term, text)
                    for term in rule.get("forbidden_text", []):
                        self.assertNotIn(term, text)

    def test_source_roles_and_paving_replace_removed_fictions(self):
        for chapter in CASES["chapters"]:
            source = (CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8")
            literals = {node.value for node in ast.walk(ast.parse(source)) if isinstance(node, ast.Constant) and isinstance(node.value, str)}
            self.assertFalse({"C2", "TY-PILOTI", "roof_soil", "roof_soil_alt", "soil", "waterproofing", "screed", "planted"} & literals)
        source = (CLIP / "🔗️consistency" / "🐍️.py").read_text(encoding="utf-8")
        self.assertIn('"roof_paving"', source)
        self.assertIn('"roof_paving_alt"', source)

    def test_chapter_callbacks_reference_defined_globals(self):
        """🔎 Every callback global must be imported, assigned or supplied by Python itself."""
        for chapter in CASES["chapters"]:
            path=CLIP/chapter["folder"]/'🐍️.py'
            table=symtable.symtable(path.read_text(encoding='utf-8'),str(path),'exec')
            defined={s.get_name() for s in table.get_symbols() if s.is_imported() or s.is_assigned() or s.is_namespace()}|set(dir(builtins))|{'__file__','__name__','__package__','__class__'}
            def check(scope):
                missing={s.get_name() for s in scope.get_symbols() if s.is_global() and s.is_referenced() and s.get_name() not in defined}
                self.assertFalse(missing,f"{chapter['folder']} {scope.get_name()}: {missing}")
                for child in scope.get_children():
                    check(child)
            check(table)

    def test_window_highlights_filter_fill_class(self):
        source = (CLIP / "🧬️objects" / "🐍️.py").read_text(encoding="utf-8")
        scene = next(node for node in ast.parse(source).body if isinstance(node, ast.ClassDef) and node.name == "Beat2_ObjectOrientation")
        method = next(node for node in scene.body if isinstance(node, ast.FunctionDef) and node.name == "_tags")
        filters = [node for node in ast.walk(method) if isinstance(node, ast.comprehension)]
        self.assertTrue(any(isinstance(test, ast.Compare) and isinstance(test.left, ast.Subscript) and isinstance(test.left.slice, ast.Constant) and test.left.slice.value == "fill_class" and any(isinstance(value, ast.Constant) and value.value == "IfcWindow" for value in test.comparators) for clause in filters for test in clause.ifs))

    def test_bilingual_narration_matches_accessible_transcripts(self):
        for chapter in CASES["chapters"]:
            source = (CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8")
            scene = next(node for node in ast.parse(source).body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
            rows = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
            for language, index in (("en", 1), ("de", 2)):
                transcript = (CLIP / f"🗣️transcript.{language}.md").read_text(encoding="utf-8")
                for row in rows:
                    with self.subTest(chapter=chapter["folder"], language=language, clause=row[0]):
                        self.assertIn(row[index], transcript)

    def test_discipline_tables_and_loss_rows_partition_native_identities(self):
        """🗃️ Every native class is counted and every omitted identity occurs exactly once in both languages."""
        import importlib.util
        from collections import Counter
        from types import SimpleNamespace
        spec=importlib.util.spec_from_file_location("chapter_model_contract",CLIP/'🧱️model'/'🐍️.py')
        api=importlib.util.module_from_spec(spec)
        spec.loader.exec_module(api)
        model=globals().get("CAPTURED_MODEL") or api.load_model()
        info={item["id"]:api.partial_model(item["id"],model) for item in model["models"]}
        source=(CLIP/'🧩️models'/'🐍️.py').read_text(encoding='utf-8')
        scene=next(node for node in ast.parse(source).body if isinstance(node,ast.ClassDef))
        names={'_discipline_rows','_loss_groups','_piloti_label'}
        functions=[node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name in names]
        space={'STAR':'*'}
        exec(compile(ast.fix_missing_locations(ast.Module(body=functions,type_ignores=[])),'chapter-partition','exec'),space)
        subject=type('Partition',(),{name:space[name] for name in names})()
        subject.info,subject.model=info,model
        subject.analysis={'members':[{'source_tag':tag} for tag in CASES['class_partition']['kept']]}
        subject.number=lambda value,digits:str(value)
        registry={item['tag']:item for group in ('elements','spaces') for item in model[group]}
        source_tags=set().union(*(set(partial['element_tags']) for partial in info.values()))
        for language in ('en','de'):
            subject.tr=lambda en,de:en if language=='en' else de
            for name,partial in info.items():
                rows=subject._discipline_rows(name)
                independent=Counter(registry[tag]['class'] for tag in partial['element_tags'])
                self.assertEqual({key for key,_ in rows},set(independent))
                self.assertTrue(all(en.strip() and de.strip() for _,(en,de) in rows))
                self.assertEqual(sum(independent.values()),len(partial['element_guids']))
            groups=subject._loss_groups()
            omitted=[tag for _,tags,_,_,_ in groups for tag in tags]
            self.assertEqual(set(omitted),source_tags-set(CASES['class_partition']['kept']))
            self.assertEqual(len(omitted),len(set(omitted)))
            self.assertEqual(omitted.count(CASES['class_partition']['shared']),1)


    def test_elevation_labels_preserve_source_millimetres_and_negative_signs(self):
        """📏 Highest crest and cellar datums retain their exact source precision in both languages."""
        source=(CLIP/'👁️representations'/'🐍️.py').read_text(encoding='utf-8')
        scene=next(node for node in ast.parse(source).body if isinstance(node,ast.ClassDef))
        method=next(node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name=='_sign')
        scope={}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),'datum-label','exec'),scope)
        subject=type('Datum',(),{'_sign':scope['_sign']})()
        for language in ('en','de'):
            subject.number=lambda value,digits:format(value,f'.{digits}f').replace('.',',' if language=='de' else '.')
            for case in CASES['datum_labels']:
                self.assertEqual(subject._sign(case['value']),case[language])



if __name__ == "__main__":
    unittest.main()
