use std::{
    any::TypeId,
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

impl ToInteger for i8 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for i16 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for i32 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for i64 {
    fn to_integer(&self) -> i64 {
        *self
    }
}

impl ToInteger for u8 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for u16 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for u32 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for u64 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for f32 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for f64 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToInteger for String {
    fn to_integer(&self) -> i64 {
        self.parse().unwrap()
    }
}

impl ToInteger for bool {
    fn to_integer(&self) -> i64 {
        if *self {
            1
        } else {
            0
        }
    }
}

impl<T> ToInteger for Vec<T>
where
    T: AutoCast,
{
    fn to_integer(&self) -> i64 {
        0 as i64
    }
}

impl<K, V> ToInteger for std::collections::HashMap<K, V>
where
    K: AutoCast,
    V: AutoCast,
{
    fn to_integer(&self) -> i64 {
        0 as i64
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl<T> ToString for Vec<T>
where
    T: AutoCast,
{
    fn to_string(&self) -> String {
        let mut result = String::from("[");

        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }

            result.push_str(&item.to_string());
        }

        result.push_str("]");

        result
    }
}

impl<K, V> ToString for std::collections::HashMap<K, V>
where
    K: AutoCast,
    V: AutoCast,
{
    fn to_string(&self) -> String {
        let mut result = String::from("{");

        for (i, (key, value)) in self.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }

            result.push_str(&key.to_string());
            result.push_str(": ");
            result.push_str(&value.to_string());
        }

        result.push_str("}");

        result
    }
}

pub trait ToFloat {
    fn to_float(&self) -> f64;
}

pub trait ToArray {
    fn to_array(&self) -> Vec<Any>;
}

pub trait ToMap {
    fn to_map(&self) -> std::collections::HashMap<Any, Any>;
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
                type_id => if type_id == TypeId::of::<i32>() {},
                _ => panic!("Type mismatch"),
            }

            unimplemented!()
        } else {
            panic!("Type mismatch");
        }
    }
}
