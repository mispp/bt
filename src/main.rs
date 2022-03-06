#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(dead_code)]


//mod integer_object;
mod filerow;

use gtk::prelude::*;
use gtk::gio;
use gtk::{Application, ApplicationWindow, Paned, Label, Orientation, ScrolledWindow, ListView, ColumnView, ColumnViewColumn, SingleSelection, ListBox, Widget};
use gtk::builders::ColumnViewColumnBuilder;

use std::fs;
use std::fs::DirEntry;

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

    for path in fs::read_dir("/").unwrap() {
        left_side.add(path.unwrap());
    }

    //let _ = left_side.clear();

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
        
        let columnviewcolumn_name = ColumnViewColumnBuilder::new()
            .expand(true)
            .title("Name")
            .resizable(true)
            .factory(&widget_factory)
            .build();

        let columnviewcolumn_last_modified = ColumnViewColumnBuilder::new()
            .expand(false)
            .title("Last Modified")
            .resizable(true)
            .factory(&widget_factory)
            .build();

        view.append_column(&columnviewcolumn_name);
        view.append_column(&columnviewcolumn_last_modified);

        let widget = ScrolledWindow::builder()
            .min_content_width(360)
            .child(&view)
            .build();

        Self {
            model: model,
            widget: widget,
        }
    }

    fn widget<'a>(self) -> ScrolledWindow {
        self.widget
    }

    fn clear(&self) {
        &self.model.remove_all();
    }
    
    fn add(&self, path: DirEntry) {
        let path_string = path.file_name().into_string().unwrap();
        let fsitem = FsItem::new(path_string, String::from("last_modified test"));
        self.model.append(&fsitem);
    }
}

