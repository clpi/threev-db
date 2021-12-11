use std::marker::PhantomData;

use futures::prelude::*;

pub struct Key<K, V> {
    key: [u8; 5],
    kpd: PhantomData<K>,
    vpd: PhantomData<V>
}

pub struct DoubleValued<K, V>{
    kpd: PhantomData<K>,
    vpd: PhantomData<V>
}

pub struct FactValued<K, V> {
    kpd: PhantomData<K>,
    vpd: PhantomData<V>

}


pub trait OnDisk<Write>
where
    Self: Clone + Send + Sync,
    Write: AsyncWrite + Send + Sync,
{

}
