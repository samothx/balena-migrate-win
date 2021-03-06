use std::cell::RefCell;
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;

use super::{DeviceProps, HarddiskVolumeInfo};
use crate::mswin::win_api::query_dos_device;
use crate::MigError;

#[derive(Debug)]
pub struct VolumeInfo {
    dev_name: String,
    uuid: String,
    device: String,    
}

impl<'a> VolumeInfo {
    pub fn try_from_device(device: &str) -> Result<Option<VolumeInfo>, MigError> {
        lazy_static! {
            static ref RE_DL: Regex = Regex::new(r"^Volume\{([0-9a-z\-]+)\}$").unwrap();
        }
        if let Some(cap) = RE_DL.captures(device) {
            Ok(Some(VolumeInfo::new(device, cap.get(1).unwrap().as_str())?))
        } else {
            Ok(None)
        }
    }

    fn new(device: &str, uuid: &str) -> Result<VolumeInfo, MigError> {
        Ok(VolumeInfo {
            dev_name: String::from(device),
            uuid: String::from(uuid),
            device: query_dos_device(Some(&device))?.get(0).unwrap().clone(),
        })
    }

    pub fn get_dev_name(&'a self) -> &'a str {
        &self.dev_name
    }

    pub fn get_uuid(&'a self) -> &'a str {
        &self.uuid
    }

    pub fn get_device(&'a self) -> &'a str {
        &self.device
    }
}

impl DeviceProps for VolumeInfo {
    fn get_device_name<'a>(&'a self) -> &'a str {
        &self.dev_name
    }

    fn get_device<'a>(&'a self) -> &'a str {
        &self.device
    }
}
