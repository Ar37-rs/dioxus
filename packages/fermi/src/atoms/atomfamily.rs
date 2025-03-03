use crate::{AtomId, AtomRoot, Readable, Writable};
use im_rc::HashMap as ImMap;

pub struct AtomFamilyBuilder;
pub type AtomFamily<K, V> = fn(AtomFamilyBuilder) -> ImMap<K, V>;

impl<K, V> Readable<ImMap<K, V>> for AtomFamily<K, V> {
    fn read(&self, _root: AtomRoot) -> Option<ImMap<K, V>> {
        todo!()
    }

    fn init(&self) -> ImMap<K, V> {
        (*self)(AtomFamilyBuilder)
    }

    fn unique_id(&self) -> AtomId {
        *self as *const ()
    }
}

impl<K, V> Writable<ImMap<K, V>> for AtomFamily<K, V> {
    fn write(&self, _root: AtomRoot, _value: ImMap<K, V>) {
        todo!()
    }
}
