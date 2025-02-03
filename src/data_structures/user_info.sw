library;
use std::identity::Identity;


pub struct Owner {
    pub owner: Identity,
    pub deployement_date: u64 
}

impl Owner {
    pub fn new(owner: Identity, deployement_date: u64)-> Self {
        Self{
            owner, deployement_date
        }
    }
}