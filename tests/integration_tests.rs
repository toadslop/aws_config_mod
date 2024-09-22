use aws_config_mod::{
    AwsConfigFile, AwsCredentialsFile, SectionName, SectionPath, SectionType, SettingName,
    SettingPath, Value, ValueType,
};

const SAMPLE_FILE: &str = r#"
[profile A]
credential_source = Ec2InstanceMetadata
endpoint_url = https://profile-a-endpoint.aws/

[profile B]
source_profile = A
role_arn = arn:aws:iam::123456789012:role/roleB
services = profileB

[services profileB]
ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws
"#;

const SAMPLE_CRED_FILE: &str = r#"
[default]
aws_access_key_id=AKIAIOSFODNN7EXAMPLE
aws_secret_access_key=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
aws_session_token=IQoJb3JpZ2luX2IQoJb3JpZ2luX2IQoJb3JpZ2luX2IQoJb3JpZ2luX2IQoJb3JpZVERYLONGSTRINGEXAMPLE

[other]
aws_access_key_id=AKIAIOSFODNN7EXAMPLE
aws_secret_access_key=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
aws_session_token=IQoJb3JpZ2luX2IQoJb3JpZ2luX2IQoJb3JpZ2luX2IQoJb3JpZ2luX2IQoJb3JpZVERYLONGSTRINGEXAMPLE
"#;

#[test]
fn can_get_a_section_with_a_string() {
    let config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");

    let section_path = SectionPath::try_from("profile.A").expect("Should parse");
    let section = config
        .get_section(&section_path)
        .expect("This section should exist");

    let name = section.get_name().expect("Should have a name");
    assert_eq!(name, "A");
    let mut settings = section.settings().iter();
    let first = settings.next().expect("Should have one entry");
    assert_eq!(first.name(), "credential_source");

    let value = match first.value() {
        ValueType::Single(first) => first,
        ValueType::Nested(_) => panic!("Should not be nested"),
    };

    assert_eq!(value, "Ec2InstanceMetadata");

    let second = settings.next().expect("Should have second entry");
    assert_eq!(second.name(), "endpoint_url");

    let value = match second.value() {
        ValueType::Single(second) => second,
        ValueType::Nested(_) => panic!("Should not be nested"),
    };

    assert_eq!(value, "https://profile-a-endpoint.aws/");
}

#[test]
fn can_get_a_section_with_a_tuple_of_strings() {
    let config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");
    let section_path = SectionPath::try_from(("profile", "A")).expect("Should parse");
    config.get_section(&section_path);
}

#[test]
fn can_get_a_section_with_a_section_type_and_string() {
    let config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");
    let section_path = SectionPath::try_from((SectionType::Profile, "B")).expect("Should parse");
    config
        .get_section(&section_path)
        .expect("Section B should exist");
}

#[test]
fn can_get_a_nested_section() {
    let config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");
    let section_path =
        SectionPath::try_from((SectionType::Services, "profileB")).expect("Should parse");
    let section = config
        .get_section(&section_path)
        .expect("Services for profileB should exist");

    let setting = section
        .settings()
        .first()
        .expect("Should be a first setting");

    assert_eq!(setting.name(), "ec2");

    let settings = match setting.value() {
        ValueType::Single(_) => panic!("Should be nested"),
        ValueType::Nested(nested) => nested,
    };

    let setting = settings
        .first()
        .expect("Should have a first nested setting");

    assert_eq!(setting.name(), "endpoint_url");

    assert_eq!(setting.value(), "https://profile-b-ec2-endpoint.aws");
}

#[test]
fn can_get_a_value_from_path_string() {
    let config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");
    let setting_path = SettingPath::try_from("profile.A.credential_source").expect("Should parse");

    let setting = config
        .get_setting(&setting_path)
        .expect("should have the setting");

    assert_eq!(setting.name(), "credential_source");

    let value = match setting.value() {
        ValueType::Single(value) => value,
        ValueType::Nested(_) => panic!("Should not be nested"),
    };

    assert_eq!(value, "Ec2InstanceMetadata")
}

#[test]
fn can_get_a_value_from_path_tuple() {
    let config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");
    let setting_path =
        SettingPath::try_from(("profile", "A", "credential_source")).expect("Should parse");

    let setting = config
        .get_setting(&setting_path)
        .expect("should have the setting");

    assert_eq!(setting.name(), "credential_source");

    let value = match setting.value() {
        ValueType::Single(value) => value,
        ValueType::Nested(_) => panic!("Should not be nested"),
    };

    assert_eq!(value, "Ec2InstanceMetadata")
}

#[test]
fn can_change_a_value() {
    const EXPECTED: &str = r#"
[profile A]
credential_source = my-new-credential-source
endpoint_url = https://profile-a-endpoint.aws/

[profile B]
source_profile = A
role_arn = arn:aws:iam::123456789012:role/roleB
services = profileB

[services profileB]
ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws
"#;
    let mut config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");

    let setting_path = SettingPath::try_from("profile.A.credential_source").expect("Should parse");
    config.set(setting_path, Value::from("my-new-credential-source"));
    let stringified = config.to_string();
    assert_eq!(stringified, EXPECTED)
}

#[test]
fn can_add_a_setting_to_existing_section() {
    const EXPECTED: &str = r#"
[profile A]
credential_source = Ec2InstanceMetadata
endpoint_url = https://profile-a-endpoint.aws/
other_setting = my-other-setting

[profile B]
source_profile = A
role_arn = arn:aws:iam::123456789012:role/roleB
services = profileB

[services profileB]
ec2 = 
  endpoint_url = https://profile-b-ec2-endpoint.aws
"#;
    let mut config = SAMPLE_FILE
        .parse::<AwsConfigFile>()
        .expect("Sample file should be valid");

    let setting_path = SettingPath::try_from("profile.A.other_setting").expect("Should parse");
    config.set(setting_path, Value::from("my-other-setting"));

    let stringified = config.to_string();
    assert_eq!(stringified, EXPECTED)
}

#[test]
fn can_parse_a_credential_file() {
    let config = SAMPLE_CRED_FILE
        .parse::<AwsCredentialsFile>()
        .expect("Should be valid");
    let section_name = "default".parse::<SectionName>().expect("Should be valid");
    let default_profile = config
        .get_profile(section_name)
        .expect("Should have found it");

    let aws_access_key_id = "aws_access_key_id"
        .parse::<SettingName>()
        .expect("Should be valid");
    let value = default_profile
        .get_value(&aws_access_key_id)
        .expect("Should be present");

    let value = match value {
        ValueType::Single(value) => value,
        ValueType::Nested(_) => panic!("Should not be nested"),
    };

    assert_eq!(value, "AKIAIOSFODNN7EXAMPLE");
}

#[test]
fn update_a_credential_file() {
    let mut config = SAMPLE_CRED_FILE
        .parse::<AwsCredentialsFile>()
        .expect("Should be valid");
    let section_name = "default".parse::<SectionName>().expect("Should be valid");
    let default_profile = config
        .get_profile_mut(section_name)
        .expect("Should have found it");

    let aws_access_key_id = "aws_access_key_id"
        .parse::<SettingName>()
        .expect("Should be valid");

    default_profile.set_value(&aws_access_key_id, ValueType::Single(Value::from("hi")));

    let setting = default_profile
        .get_value(&aws_access_key_id)
        .expect("Should exist");

    assert_eq!(*setting, ValueType::Single(Value::from("hi")))
}
