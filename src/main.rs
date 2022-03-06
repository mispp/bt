#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(dead_code)]


//mod integer_object;
mod filerow;

use gtk::prelude::*;
use gtk::gio;
use gtk::{Application, ApplicationWindow, Paned, Label, Orientation, ScrolledWindow, ListView, ColumnView, ColumnViewColumn, SingleSelection, ListBox, Widget};

//use integer_object::IntegerObject;
use filerow::FsItem;

fn main() {
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}


fn build_ui(app: &Application) {
    let left_side = Side::new();
    let right_side = Side::new();

    let _ = left_side.clear();

    let paned = Paned::builder()
        .start_child(&left_side.widget())
        .end_child(&right_side.widget())
        .orientation(Orientation::Horizontal)
        .build();


    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&paned)
        .build();

    // Present window
    window.present();
}



#[derive(Debug)]
pub struct Side {
    model: gio::ListStore,
    widget_factory: gtk::SignalListItemFactory,
    widget: ScrolledWindow,
}

impl Side {
    fn new() -> Self {
        //let model = gio::ListStore::new(IntegerObject::static_type());
        let model = gio::ListStore::new(FsItem::static_type());
        let widget_factory = gtk::SignalListItemFactory::new();

        widget_factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            list_item.set_child(Some(&label));

            // Bind `list_item->item->number` to `label->label`
            list_item
                .property_expression("item")
                .chain_property::<FsItem>("name")
                .bind(&label, "label", Widget::NONE);
        });

        let selection_model = SingleSelection::new(Some(&model));
        let view = ColumnView::new(Some(&selection_model));
        view.append_column(&ColumnViewColumn::new(Some("title"), Some(&widget_factory)));

        let widget = ScrolledWindow::builder()
            .min_content_width(360)
            .child(&view)
            .build();

        Self {
            model: model,
            widget_factory: widget_factory,
            widget: widget,
        }
    }

    fn widget<'a>(self) -> ScrolledWindow {
        self.widget
    }

    fn clear(&self) {
        &self.model.remove_all();
    }
}

