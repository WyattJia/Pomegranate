use skiplist::run;

pub struct DiskLevel {


}

type KVpairT<K, V> = run::KVpair<K, V>;
type KVIntPairT = tuple<KVIntPairT<K, V>, isize>;

pub struct StaticHeap<K, V> {
    pub size: isize,
    pub arr: Vec<*mut KVpairT<K, V>>,
    pub max: Option<*mut KVIntPairT<<K, V>,isize>, 
}


impl <K, V> StaticHeap<K, V> {

    fn new(sz: usize, mx: KVIntPairT) -> Self {}
    fn push(&mut self, blob: KVpairT<K, V>) {

    }

    fn heapify(&mut self, i: isize) {} 
    fn pop(&mut self) -> KVIntPairT {

    }
}