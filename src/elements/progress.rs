#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made.
    #[prop_or_else(|| 0.0)]
    pub value: f32,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
pub struct Progress;

impl Component for Progress {
    type Message = ();
    type Properties = ProgressProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ProgressProps { classes, max, value } = ctx.props();
        let mut progress_classes = Classes::from("progress");
        progress_classes.push(classes);
        let max = max.to_string();
        let value = value.to_string();
        let value_txt = html! {{format!("{}%", value)}};
        html! {
            <progress class={progress_classes} max={max} value={value}>
                {value_txt}
            </progress>
        }
    }
}
