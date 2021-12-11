//! Final form data representations to choose
//! how best to tore information stored & retrieved
//! D

use futures::prelude::*;
use std::{sync::Arc, marker::PhantomData};


/* #[derive(Debug, Clone)]
pub struct AdjMatrix<Nx: AsRef<[u8]>, Ex: AsRef<[u8]>> + Clone {
    pub nreps: Vec<u8>,
    pub ereps: Vec<u8>,
}

impl<Nx: AsRef<[u8]> */
pub struct TripleVal<V> 
where
    V: 'static + AsRef<[u8]> + Clone + Send + Sync
{
    ident: Vec<u8>,
    vpd: PhantomData<V>
    // ...

}

pub struct BTreeMap<N, E> {
    keys: Vec<N>, 
    len:  u32,
    depth: u32,
    b: u32,
    epd: PhantomData<E>
}

impl <N: Ord, E> BTreeMap<N, E> {
    pub fn insert(&mut self, mut node: N, mut edge: E) -> Option<E> {
        //..
        None

    }
}
