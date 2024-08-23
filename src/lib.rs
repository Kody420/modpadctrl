use error::ModpadApiError;
use hidapi::{HidApi, HidDevice};
use command::*;

pub mod error;
pub mod command;

pub struct ModpadApi {
    //hidapi_ctx: HidApi,
    modpad_device: HidDevice
}

impl ModpadApi {
    pub fn new() -> Result<Self, ModpadApiError> {
        const VID: u16 = 0x03eb;
        const PID: u16 = 0x2066;
        const USAGE_PAGE: u16 = 0xff;

        let hidapi_ctx = HidApi::new()?;
        let modpad_device_info_opt = hidapi_ctx.device_list().find(|device| {
            device.vendor_id() == VID &&
            device.product_id() == PID &&
            device.usage_page() == USAGE_PAGE
        });

        let modpad_device_path = match modpad_device_info_opt {
            Some(modpad_device_info) => modpad_device_info.path(),
            None => return Err(ModpadApiError::ModpadNotFound)
        };

        let modpad_device = hidapi_ctx.open_path(modpad_device_path)?;

        Ok(Self {
            //hidapi_ctx,
            modpad_device
        })
    }

    fn send_command(&self, command: impl ModpadReport) -> Result<(), ModpadApiError> {
        let modpad_command_report = command.build_report();
        let mut buffer = [0; 8];

        buffer[0] = modpad_command_report.report_id;
        buffer[2] = (modpad_command_report.command_type >> 8) as u8;
        buffer[1] = (modpad_command_report.command_type & 0xff) as u8;
        buffer[4] = (modpad_command_report.value >> 8) as u8;
        buffer[3] = (modpad_command_report.value & 0xff) as u8;
        buffer[5] = modpad_command_report.profile;
        buffer[6] = modpad_command_report.row;
        buffer[7] = modpad_command_report.column;

        self.modpad_device.send_feature_report(&buffer)?;

        Ok(())
    }

    pub fn set_effect(&self, effect: Effect) -> Result<(), ModpadApiError> {
        self.send_command(effect)?;
        Ok(())
    }

    pub fn change_brightness(&self, brightness_dir: Brightness) -> Result<(), ModpadApiError> {
        self.send_command(brightness_dir)?;
        Ok(())
    }

    pub fn switch_profile(&self, profile_number: u8) -> Result<(), ModpadApiError> {
        self.send_command(Profile::new(profile_number)?)?;
        Ok(())
    }

    pub fn remap(&self, key_code: u16, profile_number: u8, row: u8, column: u8) -> Result<(), ModpadApiError> {
        self.send_command(Mapping::new(key_code, profile_number, row, column)?)?;
        Ok(())
    }
}

trait ModpadReport {
    fn build_report(&self) -> ModpadCommandReport;
}

struct ModpadCommandReport {
    pub report_id: u8,
    pub command_type: u16,
    pub value: u16,
    pub profile: u8,
    pub row: u8,
    pub column: u8
}
