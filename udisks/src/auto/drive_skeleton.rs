// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{Drive};
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "UDisksDriveSkeleton")]
    pub struct DriveSkeleton(Object<ffi::UDisksDriveSkeleton, ffi::UDisksDriveSkeletonClass>) @implements Drive;

    match fn {
        type_ => || ffi::udisks_drive_skeleton_get_type(),
    }
}

impl DriveSkeleton {
        pub const NONE: Option<&'static DriveSkeleton> = None;
    

    #[doc(alias = "udisks_drive_skeleton_new")]
    pub fn new() -> DriveSkeleton {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::udisks_drive_skeleton_new())
        }
    }
}

impl Default for DriveSkeleton {
                     fn default() -> Self {
                         Self::new()
                     }
                 }
