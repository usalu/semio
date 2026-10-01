CREATE TABLE dwg_associative_dependency (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), status TEXT NOT NULL CHECK(status='up_to_date'), is_read_dependency INTEGER NOT NULL CHECK(is_read_dependency IN (0,1)), is_write_dependency INTEGER NOT NULL CHECK(is_write_dependency IN (0,1)),
 is_attached_to_object INTEGER NOT NULL CHECK(is_attached_to_object IN (0,1)), is_delegating_to_owning_action INTEGER NOT NULL CHECK(is_delegating_to_owning_action IN (0,1)), dependency_order INTEGER NOT NULL CHECK(dependency_order BETWEEN -2147483648 AND 2147483647),
 dependent_on_object_handle_high INTEGER NOT NULL CHECK(dependent_on_object_handle_high BETWEEN 0 AND 4294967295), dependent_on_object_handle_low INTEGER NOT NULL CHECK(dependent_on_object_handle_low BETWEEN 0 AND 4294967295), name TEXT,
 read_dependency_handle_high INTEGER CHECK(read_dependency_handle_high BETWEEN 0 AND 4294967295), read_dependency_handle_low INTEGER CHECK(read_dependency_handle_low BETWEEN 0 AND 4294967295),
 dependency_node_handle_high INTEGER CHECK(dependency_node_handle_high BETWEEN 0 AND 4294967295), dependency_node_handle_low INTEGER CHECK(dependency_node_handle_low BETWEEN 0 AND 4294967295),
 dependency_body_handle_high INTEGER CHECK(dependency_body_handle_high BETWEEN 0 AND 4294967295), dependency_body_handle_low INTEGER CHECK(dependency_body_handle_low BETWEEN 0 AND 4294967295),
 dependency_body_id INTEGER NOT NULL CHECK(dependency_body_id BETWEEN -2147483648 AND 2147483647),
 CHECK((read_dependency_handle_high IS NULL)=(read_dependency_handle_low IS NULL)), CHECK((dependency_node_handle_high IS NULL)=(dependency_node_handle_low IS NULL)), CHECK((dependency_body_handle_high IS NULL)=(dependency_body_handle_low IS NULL))
);
CREATE TABLE dwg_associative_value_dependency (
 id INTEGER PRIMARY KEY REFERENCES dwg_associative_dependency(id), cached_value_kind TEXT NOT NULL CHECK(cached_value_kind='integer32'), cached_integer32 INTEGER NOT NULL CHECK(cached_integer32 BETWEEN -2147483648 AND 2147483647), value_name TEXT NOT NULL
);
CREATE TABLE dwg_associative_geometry_dependency (
 id INTEGER PRIMARY KEY REFERENCES dwg_associative_dependency(id), enabled INTEGER NOT NULL CHECK(enabled IN (0,1)), persistent_subentity_class_name TEXT NOT NULL, dependent_on_compound_object INTEGER NOT NULL CHECK(dependent_on_compound_object IN (0,1))
);
CREATE TABLE dwg_associative_action (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), status TEXT NOT NULL CHECK(status='up_to_date'),
 owning_network_handle_high INTEGER CHECK(owning_network_handle_high BETWEEN 0 AND 4294967295), owning_network_handle_low INTEGER CHECK(owning_network_handle_low BETWEEN 0 AND 4294967295),
 action_body_handle_high INTEGER CHECK(action_body_handle_high BETWEEN 0 AND 4294967295), action_body_handle_low INTEGER CHECK(action_body_handle_low BETWEEN 0 AND 4294967295),
 action_index INTEGER NOT NULL CHECK(action_index BETWEEN -2147483648 AND 2147483647), maximum_dependency_index INTEGER NOT NULL CHECK(maximum_dependency_index BETWEEN -2147483648 AND 2147483647),
 CHECK((owning_network_handle_high IS NULL)=(owning_network_handle_low IS NULL)), CHECK((action_body_handle_high IS NULL)=(action_body_handle_low IS NULL))
);
CREATE TABLE dwg_associative_action_dependency (
 id INTEGER PRIMARY KEY, associative_action_id INTEGER NOT NULL REFERENCES dwg_associative_action(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), owned INTEGER NOT NULL CHECK(owned IN (0,1)),
 dependency_handle_high INTEGER NOT NULL CHECK(dependency_handle_high BETWEEN 0 AND 4294967295), dependency_handle_low INTEGER NOT NULL CHECK(dependency_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_associative_variable (
 id INTEGER PRIMARY KEY REFERENCES dwg_associative_action(id), name TEXT NOT NULL, expression TEXT NOT NULL, evaluator_id TEXT NOT NULL, description TEXT NOT NULL,
 evaluated_value_kind TEXT NOT NULL CHECK(evaluated_value_kind='integer32'), evaluated_integer32 INTEGER NOT NULL CHECK(evaluated_integer32 BETWEEN -2147483648 AND 2147483647),
 mergeable INTEGER NOT NULL CHECK(mergeable IN (0,1)), mergeable_variable_name TEXT, must_merge INTEGER NOT NULL CHECK(must_merge IN (0,1))
);
CREATE TABLE dwg_variable_value_dependency_handle (
 id INTEGER PRIMARY KEY, associative_variable_id INTEGER NOT NULL REFERENCES dwg_associative_variable(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 handle_high INTEGER NOT NULL CHECK(handle_high BETWEEN 0 AND 4294967295), handle_low INTEGER NOT NULL CHECK(handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_assoc_network (
 id INTEGER PRIMARY KEY REFERENCES dwg_associative_action(id), network_action_index INTEGER NOT NULL CHECK(network_action_index BETWEEN -2147483648 AND 2147483647)
);
CREATE TABLE dwg_assoc_network_member (
 id INTEGER PRIMARY KEY, assoc_network_id INTEGER NOT NULL REFERENCES dwg_assoc_network(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), kind TEXT NOT NULL CHECK(kind IN ('network','action')),
 handle_high INTEGER NOT NULL CHECK(handle_high BETWEEN 0 AND 4294967295), handle_low INTEGER NOT NULL CHECK(handle_low BETWEEN 0 AND 4294967295)
);
