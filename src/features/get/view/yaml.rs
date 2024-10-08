use std::{cell::RefCell, rc::Rc};

use crate::{
    clipboard::Clipboard,
    features::component_id::YAML_DIALOG_ID,
    ui::widget::{Text, Widget, WidgetBase, WidgetTrait},
};

pub struct YamlDialog {
    pub widget: Widget<'static>,
}

impl YamlDialog {
    pub fn new(clipboard: &Option<Rc<RefCell<Clipboard>>>) -> Self {
        Self {
            widget: widget(clipboard),
        }
    }
}

fn widget(clipboard: &Option<Rc<RefCell<Clipboard>>>) -> Widget<'static> {
    let mut builder = Text::builder()
        .id(YAML_DIALOG_ID)
        .widget_base(WidgetBase::builder().title("Yaml").build())
        .block_injection(|text: &Text, is_active: bool, is_mouse_over: bool| {
            let (index, size) = text.state();

            let mut base = text.widget_base().clone();

            *base.title_mut() = format!("Yaml [{}/{}]", index, size).into();

            base.render_block(text.can_activate() && is_active, is_mouse_over)
        })
        .wrap();

    if let Some(clipboard) = clipboard {
        builder = builder.clipboard(clipboard.clone());
    }

    builder.build().into()
}
