#![allow(clippy::redundant_closure_call)]

use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML content of this panel's heading; it is automatically wrapped in a `p.panel-heading`.
    #[prop_or_default]
    pub heading: Html,
}

/// A composable panel, for compact controls.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct Panel;

impl Component for Panel {
    type Message = ();
    type Properties = PanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PanelProps { children, classes, heading } = ctx.props();
        let mut panel_classes = Classes::from("panel");
        panel_classes.push(classes);
        html! {
            <nav class={panel_classes}>
                <p class="panel-heading">{heading.clone()}</p>
                {children.clone()}
            </nav>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelTabsProps {
    #[prop_or_default]
    pub children: Children,
}

/// A container for the navigation tabs of a panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct PanelTabs;

impl Component for PanelTabs {
    type Message = ();
    type Properties = PanelTabsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p class="panel-tabs">
                {ctx.props().children.clone()}
            </p>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelBlockProps {
    #[prop_or_default]
    pub children: Children,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// Make this element the active / highlighted element.
    #[prop_or_default]
    pub active: bool,
    /// The click handler for this element.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// An individual element of the panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct PanelBlock;

impl Component for PanelBlock {
    type Message = ();
    type Properties = PanelBlockProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PanelBlockProps { active, children, onclick, tag } = ctx.props();
        let mut panel_classes = Classes::from("panel-block");
        if *active {
            panel_classes.push("is-active");
        }
        let tag = tag.clone();
        html! {
            <@{tag} class={panel_classes} onclick={onclick.clone()}>
                {children.clone()}
            </@>
        }
    }
}
