mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::glib::Object;


glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}


impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::new(&[("number", &number)]).expect("Failed to create `IntegerObject`.")
    }

    pub fn increase_number(self) {
        let old_number = self.property::<i32>("number");
        self.set_property("number", old_number + 1);
    }
}

