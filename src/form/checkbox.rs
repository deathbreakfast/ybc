use yew::prelude::*;

pub enum CheckboxMsg {
    Checked(bool),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CheckboxProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub checked: bool,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// The 2-state checkbox in its native format.
///
/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Checkbox;

impl Component for Checkbox {
    type Message = CheckboxMsg;
    type Properties = CheckboxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CheckboxMsg::Checked(checked) => {
                ctx.props().update.emit(checked);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CheckboxProps { children, classes, checked, disabled, name, .. } = ctx.props();
        let mut checkbox_classes = Classes::from("checkbox");
        checkbox_classes.push(classes);
        let checked_value = *checked;
        html! {
            <label class={checkbox_classes} disabled={*disabled}>
                <input
                    type="checkbox"
                    checked={*checked}
                    name={name.clone()}
                    onclick={ctx.link().callback(move |_| CheckboxMsg::Checked(!checked_value))}
                    disabled={*disabled}
                    />
                {children.clone()}
            </label>
        }
    }
}
