use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add a `32px` margin to the left and right sides of the container.
    #[prop_or_default]
    pub fluid: bool,
}

/// A simple container to center your content horizontally.
///
/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
pub struct Container;

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ContainerProps { children, classes, fluid } = ctx.props();
        let mut container_classes = Classes::from("container");
        container_classes.push(classes);
        if *fluid {
            container_classes.push("is-fluid");
        }
        html! {
            <div class={container_classes}>
                {children.clone()}
            </div>
        }
    }
}
