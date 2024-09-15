use aws_config_modify::{AwsConfigFile, Entry, SectionPath, SectionType, SettingPath};

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

#[test]
fn can_get_a_section_with_a_string() {
    let config = AwsConfigFile::parse(SAMPLE_FILE).expect("Sample file should be valid");
    let section_path = SectionPath::try_from("profile.A").expect("Should parse");
    let section = config
        .get_section(section_path)
        .expect("This section should exist");

    let name = section.get_name().expect("Should have a name");
    assert_eq!(**name, "A");
    let mut settings = section.settings().iter();
    let first = settings.next().expect("Should have one entry");
    assert_eq!(**first.key(), "credential_source");

    let first = match first {
        Entry::Single(setting) => setting,
        Entry::WithSubsettings(_) => panic!("Should not have subsettings"),
    };
    let value = first.value();
    assert_eq!(**value, "Ec2InstanceMetadata");

    let second = settings.next().expect("Should have second entry");
    assert_eq!(**second.key(), "endpoint_url");

    let second = match second {
        Entry::Single(setting) => setting,
        Entry::WithSubsettings(_) => panic!("Should not have subsettings"),
    };
    let value = second.value();
    assert_eq!(**value, "https://profile-a-endpoint.aws/");
}

#[test]
fn can_get_a_section_with_a_tuple_of_strings() {
    let config = AwsConfigFile::parse(SAMPLE_FILE).expect("Sample file should be valid");
    let section_path = SectionPath::try_from(("profile", "A")).expect("Should parse");
    config.get_section(section_path);
}

#[test]
fn can_get_a_section_with_a_section_type_and_string() {
    let config = AwsConfigFile::parse(SAMPLE_FILE).expect("Sample file should be valid");
    let section_path = SectionPath::try_from((SectionType::Profile, "B")).expect("Should parse");
    config
        .get_section(section_path)
        .expect("Section B should exist");
}

#[test]
fn can_get_a_nested_section() {
    let config = AwsConfigFile::parse(SAMPLE_FILE).expect("Sample file should be valid");
    let section_path =
        SectionPath::try_from((SectionType::Services, "profileB")).expect("Should parse");
    let section = config
        .get_section(section_path)
        .expect("Services for profileB should exist");

    let setting = section
        .settings()
        .first()
        .expect("Should be a first setting");
    dbg!(setting);
    let setting = match setting {
        Entry::Single(_) => panic!("Should not be a single setting"),
        Entry::WithSubsettings(setting) => setting,
    };

    assert_eq!(**setting.name(), "ec2");

    let setting = setting
        .value()
        .first()
        .expect("Should be a first subsetting");

    assert_eq!(**setting.name(), "endpoint_url");
    assert_eq!(**setting.value(), "https://profile-b-ec2-endpoint.aws");
    assert!(setting.is_nested())
}

#[test]
fn can_get_a_value_from_path_string() {
    let config = AwsConfigFile::parse(SAMPLE_FILE).expect("Sample file should be valid");
    let section_path = SettingPath::try_from("profile.A.credential_source").expect("Should parse");

    // config.get_setting();
}
