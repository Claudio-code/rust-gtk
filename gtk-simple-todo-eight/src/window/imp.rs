use std::cell::RefCell;
use std::fs::File;

use adw::subclass::prelude::*;
use adw::{prelude::*, NavigationSplitView};
use gio::Settings;
use glib::subclass::InitializingObject;
use gtk::glib::SignalHandlerId;
use gtk::{gio, glib, CompositeTemplate, Entry, FilterListModel, ListBox, Stack};
use std::cell::OnceCell;



