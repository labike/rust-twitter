#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::{elements::{keyed_notification_box::KeyedNotifications, KeyedNotificationBox}, maybe_class, prelude::*};

pub struct PageState {
  username: UseState<String>,
  password: UseState<String>,
  form_errors: KeyedNotifications,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        !(self.form_errors.has_message() ||
          self.username.current().is_empty() ||
          self.password.current().is_empty())
    }
}

// 事件处理需要有生命周期并且与scope关联
#[inline_props]
pub fn UserNameInput<'a> (
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render (
        rsx! {
            div {
                class: "flex flex-col",
                label {
                    r#for: "username",
                    "Username",
                },
                input {
                    id: "username",
                    name: "username",
                    class: "input-field",
                    placeholder: "User Name",
                    value: "{state.current()}",
                    oninput: move |ev| oninput.call(ev),
                }
            }
        }
    )
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render (
        rsx! {
            div {
                class: "flex flex-col",
                label {
                    r#for: "password",
                    "Password",
                },
                input {
                    id: "password",
                    name: "password",
                    class: "input-field",
                    placeholder: "User Password",
                    value: "{state.current()}",
                    oninput: move |ev| oninput.call(ev),
                }
            }
        }
    )
}

pub fn Register(cx: Scope) -> Element {
    // 创建页面状态
    let page_state = PageState::new(cx);
    // 用use_ref创建持久引用， 让该状态可以在渲染周期和回调中安全的使用而不会被丢失
    let page_state = use_ref(cx, || page_state);

    // with_mut用于获取一个可变引用来修改状态
    // clone用来复制value, 由于e.value可能只是一个引用, 而Rust所有权规则要求在闭包中move进去的变量需要保证其完整性
    // 如果只是普通引用, 一旦引用的原始数据发生变化闭包内的数据就会变得不可靠
    let username_input = sync_handler!(
        [page_state],
        move |ev: FormEvent| {
            if let Err(e) = uchat_domain::Username::new(&ev.value) {
                page_state.with_mut(|state| state.form_errors.set("bad-username", e.to_string()));
            } else {
                page_state.with_mut(|state| state.form_errors.remove("bad-username"));
            };
            page_state.with_mut(|state| state.username.set(ev.value.clone()));
        }
    );

    let password_input = sync_handler!(
        [page_state],
        move |ev: FormEvent| {
            if let Err(e) = uchat_domain::Password::new(&ev.value) {
                page_state.with_mut(|state| state.form_errors.set("bad-password", e.to_string()));
            } else {
                page_state.with_mut(|state| state.form_errors.remove("bad-password"));
            };
            page_state.with_mut(|state| state.password.set(ev.value.clone()));
        }
    );

    let submit_btn_style = maybe_class!("btn-disabled", !page_state.with(|state| state.can_submit()));

    // let submit_btn_style = match page_state.with(|state| state.can_submit()){
    //     false  => "btn-disabled",
    //     true => ""
    // };

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            // 创建闭包, 在处理表单时填充闭包
            onsubmit: move |_| {},

            UserNameInput {
                // with方法用于临时获取状态中的值的引用
                state: page_state.with(|state| state.username.clone()),
                oninput: username_input,
            },

            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_input,
            }

            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.clone().with(|state| state.form_errors.clone())
            }

            button {
                class: "btn {submit_btn_style}",
                disabled: !page_state.with(|state| state.can_submit()),
                // type是rust中的保留字
                r#type: "submit",
                "Signup"
            }
        }
    })
}