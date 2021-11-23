use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Align child columns vertically.
    #[prop_or_default]
    pub vcentered: bool,
    /// Allow for multiline rows.
    #[prop_or_default]
    pub multiline: bool,
    /// Center all child columns within their row.
    #[prop_or_default]
    pub centered: bool,
}

/// The container for a set of responsive columns.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
pub struct Columns;

impl Component for Columns {
    type Message = ();
    type Properties = ColumnsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ColumnsProps { centered, children, classes, multiline, vcentered } = ctx.props();
        let mut columns_classes = Classes::from("columns");
        columns_classes.push(classes);
        if *vcentered {
            columns_classes.push("is-vcentered");
        }
        if *multiline {
            columns_classes.push("is-multiline");
        }
        if *centered {
            columns_classes.push("is-centered");
        }
        html! {
            <div class={columns_classes}>
                {children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A flexbox-based responsive column.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
///
/// This component has a very large number of valid class combinations which users may want.
/// Modelling all of these is particularly for this component, so for now you are encouraged to
/// add classes to this Component manually via the `classes` prop.
pub struct Column;

impl Component for Column {
    type Message = ();
    type Properties = ColumnProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ColumnProps { children, classes } = ctx.props();
        let mut column_classes = Classes::from("column");
        column_classes.push(classes);
        html! {
            <div class={column_classes}>
                {children.clone()}
            </div>
        }
    }
}
