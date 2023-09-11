// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ManagerBTRFS};
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "UDisksManagerBTRFSSkeleton")]
    pub struct ManagerBTRFSSkeleton(Object<ffi::UDisksManagerBTRFSSkeleton, ffi::UDisksManagerBTRFSSkeletonClass>) @implements ManagerBTRFS;

    match fn {
        type_ => || ffi::udisks_manager_btrfs_skeleton_get_type(),
    }
}

impl ManagerBTRFSSkeleton {
        pub const NONE: Option<&'static ManagerBTRFSSkeleton> = None;
    

    #[doc(alias = "udisks_manager_btrfs_skeleton_new")]
    pub fn new() -> ManagerBTRFSSkeleton {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::udisks_manager_btrfs_skeleton_new())
        }
    }
}

#[cfg(feature = "v2_1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_1_3")))]
impl Default for ManagerBTRFSSkeleton {
                     fn default() -> Self {
                         Self::new()
                     }
                 }
