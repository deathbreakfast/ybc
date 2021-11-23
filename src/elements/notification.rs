use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bold notification blocks, to alert your users of something.
///
/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
pub struct Notification;

impl Component for Notification {
    type Message = ();
    type Properties = NotificationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let NotificationProps { children, classes } = ctx.props();
        let mut notification_classes = Classes::from("notification");
        notification_classes.push(classes);
        html! {
            <div class={notification_classes}> {children.clone()}
            </div>
        }
    }
}
