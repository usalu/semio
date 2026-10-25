//! 🌳️ Trinity jack query AST.

use semio_framework_graph_manifest::PropertyValue;

/// 🌳️ Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct QueryResult {
    #[value(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    /// 📦️ Query projection payload stays boxed under the transient lane's inline bound.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub graph_snapshot: Option<Box<JackQueryGraphSnapshot>>,
}

impl semio_framework_dsl_record::DslField for QueryResult {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => semio_framework_value::FromValue::from_value(value.clone()).map_err(|error| error.to_string()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

impl semio_framework_dsl_record::BorrowedDslField for QueryResult {
    const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;
}

impl QueryResult {
    pub fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_snapshot: None }
    }

    pub fn graph(columns: Vec<String>, graph_snapshot: JackQueryGraphSnapshot) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_snapshot: Some(Box::new(graph_snapshot)) }
    }
}
// #endregion 🔖️Ast

/// 🧮️ Owns the already admitted query projection; it never replaces or serializes the selected rich source.
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase")]
pub struct JackQueryGraphSnapshot{
 pub schema:String,pub name:String,pub manifest_id:Option<String>,pub manifest:crate::standards::v1::subsets::any::schema::manifest::Manifest,pub camera:crate::standards::v1::subsets::any::schema::graph::Camera,pub nodes:Vec<crate::standards::v1::subsets::any::schema::graph::Node>,pub edges:Vec<crate::standards::v1::subsets::any::schema::graph::Edge>,pub root_node_id:Option<String>,pub query:String,
}
impl JackQueryGraphSnapshot{
 pub fn into_graph(self)->Result<crate::standards::v1::subsets::any::schema::memory::Graph,crate::standards::v1::subsets::any::schema::errors::TrinityRamError>{crate::standards::v1::subsets::any::schema::memory::Graph::with_scene(crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot{schema:self.schema,name:self.name,manifest_id:self.manifest_id,manifest:self.manifest,camera:self.camera,root_node_id:self.root_node_id,query:self.query,..Default::default()},crate::standards::v1::subsets::any::schema::content::JackWorkingScene{nodes:self.nodes,edges:self.edges})}

 /// 📥️ Decodes the self-contained scene document [`Self::to_json`] writes.
 pub fn from_json(json:&str)->Result<Self,crate::standards::v1::subsets::any::schema::errors::TrinityRamError>{let parsed=semio_framework_pack_json::parse(json,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))?;Ok(<Self as semio_framework_value::FromValue>::from_value(crate::standards::v1::subsets::any::schema::json_transport::convert_scene(semio_framework_pack_json::to_dsl_value(&parsed),true)?)?)}

 pub fn to_json(&self)->Result<String,crate::standards::v1::subsets::any::schema::errors::TrinityRamError>{let value=crate::standards::v1::subsets::any::schema::json_transport::convert(semio_framework_value::ToValue::to_value(self),false)?;Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&value)))}
}
impl semio_s_artifact_trinity_jack_graph_query::JackContentVisual for JackQueryGraphSnapshot{
 fn visual_counts(&self)->(usize,usize){(self.nodes.len(),self.edges.len())}
 fn node_view(&self,index:usize)->Option<semio_s_artifact_trinity_jack_graph_query::JackContentNodeView<'_>>{self.nodes.get(index).map(|node|semio_s_artifact_trinity_jack_graph_query::JackContentNodeView{id:&node.id,kind:&node.kind,label:&node.name,x:node.x,y:node.y,width:node.width,height:node.height,port_count:node.ports.len()})}
 fn port_view(&self,node_index:usize,port_index:usize)->Option<semio_s_artifact_trinity_jack_graph_query::JackContentPortView<'_>>{self.nodes.get(node_index)?.ports.get(port_index).map(|port|semio_s_artifact_trinity_jack_graph_query::JackContentPortView{id:&port.id,label:&port.id,inbound:matches!(port.direction,semio_framework_graph_manifest::PortDirection::In),outbound:matches!(port.direction,semio_framework_graph_manifest::PortDirection::Out)})}
 fn edge_view(&self,index:usize)->Option<semio_s_artifact_trinity_jack_graph_query::JackContentEdgeView<'_>>{self.edges.get(index).map(|edge|{let(source,source_port)=edge.source.split_once('@').map_or((edge.source.as_str(),None),|(node,port)|(node,Some(port)));let(target,target_port)=edge.target.split_once('@').map_or((edge.target.as_str(),None),|(node,port)|(node,Some(port)));semio_s_artifact_trinity_jack_graph_query::JackContentEdgeView{id:&edge.id,kind:&edge.kind,label:"",source,target,source_port,target_port}})}
}
