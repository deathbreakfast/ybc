use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add borders to all the cells.
    #[prop_or_default]
    pub bordered: bool,
    /// Add stripes to the table.
    #[prop_or_default]
    pub striped: bool,
    /// Make the cells narrower.
    #[prop_or_default]
    pub narrow: bool,
    /// Add a hover effect on each row.
    #[prop_or_default]
    pub hoverable: bool,
    /// Make the table fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    #[prop_or_default]
    pub scrollable: bool,
}

/// An HTML table component.
///
/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
pub struct Table;

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let TableProps {
            children,
            classes,
            bordered,
            narrow,
            hoverable,
            fullwidth,
            scrollable,
            striped,
        } = ctx.props();
        let mut table_classes = Classes::from("table");
        table_classes.push(classes);
        if *bordered {
            table_classes.push("is-bordered");
        }
        if *striped {
            table_classes.push("is-striped");
        }
        if *narrow {
            table_classes.push("is-narrow");
        }
        if *hoverable {
            table_classes.push("is-hoverable");
        }
        if *fullwidth {
            table_classes.push("is-fullwidth");
        }
        if *scrollable {
            html! {
                <div class="table-container">
                    <table class={table_classes}>
                        {children.clone()}
                    </table>
                </div>
            }
        } else {
            html! {
                <table class={table_classes}>
                    {children.clone()}
                </table>
            }
        }
    }
}
