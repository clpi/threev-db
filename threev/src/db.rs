use core::ops::Deref;
use std::{
    sync::{Arc, RwLock}, ops::DerefMut
};


#[derive(Clone)]
pub struct AdjMatrix<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
}

#[derive(Clone)]
pub struct Db<K, V> {
    pub(crate) net: AdjMatrix<K, V>,
    clients_conn: Arc<RwLock<Vec<u8>>>,
}

impl<K, V> Deref for Db<K, V> {
    type Target = AdjMatrix<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.net
    }
}
impl<K, V> DerefMut for Db<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.net
    }
}
