use leptos::{Scope, Serializable};
use serde::{Deserialize, Serialize};

pub fn navbar() {
    
}


#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct NavBar {
    pub id: usize,
    pub title: String,
    pub width: usize,
    pub height: usize,
}
