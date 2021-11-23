#![allow(clippy::redundant_closure_call)]
use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: String,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// A versatile delete cross.
///
/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
pub struct Delete;

impl Component for Delete {
    type Message = ();
    type Properties = DeleteProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let DeleteProps { children, classes, onclick, tag } = ctx.props();
        let mut delete_classes = Classes::from("delete");
        delete_classes.push(classes);
        let tag = tag.clone();
        html! {
            <@{tag} class={delete_classes} onclick={onclick.clone()}>
                {children.clone()}
            </@>
        }
    }
}
