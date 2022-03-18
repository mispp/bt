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
use filerow::ModelItem;
use filerow::imp::FsEntry;
use filerow::imp::EntryType;
//use filerow::DirEntryBoxed;



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

    left_side.populate_for_path(&String::from("/"));
    right_side.populate_for_path(&String::from("/var"));

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
        let model = gio::ListStore::new(ModelItem::static_type());
        let widget_factory = gtk::SignalListItemFactory::new();

        widget_factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            list_item.set_child(Some(&label));

            // Bind `list_item->item->number` to `label->label`
            list_item
                .property_expression("item")
                .chain_property::<ModelItem>("name")
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
        let md = path.metadata().unwrap();

        let filetype =
            if md.is_file() {
                EntryType::FILE
            }
            else if md.is_dir() {
                EntryType::DIRECTORY
            }
            else if md.is_symlink() {
                EntryType::SYMLINK
            }
            else {
                EntryType::UNKNOWN
            };

        let name = match path.file_name().to_str() {
            Some(name) => name.to_string(),
            None => String::from("n/a"),
        };

        let path = match path.path().to_str() {
            Some(path) => path.to_string(),
            None => String::from("n/a"),
        };

        /*
        let fse = FsEntry {
            name: name,
            path: path,
            last_modified: Some(md.modified().unwrap()),
            size: md.len(),
            entry_type: Some(filetype),
        };
        */

        let fse = FsEntry::new(name, path, md.modified().unwrap(), md.len(), filetype);

        let fsi = ModelItem::new(&fse, false);

        self.model.append(&fsi);
    }

    fn populate_for_path(&self, path: &String) {
        let _ = self.clear();

        for path in fs::read_dir(path).unwrap() {
            self.add(path.unwrap());
        }
    }
}

