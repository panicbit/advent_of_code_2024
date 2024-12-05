use regex::Captures;
use std::{collections::BTreeMap, fmt::Debug, str::FromStr, sync::Mutex};

#[doc(hidden)]
pub use lazy_static::lazy_static;
#[doc(hidden)]
pub use regex::Regex;

#[macro_export]
macro_rules! re {
    ($re:expr, $str:expr) => {{
        let re = re!($re);
        re.captures($str).unwrap()
    }};
    ($re:expr) => {{
        $crate::lazy_static! {
            static ref RE: $crate::Regex = $crate::Regex::new($re).unwrap();
        }

        &*RE
    }};
}

pub trait CapturesExt<'a> {
    fn str(self, name: &str) -> &'a str;
    fn parse<T: FromStr>(self, name: &str) -> T
    where
        T::Err: Debug;
}

impl<'a> CapturesExt<'a> for &'_ Captures<'a> {
    fn str(self, name: &str) -> &'a str {
        self.name(name).unwrap().as_str()
    }

    fn parse<T: FromStr>(self, name: &str) -> T
    where
        T::Err: Debug,
    {
        self.str(name).parse().unwrap()
    }
}

pub trait StrExt {
    fn u8(&self) -> u8;
    fn i32(&self) -> i32;
    fn u32(&self) -> u32;
    fn usize(&self) -> usize;
    fn char(&self) -> char;
    fn is_match(&self, regex: &str) -> bool;
}

impl<S: AsRef<str>> StrExt for S {
    fn u8(&self) -> u8 {
        self.as_ref().u8()
    }

    fn i32(&self) -> i32 {
        self.as_ref().i32()
    }

    fn u32(&self) -> u32 {
        self.as_ref().u32()
    }

    fn usize(&self) -> usize {
        self.as_ref().usize()
    }

    fn char(&self) -> char {
        self.as_ref().char()
    }

    fn is_match(&self, regex: &str) -> bool {
        self.as_ref().is_match(regex)
    }
}

impl StrExt for str {
    fn u8(&self) -> u8 {
        self.parse().unwrap()
    }

    fn i32(&self) -> i32 {
        self.parse().unwrap()
    }

    fn u32(&self) -> u32 {
        self.parse().unwrap()
    }

    fn usize(&self) -> usize {
        self.parse().unwrap()
    }

    fn char(&self) -> char {
        assert_eq!(self.len(), 1);

        self.chars().next().unwrap()
    }

    fn is_match(&self, regex: &str) -> bool {
        with_cached_regex(regex, |regex| regex.is_match(self.as_ref()))
    }
}

// TODO: optimize for concurrent access
fn with_cached_regex<F, R>(regex: &str, f: F) -> R
where
    F: FnOnce(&Regex) -> R,
{
    let mut regex_cache = REGEX_CACHE.lock().unwrap();

    let regex = match regex_cache.get(regex) {
        Some(regex) => regex,
        None => {
            let compiled_regex = Regex::new(regex).unwrap();
            regex_cache.insert(regex.into(), compiled_regex);
            regex_cache.get(regex).unwrap()
        }
    };

    f(regex)
}

lazy_static! {
    static ref REGEX_CACHE: Mutex<BTreeMap<String, Regex>> = Mutex::default();
}

pub trait ArrayExt<T> {
    fn from_fn(f: impl FnMut() -> T) -> Self;
}

impl<T> ArrayExt<T> for [T; 2] {
    fn from_fn(mut f: impl FnMut() -> T) -> Self {
        [f(), f()]
    }
}

impl<T> ArrayExt<T> for [T; 3] {
    fn from_fn(mut f: impl FnMut() -> T) -> Self {
        [f(), f(), f()]
    }
}

pub fn array_split<'a, C>(input: &'a str, separator: &str) -> C
where
    C: ArrayExt<&'a str>,
{
    let mut it = input.split(separator);

    let arr = C::from_fn(|| it.next().expect("not enough parts"));

    if it.next().is_some() {
        panic!("extraneous part")
    }

    arr
}

pub fn array_split_parse<'a, T: FromStr, C: ArrayExt<T>>(input: &'a str, separator: &str) -> C
where
    C: ArrayExt<T>,
    T::Err: Debug,
{
    let mut it = input.split(separator);

    let arr = C::from_fn(|| {
        it.next()
            .expect("not enough parts")
            .parse::<T>()
            .expect("failed to parse part")
    });

    if it.next().is_some() {
        panic!("extraneous part")
    }

    arr
}

pub trait TupleExt<T> {
    fn from_fn(f: impl FnMut() -> T) -> Self;
}

impl<T> TupleExt<T> for (T, T) {
    fn from_fn(mut f: impl FnMut() -> T) -> Self {
        (f(), f())
    }
}

impl<T> TupleExt<T> for (T, T, T) {
    fn from_fn(mut f: impl FnMut() -> T) -> Self {
        (f(), f(), f())
    }
}

pub fn tuple_split<'a, C>(input: &'a str, separator: &str) -> C
where
    C: TupleExt<&'a str>,
{
    let mut it = input.split(separator);

    let tuple = C::from_fn(|| it.next().expect("not enough parts"));

    if it.next().is_some() {
        panic!("extraneous part")
    }

    tuple
}

pub fn tuple_split_parse<'a, T, C>(input: &'a str, separator: &str) -> C
where
    C: TupleExt<T>,
    T: FromStr,
    T::Err: Debug,
{
    let mut it = input.split(separator);

    let tuple = C::from_fn(|| {
        it.next()
            .expect("not enough parts")
            .parse::<T>()
            .expect("failed to parse part")
    });

    if it.next().is_some() {
        panic!("extraneous part")
    }

    tuple
}
