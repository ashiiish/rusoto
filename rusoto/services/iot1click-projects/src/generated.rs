// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateDeviceWithPlacementRequest {
    /// <p>The ID of the physical device to be associated with the given placement in the project. Note that a mandatory 4 character prefix is required for all <code>deviceId</code> values.</p>
    #[serde(rename = "deviceId")]
    pub device_id: String,
    /// <p>The device template name to associate with the device ID.</p>
    #[serde(rename = "deviceTemplateName")]
    pub device_template_name: String,
    /// <p>The name of the placement in which to associate the device.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement in which to associate the device.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateDeviceWithPlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePlacementRequest {
    /// <p>Optional user-defined key/value pairs providing contextual data (such as location or function) for the placement.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the placement to be created.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project in which to create the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProjectRequest {
    /// <p>An optional description for the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The schema defining the placement to be created. A placement template defines placement default attributes and device templates. You cannot add or remove device templates after the project has been created. However, you can update <code>callbackOverrides</code> for the device templates using the <code>UpdateProject</code> API.</p>
    #[serde(rename = "placementTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_template: Option<PlacementTemplate>,
    /// <p>The name of the project to create.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateProjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePlacementRequest {
    /// <p>The name of the empty placement to delete.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The project containing the empty placement to delete.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProjectRequest {
    /// <p>The name of the empty project to delete.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteProjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePlacementRequest {
    /// <p>The name of the placement within a project.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The project containing the placement to be described.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribePlacementResponse {
    /// <p>An object describing the placement.</p>
    #[serde(rename = "placement")]
    pub placement: PlacementDescription,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProjectRequest {
    /// <p>The name of the project to be described.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProjectResponse {
    /// <p>An object describing the project.</p>
    #[serde(rename = "project")]
    pub project: ProjectDescription,
}

/// <p>An object representing a device for a placement template (see <a>PlacementTemplate</a>).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplate {
    /// <p>An optional Lambda function to invoke instead of the default Lambda function provided by the placement template.</p>
    #[serde(rename = "callbackOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_overrides: Option<::std::collections::HashMap<String, String>>,
    /// <p>The device type, which currently must be <code>"button"</code>.</p>
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateDeviceFromPlacementRequest {
    /// <p>The device ID that should be removed from the placement.</p>
    #[serde(rename = "deviceTemplateName")]
    pub device_template_name: String,
    /// <p>The name of the placement that the device should be removed from.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project that contains the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateDeviceFromPlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevicesInPlacementRequest {
    /// <p>The name of the placement to get the devices from.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDevicesInPlacementResponse {
    /// <p>An object containing the devices (zero or more) within the placement.</p>
    #[serde(rename = "devices")]
    pub devices: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPlacementsRequest {
    /// <p>The maximum number of results to return per request. If not set, a default value of 100 is used.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The project containing the placements to be listed.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPlacementsResponse {
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An object listing the requested placements.</p>
    #[serde(rename = "placements")]
    pub placements: Vec<PlacementSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProjectsRequest {
    /// <p>The maximum number of results to return per request. If not set, a default value of 100 is used.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListProjectsResponse {
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An object containing the list of projects.</p>
    #[serde(rename = "projects")]
    pub projects: Vec<ProjectSummary>,
}

/// <p>An object describing a project's placement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PlacementDescription {
    /// <p>The user-defined attributes associated with the placement.</p>
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>The date when the placement was initially created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The name of the placement.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

/// <p>An object providing summary information for a particular placement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PlacementSummary {
    /// <p>The date when the placement was originally created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The name of the placement being summarized.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

/// <p>An object defining the template for a placement.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementTemplate {
    /// <p>The default attributes (key/value pairs) to be applied to all placements using this template.</p>
    #[serde(rename = "defaultAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>An object specifying the <a>DeviceTemplate</a> for all placements using this (<a>PlacementTemplate</a>) template.</p>
    #[serde(rename = "deviceTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_templates: Option<::std::collections::HashMap<String, DeviceTemplate>>,
}

/// <p>An object providing detailed information for a particular project associated with an AWS account and region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProjectDescription {
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The description of the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An object describing the project's placement specifications.</p>
    #[serde(rename = "placementTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_template: Option<PlacementTemplate>,
    /// <p>The name of the project for which to obtain information from.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

/// <p>An object providing summary information for a particular project for an associated AWS account and region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProjectSummary {
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The name of the project being summarized.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePlacementRequest {
    /// <p>The user-defined object of attributes used to update the placement. The maximum number of key/value pairs is 50.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the placement to update.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement to be updated.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProjectRequest {
    /// <p>An optional user-defined description for the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An object defining the project update. Once a project has been created, you cannot add device template names to the project. However, for a given <code>placementTemplate</code>, you can update the associated <code>callbackOverrides</code> for the device definition using this API.</p>
    #[serde(rename = "placementTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_template: Option<PlacementTemplate>,
    /// <p>The name of the project to be updated.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateProjectResponse {}

/// Errors returned by AssociateDeviceWithPlacement
#[derive(Debug, PartialEq)]
pub enum AssociateDeviceWithPlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceConflict(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl AssociateDeviceWithPlacementError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateDeviceWithPlacementError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(AssociateDeviceWithPlacementError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateDeviceWithPlacementError::InvalidRequest(
                        String::from(error_message),
                    ));
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithPlacementError::ResourceConflict(String::from(
                            error_message,
                        )),
                    );
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithPlacementError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    );
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateDeviceWithPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateDeviceWithPlacementError {
    fn description(&self) -> &str {
        match *self {
            AssociateDeviceWithPlacementError::InternalFailure(ref cause) => cause,
            AssociateDeviceWithPlacementError::InvalidRequest(ref cause) => cause,
            AssociateDeviceWithPlacementError::ResourceConflict(ref cause) => cause,
            AssociateDeviceWithPlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePlacement
#[derive(Debug, PartialEq)]
pub enum CreatePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceConflict(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl CreatePlacementError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePlacementError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(CreatePlacementError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreatePlacementError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreatePlacementError::ResourceConflict(
                        String::from(error_message),
                    ));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreatePlacementError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePlacementError {
    fn description(&self) -> &str {
        match *self {
            CreatePlacementError::InternalFailure(ref cause) => cause,
            CreatePlacementError::InvalidRequest(ref cause) => cause,
            CreatePlacementError::ResourceConflict(ref cause) => cause,
            CreatePlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceConflict(String),
}

impl CreateProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateProjectError::InternalFailure(String::from(
                        error_message,
                    )));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateProjectError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateProjectError::ResourceConflict(String::from(
                        error_message,
                    )));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProjectError {
    fn description(&self) -> &str {
        match *self {
            CreateProjectError::InternalFailure(ref cause) => cause,
            CreateProjectError::InvalidRequest(ref cause) => cause,
            CreateProjectError::ResourceConflict(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePlacement
#[derive(Debug, PartialEq)]
pub enum DeletePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl DeletePlacementError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePlacementError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(DeletePlacementError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeletePlacementError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePlacementError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePlacementError::TooManyRequests(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePlacementError {
    fn description(&self) -> &str {
        match *self {
            DeletePlacementError::InternalFailure(ref cause) => cause,
            DeletePlacementError::InvalidRequest(ref cause) => cause,
            DeletePlacementError::ResourceNotFound(ref cause) => cause,
            DeletePlacementError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl DeleteProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProjectError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteProjectError::InternalFailure(String::from(
                        error_message,
                    )));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteProjectError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProjectError::ResourceNotFound(String::from(
                        error_message,
                    )));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteProjectError::TooManyRequests(String::from(
                        error_message,
                    )));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteProjectError::InternalFailure(ref cause) => cause,
            DeleteProjectError::InvalidRequest(ref cause) => cause,
            DeleteProjectError::ResourceNotFound(ref cause) => cause,
            DeleteProjectError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePlacement
#[derive(Debug, PartialEq)]
pub enum DescribePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl DescribePlacementError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePlacementError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribePlacementError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribePlacementError::InvalidRequest(
                        String::from(error_message),
                    ));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePlacementError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePlacementError {
    fn description(&self) -> &str {
        match *self {
            DescribePlacementError::InternalFailure(ref cause) => cause,
            DescribePlacementError::InvalidRequest(ref cause) => cause,
            DescribePlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProject
#[derive(Debug, PartialEq)]
pub enum DescribeProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl DescribeProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProjectError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeProjectError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeProjectError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProjectError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProjectError {
    fn description(&self) -> &str {
        match *self {
            DescribeProjectError::InternalFailure(ref cause) => cause,
            DescribeProjectError::InvalidRequest(ref cause) => cause,
            DescribeProjectError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateDeviceFromPlacement
#[derive(Debug, PartialEq)]
pub enum DisassociateDeviceFromPlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl DisassociateDeviceFromPlacementError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateDeviceFromPlacementError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::InternalFailure(String::from(
                            error_message,
                        )),
                    );
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::InvalidRequest(String::from(
                            error_message,
                        )),
                    );
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    );
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::TooManyRequests(String::from(
                            error_message,
                        )),
                    );
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateDeviceFromPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateDeviceFromPlacementError {
    fn description(&self) -> &str {
        match *self {
            DisassociateDeviceFromPlacementError::InternalFailure(ref cause) => cause,
            DisassociateDeviceFromPlacementError::InvalidRequest(ref cause) => cause,
            DisassociateDeviceFromPlacementError::ResourceNotFound(ref cause) => cause,
            DisassociateDeviceFromPlacementError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevicesInPlacement
#[derive(Debug, PartialEq)]
pub enum GetDevicesInPlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl GetDevicesInPlacementError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevicesInPlacementError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(GetDevicesInPlacementError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetDevicesInPlacementError::InvalidRequest(
                        String::from(error_message),
                    ));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDevicesInPlacementError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDevicesInPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevicesInPlacementError {
    fn description(&self) -> &str {
        match *self {
            GetDevicesInPlacementError::InternalFailure(ref cause) => cause,
            GetDevicesInPlacementError::InvalidRequest(ref cause) => cause,
            GetDevicesInPlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPlacements
#[derive(Debug, PartialEq)]
pub enum ListPlacementsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl ListPlacementsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPlacementsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(ListPlacementsError::InternalFailure(String::from(
                        error_message,
                    )));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPlacementsError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPlacementsError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPlacementsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPlacementsError {
    fn description(&self) -> &str {
        match *self {
            ListPlacementsError::InternalFailure(ref cause) => cause,
            ListPlacementsError::InvalidRequest(ref cause) => cause,
            ListPlacementsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
}

impl ListProjectsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProjectsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(ListProjectsError::InternalFailure(String::from(
                        error_message,
                    )));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListProjectsError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListProjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProjectsError {
    fn description(&self) -> &str {
        match *self {
            ListProjectsError::InternalFailure(ref cause) => cause,
            ListProjectsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePlacement
#[derive(Debug, PartialEq)]
pub enum UpdatePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl UpdatePlacementError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePlacementError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdatePlacementError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdatePlacementError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdatePlacementError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePlacementError::TooManyRequests(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePlacementError {
    fn description(&self) -> &str {
        match *self {
            UpdatePlacementError::InternalFailure(ref cause) => cause,
            UpdatePlacementError::InvalidRequest(ref cause) => cause,
            UpdatePlacementError::ResourceNotFound(ref cause) => cause,
            UpdatePlacementError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl UpdateProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProjectError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateProjectError::InternalFailure(String::from(
                        error_message,
                    )));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateProjectError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProjectError::ResourceNotFound(String::from(
                        error_message,
                    )));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateProjectError::TooManyRequests(String::from(
                        error_message,
                    )));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProjectError {
    fn description(&self) -> &str {
        match *self {
            UpdateProjectError::InternalFailure(ref cause) => cause,
            UpdateProjectError::InvalidRequest(ref cause) => cause,
            UpdateProjectError::ResourceNotFound(ref cause) => cause,
            UpdateProjectError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT 1-Click Projects API. AWS IoT 1-Click Projects clients implement this trait.
pub trait Iot1ClickProjects {
    /// <p>Associates a physical device with a placement.</p>
    fn associate_device_with_placement(
        &self,
        input: AssociateDeviceWithPlacementRequest,
    ) -> RusotoFuture<AssociateDeviceWithPlacementResponse, AssociateDeviceWithPlacementError>;

    /// <p>Creates an empty placement.</p>
    fn create_placement(
        &self,
        input: CreatePlacementRequest,
    ) -> RusotoFuture<CreatePlacementResponse, CreatePlacementError>;

    /// <p>Creates an empty project with a placement template. A project contains zero or more placements that adhere to the placement template defined in the project.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResponse, CreateProjectError>;

    /// <p><p>Deletes a placement. To delete a placement, it must not have any devices associated with it.</p> <note> <p>When you delete a placement, all associated data becomes irretrievable.</p> </note></p>
    fn delete_placement(
        &self,
        input: DeletePlacementRequest,
    ) -> RusotoFuture<DeletePlacementResponse, DeletePlacementError>;

    /// <p><p>Deletes a project. To delete a project, it must not have any placements associated with it.</p> <note> <p>When you delete a project, all associated data becomes irretrievable.</p> </note></p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResponse, DeleteProjectError>;

    /// <p>Describes a placement in a project.</p>
    fn describe_placement(
        &self,
        input: DescribePlacementRequest,
    ) -> RusotoFuture<DescribePlacementResponse, DescribePlacementError>;

    /// <p>Returns an object describing a project.</p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResponse, DescribeProjectError>;

    /// <p>Removes a physical device from a placement.</p>
    fn disassociate_device_from_placement(
        &self,
        input: DisassociateDeviceFromPlacementRequest,
    ) -> RusotoFuture<DisassociateDeviceFromPlacementResponse, DisassociateDeviceFromPlacementError>;

    /// <p>Returns an object enumerating the devices in a placement.</p>
    fn get_devices_in_placement(
        &self,
        input: GetDevicesInPlacementRequest,
    ) -> RusotoFuture<GetDevicesInPlacementResponse, GetDevicesInPlacementError>;

    /// <p>Lists the placement(s) of a project.</p>
    fn list_placements(
        &self,
        input: ListPlacementsRequest,
    ) -> RusotoFuture<ListPlacementsResponse, ListPlacementsError>;

    /// <p>Lists the AWS IoT 1-Click project(s) associated with your AWS account and region.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResponse, ListProjectsError>;

    /// <p>Updates a placement with the given attributes. To clear an attribute, pass an empty value (i.e., "").</p>
    fn update_placement(
        &self,
        input: UpdatePlacementRequest,
    ) -> RusotoFuture<UpdatePlacementResponse, UpdatePlacementError>;

    /// <p>Updates a project associated with your AWS account and region. With the exception of device template names, you can pass just the values that need to be updated because the update request will change only the values that are provided. To clear a value, pass the empty string (i.e., <code>""</code>).</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResponse, UpdateProjectError>;
}
/// A client for the AWS IoT 1-Click Projects API.
#[derive(Clone)]
pub struct Iot1ClickProjectsClient {
    client: Client,
    region: region::Region,
}

impl Iot1ClickProjectsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Iot1ClickProjectsClient {
        Iot1ClickProjectsClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Iot1ClickProjectsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Iot1ClickProjectsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Iot1ClickProjects for Iot1ClickProjectsClient {
    /// <p>Associates a physical device with a placement.</p>
    fn associate_device_with_placement(
        &self,
        input: AssociateDeviceWithPlacementRequest,
    ) -> RusotoFuture<AssociateDeviceWithPlacementResponse, AssociateDeviceWithPlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}/devices/{device_template_name}",
            device_template_name = input.device_template_name,
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<AssociateDeviceWithPlacementResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDeviceWithPlacementError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an empty placement.</p>
    fn create_placement(
        &self,
        input: CreatePlacementRequest,
    ) -> RusotoFuture<CreatePlacementResponse, CreatePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("POST", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreatePlacementResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an empty project with a placement template. A project contains zero or more placements that adhere to the placement template defined in the project.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResponse, CreateProjectError> {
        let request_uri = "/projects";

        let mut request = SignedRequest::new("POST", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateProjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes a placement. To delete a placement, it must not have any devices associated with it.</p> <note> <p>When you delete a placement, all associated data becomes irretrievable.</p> </note></p>
    fn delete_placement(
        &self,
        input: DeletePlacementRequest,
    ) -> RusotoFuture<DeletePlacementResponse, DeletePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeletePlacementResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes a project. To delete a project, it must not have any placements associated with it.</p> <note> <p>When you delete a project, all associated data becomes irretrievable.</p> </note></p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResponse, DeleteProjectError> {
        let request_uri = format!(
            "/projects/{project_name}",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteProjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes a placement in a project.</p>
    fn describe_placement(
        &self,
        input: DescribePlacementRequest,
    ) -> RusotoFuture<DescribePlacementResponse, DescribePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribePlacementResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an object describing a project.</p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResponse, DescribeProjectError> {
        let request_uri = format!(
            "/projects/{project_name}",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeProjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a physical device from a placement.</p>
    fn disassociate_device_from_placement(
        &self,
        input: DisassociateDeviceFromPlacementRequest,
    ) -> RusotoFuture<DisassociateDeviceFromPlacementResponse, DisassociateDeviceFromPlacementError>
    {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}/devices/{device_template_name}",
            device_template_name = input.device_template_name,
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DisassociateDeviceFromPlacementResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateDeviceFromPlacementError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns an object enumerating the devices in a placement.</p>
    fn get_devices_in_placement(
        &self,
        input: GetDevicesInPlacementRequest,
    ) -> RusotoFuture<GetDevicesInPlacementResponse, GetDevicesInPlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}/devices",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDevicesInPlacementResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDevicesInPlacementError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the placement(s) of a project.</p>
    fn list_placements(
        &self,
        input: ListPlacementsRequest,
    ) -> RusotoFuture<ListPlacementsResponse, ListPlacementsError> {
        let request_uri = format!(
            "/projects/{project_name}/placements",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListPlacementsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPlacementsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the AWS IoT 1-Click project(s) associated with your AWS account and region.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResponse, ListProjectsError> {
        let request_uri = "/projects";

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListProjectsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListProjectsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a placement with the given attributes. To clear an attribute, pass an empty value (i.e., "").</p>
    fn update_placement(
        &self,
        input: UpdatePlacementRequest,
    ) -> RusotoFuture<UpdatePlacementResponse, UpdatePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdatePlacementResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a project associated with your AWS account and region. With the exception of device template names, you can pass just the values that need to be updated because the update request will change only the values that are provided. To clear a value, pass the empty string (i.e., <code>""</code>).</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResponse, UpdateProjectError> {
        let request_uri = format!(
            "/projects/{project_name}",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateProjectResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateProjectError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
