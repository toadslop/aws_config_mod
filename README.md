# aws_config_mod

## aws_config_mod

A missing piece of the Rust AWS SDK. A current missing feature of the SDK is the ability
to read and modify AWS config files programatically, such as what we can do with the set command:
<https://docs.aws.amazon.com/cli/latest/reference/configure/set.html>

The goal of this library is allowing you to add, modify, and remove AWS CLI configuration settings for
existing configuration files to to generate new ones while any existing whitespace or comments
in the original file.

This crate is still a work-in-progress, so if there are any features that you need right away, please
open an issue and they will be prioritized.

### Usage

```rust
// Assuming you already read the configuration file to a string
let config_content = r#"
[profile A]
credential_source = Ec2InstanceMetadata
endpoint_url = https://profile-a-endpoint.aws/

[profile B]
source_profile = A
role_arn = arn:aws:iam::123456789012:role/roleB
services = profileB

[services profileB]
ec2 =
  endpoint_url = https://profile-b-ec2-endpoint.aws"#;

let mut config = AwsConfigFile::parse(SAMPLE_FILE).expect("Sample file should be valid");

let setting_path = SettingPath::try_from("profile.A.credential_source").expect("Should parse");
config.set(setting_path, Value::from("my-new-credential-source"));
let stringified = config.to_string();
// Write the content back to your file
```

### TODOs

- improved error messages
- automatic config file loading via standard aws config locations and environment variables
- detect and match formatting
- set formatting
- utilize aws types

License: MIT OR Apache-2.0
