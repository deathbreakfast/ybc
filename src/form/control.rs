#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ControlProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: bool,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
pub struct Control;

impl Component for Control {
    type Message = ();
    type Properties = ControlProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ControlProps { children, classes, expanded, tag } = ctx.props();
        let mut control_classes = Classes::from("control");
        control_classes.push(classes);
        if *expanded {
            control_classes.push("is-expanded");
        }
        let tag = tag.clone();
        html! {
            <@{tag} class={control_classes}>
                {children.clone()}
            </@>
        }
    }
}
