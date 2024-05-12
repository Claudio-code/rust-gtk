mod imp;

use std::fs::File;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{ActionRow, AlertDialog, ResponseAppearance};
use gio::Settings;
use glib::{clone, Object};
use gtk::{
    gio, glib, pango, Align, CheckButton, CustomFilter, Entry, FilterListModel, Label,
    ListBoxRow, NoSelection,
};

use crate::collection_object::{CollectionData, CollectionObject};
use crate::task_object::TaskObject;
use crate::utils::data_path;
use crate::APP_ID;
