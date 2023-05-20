#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "SCREAMING_SNAKE_CASE"))]
#[ts(export)]
enum A {
    MessageOne {
        sender_id: String,
        number_of_snakes: u64,
    },
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    MessageTwo {
        sender_id: String,
        number_of_camels: u64,
    },
}

#[test]
fn test_enum_variant_rename_all() {
    assert_eq!(
        A::inline(),
        "{ MESSAGE_ONE: { sender_id: string } & { number_of_snakes: bigint } } | { MESSAGE_TWO: { senderId: string } & { numberOfCamels: bigint } }",
    );
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
enum B {
    #[cfg_attr(feature = "serde-compat", serde(rename = "SnakeMessage"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename = "SnakeMessage"))]
    MessageOne {
        sender_id: String,
        number_of_snakes: u64,
    },
    #[cfg_attr(feature = "serde-compat", serde(rename = "CamelMessage"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename = "CamelMessage"))]
    MessageTwo {
        sender_id: String,
        number_of_camels: u64,
    },
}

#[test]
fn test_enum_variant_rename() {
    assert_eq!(
        B::inline(),
        "{ SnakeMessage: { sender_id: string } & { number_of_snakes: bigint } } | { CamelMessage: { sender_id: string } & { number_of_camels: bigint } }",
    );
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "kind")]
#[ts(export)]
pub enum C {
    #[serde(rename = "SQUARE_THING")]
    SquareThing {
        name: String,
        // ...
    },
}

#[cfg(feature = "serde")]
#[test]
fn test_enum_variant_with_tag() {
    assert_eq!(C::inline(), "{ kind: \"SQUARE_THING\", name: string, }");
}
