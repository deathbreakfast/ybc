use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroProps {
    /// Extra classes for the hero container.
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The contents of the hero-head section.
    #[prop_or_default]
    pub head: Option<Html>,
    /// Optional classes to add to the hero-head container.
    #[prop_or_default]
    pub head_classes: Option<Classes>,
    /// The contents of the hero-body section.
    pub body: Html,
    /// Optional classes to add to the hero-body container.
    #[prop_or_default]
    pub body_classes: Option<Classes>,
    /// The contents of the hero-foot section.
    #[prop_or_default]
    pub foot: Option<Html>,
    /// Optional classes to add to the hero-foot container.
    #[prop_or_default]
    pub foot_classes: Option<Classes>,
    /// If you are using a [fixed navbar](https://bulma.io/documentation/components/navbar/#fixed-navbar),
    /// you can use the `fixed_nav=true` modifier on the hero for it to occupy the viewport height minus
    /// the navbar height.
    ///
    /// https://bulma.io/documentation/layout/hero/#fullheight-with-navbar
    #[prop_or_default]
    pub fixed_nav: bool,
    /// Generate a subtle gradient for the hero.
    #[prop_or_default]
    pub bold: bool,
    /// The size for this hero.
    #[prop_or_default]
    pub size: Option<HeroSize>,
}

/// An imposing hero banner to showcase something.
///
/// [https://bulma.io/documentation/layout/hero/](https://bulma.io/documentation/layout/hero/)
pub struct Hero;

impl Component for Hero {
    type Message = ();
    type Properties = HeroProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let HeroProps {
            classes,
            bold,
            body,
            body_classes,
            fixed_nav,
            foot,
            foot_classes,
            head,
            head_classes,
            size,
        } = ctx.props();
        let mut hero_classes = Classes::from("hero");
        hero_classes.push(classes);
        if *fixed_nav {
            hero_classes.push("is-fullheight-with-navbar");
        }
        if *bold {
            hero_classes.push("is-bold");
        }
        if let Some(size) = &size {
            hero_classes.push(&size.to_string());
        }

        // Build the header section.
        let head = if let Some(head) = &head {
            let mut classes = Classes::from("hero-head");
            classes.push(head_classes);
            html! {<div class={classes}>{head.clone()}</div>}
        } else {
            html! {}
        };
        // Build the footer section.
        let foot = if let Some(foot) = &foot {
            let mut classes = Classes::from("hero-foot");
            classes.push(foot_classes);
            html! {<div class={classes}>{foot.clone()}</div>}
        } else {
            html! {}
        };

        let mut bclasses = Classes::from("hero-body");
        bclasses.push(body_classes);
        html! {
            <section class={hero_classes}>
                {head}
                <div class={bclasses}>{body.clone()}</div>
                {foot}
            </section>
        }
    }
}

/// The 4 sizes available for heros.
///
/// [https://bulma.io/documentation/layout/hero/#sizes](https://bulma.io/documentation/layout/hero/#sizes)
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum HeroSize {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
    #[display(fmt = "fullheight")]
    Fullheight,
    #[display(fmt = "fullheight-with-navbar")]
    FullheightWithNavbar,
}
