use crate::sys::Store;
use std::any::Any;

use wasmer_vm::{StoreHandle, VMExternObj, VMExternRef};

#[derive(Debug, Clone)]
#[repr(transparent)]
/// An opaque reference to some data. This reference can be passed through Wasm.
pub struct ExternRef {
    handle: StoreHandle<VMExternObj>,
}

impl ExternRef {
    /// Make a new extern reference
    pub fn new<T>(store: &mut Store, value: T) -> Self
    where
        T: Any + Send + Sync + 'static + Sized,
    {
        Self {
            handle: StoreHandle::new(store.objects_mut(), VMExternObj::new(value)),
        }
    }

    /// Try to downcast to the given value.
    pub fn downcast<'a, T>(&self, store: &'a Store) -> Option<&'a T>
    where
        T: Any + Send + Sync + 'static + Sized,
    {
        self.handle
            .get(store.objects())
            .as_ref()
            .downcast_ref::<T>()
    }

    pub(crate) fn vm_externref(&self) -> VMExternRef {
        VMExternRef(self.handle.internal_handle())
    }

    pub(crate) unsafe fn from_vm_externref(store: &Store, vm_externref: VMExternRef) -> Self {
        Self {
            handle: StoreHandle::from_internal(store.objects().id(), vm_externref.0),
        }
    }

    /// Checks whether this `ExternRef` can be used with the given store.
    ///
    /// Primitive (`i32`, `i64`, etc) and null funcref/externref values are not
    /// tied to a context and can be freely shared between contexts.
    ///
    /// Externref and funcref values are tied to a context and can only be used
    /// with that context.
    pub fn is_from_store(&self, store: &Store) -> bool {
        self.handle.store_id() == store.objects().id()
    }
}
