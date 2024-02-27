use std::collections::HashSet;

use crate::{predicate::Predicate, rule::Rule};

pub struct DatalogProgram {
    pub schemes: Vec<Predicate>,
    pub facts: Vec<Predicate>,
    pub rules: Vec<Rule>,
    pub queries: Vec<Predicate>,
    pub domain: HashSet<String>,
}
