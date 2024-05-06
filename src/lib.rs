use std::{
    any::TypeId,
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Add,
};

use dyn_clone::{clone_trait_object, DynClone};

// any trait
pub trait Anyable:
    std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display + AutoCast + DynClone
{
}

clone_trait_object!(Anyable);

impl<
        T: std::any::Any + Send + Sync + std::fmt::Debug + DynClone + Display + AutoCast + DynClone,
    > Anyable for T
{
}

// null type
#[derive(Debug, Clone)]
pub struct Null;

// null value
#[allow(non_upper_case_globals)]
pub const null: Null = Null {};

// array type
#[derive(Debug, Clone)]
pub struct Array(Vec<Any>);

impl From<Vec<Any>> for Array {
    fn from(vec: Vec<Any>) -> Self {
        Self(vec)
    }
}

// key-value map type
#[derive(Debug, Clone)]
pub struct Map(std::collections::HashMap<Any, Any>);

// castable trait
pub trait AutoCast: ToInteger + ToFloat + ToArray + ToMap + ToBoolean + ToStr {}

impl<T: ToInteger + ToFloat + ToArray + ToMap + ToBoolean + ToStr> AutoCast for T {}

// Trait for casting: Defines how to convert when cast to an integer.
pub trait ToInteger {
    fn to_integer(&self) -> i64;
}

// Trait for casting: Defines how to convert when cast to a float.
pub trait ToFloat {
    fn to_float(&self) -> f64;
}

// Trait for casting: Defines how to convert when cast to a string.
pub trait ToStr {
    fn to_str(&self) -> String;
}

// Trait for casting: Defines how to convert when cast to an array.
pub trait ToArray {
    fn to_array(&self) -> Array;
}

// Trait for casting: Defines how to convert when cast to a map.
pub trait ToMap {
    fn to_map(&self) -> Map;
}

// Trait for casting: Defines how to convert when cast to a boolean.
pub trait ToBoolean {
    fn to_boolean(&self) -> bool;
}

impl Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

// array 트레잇 구현
impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("[");
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&item.to_string());
        }
        result.push_str("]");
        write!(f, "{}", result)
    }
}

impl ToArray for Array {
    fn to_array(&self) -> Array {
        self.clone()
    }
}

impl ToMap for Array {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for Array {
    fn to_boolean(&self) -> bool {
        true
    }
}

impl ToInteger for Array {
    fn to_integer(&self) -> i64 {
        0
    }
}

impl ToFloat for Array {
    fn to_float(&self) -> f64 {
        0.0
    }
}

impl ToStr for Array {
    fn to_str(&self) -> String {
        let mut result = String::from("[");
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&item.to_string());
        }
        result.push_str("]");
        result
    }
}

// i8 트레잇 구현
impl From<i8> for Any {
    fn from(value: i8) -> Self {
        Any::new(value)
    }
}

impl ToInteger for i8 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for i8 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for i8 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for i8 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for i8 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for i8 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// i16 트레잇 구현
impl From<i16> for Any {
    fn from(value: i16) -> Self {
        Any::new(value)
    }
}

impl ToInteger for i16 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for i16 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for i16 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for i16 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for i16 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for i16 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// i32 트레잇 구현
impl From<i32> for Any {
    fn from(value: i32) -> Self {
        Any::new(value)
    }
}

impl ToInteger for i32 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for i32 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for i32 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for i32 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for i32 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for i32 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// i64 트레잇 구현
impl From<i64> for Any {
    fn from(value: i64) -> Self {
        Any::new(value)
    }
}

impl ToInteger for i64 {
    fn to_integer(&self) -> i64 {
        *self
    }
}

impl ToStr for i64 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for i64 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for i64 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for i64 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for i64 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// u8 트레잇 구현
impl From<u8> for Any {
    fn from(value: u8) -> Self {
        Any::new(value)
    }
}

impl ToInteger for u8 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for u8 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for u8 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for u8 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for u8 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for u8 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// u16 트레잇 구현
impl From<u16> for Any {
    fn from(value: u16) -> Self {
        Any::new(value)
    }
}

impl ToInteger for u16 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for u16 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for u16 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for u16 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for u16 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for u16 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// u32 트레잇 구현
impl From<u32> for Any {
    fn from(value: u32) -> Self {
        Any::new(value)
    }
}

impl ToInteger for u32 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for u32 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for u32 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for u32 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for u32 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for u32 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// u64 트레잇 구현
impl From<u64> for Any {
    fn from(value: u64) -> Self {
        Any::new(value)
    }
}

impl ToInteger for u64 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for u64 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for u64 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for u64 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for u64 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for u64 {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// f32 트레잇 구현
impl ToInteger for f32 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for f32 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for f32 {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for f32 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for f32 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for f32 {
    fn to_boolean(&self) -> bool {
        *self != 0.0
    }
}
// ---------------

// f64 트레잇 구현
impl ToInteger for f64 {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for f64 {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for f64 {
    fn to_float(&self) -> f64 {
        *self
    }
}

impl ToArray for f64 {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for f64 {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for f64 {
    fn to_boolean(&self) -> bool {
        *self != 0.0
    }
}
// ---------------

// 문자열 트레잇 구현
impl ToInteger for String {
    fn to_integer(&self) -> i64 {
        self.parse().unwrap()
    }
}

impl ToStr for String {
    fn to_str(&self) -> String {
        self.clone()
    }
}

impl ToFloat for String {
    fn to_float(&self) -> f64 {
        self.parse().unwrap()
    }
}

impl ToArray for String {
    fn to_array(&self) -> Array {
        vec![Any::new(self.clone())].into()
    }
}

impl ToMap for String {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for String {
    fn to_boolean(&self) -> bool {
        self.parse().unwrap_or(false)
    }
}
// ---------------

// 문자열 슬라이스 트레잇 구현
impl ToInteger for &str {
    fn to_integer(&self) -> i64 {
        self.parse().unwrap()
    }
}

impl ToStr for &str {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for &str {
    fn to_float(&self) -> f64 {
        self.parse().unwrap()
    }
}

impl ToArray for &str {
    fn to_array(&self) -> Array {
        vec![Any::new(self.to_string())].into()
    }
}

impl ToMap for &str {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for &str {
    fn to_boolean(&self) -> bool {
        self.parse().unwrap_or(false)
    }
}
// ---------------

// 불리언 트레잇 구현
impl ToInteger for bool {
    fn to_integer(&self) -> i64 {
        if *self {
            1
        } else {
            0
        }
    }
}

impl ToStr for bool {
    fn to_str(&self) -> String {
        self.to_string()
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

impl ToArray for bool {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for bool {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for bool {
    fn to_boolean(&self) -> bool {
        *self
    }
}
// ---------------

// Map 트레잇 구현
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ToInteger for Map {
    fn to_integer(&self) -> i64 {
        0 as i64
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

impl ToFloat for Map {
    fn to_float(&self) -> f64 {
        0 as f64
    }
}

impl ToArray for Map {
    fn to_array(&self) -> Array {
        vec![Any::new(self.clone())].into()
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

// Null 트레잇 구현
impl ToInteger for Null {
    fn to_integer(&self) -> i64 {
        0 as i64
    }
}

impl ToStr for Null {
    fn to_str(&self) -> String {
        String::from("null")
    }
}

impl ToFloat for Null {
    fn to_float(&self) -> f64 {
        0 as f64
    }
}

impl ToArray for Null {
    fn to_array(&self) -> Array {
        vec![].into()
    }
}

impl ToMap for Null {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for Null {
    fn to_boolean(&self) -> bool {
        false
    }
}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}
// ---------------

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

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    // pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
    //     if self.type_id == std::any::TypeId::of::<T>() {
    //         unsafe { Some(&*(self.data.as_ref() as *const dyn std::any::Any as *const T)) }
    //     } else {
    //         None
    //     }
    // }

    // pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
    //     if self.type_id == std::any::TypeId::of::<T>() {
    //         unsafe { Some(&mut *(self.data.as_mut() as *mut dyn std::any::Any as *mut T)) }
    //     } else {
    //         None
    //     }
    // }
}

lazy_static::lazy_static! {
    pub static ref I8: TypeId = TypeId::of::<i8>();
    pub static ref I16: TypeId = TypeId::of::<i16>();
    pub static ref I32: TypeId = TypeId::of::<i32>();
    pub static ref I64: TypeId = TypeId::of::<i64>();
    pub static ref U8: TypeId = TypeId::of::<u8>();
    pub static ref U16: TypeId = TypeId::of::<u16>();
    pub static ref U32: TypeId = TypeId::of::<u32>();
    pub static ref U64: TypeId = TypeId::of::<u64>();
    pub static ref F32: TypeId = TypeId::of::<f32>();
    pub static ref F64: TypeId = TypeId::of::<f64>();
    pub static ref STRING: TypeId = TypeId::of::<String>();
    pub static ref STR: TypeId = TypeId::of::<&str>();
    pub static ref BOOL: TypeId = TypeId::of::<bool>();
    pub static ref ARRAY: TypeId = TypeId::of::<Array>();
    pub static ref MAP: TypeId = TypeId::of::<Map>();
    pub static ref NULL: TypeId = TypeId::of::<Null>();
}

impl Add for Any {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.type_id == *NULL || other.type_id == *NULL {
            Any::new(null)
        } else if self.type_id == *STRING || other.type_id == *STRING {
            let a = self.data.to_string();
            let b = other.data.to_string();
            Any::new(a + &b)
        } else if self.type_id == *STR || other.type_id == *STR {
            let a = self.data.to_string();
            let b = other.data.to_string();
            Any::new(a + &b)
        } else if self.type_id == *F64 || other.type_id == *F64 {
            let a: f64 = self.data.to_float();
            let b = other.data.to_float();
            Any::new(a + b)
        } else if self.type_id == *I64 || other.type_id == *I64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == *I32 || other.type_id == *I32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == *I16 || other.type_id == *I16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == *I8 || other.type_id == *I8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == *U64 || other.type_id == *U64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == *U32 || other.type_id == *U32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == *U16 || other.type_id == *U16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == *U8 || other.type_id == *U8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a + b)
        } else if self.type_id == other.type_id {
            match self.type_id {
                type_id if type_id == *I8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *I16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *I32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *I64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *U8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *U16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *U32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *U64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a + b)
                }
                type_id if type_id == *F32 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a + b)
                }
                type_id if type_id == *F64 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a + b)
                }
                type_id if type_id == *STRING => {
                    let mut a = self.data.to_string();
                    let b = other.data.to_string();
                    a.push_str(b.as_str());
                    Any::new(a)
                }
                type_id if type_id == *STR => {
                    let mut a = self.data.to_string();
                    let b = other.data.to_string();
                    a.push_str(b.as_str());
                    Any::new(a)
                }
                type_id if type_id == *BOOL => {
                    let a = self.data.to_boolean();
                    let b = other.data.to_boolean();
                    Any::new(a || b)
                }
                // TODO: ARRAY, MAP
                type_id if type_id == *ARRAY => {
                    let a = self.data.to_array();
                    let b = other.data.to_array();
                    let mut result = a.clone();
                    result.0.extend(b.0.clone());
                    Any::new(result)
                }
                _ => {
                    let a = self.data.to_string();
                    let b = other.data.to_string();
                    Any::new(a + &b)
                }
            }
        } else {
            let a = self.data.to_string();
            let b = other.data.to_string();
            Any::new(a + &b)
        }
    }
}
