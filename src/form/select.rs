#![allow(clippy::redundant_closure_call)]

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::events::Event;
use yew::prelude::*;

use crate::Size;

pub enum SelectMsg {
    Text(String),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
pub struct Select;

impl Component for Select {
    type Message = SelectMsg;
    type Properties = SelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SelectMsg::Text(text) => {
                ctx.props().update.emit(text);
            }
        };
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SelectProps {
            children,
            classes,
            disabled,
            name,
            size,
            loading,
            value,
            ..
        } = ctx.props();
        let mut select_classes = Classes::from("select");
        select_classes.push(classes);
        if let Some(size) = &size {
            select_classes.push(&size.to_string());
        }
        if *loading {
            select_classes.push("is-loading");
        }
        html! {
            <div class={select_classes}>
                <select
                    name={name.clone()}
                    value={value.clone()}
                    disabled={*disabled}
                    onchange={ctx.link().batch_callback(|event: Event| {
                        let target: Option<EventTarget> = event.target();
                        let select = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
                        select.map(|select| SelectMsg::Text(select.value()))
                    })}
                >
                    {children.clone()}
                </select>
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

pub enum MultiSelectMsg {
    Selections(Vec<String>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: Vec<String>,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<Vec<String>>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Size of the list to display.
    #[prop_or_else(|| 4)]
    pub list_size: u32,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag with the `multiple=true` attribute.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
pub struct MultiSelect;

impl Component for MultiSelect {
    type Message = MultiSelectMsg;
    type Properties = MultiSelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MultiSelectMsg::Selections(selections) => {
                ctx.props().update.emit(selections);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let MultiSelectProps {
            children,
            classes,
            disabled,
            loading,
            list_size,
            name,
            size,
            value,
            ..
        } = ctx.props();
        let mut select_classes = Classes::from("select is-multiple");
        select_classes.push(classes);
        if let Some(size) = &size {
            select_classes.push(&size.to_string());
        }
        if *loading {
            select_classes.push("is-loading");
        }

        let size: String = list_size.to_string();
        html! {
            <div class={select_classes}>
                <select
                    multiple=true
                    size={size}
                    name={name.clone()}
                    value={value.join(",")}
                    disabled={*disabled}
                    onchange={ctx.link().batch_callback(|event: Event| {
                        let target: Option<EventTarget> = event.target();
                        let select = target
                            .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
                        select.map(|select| {
                            let opts = select.selected_options();
                            let opts = (0..opts.length()).into_iter()
                                .filter_map(|idx| opts.item(idx))
                                .filter_map(|elem| elem.get_attribute("value")
                                    .or_else(|| elem.text_content())
                                )
                                .collect::<Vec<_>>();
                            MultiSelectMsg::Selections(opts)
                        })
                    })}
                >
                    {children.clone()}
                </select>
            </div>
        }
    }
}
