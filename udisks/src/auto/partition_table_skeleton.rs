// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{PartitionTable};
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "UDisksPartitionTableSkeleton")]
    pub struct PartitionTableSkeleton(Object<ffi::UDisksPartitionTableSkeleton, ffi::UDisksPartitionTableSkeletonClass>) @implements PartitionTable;

    match fn {
        type_ => || ffi::udisks_partition_table_skeleton_get_type(),
    }
}

impl PartitionTableSkeleton {
        pub const NONE: Option<&'static PartitionTableSkeleton> = None;
    

    #[doc(alias = "udisks_partition_table_skeleton_new")]
    pub fn new() -> PartitionTableSkeleton {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::udisks_partition_table_skeleton_new())
        }
    }
}

impl Default for PartitionTableSkeleton {
                     fn default() -> Self {
                         Self::new()
                     }
                 }
