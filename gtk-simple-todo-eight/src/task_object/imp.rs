use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::TaskData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::TaskObject)]
pub struct TaskObject {
    #[property(name = "completed", get, set, type = bool, member = completed)]
    #[property(name = "content", get, set, type = String, member = content)]
    pub data: RefCell<TaskData>,
}

#[glib::object_subclass]
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";
    type Type = super::TaskObject;
}

#[glib::derived_properties]
impl ObjectImpl for TaskObject {}

