extern crate rustc_serialize;

use self::rustc_serialize::json;
use std::collections::BTreeMap;

#[derive(RustcEncodable, RustcDecodable)]
pub enum DracoNode {
	Float(f64),
	Integer(i64),
}
