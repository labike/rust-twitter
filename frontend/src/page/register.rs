#![allow(non_snake_case)]

// 用dioxus中所有元素
use dioxus::prelude::*;

pub fn Register(cx: Scope) -> Element {
  cx.render(rsx! {
    "register page"
  })
}
