use std::collections::HashMap;

use annotated_merge::{Annotated, MergeAnnotation};
pub struct Recurse;

impl MergeAnnotation for 
pub type MergableMap = Annotated<HashMap<>, merge::hashmap::recurse>;