use crate::http::bucket_access_controls::BucketAccessControl;
use crate::http::Escape;
use reqwest::{Client, RequestBuilder};

#[derive(Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchBucketAccessControlRequest {
    /// Name of a bucket.
    pub bucket: String,
    /// The entity holding the permission. Can be user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    pub acl: BucketAccessControl,
}

pub fn build(base_url: &str, client: &Client, req: &PatchBucketAccessControlRequest) -> RequestBuilder {
    let url = format!("{}/b/{}/acl/{}", base_url, req.bucket.escape(), req.entity.escape());
    client.patch(url).json(&req.acl)
}
