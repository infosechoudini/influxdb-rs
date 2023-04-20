/// User data model
use serde::{Serialize, Deserialize};



/// User Status Enum
#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

/// User Struct to Create User
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    /// Name of the User
    pub name: String,
    /// Status of User
    pub status: String,
}

/// Create User Response
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    /// User ID
    pub id: String,
    /// User Name
    pub name: String,
    /// User Status
    pub status: String,
    /// Links
    pub links: UserLinks,
}

/// User Links
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLinks {
    /// Self
    #[serde(rename = "self")]
    pub user_self: String,
}


/// List User Response
#[derive(Serialize, Deserialize, Debug)]
pub struct ListUserResponse {
    /// Users
    pub users: Vec<UserResponse>,
    /// Links
    pub links: UserLinks,
}