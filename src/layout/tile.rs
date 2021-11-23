#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TileProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// The context modifier to use for this tile element, else none.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub tile_ctx: Option<TileCtx>,
    /// Stack tiles vertically.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub vertical: bool,
    /// The size to assign to this tile element.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// A single tile element to build 2-dimensional whatever-you-like grids.
///
/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
pub struct Tile;

impl Component for Tile {
    type Message = ();
    type Properties = TileProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let TileProps { children, classes, vertical, size, tag, tile_ctx } = ctx.props();
        let mut tile_classes = Classes::from("tile");
        tile_classes.push(classes);
        if let Some(ctx) = &tile_ctx {
            tile_classes.push(&ctx.to_string());
        }
        if *vertical {
            tile_classes.push("is-vertical");
        }
        if let Some(size) = &size {
            tile_classes.push(&size.to_string());
        }
        html! {
            <@{tag.clone()} class={tile_classes}>
                {children.clone()}
            </@>
        }
    }
}

/// Tile context modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum TileCtx {
    #[display(fmt = "ancestor")]
    Ancestor,
    #[display(fmt = "parent")]
    Parent,
    #[display(fmt = "child")]
    Child,
}

/// Tile size modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum TileSize {
    #[display(fmt = "1")]
    One,
    #[display(fmt = "2")]
    Two,
    #[display(fmt = "3")]
    Three,
    #[display(fmt = "4")]
    Four,
    #[display(fmt = "5")]
    Five,
    #[display(fmt = "6")]
    Six,
    #[display(fmt = "7")]
    Seven,
    #[display(fmt = "8")]
    Eight,
    #[display(fmt = "9")]
    Nine,
    #[display(fmt = "10")]
    Ten,
    #[display(fmt = "11")]
    Eleven,
    #[display(fmt = "12")]
    Twelve,
}
