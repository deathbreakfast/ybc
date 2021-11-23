use derive_more::Display;
use yew::prelude::*;

use crate::Alignment;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<BreadcrumbSize>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The separator type to use between breadcrumb segments.
    #[prop_or_default]
    pub separator: Option<BreadcrumbSeparator>,
}

/// A simple breadcrumb component to improve your navigation experience.
///
/// [https://bulma.io/documentation/components/breadcrumb/](https://bulma.io/documentation/components/breadcrumb/)
pub struct Breadcrumb;

impl Component for Breadcrumb {
    type Message = ();
    type Properties = BreadcrumbProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let BreadcrumbProps { alignment, children, classes, separator, size } = ctx.props();
        let mut breadcrumb_classes = Classes::from("breadcrumb");
        breadcrumb_classes.push(classes);
        if let Some(size) = &size {
            breadcrumb_classes.push(&size.to_string());
        }
        if let Some(alignment) = &alignment {
            breadcrumb_classes.push(&alignment.to_string());
        }
        if let Some(separator) = &separator {
            breadcrumb_classes.push(&separator.to_string());
        }
        html! {
            <nav class={breadcrumb_classes} aria-label="breadcrumbs">
                <ul>
                    {children.clone()}
                </ul>
            </nav>
        }
    }
}

/// The 3 sizes available for a breadcrumb.
///
/// https://bulma.io/documentation/components/breadcrumb/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "are-{}")]
pub enum BreadcrumbSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

/// The 4 additional separators for a breadcrump.
///
/// https://bulma.io/documentation/components/breadcrumb/#alternative-separators
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "has-{}-separator")]
pub enum BreadcrumbSeparator {
    #[display(fmt = "arrow")]
    Arrow,
    #[display(fmt = "bullet")]
    Bullet,
    #[display(fmt = "dot")]
    Dot,
    #[display(fmt = "succeeds")]
    Succeeds,
}
