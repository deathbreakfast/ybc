#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::events::InputEvent;
use yew::prelude::*;

use crate::Size;

pub enum InputMsg {
    Text(Option<String>),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The input type of this component.
    #[prop_or_else(|| InputType::Text)]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Input;

impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMsg::Text(text) => {
                if let Some(text) = text {
                    ctx.props().update.emit(text);
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let InputProps {
            classes,
            disabled,
            size,
            rounded,
            loading,
            name,
            placeholder,
            readonly,
            r#static,
            r#type,
            value,
            ..
        } = ctx.props();
        let mut input_classes = Classes::from("input");
        input_classes.push(classes);
        if let Some(size) = &size {
            input_classes.push(&size.to_string());
        }
        if *rounded {
            input_classes.push("is-rounded");
        }
        if *loading {
            input_classes.push("is-loading");
        }
        if *r#static {
            input_classes.push("is-static");
        }
        html! {
            <input
                name={name.clone()}
                value={value.clone()}
                oninput={ctx.link().callback(|input: InputEvent| {
                    let target: Option<EventTarget> = input.target();
                    let element = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    InputMsg::Text(element.map(|m| m.value()))
                })}
                class={input_classes}
                type={r#type.to_string()}
                placeholder={placeholder.clone()}
                disabled={*disabled}
                readonly={*readonly}
                />
        }
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq)]
pub enum InputType {
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "tel")]
    Tel,
}
