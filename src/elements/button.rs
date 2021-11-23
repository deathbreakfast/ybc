use derive_more::Display;
use yew::events::{Event, FocusEvent, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size for all buttons within this group.
    #[prop_or_default]
    pub size: Option<ButtonGroupSize>,
}

/// A container for a group of buttons.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct Buttons;

impl Component for Buttons {
    type Message = ();
    type Properties = ButtonsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ButtonsProps { children, classes, size } = ctx.props();
        let mut button_classes = Classes::from("buttons");
        button_classes.push(classes);
        if let Some(size) = &size {
            button_classes.push(&size.to_string());
        }
        html! {
            <div class={button_classes}>
                {children.clone()}
            </div>
        }
    }
}

/// The 3 sizes available for a button group.
///
/// https://bulma.io/documentation/elements/button/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "are-{}")]
pub enum ButtonGroupSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A button element.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ButtonProps {
            children,
            classes,
            disabled,
            onclick,
            loading,
            r#static,
        } = ctx.props();
        let mut button_classes = Classes::from("button");
        button_classes.push(classes);
        if *loading {
            button_classes.push("is-loading")
        }
        if *r#static {
            button_classes.push("is-static")
        }
        html! {
            <button class={button_classes} onclick={onclick.clone()} disabled={*disabled}>
                {children.clone()}
            </button>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use yew_router::components::{RouterAnchor, RouterButton as RouterBtn};
    use yew_router::{RouterState, Switch};

    #[derive(Clone, Properties, PartialEq)]
    pub struct ButtonRouterProps<SW: Switch + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: SW,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// Classes to be added to component.
        #[prop_or_default]
        pub classes: Option<Classes>,
        /// Render a loading spinner within this component.
        #[prop_or_default]
        pub loading: bool,
        /// Make this component static.
        #[prop_or_default]
        pub r#static: bool,
        /// Disable this component.
        #[prop_or_default]
        pub disabled: bool,
    }

    /// A Yew Router button element with Bulma styling.
    pub struct ButtonRouter<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState = ()> {
        props: ButtonRouterProps<SW>,
        marker: std::marker::PhantomData<STATE>,
    }

    impl<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState> Component for ButtonRouter<SW, STATE> {
        type Message = ();
        type Properties = ButtonRouterProps<SW>;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self { props, marker: std::marker::PhantomData }
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            self.props.neq_assign(props)
        }

        #[allow(deprecated)]
        fn view(&self) -> Html {
            let mut classes = Classes::from(&self.props.classes);
            if !classes.contains("button") {
                classes.push("button")
            }
            if self.props.loading {
                classes.push("is-loading");
            }
            html! {
                <RouterBtn<SW, STATE>
                    route=self.props.route.clone()
                    disabled=self.props.disabled
                    classes=classes.to_string()
                    children=self.props.children.clone()
                />
            }
        }
    }

    /// A Yew Router anchor button element with Bulma styling.
    pub struct ButtonAnchorRouter<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState = ()> {
        props: ButtonRouterProps<SW>,
        marker: std::marker::PhantomData<STATE>,
    }

    impl<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState> Component for ButtonAnchorRouter<SW, STATE> {
        type Message = ();
        type Properties = ButtonRouterProps<SW>;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self { props, marker: std::marker::PhantomData }
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            self.props.neq_assign(props)
        }

        #[allow(deprecated)]
        fn view(&self) -> Html {
            let mut classes = Classes::from(&self.props.classes);
            if !classes.contains("button") {
                classes.push("button")
            }
            if self.props.loading {
                classes.push("is-loading");
            }
            html! {
                <RouterAnchor<SW, STATE>
                    route=self.props.route.clone()
                    disabled=self.props.disabled
                    classes=classes.to_string()
                    children=self.props.children.clone()
                />
            }
        }
    }
}

#[cfg(feature = "router")]
pub use router::{ButtonAnchorRouter, ButtonRouter, ButtonRouterProps};

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonAnchorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The `href` attribute value to use for this component.
    #[prop_or_default]
    pub href: String,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
}

/// An anchor element styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonAnchor;

impl Component for ButtonAnchor {
    type Message = ();
    type Properties = ButtonAnchorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ButtonAnchorProps {
            children,
            classes,
            disabled,
            href,
            loading,
            onclick,
            target,
            r#static,
            rel,
        } = ctx.props();
        let mut button_classes = Classes::from("button");
        button_classes.push(classes);
        if *loading {
            button_classes.push("is-loading")
        }
        if *r#static {
            button_classes.push("is-static")
        }
        html! {
            <a
                class={button_classes}
                onclick={onclick.clone()}
                href={href.clone()}
                rel={rel.clone().unwrap_or_default()}
                target={target.clone().unwrap_or_default()}
                disabled={*disabled}
            >
                {children.clone()}
            </a>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputSubmitProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The submit handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onsubmit: Callback<FocusEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="submit"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonInputSubmit;

impl Component for ButtonInputSubmit {
    type Message = ();
    type Properties = ButtonInputSubmitProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ButtonInputSubmitProps { classes, disabled, loading, onsubmit, r#static } = ctx.props();
        let mut button_classes = Classes::from("button");
        button_classes.push(classes);
        if *loading {
            button_classes.push("is-loading")
        }
        if *r#static {
            button_classes.push("is-static")
        }
        html! {
            <input type="submit" class={button_classes} onsubmit={onsubmit.clone()} disabled={*disabled} />
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputResetProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The reset handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onreset: Callback<Event>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="reset"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonInputReset;

impl Component for ButtonInputReset {
    type Message = ();
    type Properties = ButtonInputResetProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ButtonInputResetProps { classes, disabled, onreset, loading, r#static } = ctx.props();
        let mut button_classes = Classes::from("button");
        button_classes.push(classes);
        if *loading {
            button_classes.push("is-loading")
        }
        if *r#static {
            button_classes.push("is-static")
        }
        html! {
            <input type="reset" class={button_classes} onreset={onreset.clone()} disabled={*disabled} />
        }
    }
}
