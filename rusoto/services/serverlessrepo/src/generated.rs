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
/// <p>Details about the application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Application {
    /// <p>The application Amazon Resource Name (ARN).</p>
    pub application_id: String,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    pub author: String,
    /// <p>The date and time this resource was created.</p>
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    pub name: String,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    pub version: Option<Version>,
}

/// <p>A list of application summaries nested in the application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationDependencyPage {
    /// <p>An array of application summaries nested in the application.</p>
    pub dependencies: Vec<ApplicationDependencySummary>,
    /// <p>The token to request the next page of results.</p>
    pub next_token: Option<String>,
}

/// <p>A nested application summary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationDependencySummary {
    /// <p>The Amazon Resource Name (ARN) of the nested application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the nested application.</p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
}

/// <p>A list of application details.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationPage {
    /// <p>An array of application summaries.</p>
    pub applications: Vec<ApplicationSummary>,
    /// <p>The token to request the next page of results.</p>
    pub next_token: Option<String>,
}

/// <p>Policy statements applied to the application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationPolicy {
    /// <p>An array of policy statements applied to the application.</p>
    pub statements: Vec<ApplicationPolicyStatement>,
}

/// <p>Policy statement applied to the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPolicyStatement {
    /// <p>For the list of actions supported for this operation, see <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application
    /// Permissions</a>.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p>An AWS account ID, or * to make the application public.</p>
    #[serde(rename = "Principals")]
    pub principals: Vec<String>,
    /// <p>A unique ID for the statement.</p>
    #[serde(rename = "StatementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

/// <p>Summary of details about the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationSummary {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    pub author: String,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
}

/// <p>A list of version summaries for the application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationVersionPage {
    /// <p>The token to request the next page of results.</p>
    pub next_token: Option<String>,
    /// <p>An array of version summaries for the application.</p>
    pub versions: Vec<VersionSummary>,
}

/// <p>Details of the change set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeSetDetails {
    /// <p>The application Amazon Resource Name (ARN).</p>
    pub application_id: String,
    /// <p>The Amazon Resource Name (ARN) of the change set.</p><p>Length constraints: Minimum length of 1.</p><p>Pattern: ARN:[-a-zA-Z0-9:/]*</p>
    pub change_set_id: String,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    pub semantic_version: String,
    /// <p>The unique ID of the stack.</p>
    pub stack_id: String,
}

/// <p>Create an application request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateApplicationInput {
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    pub author: String,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    pub labels: Option<Vec<String>>,
    /// <p>A local text file that contains the license of the app that matches the spdxLicenseID value of your application.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    pub license_body: Option<String>,
    /// <p>A link to the S3 object that contains the license of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    pub license_url: Option<String>,
    /// <p>The name of the application that you want to publish.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    pub name: String,
    /// <p>A local text readme file in Markdown language that contains a more detailed description of the application and how it works.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    pub readme_body: Option<String>,
    /// <p>A link to the S3 object in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    pub readme_url: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    pub source_code_url: Option<String>,
    /// <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    pub spdx_license_id: Option<String>,
    /// <p>The local raw packaged AWS SAM template file of your application.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
    pub template_body: Option<String>,
    /// <p>A link to the S3 object containing the packaged AWS SAM template of your application.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationRequest {
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    pub author: String,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A local text file that contains the license of the app that matches the spdxLicenseID value of your application.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    #[serde(rename = "LicenseBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_body: Option<String>,
    /// <p>A link to the S3 object that contains the license of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application that you want to publish.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A local text readme file in Markdown language that contains a more detailed description of the application and how it works.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    #[serde(rename = "ReadmeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_body: Option<String>,
    /// <p>A link to the S3 object in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>The local raw packaged AWS SAM template file of your application.
    /// The file has the format file://&lt;path>/&lt;filename>.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>A link to the S3 object containing the packaged AWS SAM template of your application.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

/// <p>Create a version request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateApplicationVersionInput {
    /// <p>A link to a public repository for the source code of your application.</p>
    pub source_code_url: Option<String>,
    /// <p>The raw packaged AWS SAM template of your application.</p>
    pub template_body: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationVersionRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the new version.</p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>The raw packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateApplicationVersionResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>An array of parameter types supported by the application.</p>
    #[serde(rename = "ParameterDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_definitions: Option<Vec<ParameterDefinition>>,
    /// <p>A list of values that you must specify before you can deploy certain applications.
    /// Some applications might include resources that can affect permissions in your AWS
    /// account, for example, by creating new AWS Identity and Access Management (IAM) users.
    /// For those applications, you must explicitly acknowledge their capabilities by
    /// specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
    /// and CAPABILITY_RESOURCE_POLICY.</p><p>The following resources require you to specify CAPABILITY_IAM or
    /// CAPABILITY_NAMED_IAM:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
    /// If the application contains IAM resources, you can specify either CAPABILITY_IAM
    /// or CAPABILITY_NAMED_IAM. If the application contains IAM resources
    /// with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS::TopicPolicy</a>.</p><p>If your application template contains any of the above resources, we recommend that you review
    /// all permissions associated with the application before deploying. If you don't specify
    /// this parameter for an application that requires capabilities, the call will fail.</p><p>Valid values: CAPABILITY_IAM | CAPABILITY_NAMED_IAM | CAPABILITY_RESOURCE_POLICY
    /// </p>
    #[serde(rename = "RequiredCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_capabilities: Option<Vec<String>>,
    /// <p>Whether all of the AWS resources contained in this application are supported in the region
    /// in which it is being retrieved.</p>
    #[serde(rename = "ResourcesSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_supported: Option<bool>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

/// <p>Create an application change set request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateCloudFormationChangeSetInput {
    /// <p>A list of values that you must specify before you can deploy certain applications.
    /// Some applications might include resources that can affect permissions in your AWS
    /// account, for example, by creating new AWS Identity and Access Management (IAM) users.
    /// For those applications, you must explicitly acknowledge their capabilities by
    /// specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
    /// and CAPABILITY_RESOURCE_POLICY.</p><p>The following resources require you to specify CAPABILITY_IAM or
    /// CAPABILITY_NAMED_IAM:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
    /// If the application contains IAM resources, you can specify either CAPABILITY_IAM
    /// or CAPABILITY_NAMED_IAM. If the application contains IAM resources
    /// with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS:TopicPolicy</a>.</p><p>If your application template contains any of the above resources, we recommend that you review
    /// all permissions associated with the application before deploying. If you don't specify
    /// this parameter for an application that requires capabilities, the call will fail.</p><p>Valid values: CAPABILITY_IAM | CAPABILITY_NAMED_IAM | CAPABILITY_RESOURCE_POLICY
    /// </p>
    pub capabilities: Option<Vec<String>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub change_set_name: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub client_token: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub description: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub notification_arns: Option<Vec<String>>,
    /// <p>A list of parameter values for the parameters of the application.</p>
    pub parameter_overrides: Option<Vec<ParameterValue>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub resource_types: Option<Vec<String>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    pub semantic_version: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub stack_name: String,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    pub template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCloudFormationChangeSetRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A list of values that you must specify before you can deploy certain applications.
    /// Some applications might include resources that can affect permissions in your AWS
    /// account, for example, by creating new AWS Identity and Access Management (IAM) users.
    /// For those applications, you must explicitly acknowledge their capabilities by
    /// specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
    /// and CAPABILITY_RESOURCE_POLICY.</p><p>The following resources require you to specify CAPABILITY_IAM or
    /// CAPABILITY_NAMED_IAM:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
    /// If the application contains IAM resources, you can specify either CAPABILITY_IAM
    /// or CAPABILITY_NAMED_IAM. If the application contains IAM resources
    /// with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS:TopicPolicy</a>.</p><p>If your application template contains any of the above resources, we recommend that you review
    /// all permissions associated with the application before deploying. If you don't specify
    /// this parameter for an application that requires capabilities, the call will fail.</p><p>Valid values: CAPABILITY_IAM | CAPABILITY_NAMED_IAM | CAPABILITY_RESOURCE_POLICY
    /// </p>
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "ChangeSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    /// <p>A list of parameter values for the parameters of the application.</p>
    #[serde(rename = "ParameterOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_overrides: Option<Vec<ParameterValue>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "RollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a></i> API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCloudFormationChangeSetResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the change set.</p><p>Length constraints: Minimum length of 1.</p><p>Pattern: ARN:[-a-zA-Z0-9:/]*</p>
    #[serde(rename = "ChangeSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>The unique ID of the stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCloudFormationTemplateRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCloudFormationTemplateResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The date and time this template expires. Templates
    /// expire 1 hour after creation.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>Status of the template creation workflow.</p><p>Possible values: PREPARING | ACTIVE | EXPIRED</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// <p>A link to the template that can be used to deploy the application using
    /// AWS CloudFormation.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApplicationPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApplicationPolicyResponse {
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the application to get.</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCloudFormationTemplateRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCloudFormationTemplateResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The date and time this template expires. Templates
    /// expire 1 hour after creation.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>Status of the template creation workflow.</p><p>Possible values: PREPARING | ACTIVE | EXPIRED</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// <p>A link to the template that can be used to deploy the application using
    /// AWS CloudFormation.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApplicationDependenciesRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The total number of items to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>A token to specify where to start paginating.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The semantic version of the application to get.</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListApplicationDependenciesResponse {
    /// <p>An array of application summaries nested in the application.</p>
    #[serde(rename = "Dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<ApplicationDependencySummary>>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApplicationVersionsRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The total number of items to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>A token to specify where to start paginating.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListApplicationVersionsResponse {
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of version summaries for the application.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApplicationsRequest {
    /// <p>The total number of items to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>A token to specify where to start paginating.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListApplicationsResponse {
    /// <p>An array of application summaries.</p>
    #[serde(rename = "Applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<ApplicationSummary>>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Parameters supported by the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ParameterDefinition {
    /// <p>A regular expression that represents the patterns to allow for String types.</p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p>An array containing the list of values allowed for the parameter.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    /// <p>A string that explains a constraint when the constraint is violated. For example, without a constraint description,
    /// a parameter that has an allowed pattern of [A-Za-z0-9]+ displays the following error message when the user
    /// specifies an invalid value:</p><p>
    /// Malformed input-Parameter MyParameter must match pattern [A-Za-z0-9]+
    /// </p><p>By adding a constraint description, such as "must contain only uppercase and lowercase letters and numbers," you can display
    /// the following customized error message:</p><p>
    /// Malformed input-Parameter MyParameter must contain only uppercase and lowercase letters and numbers.
    /// </p>
    #[serde(rename = "ConstraintDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_description: Option<String>,
    /// <p>A value of the appropriate type for the template to use if no value is specified when a stack is created.
    /// If you define constraints for the parameter, you must specify a value that adheres to those constraints.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>A string of up to 4,000 characters that describes the parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An integer value that determines the largest number of characters that you want to allow for String types.</p>
    #[serde(rename = "MaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    /// <p>A numeric value that determines the largest numeric value that you want to allow for Number types.</p>
    #[serde(rename = "MaxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
    /// <p>An integer value that determines the smallest number of characters that you want to allow for String types.</p>
    #[serde(rename = "MinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    /// <p>A numeric value that determines the smallest numeric value that you want to allow for Number types.</p>
    #[serde(rename = "MinValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Whether to mask the parameter value whenever anyone makes a call that describes the stack. If you set the
    /// value to true, the parameter value is masked with asterisks (*****).</p>
    #[serde(rename = "NoEcho")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_echo: Option<bool>,
    /// <p>A list of AWS SAM resources that use this parameter.</p>
    #[serde(rename = "ReferencedByResources")]
    pub referenced_by_resources: Vec<String>,
    /// <p>The type of the parameter.</p><p>Valid values: String | Number | List&lt;Number> | CommaDelimitedList
    /// </p><p>
    /// String: A literal string.</p><p>For example, users can specify "MyUserName".</p><p>
    /// Number: An integer or float. AWS CloudFormation validates the parameter value as a number. However, when you use the
    /// parameter elsewhere in your template (for example, by using the Ref intrinsic function), the parameter value becomes a string.</p><p>For example, users might specify "8888".</p><p>
    /// List&lt;Number>: An array of integers or floats that are separated by commas. AWS CloudFormation validates the parameter value as numbers. However, when
    /// you use the parameter elsewhere in your template (for example, by using the Ref intrinsic function), the parameter value becomes a list of strings.</p><p>For example, users might specify "80,20", and then Ref results in ["80","20"].</p><p>
    /// CommaDelimitedList: An array of literal strings that are separated by commas. The total number of strings should be one more than the total number of commas.
    /// Also, each member string is space-trimmed.</p><p>For example, users might specify "test,dev,prod", and then Ref results in ["test","dev","prod"].</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Parameter value of the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParameterValue {
    /// <p>The key associated with the parameter. If you don't specify a key and value for a particular parameter, AWS CloudFormation
    /// uses the default value that is specified in your template.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The input value associated with the parameter.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutApplicationPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    pub statements: Vec<ApplicationPolicyStatement>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutApplicationPolicyResponse {
    /// <p>An array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

/// <p>This property corresponds to the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackConfiguration">RollbackConfiguration</a></i> Data Type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RollbackConfiguration {
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackConfiguration">RollbackConfiguration</a></i> Data Type.</p>
    #[serde(rename = "MonitoringTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_time_in_minutes: Option<i64>,
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackConfiguration">RollbackConfiguration</a></i> Data Type.</p>
    #[serde(rename = "RollbackTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_triggers: Option<Vec<RollbackTrigger>>,
}

/// <p>This property corresponds to the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackTrigger">RollbackTrigger</a></i> Data Type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RollbackTrigger {
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackTrigger">RollbackTrigger</a></i> Data Type.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/RollbackTrigger">RollbackTrigger</a></i> Data Type.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>This property corresponds to the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/Tag">Tag</a></i> Data Type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Tag {
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/Tag">Tag</a></i> Data Type.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>This property corresponds to the content of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/Tag">
    /// Tag</a>
    /// </i>
    /// Data Type.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Details of the template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TemplateDetails {
    /// <p>The application Amazon Resource Name (ARN).</p>
    pub application_id: String,
    /// <p>The date and time this resource was created.</p>
    pub creation_time: String,
    /// <p>The date and time this template expires. Templates
    /// expire 1 hour after creation.</p>
    pub expiration_time: String,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    pub semantic_version: String,
    /// <p>Status of the template creation workflow.</p><p>Possible values: PREPARING | ACTIVE | EXPIRED</p>
    pub status: String,
    /// <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p>
    pub template_id: String,
    /// <p>A link to the template that can be used to deploy the application using
    /// AWS CloudFormation.</p>
    pub template_url: String,
}

/// <p>Update the application request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateApplicationInput {
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    pub author: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    pub labels: Option<Vec<String>>,
    /// <p>A text readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    pub readme_body: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    pub readme_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApplicationRequest {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A text readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_body: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A URL with more information about the application, for example
    /// the location of your GitHub repository for the application.</p>
    #[serde(rename = "HomePageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

/// <p>Application version details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Version {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: String,
    /// <p>An array of parameter types supported by the application.</p>
    #[serde(rename = "ParameterDefinitions")]
    pub parameter_definitions: Vec<ParameterDefinition>,
    /// <p>A list of values that you must specify before you can deploy certain applications.
    /// Some applications might include resources that can affect permissions in your AWS
    /// account, for example, by creating new AWS Identity and Access Management (IAM) users.
    /// For those applications, you must explicitly acknowledge their capabilities by
    /// specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
    /// and CAPABILITY_RESOURCE_POLICY.</p><p>The following resources require you to specify CAPABILITY_IAM or
    /// CAPABILITY_NAMED_IAM:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
    /// If the application contains IAM resources, you can specify either CAPABILITY_IAM
    /// or CAPABILITY_NAMED_IAM. If the application contains IAM resources
    /// with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
    /// <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS::TopicPolicy</a>.</p><p>If your application template contains any of the above resources, we recommend that you review
    /// all permissions associated with the application before deploying. If you don't specify
    /// this parameter for an application that requires capabilities, the call will fail.</p><p>Valid values: CAPABILITY_IAM | CAPABILITY_NAMED_IAM | CAPABILITY_RESOURCE_POLICY
    /// </p>
    #[serde(rename = "RequiredCapabilities")]
    pub required_capabilities: Vec<String>,
    /// <p>Whether all of the AWS resources contained in this application are supported in the region
    /// in which it is being retrieved.</p>
    #[serde(rename = "ResourcesSupported")]
    pub resources_supported: bool,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    pub template_url: String,
}

/// <p>An application version summary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VersionSummary {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The date and time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: String,
    /// <p>The semantic version of the application:</p><p>
    /// <a href="https://semver.org/">https://semver.org/</a>
    /// </p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
}

/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl CreateApplicationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(CreateApplicationError::BadRequest(String::from(
                        error_message,
                    )));
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateApplicationError::Conflict(String::from(
                        error_message,
                    )));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateApplicationError::Forbidden(String::from(
                        error_message,
                    )));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateApplicationError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateApplicationError::TooManyRequests(
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
impl fmt::Display for CreateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationError::BadRequest(ref cause) => cause,
            CreateApplicationError::Conflict(ref cause) => cause,
            CreateApplicationError::Forbidden(ref cause) => cause,
            CreateApplicationError::InternalServerError(ref cause) => cause,
            CreateApplicationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplicationVersion
#[derive(Debug, PartialEq)]
pub enum CreateApplicationVersionError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl CreateApplicationVersionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationVersionError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(CreateApplicationVersionError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateApplicationVersionError::Conflict(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateApplicationVersionError::Forbidden(
                        String::from(error_message),
                    ));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateApplicationVersionError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateApplicationVersionError::TooManyRequests(
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
impl fmt::Display for CreateApplicationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationVersionError::BadRequest(ref cause) => cause,
            CreateApplicationVersionError::Conflict(ref cause) => cause,
            CreateApplicationVersionError::Forbidden(ref cause) => cause,
            CreateApplicationVersionError::InternalServerError(ref cause) => cause,
            CreateApplicationVersionError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCloudFormationChangeSet
#[derive(Debug, PartialEq)]
pub enum CreateCloudFormationChangeSetError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl CreateCloudFormationChangeSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCloudFormationChangeSetError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(CreateCloudFormationChangeSetError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateCloudFormationChangeSetError::Forbidden(
                        String::from(error_message),
                    ));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateCloudFormationChangeSetError::InternalServerError(String::from(
                            error_message,
                        )),
                    );
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateCloudFormationChangeSetError::TooManyRequests(String::from(
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
impl fmt::Display for CreateCloudFormationChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCloudFormationChangeSetError {
    fn description(&self) -> &str {
        match *self {
            CreateCloudFormationChangeSetError::BadRequest(ref cause) => cause,
            CreateCloudFormationChangeSetError::Forbidden(ref cause) => cause,
            CreateCloudFormationChangeSetError::InternalServerError(ref cause) => cause,
            CreateCloudFormationChangeSetError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCloudFormationTemplate
#[derive(Debug, PartialEq)]
pub enum CreateCloudFormationTemplateError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl CreateCloudFormationTemplateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCloudFormationTemplateError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(CreateCloudFormationTemplateError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateCloudFormationTemplateError::Forbidden(
                        String::from(error_message),
                    ));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateCloudFormationTemplateError::InternalServerError(String::from(
                            error_message,
                        )),
                    );
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateCloudFormationTemplateError::NotFound(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateCloudFormationTemplateError::TooManyRequests(
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
impl fmt::Display for CreateCloudFormationTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCloudFormationTemplateError {
    fn description(&self) -> &str {
        match *self {
            CreateCloudFormationTemplateError::BadRequest(ref cause) => cause,
            CreateCloudFormationTemplateError::Forbidden(ref cause) => cause,
            CreateCloudFormationTemplateError::InternalServerError(ref cause) => cause,
            CreateCloudFormationTemplateError::NotFound(ref cause) => cause,
            CreateCloudFormationTemplateError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl DeleteApplicationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApplicationError::BadRequest(String::from(
                        error_message,
                    )));
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteApplicationError::Conflict(String::from(
                        error_message,
                    )));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteApplicationError::Forbidden(String::from(
                        error_message,
                    )));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteApplicationError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApplicationError::NotFound(String::from(
                        error_message,
                    )));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApplicationError::TooManyRequests(
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
impl fmt::Display for DeleteApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationError::BadRequest(ref cause) => cause,
            DeleteApplicationError::Conflict(ref cause) => cause,
            DeleteApplicationError::Forbidden(ref cause) => cause,
            DeleteApplicationError::InternalServerError(ref cause) => cause,
            DeleteApplicationError::NotFound(ref cause) => cause,
            DeleteApplicationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplication
#[derive(Debug, PartialEq)]
pub enum GetApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl GetApplicationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(GetApplicationError::BadRequest(String::from(
                        error_message,
                    )));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApplicationError::Forbidden(String::from(
                        error_message,
                    )));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApplicationError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApplicationError::NotFound(String::from(
                        error_message,
                    )));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApplicationError::TooManyRequests(String::from(
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
impl fmt::Display for GetApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationError::BadRequest(ref cause) => cause,
            GetApplicationError::Forbidden(ref cause) => cause,
            GetApplicationError::InternalServerError(ref cause) => cause,
            GetApplicationError::NotFound(ref cause) => cause,
            GetApplicationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplicationPolicy
#[derive(Debug, PartialEq)]
pub enum GetApplicationPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl GetApplicationPolicyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationPolicyError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(GetApplicationPolicyError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApplicationPolicyError::Forbidden(String::from(
                        error_message,
                    )));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApplicationPolicyError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApplicationPolicyError::NotFound(String::from(
                        error_message,
                    )));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApplicationPolicyError::TooManyRequests(
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
impl fmt::Display for GetApplicationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationPolicyError::BadRequest(ref cause) => cause,
            GetApplicationPolicyError::Forbidden(ref cause) => cause,
            GetApplicationPolicyError::InternalServerError(ref cause) => cause,
            GetApplicationPolicyError::NotFound(ref cause) => cause,
            GetApplicationPolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCloudFormationTemplate
#[derive(Debug, PartialEq)]
pub enum GetCloudFormationTemplateError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl GetCloudFormationTemplateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCloudFormationTemplateError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::Forbidden(
                        String::from(error_message),
                    ));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetCloudFormationTemplateError::InternalServerError(String::from(
                            error_message,
                        )),
                    );
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::NotFound(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCloudFormationTemplateError::TooManyRequests(
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
impl fmt::Display for GetCloudFormationTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCloudFormationTemplateError {
    fn description(&self) -> &str {
        match *self {
            GetCloudFormationTemplateError::BadRequest(ref cause) => cause,
            GetCloudFormationTemplateError::Forbidden(ref cause) => cause,
            GetCloudFormationTemplateError::InternalServerError(ref cause) => cause,
            GetCloudFormationTemplateError::NotFound(ref cause) => cause,
            GetCloudFormationTemplateError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplicationDependencies
#[derive(Debug, PartialEq)]
pub enum ListApplicationDependenciesError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl ListApplicationDependenciesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListApplicationDependenciesError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::Forbidden(
                        String::from(error_message),
                    ));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListApplicationDependenciesError::InternalServerError(String::from(
                            error_message,
                        )),
                    );
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::NotFound(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListApplicationDependenciesError::TooManyRequests(
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
impl fmt::Display for ListApplicationDependenciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationDependenciesError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationDependenciesError::BadRequest(ref cause) => cause,
            ListApplicationDependenciesError::Forbidden(ref cause) => cause,
            ListApplicationDependenciesError::InternalServerError(ref cause) => cause,
            ListApplicationDependenciesError::NotFound(ref cause) => cause,
            ListApplicationDependenciesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplicationVersions
#[derive(Debug, PartialEq)]
pub enum ListApplicationVersionsError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl ListApplicationVersionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationVersionsError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(ListApplicationVersionsError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListApplicationVersionsError::Forbidden(
                        String::from(error_message),
                    ));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListApplicationVersionsError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListApplicationVersionsError::NotFound(
                        String::from(error_message),
                    ));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListApplicationVersionsError::TooManyRequests(
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
impl fmt::Display for ListApplicationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationVersionsError::BadRequest(ref cause) => cause,
            ListApplicationVersionsError::Forbidden(ref cause) => cause,
            ListApplicationVersionsError::InternalServerError(ref cause) => cause,
            ListApplicationVersionsError::NotFound(ref cause) => cause,
            ListApplicationVersionsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
}

impl ListApplicationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(ListApplicationsError::BadRequest(String::from(
                        error_message,
                    )));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListApplicationsError::Forbidden(String::from(
                        error_message,
                    )));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListApplicationsError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListApplicationsError::NotFound(String::from(
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
impl fmt::Display for ListApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationsError::BadRequest(ref cause) => cause,
            ListApplicationsError::Forbidden(ref cause) => cause,
            ListApplicationsError::InternalServerError(ref cause) => cause,
            ListApplicationsError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutApplicationPolicy
#[derive(Debug, PartialEq)]
pub enum PutApplicationPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl PutApplicationPolicyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutApplicationPolicyError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(PutApplicationPolicyError::BadRequest(
                        String::from(error_message),
                    ));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutApplicationPolicyError::Forbidden(String::from(
                        error_message,
                    )));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PutApplicationPolicyError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutApplicationPolicyError::NotFound(String::from(
                        error_message,
                    )));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutApplicationPolicyError::TooManyRequests(
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
impl fmt::Display for PutApplicationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutApplicationPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutApplicationPolicyError::BadRequest(ref cause) => cause,
            PutApplicationPolicyError::Forbidden(ref cause) => cause,
            PutApplicationPolicyError::InternalServerError(ref cause) => cause,
            PutApplicationPolicyError::NotFound(ref cause) => cause,
            PutApplicationPolicyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request doesn't exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
    TooManyRequests(String),
}

impl UpdateApplicationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
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
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApplicationError::BadRequest(String::from(
                        error_message,
                    )));
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateApplicationError::Conflict(String::from(
                        error_message,
                    )));
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateApplicationError::Forbidden(String::from(
                        error_message,
                    )));
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateApplicationError::InternalServerError(
                        String::from(error_message),
                    ));
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApplicationError::NotFound(String::from(
                        error_message,
                    )));
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApplicationError::TooManyRequests(
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
impl fmt::Display for UpdateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationError::BadRequest(ref cause) => cause,
            UpdateApplicationError::Conflict(ref cause) => cause,
            UpdateApplicationError::Forbidden(ref cause) => cause,
            UpdateApplicationError::InternalServerError(ref cause) => cause,
            UpdateApplicationError::NotFound(ref cause) => cause,
            UpdateApplicationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSServerlessApplicationRepository API. AWSServerlessApplicationRepository clients implement this trait.
pub trait ServerlessRepo {
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError>;

    /// <p>Creates an application version.</p>
    fn create_application_version(
        &self,
        input: CreateApplicationVersionRequest,
    ) -> RusotoFuture<CreateApplicationVersionResponse, CreateApplicationVersionError>;

    /// <p>Creates an AWS CloudFormation change set for the given application.</p>
    fn create_cloud_formation_change_set(
        &self,
        input: CreateCloudFormationChangeSetRequest,
    ) -> RusotoFuture<CreateCloudFormationChangeSetResponse, CreateCloudFormationChangeSetError>;

    /// <p>Creates an AWS CloudFormation template.</p>
    fn create_cloud_formation_template(
        &self,
        input: CreateCloudFormationTemplateRequest,
    ) -> RusotoFuture<CreateCloudFormationTemplateResponse, CreateCloudFormationTemplateError>;

    /// <p>Deletes the specified application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> RusotoFuture<(), DeleteApplicationError>;

    /// <p>Gets the specified application.</p>
    fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> RusotoFuture<GetApplicationResponse, GetApplicationError>;

    /// <p>Retrieves the policy for the application.</p>
    fn get_application_policy(
        &self,
        input: GetApplicationPolicyRequest,
    ) -> RusotoFuture<GetApplicationPolicyResponse, GetApplicationPolicyError>;

    /// <p>Gets the specified AWS CloudFormation template.</p>
    fn get_cloud_formation_template(
        &self,
        input: GetCloudFormationTemplateRequest,
    ) -> RusotoFuture<GetCloudFormationTemplateResponse, GetCloudFormationTemplateError>;

    /// <p>Retrieves the list of applications nested in the containing application.</p>
    fn list_application_dependencies(
        &self,
        input: ListApplicationDependenciesRequest,
    ) -> RusotoFuture<ListApplicationDependenciesResponse, ListApplicationDependenciesError>;

    /// <p>Lists versions for the specified application.</p>
    fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> RusotoFuture<ListApplicationVersionsResponse, ListApplicationVersionsError>;

    /// <p>Lists applications owned by the requester.</p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError>;

    /// <p>Sets the permission policy for an application. For the list of actions supported for this operation, see
    /// <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application
    /// Permissions</a>
    /// .</p>
    fn put_application_policy(
        &self,
        input: PutApplicationPolicyRequest,
    ) -> RusotoFuture<PutApplicationPolicyResponse, PutApplicationPolicyError>;

    /// <p>Updates the specified application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError>;
}
/// A client for the AWSServerlessApplicationRepository API.
#[derive(Clone)]
pub struct ServerlessRepoClient {
    client: Client,
    region: region::Region,
}

impl ServerlessRepoClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServerlessRepoClient {
        ServerlessRepoClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServerlessRepoClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ServerlessRepoClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ServerlessRepo for ServerlessRepoClient {
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateApplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an application version.</p>
    fn create_application_version(
        &self,
        input: CreateApplicationVersionRequest,
    ) -> RusotoFuture<CreateApplicationVersionResponse, CreateApplicationVersionError> {
        let request_uri = format!(
            "/applications/{application_id}/versions/{semantic_version}",
            application_id = input.application_id,
            semantic_version = input.semantic_version
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateApplicationVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an AWS CloudFormation change set for the given application.</p>
    fn create_cloud_formation_change_set(
        &self,
        input: CreateCloudFormationChangeSetRequest,
    ) -> RusotoFuture<CreateCloudFormationChangeSetResponse, CreateCloudFormationChangeSetError>
    {
        let request_uri = format!(
            "/applications/{application_id}/changesets",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateCloudFormationChangeSetResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCloudFormationChangeSetError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an AWS CloudFormation template.</p>
    fn create_cloud_formation_template(
        &self,
        input: CreateCloudFormationTemplateRequest,
    ) -> RusotoFuture<CreateCloudFormationTemplateResponse, CreateCloudFormationTemplateError> {
        let request_uri = format!(
            "/applications/{application_id}/templates",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateCloudFormationTemplateResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCloudFormationTemplateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> RusotoFuture<(), DeleteApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the specified application.</p>
    fn get_application(
        &self,
        input: GetApplicationRequest,
    ) -> RusotoFuture<GetApplicationResponse, GetApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.semantic_version {
            params.put("semanticVersion", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the policy for the application.</p>
    fn get_application_policy(
        &self,
        input: GetApplicationPolicyRequest,
    ) -> RusotoFuture<GetApplicationPolicyResponse, GetApplicationPolicyError> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetApplicationPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetApplicationPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the specified AWS CloudFormation template.</p>
    fn get_cloud_formation_template(
        &self,
        input: GetCloudFormationTemplateRequest,
    ) -> RusotoFuture<GetCloudFormationTemplateResponse, GetCloudFormationTemplateError> {
        let request_uri = format!(
            "/applications/{application_id}/templates/{template_id}",
            application_id = input.application_id,
            template_id = input.template_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetCloudFormationTemplateResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCloudFormationTemplateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the list of applications nested in the containing application.</p>
    fn list_application_dependencies(
        &self,
        input: ListApplicationDependenciesRequest,
    ) -> RusotoFuture<ListApplicationDependenciesResponse, ListApplicationDependenciesError> {
        let request_uri = format!(
            "/applications/{application_id}/dependencies",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxItems", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.semantic_version {
            params.put("semanticVersion", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListApplicationDependenciesResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationDependenciesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists versions for the specified application.</p>
    fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> RusotoFuture<ListApplicationVersionsResponse, ListApplicationVersionsError> {
        let request_uri = format!(
            "/applications/{application_id}/versions",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxItems", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListApplicationVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists applications owned by the requester.</p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxItems", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListApplicationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListApplicationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the permission policy for an application. For the list of actions supported for this operation, see
    /// <a href="https://docs.aws.amazon.com/serverlessrepo/latest/devguide/access-control-resource-based.html#application-permissions">Application
    /// Permissions</a>
    /// .</p>
    fn put_application_policy(
        &self,
        input: PutApplicationPolicyRequest,
    ) -> RusotoFuture<PutApplicationPolicyResponse, PutApplicationPolicyError> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutApplicationPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutApplicationPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the specified application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PATCH", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApplicationError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
