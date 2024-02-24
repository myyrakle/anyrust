use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use dyn_clone::DynClone;

pub trait Anyable:
    std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display + AutoCast
{
}

impl<T: std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display + AutoCast> Anyable
    for T
{
}

pub trait ToInteger {
    fn to_integer(&self) -> i64;
}

pub trait ToFloat {
    fn to_float(&self) -> f64;
}

pub trait ToArray {
    fn to_array(&self) -> Vec<Any>;
}

pub trait ToMap {
    fn to_map(&self) -> std::collections::HashMap<String, Any>;
}

pub trait ToBoolean {
    fn to_boolean(&self) -> bool;
}

pub trait AutoCast: ToInteger + ToFloat + ToArray + ToMap + ToBoolean + ToString {}

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

impl Add for Any {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.type_id == other.type_id {
            match self.type_id {
                std::any::TypeId::of::<i32>() => {
                    let data = self.data.downcast_ref::<i32>().unwrap()
                        + other.data.downcast_ref::<i32>().unwrap();
                    Self {
                        type_id: self.type_id,
                        data: Box::new(data),
                    }
                }
                _ => panic!("Type mismatch"),
            }

            let data = self.data + other.data;
            Self {
                type_id: self.type_id,
                data: Box::new(data),
            }
        } else {
            panic!("Type mismatch");
        }
    }
}
