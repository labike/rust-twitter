use nutype::nutype;

#[nutype(validate(min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Username(String);

#[nutype(validate(min_len = 8, max_len = 16))]
#[derive(AsRef, Clone, PartialEq, Serialize, Deserialize)]
pub struct Password(String);