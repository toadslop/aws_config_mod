//! Collection of structs that represent the various structures of an aws config file.

mod config_file;
mod credentials_file;
mod equal;
mod header;
mod indent;
mod nested_setting;
mod nested_settings;
mod section;
mod section_name;
mod section_path;
mod section_type;
mod setting;
mod setting_name;
mod setting_path;
mod value;
mod value_type;
mod whitespace;

pub use config_file::AwsConfigFile;
pub use credentials_file::AwsCredentialsFile;
pub use nested_setting::NestedSetting;
pub use section::Section;
pub use section_name::SectionName;
pub use section_path::SectionPath;
pub use section_type::SectionType;
pub use setting::Setting;
pub use setting_name::SettingName;
pub use setting_path::{NestedSettingPath, SettingPath};
pub use value::Value;
pub use value_type::ValueType;
