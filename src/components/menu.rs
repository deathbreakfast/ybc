use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A simple menu, for any type of vertical navigation.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
pub struct Menu;

impl Component for Menu {
    type Message = ();
    type Properties = MenuProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let MenuProps { children, classes } = ctx.props();
        let mut menu_classes = Classes::from("menu");
        menu_classes.push(classes);
        html! {
            <aside class={menu_classes}>
                {children.clone()}
            </aside>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuListProps {
    /// The child `li` elements of this list.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for menu list `li` elements.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
pub struct MenuList;

impl Component for MenuList {
    type Message = ();
    type Properties = MenuListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let MenuListProps { children, classes } = ctx.props();
        let mut menu_classes = Classes::from("menu-list");
        menu_classes.push(classes);
        html! {
            <ul class={menu_classes}>
                {children.clone()}
            </ul>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuLabelProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The text of the label.
    #[prop_or_default]
    pub text: String,
}

/// A label for a section of the menu.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
pub struct MenuLabel;

impl Component for MenuLabel {
    type Message = ();
    type Properties = MenuLabelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let MenuLabelProps { classes, text } = ctx.props();
        let mut menu_classes = Classes::from("menu-label");
        menu_classes.push(classes);
        html! {
            <p class={menu_classes}>
                {text.clone()}
            </p>
        }
    }
}
