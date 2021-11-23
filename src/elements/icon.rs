use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// A container for any type of icon font.
///
/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
pub struct Icon;

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let IconProps { alignment, children, classes, onclick, size } = ctx.props();
        let mut icon_classes = Classes::from("icon");
        icon_classes.push(classes);
        if let Some(size) = &size {
            icon_classes.push(&size.to_string());
        }
        if let Some(alignment) = &alignment {
            icon_classes.push(&alignment.to_string());
        }
        html! {
            <span class={icon_classes} onclick={onclick.clone()}>
                {children.clone()}
            </span>
        }
    }
}
