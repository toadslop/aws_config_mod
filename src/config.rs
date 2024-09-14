use std::{iter::Enumerate, ops::Deref};

pub struct ConfigFile {
    sections: Vec<ConfigSection>,
    // tokens: Vec<Token<'a>>,
}

impl<'a> ConfigFile {
    // pub fn parse(input: &'a str) -> Self {
    //     let (_, toks) = tokens(input).unwrap();

    //     todo!()
    // }
}

pub struct CredentialsFile {
    sections: CredentialsSection,
    // tokens: Vec<Token<'a>>,
}

pub struct CredentialsSection {
    profile: ProfileName,
    aws_access_key_id: Value,
    aws_secret_access_key: Value,
    aws_session_token: Value,
}

pub struct ConfigSection {
    section_type: SectionType,
    profile: ProfileName,
    content: Vec<SectionContent>,
}

pub struct ProfileName(TokenId);

pub enum SectionType {
    Profile,
    Services,
    SsoSession,
    Default,
}

pub enum SectionContent {
    Single(TokenId, TokenId),
    Nested {
        key: TokenId,
        values: Vec<(TokenId, TokenId)>,
    },
}

pub struct Key(TokenId);
pub struct Value(TokenId);

// impl Parsable for Value {
//     type Output = Self;

//     fn parse<'a>(input: &mut Enumerate<std::slice::Iter<'_, Token<'_>>>) -> Result<Self, ()> {
//         let (id, token): (usize, &Token<'_>) = input.next().ok_or(())?;

//         if let Token::Text(..) = token {
//             Ok(Self(TokenId(id)))
//         } else {
//             Err(())
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenId(usize);

impl Deref for TokenId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// trait Parsable {
//     type Output;
//     fn parse(input: &mut Enumerate<std::slice::Iter<'_, Token<'_>>>) -> Result<Self::Output, ()>;
// }
