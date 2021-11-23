use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A simple responsive footer which can include anything.
///
/// [https://bulma.io/documentation/layout/footer/](https://bulma.io/documentation/layout/footer/)
pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = FooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let FooterProps { children, classes } = ctx.props();
        let mut footer_classes = Classes::from("footer");
        footer_classes.push(classes);
        html! {
            <footer class={footer_classes}>
                {children.clone()}
            </footer>
        }
    }
}
