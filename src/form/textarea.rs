use yew::events::InputEvent;
use yew::prelude::*;

use crate::Size;

pub enum TextAreaMsg {
    Text(Option<String>),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextAreaProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The number of rows to which this component will be locked.
    #[prop_or_default]
    pub rows: u32,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Fix the size of this component.
    #[prop_or_default]
    pub fixed_size: bool,
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

/// A multiline textarea component.
///
/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct TextArea;

impl Component for TextArea {
    type Message = TextAreaMsg;
    type Properties = TextAreaProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextAreaMsg::Text(text) => {
                if let Some(text) = text {
                    ctx.props().update.emit(text);
                }
            }
        };
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let TextAreaProps {
            classes,
            disabled,
            fixed_size,
            r#static,
            loading,
            name,
            placeholder,
            readonly,
            rows,
            size,
            value,
            ..
        } = ctx.props();
        let mut textarea_classes = Classes::from("textarea");
        textarea_classes.push(classes);
        if let Some(size) = &size {
            textarea_classes.push(&size.to_string());
        }
        if *loading {
            textarea_classes.push("is-loading");
        }
        if *r#static {
            textarea_classes.push("is-static");
        }
        if *fixed_size {
            textarea_classes.push("has-fixed-size");
        }

        html! {
            <textarea
                name={name.clone()}
                value={value.clone()}
                oninput={ctx.link().callback(|input: InputEvent| TextAreaMsg::Text(input.data()))}
                class={textarea_classes}
                rows={rows.to_string()}
                placeholder={placeholder.clone()}
                disabled={*disabled}
                readonly={*readonly}
                />
        }
    }
}
