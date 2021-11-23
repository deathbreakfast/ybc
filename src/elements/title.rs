#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// Maintain the normal spacing between titles and subtitles.
    #[prop_or_default]
    pub is_spaced: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
pub struct Title;

impl Component for Title {
    type Message = ();
    type Properties = TitleProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let TitleProps { children, classes, size, is_spaced, tag } = ctx.props();
        let mut title_classes = Classes::from("title");
        title_classes.push(classes);
        if let Some(size) = &size {
            title_classes.push(&size.to_string());
        }
        if *is_spaced {
            title_classes.push("is-spaced");
        }
        let tag = tag.clone();
        html! {
            <@{tag} class={title_classes}>
                {children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
pub struct Subtitle;

impl Component for Subtitle {
    type Message = ();
    type Properties = SubtitleProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SubtitleProps { classes, children, size, tag } = ctx.props();
        let mut sub_title_classes = Classes::from("subtitle");
        sub_title_classes.push(classes);
        if let Some(size) = &size {
            sub_title_classes.push(&size.to_string());
        }
        let tag = tag.clone();
        html! {
            <@{tag} class={sub_title_classes}>
                {children.clone()}
            </@>
        }
    }
}

/// The six sizes available for titles & subtitles.
///
/// https://bulma.io/documentation/elements/title/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum HeaderSize {
    #[display(fmt = "1")]
    Is1,
    #[display(fmt = "2")]
    Is2,
    #[display(fmt = "3")]
    Is3,
    #[display(fmt = "4")]
    Is4,
    #[display(fmt = "5")]
    Is5,
    #[display(fmt = "6")]
    Is6,
}
