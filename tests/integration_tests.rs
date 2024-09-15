use aws_config_modify::{AwsConfigFile, SectionPath, SectionType};

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
    let second = settings.next().expect("Should have second entry");
    assert_eq!(**second.key(), "endpoint_url");
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
    config.get_section(section_path);
}
