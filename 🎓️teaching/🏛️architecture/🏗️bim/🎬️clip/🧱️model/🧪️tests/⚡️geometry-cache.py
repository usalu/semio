"""⚡ Validate isolated complete-model geometry memoization through neutral operations."""
from copy import deepcopy
import importlib
import json
from pathlib import Path
import unittest
from unittest.mock import patch

CASES = json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf-8'))


class GeometryMemoContract(unittest.TestCase):
    """🧊 Keep complete canonical inputs, native output ownership and finite model reuse."""

    def setUp(self):
        self.api = importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
        self.api._geometry_models_cached.cache_clear()

    def tearDown(self):
        self.api._geometry_models_cached.cache_clear()

    def equal(self, expected, actual):
        """🔬 Compare exact canonical JSON and an independent third-party structural diff."""
        from numpy.testing import assert_equal
        assert_equal(expected, actual)
        self.assertEqual(json.dumps(expected, sort_keys=True, separators=(',', ':')),
                         json.dumps(actual, sort_keys=True, separators=(',', ':')))

    def result(self, model, version=1):
        return {'snapshot': model, 'version': version, **CASES['body']}

    def test_complete_model_keys_and_input_output_mutation_are_isolated(self):
        import jsonschema
        jsonschema.Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASES)
        model = deepcopy(CASES['model'])
        original = deepcopy(model)
        calls = []
        def build(value):
            calls.append(deepcopy(value))
            return self.result(value)
        with patch.object(self.api, '_building_geometry_uncached', build):
            first = self.api.building_geometry(model)
            self.equal(self.result(original), first)
            first['snapshot']['car']['width'] = 99
            first['parts'][0]['outline'][0][0] = 99
            self.equal(original, model)
            self.equal(self.result(original), self.api.building_geometry(dict(reversed(list(model.items())))))
            self.assertEqual(len(calls), 1)
            model['car']['width'] = 2.4
            self.equal(self.result(model), self.api.building_geometry(model))
            self.equal(self.result(original), self.api.building_geometry(original))
            self.assertEqual(len(calls), 2)
            for change in CASES['changes']:
                candidate = deepcopy(original)
                target = candidate
                for key in change['path'][:-1]:
                    target = target[key]
                target[change['path'][-1]] = change['value']
                count = len(calls)
                self.equal(self.result(candidate), self.api.building_geometry(candidate))
                self.assertEqual(len(calls), count + 1)

    def test_two_model_eviction_cancellation_and_producer_identity(self):
        for family in CASES['families']:
            with self.subTest(family=family['public']):
                self.api._geometry_models_cached.cache_clear()
                calls = []
                def build(model, *objects):
                    calls.append(model['revision'])
                    return self.result(model)
                query = lambda model: getattr(self.api, family['public'])(model) if family['key'] is None else getattr(self.api, family['public'])(family['key'], model)
                with patch.object(self.api, family['producer'], build):
                    for revision in CASES['revisions']:
                        model = deepcopy(CASES['model'])
                        model['revision'] = revision
                        returned = query(model)
                        self.equal(self.result(model), returned)
                        returned["snapshot"]["revision"] = -100
                        self.equal(self.result(model), query(model))
                    self.assertEqual(len(calls), CASES['expected_builds'])
                    self.assertLessEqual(self.api._geometry_models_cached.cache_info().currsize, 2)
                    previous=len(calls)
                    with patch.object(self.api,"is_architectural_covering",lambda obj:False):
                        self.equal(self.result(model),query(model))
                        self.equal(self.result(model),query(deepcopy(model)))
                    self.assertEqual(len(calls),previous+1)
                    with patch.object(self.api, family['producer'], lambda model, *args: self.result(model, 2)):
                        self.equal(self.result(model, 2), query(model))
                self.api._geometry_models_cached.cache_clear()
                attempts = []
                def interrupted(model, *objects):
                    attempts.append(1)
                    if len(attempts) == 1:
                        raise KeyboardInterrupt('cancelled geometry build')
                    return self.result(model)
                with patch.object(self.api, family['producer'], interrupted):
                    with self.assertRaises(KeyboardInterrupt):
                        query(CASES['model'])
                    self.equal(self.result(CASES['model']), query(CASES['model']))
                    self.assertEqual(len(attempts), 2)

    def test_building_seeds_public_service_and_solid_queries(self):
        model = deepcopy(CASES['model'])
        calls = {'service': 0, 'solid': 0}
        def service(value, obj):
            calls['service'] += 1
            return {'tag': obj['tag'], 'family': 'service', **self.result(value)}
        def solid(value, obj):
            calls['solid'] += 1
            return {'tag': obj['tag'], 'family': 'solid', **self.result(value)}
        def building(value):
            return {'services': [service(value, value['elements'][0])], 'details': [solid(value, value['elements'][1])]}
        with patch.object(self.api, '_service', service), patch.object(self.api, '_solid_geometry', solid), patch.object(self.api, '_building_geometry_uncached', building):
            built = self.api.building_geometry(model)
            self.equal(built['services'][0], self.api.service_geometry('SERVICE', model))
            self.equal(built['details'][0], self.api.solid_geometry('SOLID', model))
            self.assertEqual(calls, {'service': 1, 'solid': 1})
            built['services'][0]['parts'][0]['outline'][0][0] = 99
            self.equal(service(model, model['elements'][0]), self.api.service_geometry('SERVICE', model))


    def test_quantity_slots_track_full_inputs_volume_producers_and_output_ownership(self):
        """⚖️ Reuse exact quantity values while isolating callers and every source/producer change."""
        model=deepcopy(CASES['model'])
        calls=[]
        def produce(value,obj):
            calls.append(deepcopy(value))
            return self.result(value)
        with patch.object(self.api,'_quantities_uncached',produce):
            first=self.api.quantities('SERVICE',model)
            self.equal(self.result(model),first)
            first['parts'][0]['outline'][0][0]=999
            self.equal(self.result(model),self.api.quantities('GUID-SERVICE',deepcopy(model)))
            self.assertEqual(len(calls),1)
            for change in CASES['changes']:
                candidate=deepcopy(model)
                target=candidate
                for key in change['path'][:-1]:
                    target=target[key]
                target[change['path'][-1]]=change['value']
                count=len(calls)
                self.equal(self.result(candidate),self.api.quantities('SERVICE',candidate))
                self.assertEqual(len(calls),count+1)
            self.api.quantities('SERVICE',model)
            for helper in ('_occupied_volume','_material_quantities','_partition_volume'):
                count=len(calls)
                with patch.object(self.api,helper,lambda *args,**kwargs:None):
                    self.equal(self.result(model),self.api.quantities('SERVICE',model))
                    self.assertEqual(len(calls),count+1)


if __name__ == '__main__':
    unittest.main()
