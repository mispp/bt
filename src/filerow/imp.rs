use glib::{ParamFlags, ParamSpec, ParamSpecString, ParamSpecInt, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::cell::{Cell, RefCell};


#[derive(Default)]
pub struct FsItem {
    name: RefCell<String>,
    last_modified: RefCell<String>,
}


// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for FsItem {
    const NAME: &'static str = "MyGtkAppFsItem";
    type Type = super::FsItem;
}


// Trait shared by all GObjects
impl ObjectImpl for FsItem {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new(
                    "name", // Name
                    "name", // Nickname
                    "name", // Short description
                    None, // Default value
                    ParamFlags::READWRITE, // The property can be read and written to
                ),
                ParamSpecString::new(
                    "lastmodified", // Name
                    "lastmodified", // Nickname
                    "lastmodified", // Short description
                    None, // Default value
                    ParamFlags::READWRITE, // The property can be read and written to
                )
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name = value.get().expect("The value needs to be of type `String`.");
                self.name.replace(name);
            },
            "lastmodified" => {
                let last_modified = value.get().expect("The value needs to be of type `String`.");
                self.last_modified.replace(last_modified);
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "lastmodified" => self.last_modified.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}

