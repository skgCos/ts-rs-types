#![allow(dead_code)]

use ts_rs::TS;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TS)]
struct A {
    a: i32,
    b: i32,
}

#[derive(TS)]
struct B {
    #[ts(flatten)]
    a: A,
    c: i32,
}

#[derive(TS)]
struct C {
    #[ts(inline)]
    b: B,
    d: i32,
}

#[test]
fn test_def() {
    assert_eq!(
        C::inline(),
        "{ b: ({ a: number } & { b: number }) & { c: number } } & { d: number }"
    );
}

#[derive(Serialize, Deserialize, TS)]
struct D {
    #[serde(flatten)]
    b: E,
    d: i32,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "t", content = "c")]
enum E {
    Something1(A),
    Something2(A)
}

#[test]
fn test_flatten_adjacently_unnamed() {
    assert_eq!(
        D::inline(),
        "({ t: \"Something1\", c: A } | { t: \"Something2\", c: A }) & { d: number }"
    );
}

#[derive(Serialize, Deserialize, TS)]
struct F {
    #[serde(flatten)]
    b: G,
    d: i32,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "t", content = "c")]
enum G {
    Something1,
    Something2
}

#[test]
fn test_flatten_adjacently_unit() {
    assert_eq!(
        F::inline(),
        "({ t: \"Something1\" } | { t: \"Something2\" }) & { d: number }"
    );
}

#[derive(Serialize, Deserialize, TS)]
struct H {
    #[serde(flatten)]
    b: I,
    d: i32,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "t", content = "c")]
enum I {
    Something1(),
    Something2()
}

#[test]
fn test_flatten_adjacently_none() {
    assert_eq!(
        H::inline(),
        "({ t: \"Something1\", c: null } | { t: \"Something2\", c: null }) & { d: number }"
    );
}

#[derive(Serialize, Deserialize, TS)]
struct J {
    #[serde(flatten)]
    b: K,
    d: i32,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "t")]
enum K {
    Something1(A),
    Something2(A)
}

#[test]
fn test_flatten_internally_none_unnamed() {
    assert_eq!(
        J::inline(),
        "({ t: \"Something1\" } & A | { t: \"Something2\" } & A) & { d: number }"
    );
}

#[derive(Serialize, Deserialize, TS)]
struct L {
    #[serde(flatten)]
    b: M,
    d: i32,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "t")]
enum M {
    Something1,
    Something2
}

#[test]
fn test_flatten_internally_none_unit() {
    assert_eq!(
        J::inline(),
        "({ t: \"Something1\" } & A | { t: \"Something2\" } & A) & { d: number }"
    );
}
