pub mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::glib::Object;

use std::fs::DirEntry;

use glib::Boxed;
use glib::Type;

use std::time::SystemTime;

/*
pub enum EntryType {
    FILE,
    DIRECTORY,
    SYMLINK,
    UNKNOWN,
}

#[derive(Default)]
pub struct FsEntry {
    name: String,
    path: String,
    last_modified: Option<SystemTime>,
    size: u64,
    entry_type: Option<EntryType>,
}
*/


glib::wrapper! {
    pub struct ModelItem(ObjectSubclass<imp::ModelItem>);
}


impl ModelItem {
    pub fn new(entry: &imp::FsEntry, selected: bool) -> Self {
        Object::new(&[("entry", &entry), ("selected", &selected)]).expect("Failed to create `ModelItem`.")
    }
}

