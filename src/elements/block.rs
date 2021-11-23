use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bulma’s most basic spacer block
///
/// [https://bulma.io/documentation/elements/block/](https://bulma.io/documentation/elements/block/)
pub struct Block;

impl Component for Block {
    type Message = ();
    type Properties = BlockProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let BlockProps { children, classes } = ctx.props();
        let mut block_classes = Classes::from("block");
        block_classes.push(classes);
        html! {
            <div class={block_classes}>
                {children.clone()}
            </div>
        }
    }
}
