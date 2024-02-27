use crate::predicate::Predicate;

pub struct Rule {
    pub head_predicate: Predicate,
    pub schemes: Vec<Predicate>,
}