mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::glib::Object;


glib::wrapper! {
    pub struct FsItem(ObjectSubclass<imp::FsItem>);
}


impl FsItem {
    pub fn new(name: String, last_modified: String) -> Self {
        Object::new(&[("name", &name), ("last_modified", &last_modified)]).expect("Failed to create `FsItem`.")
    }
}

