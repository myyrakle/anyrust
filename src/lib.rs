#![allow(non_upper_case_globals)]

use std::{
    any::TypeId,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Not, Sub, SubAssign},
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
pub const _null: Null = Null {};

#[allow(non_upper_case_globals)]
// array type
#[derive(Debug, Clone)]
pub struct Array(Vec<Any>);

impl Array {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: Any) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<Any> {
        self.0.pop()
    }

    pub fn shift(&mut self) -> Option<Any> {
        let first_value = self.0.first().cloned()?;
        self.0.remove(0);

        Some(first_value)
    }

    pub fn unshift(&mut self, value: Any) {
        self.0.insert(0, value);
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.0.reverse();
        self
    }
}

#[cfg(test)]
mod test_array {
    use super::*;

    #[test]
    fn test_push() {
        let mut a = Array::new();
        assert_eq!(a.length(), 0);

        a.push(Any::new(1));
        assert_eq!(a.length(), 1);
        assert_eq!(a.0[0], Any::from(1));
    }

    #[test]
    fn test_pop() {
        let mut a = Array::new();
        a.push(Any::new(1));
        a.push(Any::new(2));
        a.push(Any::new(3));

        assert_eq!(a.length(), 3);

        let value = a.pop().unwrap();
        assert_eq!(value, Any::from(3));
        assert_eq!(a.length(), 2);
    }

    #[test]
    fn test_shift() {
        let mut a = Array::new();
        a.push(Any::new(1));
        a.push(Any::new(2));
        a.push(Any::new(3));

        assert_eq!(a.length(), 3);

        let value = a.shift().unwrap();
        assert_eq!(value, Any::from(1));
        assert_eq!(a.length(), 2);
    }

    #[test]
    fn test_unshift() {
        let mut a = Array::new();
        assert_eq!(a.length(), 0);

        a.unshift(Any::new(1));
        assert_eq!(a.length(), 1);
        assert_eq!(a.0[0], Any::from(1));

        a.unshift(Any::new(2));
        assert_eq!(a.length(), 2);
        assert_eq!(a.0[0], Any::from(2));
    }

    #[test]
    fn test_length() {
        let mut a = Array::new();
        assert_eq!(a.length(), 0);

        a.push(Any::new(1));
        assert_eq!(a.length(), 1);

        a.push(Any::new(2));
        assert_eq!(a.length(), 2);

        a.push(Any::new(3));
        assert_eq!(a.length(), 3);
    }

    #[test]
    fn test_is_empty() {
        let mut a = Array::new();
        assert!(a.is_empty());

        a.push(Any::new(1));
        assert!(!a.is_empty());
    }

    #[test]
    fn test_reverse() {
        let mut a = Array::new();
        a.push(Any::new(1));
        a.push(Any::new(2));
        a.push(Any::new(3));

        assert_eq!(a.length(), 3);
        assert_eq!(a.0[0], Any::from(1));
        assert_eq!(a.0[1], Any::from(2));
        assert_eq!(a.0[2], Any::from(3));

        a.reverse();

        assert_eq!(a.length(), 3);
        assert_eq!(a.0[0], Any::from(3));
        assert_eq!(a.0[1], Any::from(2));
        assert_eq!(a.0[2], Any::from(1));
    }
}

#[derive(Debug, Clone)]
pub struct Pair((Any, Any));

// key-value map type
#[derive(Debug, Clone)]
pub struct Map(std::collections::HashMap<Any, Any>);

impl Map {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set(&mut self, key: Any, value: Any) {
        self.0.insert(key, value);
    }

    pub fn delete(&mut self, key: &Any) -> Option<Any> {
        self.0.remove(key)
    }

    pub fn get(&self, key: &Any) -> Option<&Any> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &Any) -> Option<&mut Any> {
        self.0.get_mut(key)
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test_map {
    use super::*;

    #[test]
    fn test_set() {
        let mut m = Map::new();
        assert_eq!(m.length(), 0);

        m.set(Any::new("key"), Any::new("value"));
        assert_eq!(m.length(), 1);
        assert_eq!(m.0.get(&Any::new("key")).unwrap(), &Any::new("value"));
    }

    #[test]
    fn test_delete() {
        let mut m = Map::new();
        m.set(Any::new("key"), Any::new("value"));
        assert_eq!(m.length(), 1);

        let value = m.delete(&Any::new("key")).unwrap();
        assert_eq!(value, Any::new("value"));
        assert_eq!(m.length(), 0);
    }

    #[test]
    fn test_get() {
        let mut m = Map::new();
        m.set(Any::new("key"), Any::new("value"));

        let value = m.get(&Any::new("key")).unwrap();
        assert_eq!(value, &Any::new("value"));
    }

    #[test]
    fn test_get_mut() {
        let mut m = Map::new();
        m.set(Any::new("key"), Any::new("value"));

        let value = m.get_mut(&Any::new("key")).unwrap();
        assert_eq!(value, &Any::new("value"));
    }

    #[test]
    fn test_length() {
        let mut m = Map::new();
        assert_eq!(m.length(), 0);

        m.set(Any::new("key"), Any::new("value"));
        assert_eq!(m.length(), 1);

        m.set(Any::new("key2"), Any::new("value2"));
        assert_eq!(m.length(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut m = Map::new();
        assert!(m.is_empty());

        m.set(Any::new("key"), Any::new("value"));
        assert!(!m.is_empty());
    }
}

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

    fn to_array_ref(&self) -> &Array {
        &EMPTY_ARRAY
    }

    fn to_array_mut(&mut self) -> &mut Array {
        unreachable!()
    }
}

// Trait for casting: Defines how to convert when cast to a map.
pub trait ToMap {
    fn to_map(&self) -> Map;

    fn to_map_ref(&self) -> &Map {
        &EMPTY_MAP
    }

    fn to_map_mut(&mut self) -> &mut Map {
        unreachable!()
    }
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
impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Array {}

impl<T> From<Vec<T>> for Any
where
    T: Anyable,
{
    fn from(value: Vec<T>) -> Self {
        Any::new(Array(value.into_iter().map(|v| Any::new(v)).collect()))
    }
}

impl From<Array> for Any {
    fn from(array: Array) -> Self {
        Any::new(array)
    }
}

impl From<Vec<Any>> for Array {
    fn from(vec: Vec<Any>) -> Self {
        Self(vec)
    }
}

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

    fn to_array_ref(&self) -> &Array {
        self
    }

    fn to_array_mut(&mut self) -> &mut Array {
        self
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

// Pair 트레잇 구현
impl From<(Any, Any)> for Pair {
    fn from(value: (Any, Any)) -> Self {
        Pair(value)
    }
}

impl From<Pair> for Any {
    fn from(pair: Pair) -> Self {
        Any::new(pair)
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0 .0, self.0 .1)
    }
}

impl ToInteger for Pair {
    fn to_integer(&self) -> i64 {
        0
    }
}

impl ToStr for Pair {
    fn to_str(&self) -> String {
        format!("({}, {})", self.0 .0, self.0 .1)
    }
}

impl ToFloat for Pair {
    fn to_float(&self) -> f64 {
        0.0
    }
}

impl ToArray for Pair {
    fn to_array(&self) -> Array {
        vec![Any::new(self.clone())].into()
    }
}

impl ToMap for Pair {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for Pair {
    fn to_boolean(&self) -> bool {
        true
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

// isize 트레잇 구현
impl From<isize> for Any {
    fn from(value: isize) -> Self {
        Any::new(value)
    }
}

impl ToInteger for isize {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for isize {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for isize {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for isize {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for isize {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for isize {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}

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

// usize 트레잇 구현
impl From<usize> for Any {
    fn from(value: usize) -> Self {
        Any::new(value)
    }
}

impl ToInteger for usize {
    fn to_integer(&self) -> i64 {
        *self as i64
    }
}

impl ToStr for usize {
    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl ToFloat for usize {
    fn to_float(&self) -> f64 {
        *self as f64
    }
}

impl ToArray for usize {
    fn to_array(&self) -> Array {
        vec![Any::new(*self)].into()
    }
}

impl ToMap for usize {
    fn to_map(&self) -> Map {
        Map(HashMap::new())
    }
}

impl ToBoolean for usize {
    fn to_boolean(&self) -> bool {
        *self != 0
    }
}
// ---------------

// f32 트레잇 구현
impl From<f32> for Any {
    fn from(value: f32) -> Self {
        Any::new(value)
    }
}

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
impl From<f64> for Any {
    fn from(value: f64) -> Self {
        Any::new(value)
    }
}

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
impl From<String> for Any {
    fn from(value: String) -> Self {
        Any::new(value)
    }
}

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
impl From<&str> for Any {
    fn from(value: &str) -> Self {
        Any::new(value.to_string())
    }
}

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
impl From<bool> for Any {
    fn from(value: bool) -> Self {
        Any::new(value)
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
impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Map {}

impl From<HashMap<Any, Any>> for Any {
    fn from(value: HashMap<Any, Any>) -> Self {
        Any::new(Map(value))
    }
}

impl From<HashMap<Any, Any>> for Map {
    fn from(value: HashMap<Any, Any>) -> Self {
        Map(value)
    }
}

impl From<Map> for Any {
    fn from(value: Map) -> Self {
        Any::new(value)
    }
}

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

    fn to_map_ref(&self) -> &Map {
        self
    }

    fn to_map_mut(&mut self) -> &mut Map {
        self
    }
}

impl ToBoolean for Map {
    fn to_boolean(&self) -> bool {
        true
    }
}

// Null 트레잇 구현
impl From<Null> for Any {
    fn from(value: Null) -> Self {
        Any::new(value)
    }
}

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

// type check functions
impl Any {
    pub fn is_integer(&self) -> bool {
        self.type_id == *I8
            || self.type_id == *I16
            || self.type_id == *I32
            || self.type_id == *I64
            || self.type_id == *U8
            || self.type_id == *U16
            || self.type_id == *U32
            || self.type_id == *U64
            || self.type_id == *ISIZE
            || self.type_id == *USIZE
    }

    pub fn is_float(&self) -> bool {
        self.type_id == *F32 || self.type_id == *F64
    }

    pub fn is_number(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    pub fn is_nan(&self) -> bool {
        self.is_float() && self.data.to_float().is_nan()
    }

    pub fn is_string(&self) -> bool {
        self.type_id == *STRING || self.type_id == *STR
    }

    pub fn is_array(&self) -> bool {
        self.type_id == *ARRAY
    }

    pub fn is_map(&self) -> bool {
        self.type_id == *MAP
    }

    pub fn is_null(&self) -> bool {
        self.type_id == *NULL
    }

    pub fn is_boolean(&self) -> bool {
        self.type_id == *BOOL
    }
}

#[cfg(test)]
mod test_type_check_for_any {
    use super::*;

    #[test]
    fn test_is_integer() {
        let a = Any::new(5_i64);
        assert!(a.is_integer());

        let a = Any::new(5_u64);
        assert!(a.is_integer());

        let a = Any::new(5_i32);
        assert!(a.is_integer());

        let a = Any::new(5_u32);
        assert!(a.is_integer());

        let a = Any::new(5_i16);
        assert!(a.is_integer());

        let a = Any::new(5_u16);
        assert!(a.is_integer());

        let a = Any::new(5_i8);
        assert!(a.is_integer());

        let a = Any::new(5_u8);
        assert!(a.is_integer());

        let a = Any::new(5_isize);
        assert!(a.is_integer());

        let a = Any::new(5_usize);
        assert!(a.is_integer());

        let a = Any::new(5.0);
        assert!(!a.is_integer());

        let a = Any::new("5");
        assert!(!a.is_integer());

        let a = Any::new("5.0");
        assert!(!a.is_integer());

        let a = Any::new(true);
        assert!(!a.is_integer());

        let a = Any::new(_null);
        assert!(!a.is_integer());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_integer());

        let a = Any::from(HashMap::new());
        assert!(!a.is_integer());
    }

    #[test]
    fn test_is_float() {
        let a = Any::new(5.0);
        assert!(a.is_float());

        let a = Any::new(5.0_f32);
        assert!(a.is_float());

        let a = Any::new(5.0_f64);
        assert!(a.is_float());

        let a = Any::new(5);
        assert!(!a.is_float());

        let a = Any::new("5");
        assert!(!a.is_float());

        let a = Any::new("5.0");
        assert!(!a.is_float());

        let a = Any::new(true);
        assert!(!a.is_float());

        let a = Any::new(_null);
        assert!(!a.is_float());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_float());

        let a = Any::from(HashMap::new());
        assert!(!a.is_float());
    }

    #[test]
    fn test_is_number() {
        let a = Any::new(5.0);
        assert!(a.is_number());

        let a = Any::new(5.0_f32);
        assert!(a.is_number());

        let a = Any::new(5.0_f64);
        assert!(a.is_number());

        let a = Any::new(5);
        assert!(a.is_number());

        let a = Any::new("5");
        assert!(!a.is_number());

        let a = Any::new("5.0");
        assert!(!a.is_number());

        let a = Any::new(true);
        assert!(!a.is_number());

        let a = Any::new(_null);
        assert!(!a.is_number());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_number());

        let a = Any::from(HashMap::new());
        assert!(!a.is_number());
    }

    #[test]
    fn test_is_nan() {
        let a = Any::new(f64::NAN);
        assert!(a.is_nan());

        let a = Any::new(5.0);
        assert!(!a.is_nan());

        let a = Any::new(5);
        assert!(!a.is_nan());

        let a = Any::new("5");
        assert!(!a.is_nan());

        let a = Any::new("5.0");
        assert!(!a.is_nan());

        let a = Any::new(true);
        assert!(!a.is_nan());

        let a = Any::new(_null);
        assert!(!a.is_nan());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_nan());

        let a = Any::from(HashMap::new());
        assert!(!a.is_nan());
    }

    #[test]
    fn test_is_string() {
        let a = Any::new("5");
        assert!(a.is_string());

        let a = Any::new("5.0");
        assert!(a.is_string());

        let a = Any::new(5.0);
        assert!(!a.is_string());

        let a = Any::new(5);
        assert!(!a.is_string());

        let a = Any::new(true);
        assert!(!a.is_string());

        let a = Any::new(_null);
        assert!(!a.is_string());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_string());

        let a = Any::from(HashMap::new());
        assert!(!a.is_string());
    }

    #[test]
    fn test_is_array() {
        let a = Any::from(vec![1, 2, 3]);
        assert!(a.is_array());

        let a = Any::from(HashMap::new());
        assert!(!a.is_array());

        let a = Any::new("5");
        assert!(!a.is_array());

        let a = Any::new("5.0");
        assert!(!a.is_array());

        let a = Any::new(5.0);
        assert!(!a.is_array());

        let a = Any::new(5);
        assert!(!a.is_array());

        let a = Any::new(true);
        assert!(!a.is_array());

        let a = Any::new(_null);
        assert!(!a.is_array());
    }

    #[test]
    fn test_is_map() {
        let a = Any::from(HashMap::new());
        assert!(a.is_map());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_map());

        let a = Any::new("5");
        assert!(!a.is_map());

        let a = Any::new("5.0");
        assert!(!a.is_map());

        let a = Any::new(5.0);
        assert!(!a.is_map());

        let a = Any::new(5);
        assert!(!a.is_map());

        let a = Any::new(true);
        assert!(!a.is_map());

        let a = Any::new(_null);
        assert!(!a.is_map());
    }

    #[test]
    fn test_is_null() {
        let a = Any::new(_null);
        assert!(a.is_null());

        let a = Any::from(HashMap::new());
        assert!(!a.is_null());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_null());

        let a = Any::new("5");
        assert!(!a.is_null());

        let a = Any::new("5.0");
        assert!(!a.is_null());

        let a = Any::new(5.0);
        assert!(!a.is_null());

        let a = Any::new(5);
        assert!(!a.is_null());

        let a = Any::new(true);
        assert!(!a.is_null());
    }

    #[test]
    fn test_is_boolean() {
        let a = Any::new(true);
        assert!(a.is_boolean());

        let a = Any::new(false);
        assert!(a.is_boolean());

        let a = Any::new(5.0);
        assert!(!a.is_boolean());

        let a = Any::new(5);
        assert!(!a.is_boolean());

        let a = Any::new("5");
        assert!(!a.is_boolean());

        let a = Any::new("5.0");
        assert!(!a.is_boolean());

        let a = Any::new(_null);
        assert!(!a.is_boolean());

        let a = Any::from(vec![1, 2, 3]);
        assert!(!a.is_boolean());

        let a = Any::from(HashMap::new());
        assert!(!a.is_boolean());
    }
}

// array operations
impl Any {
    pub fn push(&mut self, value: Any) {
        if self.is_array() {
            self.data.to_array_mut().push(value)
        }
    }

    pub fn pop(&mut self) -> Option<Any> {
        if self.is_array() {
            self.data.to_array_mut().pop()
        } else {
            None
        }
    }

    pub fn unshift(&mut self, value: Any) {
        if self.is_array() {
            self.data.to_array_mut().unshift(value)
        }
    }

    pub fn shift(&mut self) -> Option<Any> {
        if self.is_array() {
            self.data.to_array_mut().shift()
        } else {
            None
        }
    }

    pub fn reverse(&mut self) -> Any {
        if self.is_array() {
            self.data.to_array_mut().reverse().clone().into()
        } else {
            Any::from(_null)
        }
    }
}

// map operations
impl Any {
    pub fn set(&mut self, key: Any, value: Any) {
        if self.is_map() {
            self.data.to_map_mut().0.insert(key, value);
        }
    }

    pub fn get(&self, key: Any) -> Any {
        if self.is_map() {
            self.data
                .to_map()
                .0
                .get(&key)
                .cloned()
                .unwrap_or_else(|| Any::from(_null))
        } else {
            Any::from(_null)
        }
    }

    pub fn delete(&mut self, key: Any) -> Any {
        if self.is_map() {
            self.data
                .to_map_mut()
                .0
                .remove(&key)
                .unwrap_or_else(|| Any::from(_null))
        } else {
            Any::from(_null)
        }
    }
}

// common operations
impl Any {
    pub fn length(&self) -> Any {
        if self.is_array() {
            self.data.to_array().length().into()
        } else if self.is_map() {
            self.data.to_map().length().into()
        } else if self.is_string() {
            self.data.to_string().len().into()
        } else {
            Any::from(_null)
        }
    }

    pub fn is_empty(&self) -> Any {
        if self.is_array() {
            self.data.to_array().is_empty().into()
        } else if self.is_map() {
            self.data.to_map().is_empty().into()
        } else if self.is_string() {
            self.data.to_string().is_empty().into()
        } else {
            Any::from(_null)
        }
    }
}

lazy_static::lazy_static! {
    pub static ref I8: TypeId = TypeId::of::<i8>();
    pub static ref I16: TypeId = TypeId::of::<i16>();
    pub static ref I32: TypeId = TypeId::of::<i32>();
    pub static ref I64: TypeId = TypeId::of::<i64>();
    pub static ref ISIZE: TypeId = TypeId::of::<isize>();
    pub static ref U8: TypeId = TypeId::of::<u8>();
    pub static ref U16: TypeId = TypeId::of::<u16>();
    pub static ref U32: TypeId = TypeId::of::<u32>();
    pub static ref U64: TypeId = TypeId::of::<u64>();
    pub static ref USIZE: TypeId = TypeId::of::<usize>();
    pub static ref F32: TypeId = TypeId::of::<f32>();
    pub static ref F64: TypeId = TypeId::of::<f64>();
    pub static ref STRING: TypeId = TypeId::of::<String>();
    pub static ref STR: TypeId = TypeId::of::<&str>();
    pub static ref BOOL: TypeId = TypeId::of::<bool>();
    pub static ref ARRAY: TypeId = TypeId::of::<Array>();
    pub static ref MAP: TypeId = TypeId::of::<Map>();
    pub static ref NULL: TypeId = TypeId::of::<Null>();


    static ref null: Any = Any::new(_null);
    static ref EMPTY_ARRAY: Array = Array(vec![]);
    static ref EMPTY_MAP: Map = Map(HashMap::new());
}

impl Add for Any {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.type_id == *NULL || other.type_id == *NULL {
            Any::new(_null)
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
        } else if self.type_id == *F32 || other.type_id == *F32 {
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
            let result = a + b;
            Any::new(result)
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
        } else {
            let a = self.data.to_string();
            let b = other.data.to_string();
            Any::new(a + &b)
        }
    }
}

impl AddAssign for Any {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

#[cfg(test)]
mod test_add_for_any {
    use super::*;

    #[test]
    fn test_add() {
        struct TestCase {
            name: String,
            a: Any,
            b: Any,
            result: Any,
        }

        let test_cases = vec![
            TestCase {
                name: "i64".to_string(),
                a: Any::new(5_i64),
                b: Any::new(10_i64),
                result: Any::new(15_i64),
            },
            TestCase {
                name: "u64".to_string(),
                a: Any::new(5_u64),
                b: Any::new(10_u64),
                result: Any::new(15_i64),
            },
            TestCase {
                name: "f32".to_string(),
                a: Any::new(5.0_f32),
                b: Any::new(10.0_f32),
                result: Any::new(15.0_f64),
            },
            TestCase {
                name: "f64".to_string(),
                a: Any::new(5.0),
                b: Any::new(10.0),
                result: Any::new(15.0),
            },
            TestCase {
                name: "string".to_string(),
                a: Any::new("5".to_string()),
                b: Any::new("10".to_string()),
                result: Any::new("510".to_string()),
            },
            TestCase {
                name: "str".to_string(),
                a: Any::new("5"),
                b: Any::new("10"),
                result: Any::new("510".to_string()),
            },
        ];

        for test_case in test_cases {
            let result = test_case.a + test_case.b;
            assert_eq!(result, test_case.result, "TC: {}", test_case.name);
        }
    }

    #[test]
    fn test_add_assign() {
        let mut a = Any::new(5_i64);
        let b = Any::new(10_i64);
        a += b;
        assert_eq!(a, Any::new(15_i64));
    }
}

impl Sub for Any {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.type_id == *NULL || other.type_id == *NULL {
            Any::new(_null)
        } else if self.type_id == other.type_id {
            match self.type_id {
                type_id if type_id == *I8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *I16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *I32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *I64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *U8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *U16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *U32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *U64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a - b)
                }
                type_id if type_id == *F32 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a - b)
                }
                type_id if type_id == *F64 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a - b)
                }
                _ => Any::new(f64::NAN),
            }
        } else if self.type_id == *STRING || other.type_id == *STRING {
            Any::new(f64::NAN)
        } else if self.type_id == *STR || other.type_id == *STR {
            Any::new(f64::NAN)
        } else if self.type_id == *F64 || other.type_id == *F64 {
            let a: f64 = self.data.to_float();
            let b = other.data.to_float();
            Any::new(a - b)
        } else if self.type_id == *F32 || other.type_id == *F32 {
            let a: f64 = self.data.to_float();
            let b = other.data.to_float();
            Any::new(a - b)
        } else if self.type_id == *I64 || other.type_id == *I64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a - b)
        } else if self.type_id == *I32 || other.type_id == *I32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a - b)
        } else if self.type_id == *I16 || other.type_id == *I16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a - b)
        } else if self.type_id == *I8 || other.type_id == *I8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            let result = a + b;
            Any::new(result)
        } else if self.type_id == *U64 || other.type_id == *U64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a - b)
        } else if self.type_id == *U32 || other.type_id == *U32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a - b)
        } else if self.type_id == *U16 || other.type_id == *U16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a - b)
        } else if self.type_id == *U8 || other.type_id == *U8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a - b)
        } else {
            Any::new(f64::NAN)
        }
    }
}

impl SubAssign for Any {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

#[cfg(test)]
mod test_sub_for_any {
    use super::*;

    #[test]
    fn test_sub() {
        struct TestCase {
            name: String,
            a: Any,
            b: Any,
            result: Any,
        }

        let test_cases = vec![
            TestCase {
                name: "i64".to_string(),
                a: Any::new(5_i64),
                b: Any::new(10_i64),
                result: Any::new(-5_i64),
            },
            TestCase {
                name: "u64".to_string(),
                a: Any::new(5_u64),
                b: Any::new(10_u64),
                result: Any::new(-5_i64),
            },
            TestCase {
                name: "f32".to_string(),
                a: Any::new(5.0_f32),
                b: Any::new(10.0_f32),
                result: Any::new(-5.0_f64),
            },
            TestCase {
                name: "f64".to_string(),
                a: Any::new(5.0),
                b: Any::new(10.0),
                result: Any::new(-5.0),
            },
        ];

        for test_case in test_cases {
            let result = test_case.a - test_case.b;
            assert_eq!(result, test_case.result, "TC: {}", test_case.name);
        }
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Any::new(5_i64);
        let b = Any::new(10_i64);
        a -= b;
        assert_eq!(a, Any::new(-5_i64));
    }
}

impl Mul for Any {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.type_id == *NULL || other.type_id == *NULL {
            Any::new(_null)
        } else if self.type_id == other.type_id {
            match self.type_id {
                type_id if type_id == *I8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *I16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *I32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *I64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *U8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *U16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *U32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *U64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a * b)
                }
                type_id if type_id == *F32 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a * b)
                }
                type_id if type_id == *F64 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a * b)
                }
                _ => Any::new(f64::NAN),
            }
        } else if self.type_id == *STRING || other.type_id == *STRING {
            Any::new(f64::NAN)
        } else if self.type_id == *STR || other.type_id == *STR {
            Any::new(f64::NAN)
        } else if self.type_id == *F64 || other.type_id == *F64 {
            let a: f64 = self.data.to_float();
            let b = other.data.to_float();
            Any::new(a * b)
        } else if self.type_id == *F32 || other.type_id == *F32 {
            let a: f64 = self.data.to_float();
            let b = other.data.to_float();
            Any::new(a * b)
        } else if self.type_id == *I64 || other.type_id == *I64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a * b)
        } else if self.type_id == *I32 || other.type_id == *I32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a * b)
        } else if self.type_id == *I16 || other.type_id == *I16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a * b)
        } else if self.type_id == *I8 || other.type_id == *I8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            let result = a + b;
            Any::new(result)
        } else if self.type_id == *U64 || other.type_id == *U64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a * b)
        } else if self.type_id == *U32 || other.type_id == *U32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a * b)
        } else if self.type_id == *U16 || other.type_id == *U16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a * b)
        } else if self.type_id == *U8 || other.type_id == *U8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a * b)
        } else {
            Any::new(f64::NAN)
        }
    }
}

impl MulAssign for Any {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

#[cfg(test)]
mod test_mul_for_any {
    use super::*;

    #[test]
    fn test_mul() {
        struct TestCase {
            name: String,
            a: Any,
            b: Any,
            result: Any,
        }

        let test_cases = vec![
            TestCase {
                name: "i64".to_string(),
                a: Any::new(5_i64),
                b: Any::new(10_i64),
                result: Any::new(50_i64),
            },
            TestCase {
                name: "u64".to_string(),
                a: Any::new(5_u64),
                b: Any::new(10_u64),
                result: Any::new(50_i64),
            },
            TestCase {
                name: "f32".to_string(),
                a: Any::new(5.0_f32),
                b: Any::new(10.0_f32),
                result: Any::new(50.0_f64),
            },
            TestCase {
                name: "f64".to_string(),
                a: Any::new(5.0),
                b: Any::new(10.0),
                result: Any::new(50.0),
            },
        ];

        for test_case in test_cases {
            let result = test_case.a * test_case.b;
            assert_eq!(result, test_case.result, "TC: {}", test_case.name);
        }
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Any::new(5_i64);
        let b = Any::new(10_i64);
        a *= b;
        assert_eq!(a, Any::new(50_i64));
    }
}

impl Div for Any {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.type_id == *NULL || other.type_id == *NULL {
            Any::new(_null)
        } else if self.type_id == other.type_id {
            match self.type_id {
                type_id if type_id == *I8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *I16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *I32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *I64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *U8 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *U16 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *U32 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *U64 => {
                    let a = self.data.to_integer();
                    let b = other.data.to_integer();
                    Any::new(a / b)
                }
                type_id if type_id == *F32 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a / b)
                }
                type_id if type_id == *F64 => {
                    let a = self.data.to_float();
                    let b = other.data.to_float();
                    Any::new(a / b)
                }
                _ => Any::new(f64::NAN),
            }
        } else if self.type_id == *STRING || other.type_id == *STRING {
            Any::new(f64::NAN)
        } else if self.type_id == *STR || other.type_id == *STR {
            Any::new(f64::NAN)
        } else if self.type_id == *F64 || other.type_id == *F64 {
            let a: f64 = self.data.to_float();
            let b = other.data.to_float();
            Any::new(a / b)
        } else if self.type_id == *F32 || other.type_id == *F32 {
            let a: f64 = self.data.to_float();
            let b = other.data.to_float();
            Any::new(a / b)
        } else if self.type_id == *I64 || other.type_id == *I64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a / b)
        } else if self.type_id == *I32 || other.type_id == *I32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a / b)
        } else if self.type_id == *I16 || other.type_id == *I16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a / b)
        } else if self.type_id == *I8 || other.type_id == *I8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            let result = a + b;
            Any::new(result)
        } else if self.type_id == *U64 || other.type_id == *U64 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a / b)
        } else if self.type_id == *U32 || other.type_id == *U32 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a / b)
        } else if self.type_id == *U16 || other.type_id == *U16 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a / b)
        } else if self.type_id == *U8 || other.type_id == *U8 {
            let a = self.data.to_integer();
            let b = other.data.to_integer();
            Any::new(a / b)
        } else {
            Any::new(f64::NAN)
        }
    }
}

impl DivAssign for Any {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

#[cfg(test)]
mod test_div_for_any {
    use super::*;

    #[test]
    fn test_div() {
        struct TestCase {
            name: String,
            a: Any,
            b: Any,
            result: Any,
        }

        let test_cases = vec![
            TestCase {
                name: "i64".to_string(),
                a: Any::new(5_i64),
                b: Any::new(10_i64),
                result: Any::new(0_i64),
            },
            TestCase {
                name: "u64".to_string(),
                a: Any::new(5_u64),
                b: Any::new(10_u64),
                result: Any::new(0_i64),
            },
            TestCase {
                name: "f32".to_string(),
                a: Any::new(5.0_f32),
                b: Any::new(10.0_f32),
                result: Any::new(0.5_f64),
            },
            TestCase {
                name: "f64".to_string(),
                a: Any::new(5.0),
                b: Any::new(10.0),
                result: Any::new(0.5),
            },
        ];

        for test_case in test_cases {
            let result = test_case.a / test_case.b;
            assert_eq!(result, test_case.result, "TC: {}", test_case.name);
        }
    }

    #[test]
    fn test_div_assign() {
        let mut a = Any::new(5_i64);
        let b = Any::new(10_i64);
        a /= b;
        assert_eq!(a, Any::new(0_i64));
    }
}

impl Not for Any {
    type Output = Self;

    fn not(self) -> Self {
        if self.type_id == *NULL {
            Any::new(_null)
        } else {
            let a = self.data.to_boolean();
            Any::new(!a)
        }
    }
}

#[cfg(test)]
mod test_not_for_any {
    use super::*;

    #[test]
    fn test_not() {
        struct TestCase {
            name: String,
            a: Any,
            result: Any,
        }

        let test_cases = vec![
            TestCase {
                name: "true".to_string(),
                a: Any::new(true),
                result: Any::new(false),
            },
            TestCase {
                name: "false".to_string(),
                a: Any::new(false),
                result: Any::new(true),
            },
            TestCase {
                name: "zero value".to_string(),
                a: Any::new(0),
                result: Any::new(true),
            },
            TestCase {
                name: "non zero value".to_string(),
                a: Any::new(4444),
                result: Any::new(false),
            },
        ];

        for test_case in test_cases {
            let result = !test_case.a;
            assert_eq!(result, test_case.result, "TC: {}", test_case.name);
        }
    }
}

impl PartialEq for Any {
    fn eq(&self, other: &Self) -> bool {
        if self.type_id != other.type_id {
            false
        } else {
            match self.type_id {
                type_id if type_id == *I8 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *I16 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *I32 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *I64 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *U8 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *U16 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *U32 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *U64 => self.data.to_integer() == other.data.to_integer(),
                type_id if type_id == *F32 => self.data.to_float() == other.data.to_float(),
                type_id if type_id == *F64 => self.data.to_float() == other.data.to_float(),
                type_id if type_id == *STRING => self.data.to_string() == other.data.to_string(),
                type_id if type_id == *STR => self.data.to_string() == other.data.to_string(),
                type_id if type_id == *BOOL => self.data.to_boolean() == other.data.to_boolean(),
                type_id if type_id == *ARRAY => self.data.to_array() == other.data.to_array(),
                type_id if type_id == *MAP => self.data.to_map() == other.data.to_map(),
                _ => self.data.to_string() == other.data.to_string(),
            }
        }
    }
}

impl Eq for Any {}

#[cfg(test)]
mod test_eq_for_any {
    use super::*;

    #[test]
    fn test_eq() {
        let a = Any::new(5);
        let b = Any::new(5);
        assert_eq!(a, b);

        let a = Any::new(5);
        let b = Any::new(10);
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::new(5.0);
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::new(5.0);
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::new("5");
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::new("5");
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::new(true);
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::new(false);
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::from(vec![1, 2, 3]);
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::from(HashMap::new());
        assert_ne!(a, b);

        let a = Any::new(5);
        let b = Any::new(_null);
        assert_ne!(a, b);
    }
}

impl Hash for Any {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        match self.type_id {
            type_id if type_id == *I8 => self.data.to_integer().hash(state),
            type_id if type_id == *I16 => self.data.to_integer().hash(state),
            type_id if type_id == *I32 => self.data.to_integer().hash(state),
            type_id if type_id == *I64 => self.data.to_integer().hash(state),
            type_id if type_id == *U8 => self.data.to_integer().hash(state),
            type_id if type_id == *U16 => self.data.to_integer().hash(state),
            type_id if type_id == *U32 => self.data.to_integer().hash(state),
            type_id if type_id == *U64 => self.data.to_integer().hash(state),
            type_id if type_id == *F32 => self.data.to_float().to_bits().hash(state),
            type_id if type_id == *F64 => self.data.to_float().to_bits().hash(state),
            type_id if type_id == *STRING => self.data.to_string().hash(state),
            type_id if type_id == *STR => self.data.to_string().hash(state),
            type_id if type_id == *BOOL => self.data.to_boolean().hash(state),
            type_id if type_id == *ARRAY => self.data.to_array().0.hash(state),
            type_id if type_id == *MAP => self.data.to_string().hash(state),
            _ => self.data.to_string().hash(state),
        }
    }
}

impl<T> Index<T> for Any
where
    T: Into<Any>,
{
    type Output = Any;

    fn index(&self, index: T) -> &Self::Output {
        let key: Any = index.into();

        if self.type_id == *ARRAY {
            let array = self.data.to_array_ref();
            let key = key.data.to_integer() as usize;
            if key >= array.0.len() {
                return &null;
            }

            &array.0[key]
        } else if self.type_id == *MAP {
            let map = self.data.to_map_ref();

            map.0.get(&key).unwrap_or(&null)
        } else {
            &null
        }
    }
}

impl<T> IndexMut<T> for Any
where
    T: Into<Any>,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let key: Any = index.into();

        if self.type_id == *ARRAY {
            let array = self.data.to_array_mut();
            let key = key.data.to_integer() as usize;
            if key >= array.0.len() {
                unsafe {
                    let uninit: std::mem::MaybeUninit<Self::Output> =
                        std::mem::MaybeUninit::uninit();
                    let ptr = uninit.as_ptr() as *mut Self::Output;
                    *ptr = null.clone();
                    return &mut *ptr;
                }
            }

            &mut array.0[key]
        } else if self.type_id == *MAP {
            let map = self.data.to_map_mut();

            if let None = map.0.get(&key) {
                map.0.insert(key.clone(), null.clone());
            }

            map.0.get_mut(&key).unwrap()
        } else {
            unsafe {
                let uninit: std::mem::MaybeUninit<Self::Output> = std::mem::MaybeUninit::uninit();
                let ptr = uninit.as_ptr() as *mut Self::Output;
                *ptr = null.clone();
                &mut *ptr
            }
        }
    }
}

#[cfg(test)]
mod test_indexer_for_any {
    use super::*;

    #[test]
    fn test_array_indexer() {
        let a = Any::from(vec![1, 2, 3]);
        assert_eq!(a[0], Any::new(1));
        assert_eq!(a[1], Any::new(2));
        assert_eq!(a[2], Any::new(3));
        assert_eq!(a[3], Any::new(_null));
    }

    #[test]
    fn test_map_indexer() {
        let mut a = Any::from(HashMap::new());
        a[Any::from(1)] = Any::new(1);
        a[Any::from(2)] = Any::new(2);
        a[3] = Any::new(3);
        assert_eq!(a[Any::from(1)], Any::new(1));
        assert_eq!(a[Any::from(2)], Any::new(2));
        assert_eq!(a[Any::from(3)], Any::new(3));
        assert_eq!(a[Any::from(4)], Any::new(_null));
    }
}

// macro
#[macro_export]

macro_rules! array {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Any::from($x));
            )*

            Any::from(anyrust::Array::from(temp_vec))
        }
    };
}
