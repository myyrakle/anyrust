use std::fmt::{Debug, Display};

use dyn_clone::DynClone;

pub trait Anyable: std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display {}

impl<T: std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display> Anyable for T {}

#[derive(Debug)]
pub struct Any {
    type_id: std::any::TypeId,
    data: Box<dyn Anyable>,
}

impl Clone for Any {
    fn clone(&self) -> Self {
        Self {
            type_id: self.type_id,
            data: dyn_clone::clone_box(&*self.data),
        }
    }
}

impl Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Any {
    pub fn new<T>(value: T) -> Self
    where
        T: Anyable,
    {
        Self {
            type_id: std::any::TypeId::of::<T>(),
            data: Box::new(value),
        }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        if self.type_id == std::any::TypeId::of::<T>() {
            unsafe { Some(&*(self.data.as_ref() as *const dyn std::any::Any as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if self.type_id == std::any::TypeId::of::<T>() {
            unsafe { Some(&mut *(self.data.as_mut() as *mut dyn std::any::Any as *mut T)) }
        } else {
            None
        }
    }
}
