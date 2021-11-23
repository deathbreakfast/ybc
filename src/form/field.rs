use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// A text label for the field.
    #[prop_or_default]
    pub label: Option<String>,
    /// Extra classes for the label container.
    #[prop_or_default]
    pub label_classes: Option<Classes>,
    /// A help message for the field.
    #[prop_or_default]
    pub help: Option<String>,
    /// Extra classes for the help message container.
    #[prop_or_default]
    pub help_classes: Option<Classes>,
    /// A convenience bool to add the `is-danger` class to the help classes when `true`.
    #[prop_or_default]
    pub help_has_error: bool,
    /// Has icons on the left of the field's controls.
    #[prop_or_default]
    pub icons_left: bool,
    /// Has icons on the right of the field's controls.
    #[prop_or_default]
    pub icons_right: bool,
    /// Allow addons to the field's controls.
    #[prop_or_default]
    pub addons: bool,
    /// Alignment for the field's addons.
    #[prop_or_default]
    pub addons_align: Option<AddonsAlign>,
    /// All controls in this field should be grouped.
    #[prop_or_default]
    pub grouped: bool,
    /// Alignment for grouped controls.
    #[prop_or_default]
    pub grouped_align: Option<GroupedAlign>,
    /// Allow the grouped controls to span multiple lines.
    #[prop_or_default]
    pub multiline: bool,
    /// Make this a horizontal field.
    #[prop_or_default]
    pub horizontal: bool,
}

/// A container for form controls
pub struct Field;

impl Component for Field {
    type Message = ();
    type Properties = FieldProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let FieldProps {
            children,
            classes,
            horizontal,
            addons_align,
            label_classes,
            help_has_error,
            icons_left,
            help,
            help_classes,
            icons_right,
            multiline,
            label,
            addons,
            grouped,
            grouped_align,
        } = ctx.props();
        let mut field_classes = Classes::from("field");
        field_classes.push(classes);
        if *icons_left {
            field_classes.push("has-icons-left");
        }
        if *icons_right {
            field_classes.push("has-icons-right");
        }
        if *addons {
            field_classes.push("has-addons");
        }
        if let Some(align) = &addons_align {
            field_classes.push(&align.to_string());
        }
        if *grouped {
            field_classes.push("is-grouped");
        }
        if let Some(align) = &grouped_align {
            field_classes.push(&align.to_string());
        }
        if *multiline {
            field_classes.push("is-grouped-multiline");
        }

        // Build the label if label content is provided.
        let label = match &label {
            Some(label_content) => match &label_classes {
                Some(label_classes_str) => {
                    let mut label_classes = label_classes_str.clone();
                    if *horizontal {
                        label_classes.push("field-label");
                        html! {
                            <div class={label_classes}>
                                <label class="label">{label_content.clone()}</label>
                            </div>
                        }
                    } else {
                        label_classes.push("label");
                        html! {<label class={label_classes}>{label_content.clone()}</label>}
                    }
                }
                None => {
                    if *horizontal {
                        html! {<div class="field-label"><label class="label">{label_content.clone()}</label></div>}
                    } else {
                        html! {<label class="label">{label_content.clone()}</label>}
                    }
                }
            },
            None => html! {},
        };

        // Build the help label if present.
        let help = match &help {
            Some(help_content) => match &help_classes {
                Some(help_classes_str) => {
                    let mut help_classes = help_classes_str.clone();
                    help_classes.push("help");
                    if *help_has_error {
                        help_classes.push("is-danger");
                    }
                    html! {<label class={help_classes}>{help_content.clone()}</label>}
                }
                None => {
                    let mut help_classes = Classes::from("help");
                    if *help_has_error {
                        help_classes.push("is-danger");
                    }
                    html! {<label class={help_classes}>{help_content.clone()}</label>}
                }
            },
            None => html! {},
        };

        // Build the body section.
        let mut body = html! {<>{children.clone()}</>};
        if *horizontal {
            body = html! {<div class={"field-body"}>{body}</div>}
        }

        html! {
            <div class={field_classes}>
                {label}
                {body}
                {help}
            </div>
        }
    }
}

/// The two alignment options available for field addons.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "has-addons-{}")]
pub enum AddonsAlign {
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

/// The two alignment options available for grouped field controls.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-grouped-{}")]
pub enum GroupedAlign {
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

/// The three sizes available for horizontal field labels.
///
/// https://bulma.io/documentation/form/general/#horizontal-form
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum LabelSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}
