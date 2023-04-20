/// Authorization Data Model
use serde::{Serialize, Deserialize};

/// Create a new authorization
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAuthorization {
    /// Description of the authorization
    pub description: String,
    /// Permissions of the authorization
    pub permissions: Vec<AuthPermissions>,
    /// Org Id
    #[serde(rename = "orgID")]
    pub org_id: String,
    /// Status of the authorization
    pub status: String,
    /// User Id
    pub user_id: Option<String>,
}

/// Authorization Permissions
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthPermissions {
    /// Action of the authorization
    pub action: String,
    /// Resource of the authorization
    pub resource: AuthResource,
}

/// Authorization Resource
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResource {
    /// Type of the authorization
    pub r#type: String,
    /// Org ID of the authorization
    pub org_id: Option<String>,
    /// Bucket ID of the authorization
    pub bucket_id: Option<String>,
}

/// Authorization Resource Type
#[derive(Serialize, Deserialize, Debug)]
pub enum AuthResourceType {
    /// Bucket
    Bucket,
    /// Organization
    Organization,
    /// Dashboards
    Dashboards,
    /// Orgs
    Orgs,
    /// Tasks
    Tasks,
    /// Telegrafs
    Telegrafs,
    /// Users
    Users,
    /// Variables
    Variables,
    /// Secrets
    Secrets,
    /// Labels
    Labels,
    /// Views
    Views,
    /// Documents
    Documents,
    /// NotificationsRules
    NotificationsRules,
    /// NotificationEndpoints
    NotificationEndpoints,
    /// Checks
    Checks,
    /// DBRP
    DBRP,
    /// Annotations
    Annotations,
    /// Sources
    Sources,
    /// Scrapers
    Scrapers,
    /// Notebooks
    Notebooks,
    /// Remotes
    Remotes,
    /// Replications
    Replications,
    /// Instance
    Instance,
    /// Flows
    Flows,
    /// Functions
    Functions,
    /// Subscriptions
    Subscriptions,
}

/// Impl Display for AuthResourceType
impl std::fmt::Display for AuthResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthResourceType::Bucket => write!(f, "buckets"),
            AuthResourceType::Organization => write!(f, "orgs"),
            AuthResourceType::Dashboards => write!(f, "dashboards"),
            AuthResourceType::Orgs => write!(f, "orgs"),
            AuthResourceType::Tasks => write!(f, "tasks"),
            AuthResourceType::Telegrafs => write!(f, "telegrafs"),
            AuthResourceType::Users => write!(f, "users"),
            AuthResourceType::Variables => write!(f, "variables"),
            AuthResourceType::Secrets => write!(f, "secrets"),
            AuthResourceType::Labels => write!(f, "labels"),
            AuthResourceType::Views => write!(f, "views"),
            AuthResourceType::Documents => write!(f, "documents"),
            AuthResourceType::NotificationsRules => write!(f, "notificationRules"),
            AuthResourceType::NotificationEndpoints => write!(f, "notificationEndpoints"),
            AuthResourceType::Checks => write!(f, "checks"),
            AuthResourceType::DBRP => write!(f, "dbrps"),
            AuthResourceType::Annotations => write!(f, "annotations"),
            AuthResourceType::Sources => write!(f, "sources"),
            AuthResourceType::Scrapers => write!(f, "scrapers"),
            AuthResourceType::Notebooks => write!(f, "notebooks"),
            AuthResourceType::Remotes => write!(f, "remotes"),
            AuthResourceType::Replications => write!(f, "replications"),
            AuthResourceType::Instance => write!(f, "instance"),
            AuthResourceType::Flows => write!(f, "flows"),
            AuthResourceType::Functions => write!(f, "functions"),
            AuthResourceType::Subscriptions => write!(f, "subscriptions"),
        }
    }
}


/// Authorization Response
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationResponse {
    /// Description
    pub description: String,
    /// Status
    pub status: String,
    /// Created At
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Authorization ID for internal tracking
    pub id: String,
    /// Links
    pub links: AuthorizationLinks,
    /// Org
    pub org: String,
    /// Org Id
    #[serde(rename = "orgID")]
    pub org_id: String,
    /// Permissions
    pub permissions: Vec<AuthPermissions>,
    /// Token
    pub token: String,
    /// Updated At
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// User 
    pub user: String,
    /// User Id
    #[serde(rename = "userID")]
    pub user_id: String,
}

/// Authorization Links
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationLinks {
    /// Self
    #[serde(rename = "self")]
    pub authorization_self: String,
    /// User
    pub user: String,
}