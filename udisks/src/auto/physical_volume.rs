// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "UDisksPhysicalVolume")]
    pub struct PhysicalVolume(Interface<ffi::UDisksPhysicalVolume, ffi::UDisksPhysicalVolumeIface>);

    match fn {
        type_ => || ffi::udisks_physical_volume_get_type(),
    }
}

impl PhysicalVolume {
        pub const NONE: Option<&'static PhysicalVolume> = None;
    

    //#[doc(alias = "udisks_physical_volume_interface_info")]
    //pub fn interface_info() -> /*Ignored*/Option<gio::DBusInterfaceInfo> {
    //    unsafe { TODO: call ffi:udisks_physical_volume_interface_info() }
    //}

    //#[doc(alias = "udisks_physical_volume_override_properties")]
    //pub fn override_properties(klass: /*Ignored*/&mut glib::ObjectClass, property_id_begin: u32) -> u32 {
    //    unsafe { TODO: call ffi:udisks_physical_volume_override_properties() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PhysicalVolume>> Sealed for T {}
}

pub trait PhysicalVolumeExt: IsA<PhysicalVolume> + sealed::Sealed + 'static {
    #[doc(alias = "udisks_physical_volume_dup_volume_group")]
    fn dup_volume_group(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::udisks_physical_volume_dup_volume_group(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "udisks_physical_volume_get_free_size")]
    #[doc(alias = "get_free_size")]
    fn free_size(&self) -> u64 {
        unsafe {
            ffi::udisks_physical_volume_get_free_size(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "udisks_physical_volume_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> u64 {
        unsafe {
            ffi::udisks_physical_volume_get_size(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "udisks_physical_volume_get_volume_group")]
    #[doc(alias = "get_volume_group")]
    fn volume_group(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::udisks_physical_volume_get_volume_group(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "udisks_physical_volume_set_free_size")]
    fn set_free_size(&self, value: u64) {
        unsafe {
            ffi::udisks_physical_volume_set_free_size(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "udisks_physical_volume_set_size")]
    fn set_size(&self, value: u64) {
        unsafe {
            ffi::udisks_physical_volume_set_size(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "udisks_physical_volume_set_volume_group")]
    fn set_volume_group(&self, value: &str) {
        unsafe {
            ffi::udisks_physical_volume_set_volume_group(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "free-size")]
    fn connect_free_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_free_size_trampoline<P: IsA<PhysicalVolume>, F: Fn(&P) + 'static>(this: *mut ffi::UDisksPhysicalVolume, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PhysicalVolume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::free-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_free_size_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P: IsA<PhysicalVolume>, F: Fn(&P) + 'static>(this: *mut ffi::UDisksPhysicalVolume, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PhysicalVolume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_size_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "volume-group")]
    fn connect_volume_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_group_trampoline<P: IsA<PhysicalVolume>, F: Fn(&P) + 'static>(this: *mut ffi::UDisksPhysicalVolume, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PhysicalVolume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::volume-group\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_volume_group_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<PhysicalVolume>> PhysicalVolumeExt for O {}
