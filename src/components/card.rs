use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// An all-around flexible and composable component; this is the card container.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct Card;

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CardProps { children, classes } = ctx.props();
        let mut card_classes = Classes::from("card");
        card_classes.push(classes);
        html! {
            <div class={card_classes}>
                {children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for card header content; rendered as a horizontal bar with a shadow.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardHeader;

impl Component for CardHeader {
    type Message = ();
    type Properties = CardHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CardHeaderProps { children, classes } = ctx.props();
        let mut card_classes = Classes::from("card-header");
        card_classes.push(classes);
        html! {
            <header class={card_classes}>
                {children.clone()}
            </header>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A fullwidth container for a responsive image.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardImage;

impl Component for CardImage {
    type Message = ();
    type Properties = CardImageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CardImageProps { children, classes } = ctx.props();
        let mut card_classes = Classes::from("card-image");
        card_classes.push(classes);
        html! {
            <div class={card_classes}>
                {children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for any other content as the body of the card.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardContent;

impl Component for CardContent {
    type Message = ();
    type Properties = CardContentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CardContentProps { children, classes } = ctx.props();
        let mut card_classes = Classes::from("card-content");
        card_classes.push(classes);
        html! {
            <div class={card_classes}>
                {children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for card footer content; rendered as a horizontal list of controls.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardFooter;

impl Component for CardFooter {
    type Message = ();
    type Properties = CardFooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CardFooterProps { children, classes } = ctx.props();
        let mut card_classes = Classes::from("card-footer");
        card_classes.push(classes);
        html! {
            <footer class={card_classes}>
                {children.clone()}
            </footer>
        }
    }
}
