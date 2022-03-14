use glib::{ParamFlags, ParamSpec, ParamSpecString, ParamSpecInt, ParamSpecBoolean, ParamSpecBoxed, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::fs::DirEntry;
use std::cell::{Cell, RefCell};

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Boxed;
use glib::Type;
use std::default::Default;
use std::time::SystemTime;


#[derive(Clone)]
pub enum EntryType {
    FILE,
    DIRECTORY,
    SYMLINK,
    UNKNOWN,
}


#[derive(Default, Clone, Boxed)]
#[boxed_type(name = "FsEntry")]
pub struct FsEntry {
    name: String,
    path: String,
    last_modified: Option<SystemTime>,
    size: u64,
    entry_type: Option<EntryType>,
}


/*
#[derive(Default)]
pub struct FsItem {
    entry: Cell<DirEntry>,
    selected: bool,
}
*/

#[derive(Default)]
pub struct ModelItem {
    //entry: RefCell<DirEntry>,
    entry: RefCell<FsEntry>,
    selected: bool,
}


// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ModelItem {
    const NAME: &'static str = "MyGtkAppFsItem";
    type Type = super::ModelItem;
}


// Trait shared by all GObjects
impl ObjectImpl for ModelItem {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecBoxed::new(
                    "entry", // Name
                    "entry", // Nickname
                    "entry", // Short description
                    Type::BOXED, // Default value
                    ParamFlags::READWRITE, // The property can be read and written to
                ),
                ParamSpecBoolean::new(
                    "selected", // Name
                    "selected", // Nickname
                    "selected", // Short description
                    false, // Default value
                    ParamFlags::READWRITE, // The property can be read and written to
                )
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "entry" => {
                let entry = value.get().expect("The value needs to be of type `FsEntry`.");
                self.entry.replace(entry);
            },
            "selected" => {
                let selected = value.get().expect("The value needs to be of type `bool`.");
                self.selected = selected;
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "entry" => self.entry.borrow().to_value(),
            "selected" => self.selected.to_value(),
            _ => unimplemented!(),
        }
    }
}

