use crossbeam::channel::Sender;

use crate::{
    features::{component_id::YAML_KIND_DIALOG_ID, yaml::message::YamlRequest},
    logger,
    message::Message,
    ui::{
        event::EventResult,
        widget::{LiteralItem, SingleSelect, Widget, WidgetBase},
        Window,
    },
};

pub fn kind_dialog(tx: &Sender<Message>) -> Widget<'static> {
    let tx = tx.clone();

    SingleSelect::builder()
        .id(YAML_KIND_DIALOG_ID)
        .widget_base(WidgetBase::builder().title("Kind").build())
        .on_select(on_select(tx))
        .build()
        .into()
}

fn on_select(tx: Sender<Message>) -> impl Fn(&mut Window, &LiteralItem) -> EventResult {
    move |w, v| {
        logger!(info, "Select Item: {:?}", v);

        w.close_dialog();

        let Some(metadata) = v.metadata.as_ref() else {
            unreachable!()
        };

        let Some(key) = metadata.get("key") else {
            unreachable!()
        };

        let Ok(kind) = serde_json::from_str(key) else {
            unreachable!()
        };

        tx.send(YamlRequest::Resource(kind).into())
            .expect("Failed to send YamlRequest::Resource");

        EventResult::Nop
    }
}
