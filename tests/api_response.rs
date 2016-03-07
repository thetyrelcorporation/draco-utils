extern crate draco_utils;
extern crate rustc_serialize;
use rustc_serialize::json;
use draco_utils::api_response::{DracoNode, ApiResponse};

#[test]
fn draco_node_serializes_to_json() {
	let draco_node = DracoNode::Integer(76);
	let serialized = json::encode(&draco_node);
	assert_eq!(serialized, "testing");
}