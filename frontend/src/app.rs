#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Router, Route};
use fermi::use_init_atom_root;

use crate::page;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    // 创建路由
    cx.render(rsx! {
        Router {
            Route { to: page::ACCOUNT_REGISTER, page::Register {} },
        }
    })
    
    // let other_things = rsx! {
    //     p {
    //         "hi rust!"
    //     }
    // };

    // cx.render(rsx! {
    //     h1 {
    //         "hello rust!"
    //     }
    //     div {}
    //     p {}
    //     other_things,
    //     match other_options {
    //         Some(_) => rsx! {""},
    //         None => rsx! {" Not Found!"}
    //     }
    // })
}
