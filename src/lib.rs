use std::{
    any::TypeId,
    fmt::{Debug, Display},
    ops::Add,
};

use dyn_clone::DynClone;

// 기본 트레잇
pub trait Anyable:
    std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display + AutoCast
{
}

impl<T: std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display + AutoCast> Anyable
    for T
{
}

// 배열 타입
type Array = Vec<Any>;

// 맵 타입
#[derive(Debug, Clone)]
struct Map(std::collections::HashMap<Any, Any>);

// 캐스팅용 트레잇: 정수로 캐스팅될때 어떻게 변환될지를 정의합니다.
pub trait ToInteger {
    fn to_integer(&self) -> i64;
}

// 캐스팅용 트레잇: 실수로 캐스팅될때 어떻게 변환될지를 정의합니다.
pub trait ToFloat {
    fn to_float(&self) -> f64;
}

// 캐스팅용 트레잇: 문자열로 캐스팅될때 어떻게 변환될지를 정의합니다.
pub trait ToStr {
    fn to_str(&self) -> String;
}

// 캐스팅용 트레잇: 배열로 캐스팅될때 어떻게 변환될지를 정의합니다.
pub trait ToArray {
    fn to_array(&self) -> Array;
}

// 캐스팅용 트레잇: 맵으로 캐스팅될때 어떻게 변환될지를 정의합니다.
pub trait ToMap {
    fn to_map(&self) -> Map;
}

// 캐스팅용 트레잇: 불리언으로 캐스팅될때 어떻게 변환될지를 정의합니다.
pub trait ToBoolean {
    fn to_boolean(&self) -> bool;
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
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

// ToStr 트레잇 구현
impl ToStr for i8 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for i16 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for i32 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for i64 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for u8 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for u16 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for u32 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for u64 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for f32 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for f64 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToStr for String {
    fn to_str(&self) -> String {
        self.clone()
    }
}

impl ToStr for bool {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl<T> ToStr for Vec<T>
where
    T: AutoCast,
{
    fn to_str(&self) -> String {
        let mut result = String::from("[");

        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }

            result.push_str(&item.to_str());
        }

        result.push_str("]");

        result
    }
}

impl ToStr for Map {
    fn to_str(&self) -> String {
        let mut result = String::from("{");

        for (i, (key, value)) in self.0.iter().enumerate() {
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

impl ToFloat for i8 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for i16 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for i32 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for i64 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for u8 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for u16 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for u32 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for u64 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for f32 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToFloat for f64 {
    fn to_float(&self) -> f64 {
        *self
    }
}

impl ToFloat for String {
    fn to_float(&self) -> f64 {
        self.parse().unwrap()
    }
}

impl ToFloat for bool {
    fn to_float(&self) -> f64 {
        if *self {
            1.0
        } else {
            0.0
        }
    }
}

impl<T> ToFloat for Vec<T>
where
    T: AutoCast,
{
    fn to_float(&self) -> f64 {
        0 as f64
    }
}

impl ToFloat for Map {
    fn to_float(&self) -> f64 {
        0 as f64
    }
}

impl ToArray for Array {
    fn to_array(&self) -> Array {
        self.clone()
    }
}

impl ToArray for Map {
    fn to_array(&self) -> Array {
        vec![Any::new(self.clone())]
    }
}

impl ToMap for Map {
    fn to_map(&self) -> Map {
        self.clone()
    }
}

impl ToBoolean for Map {
    fn to_boolean(&self) -> bool {
        true
    }
}

pub trait AutoCast: ToInteger + ToFloat + ToArray + ToMap + ToBoolean + ToStr {}

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
