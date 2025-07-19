#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

mod jsonrpc;

use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::jsonrpc::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum JsonRpcMessage {
	Request(JsonRpcRequest<A2aRequest>),
	Response(JsonRpcResponse<A2aResponse>),
	Error(JsonRpcError),
}

impl JsonRpcMessage {
	pub fn response(&self) -> Option<&A2aResponse> {
		match self {
			JsonRpcMessage::Request(_) => None,
			JsonRpcMessage::Response(resp) => Some(&resp.result),
			JsonRpcMessage::Error(_) => None,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRpcError {
	pub jsonrpc: JsonRpcVersion2_0,
	pub id: RequestId,
	pub error: ErrorData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorCode(pub i32);

/// Error information for JSON-RPC error responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorData {
	/// The error type that occurred.
	pub code: ErrorCode,

	/// A short description of the error. The message SHOULD be limited to a concise single sentence.
	pub message: Cow<'static, str>,

	/// Additional information about the error. The value of this member is defined by the
	/// sender (e.g. detailed error information, nested errors etc.).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<serde_json::Value>,
}

// TODO: this is not complete, add the rest
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum A2aRequest {
	// Legacy request types (deprecated but kept for backward compatibility)
	SendTaskRequest(SendTaskRequest),
	SendSubscribeTaskRequest(SendSubscribeTaskRequest),
	TaskPushNotificationGetRequest(TaskPushNotificationGetRequest),
	TaskPushNotificationSetRequest(TaskPushNotificationSetRequest),
	TaskResubscribeRequest(TaskResubscribeRequest),

	// New request types (current version)
	SendMessageRequest(SendMessageRequest),
	SendStreamingMessageRequest(SendStreamingMessageRequest),
	GetTaskRequest(GetTaskRequest),
	CancelTaskRequest(CancelTaskRequest),
	SetTaskPushNotificationConfigRequest(SetTaskPushNotificationConfigRequest),
	GetTaskPushNotificationConfigRequest(GetTaskPushNotificationConfigRequest),
	TaskResubscriptionRequest(TaskResubscriptionRequest),
	ListTaskPushNotificationConfigRequest(ListTaskPushNotificationConfigRequest),
	DeleteTaskPushNotificationConfigRequest(DeleteTaskPushNotificationConfigRequest),
}

impl A2aRequest {
	pub fn method(&self) -> &'static str {
		match self {
			A2aRequest::SendTaskRequest(i) => i.method.as_string(),
			A2aRequest::SendSubscribeTaskRequest(i) => i.method.as_string(),
			A2aRequest::GetTaskRequest(i) => i.method.as_string(),
			A2aRequest::CancelTaskRequest(i) => i.method.as_string(),
			A2aRequest::TaskPushNotificationGetRequest(i) => i.method.as_string(),
			A2aRequest::TaskPushNotificationSetRequest(i) => i.method.as_string(),
			A2aRequest::TaskResubscribeRequest(i) => i.method.as_string(),
			A2aRequest::SendMessageRequest(i) => i.method.as_string(),
			A2aRequest::SendStreamingMessageRequest(i) => i.method.as_string(),
			A2aRequest::SetTaskPushNotificationConfigRequest(i) => i.method.as_string(),
			A2aRequest::GetTaskPushNotificationConfigRequest(i) => i.method.as_string(),
			A2aRequest::TaskResubscriptionRequest(i) => i.method.as_string(),
			A2aRequest::ListTaskPushNotificationConfigRequest(i) => i.method.as_string(),
			A2aRequest::DeleteTaskPushNotificationConfigRequest(i) => i.method.as_string(),
		}
	}
}

// TODO: this is not complete, add the rest
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum A2aResponse {
	// Legacy response types (deprecated but kept for backward compatibility)
	SendTaskResponse(Option<Task>),
	SendTaskUpdateResponse(SendTaskStreamingResponseResult),

	// New response types (current version)
	SendMessageResponse(SendMessageResponse),
	SendStreamingMessageResponse(SendStreamingMessageResponse),
	GetTaskResponse(GetTaskResponse),
	CancelTaskResponse(CancelTaskResponse),
	SetTaskPushNotificationConfigResponse(SetTaskPushNotificationConfigResponse),
	GetTaskPushNotificationConfigResponse(GetTaskPushNotificationConfigResponse),
	TaskResubscriptionResponse(TaskResubscriptionResponse),
	ListTaskPushNotificationConfigResponse(ListTaskPushNotificationConfigResponse),
	DeleteTaskPushNotificationConfigResponse(DeleteTaskPushNotificationConfigResponse),
}

impl A2aResponse {
	pub fn id(&self) -> Option<String> {
		match self {
			A2aResponse::SendTaskResponse(i) => i.as_ref().map(|i| i.id.clone()),
			A2aResponse::SendTaskUpdateResponse(SendTaskStreamingResponseResult::Status(i)) => {
				Some(i.id.clone())
			},
			A2aResponse::SendTaskUpdateResponse(SendTaskStreamingResponseResult::Artifact(i)) => {
				Some(i.id.clone())
			},
			A2aResponse::SendTaskUpdateResponse(SendTaskStreamingResponseResult::None) => None,
			A2aResponse::SendMessageResponse(i) => Some(i.id.to_string()),
			A2aResponse::SendStreamingMessageResponse(i) => Some(i.id.to_string()),
			A2aResponse::GetTaskResponse(i) => Some(i.id.to_string()),
			A2aResponse::CancelTaskResponse(i) => Some(i.id.to_string()),
			A2aResponse::SetTaskPushNotificationConfigResponse(i) => Some(i.id.to_string()),
			A2aResponse::GetTaskPushNotificationConfigResponse(i) => Some(i.id.to_string()),
			A2aResponse::TaskResubscriptionResponse(i) => Some(i.id.to_string()),
			A2aResponse::ListTaskPushNotificationConfigResponse(i) => Some(i.id.to_string()),
			A2aResponse::DeleteTaskPushNotificationConfigResponse(i) => Some(i.id.to_string()),
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AgentAuthentication {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub credentials: Option<String>,
	pub schemes: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct AgentCapabilities {
	#[serde(
		rename = "pushNotifications",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub push_notifications: Option<bool>,
	#[serde(
		rename = "stateTransitionHistory",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub state_transition_history: Option<bool>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub streaming: Option<bool>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub extensions: Vec<AgentExtension>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AgentCard {
	// Legacy fields (deprecated but kept for backward compatibility)
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub authentication: Option<AgentAuthentication>,

	// Current fields
	#[serde(
		rename = "additionalInterfaces",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	pub additional_interfaces: Vec<AgentInterface>,
	pub capabilities: AgentCapabilities,
	#[serde(rename = "defaultInputModes")]
	pub default_input_modes: Vec<String>,
	#[serde(rename = "defaultOutputModes")]
	pub default_output_modes: Vec<String>,
	pub description: String,
	#[serde(
		rename = "documentationUrl",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub documentation_url: Option<String>,
	#[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
	pub icon_url: Option<String>,
	pub name: String,
	#[serde(
		rename = "preferredTransport",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub preferred_transport: Option<String>,
	#[serde(rename = "protocolVersion")]
	pub protocol_version: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub provider: Option<AgentProvider>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub security: Vec<std::collections::HashMap<String, Vec<String>>>,
	#[serde(
		rename = "securitySchemes",
		default,
		skip_serializing_if = "std::collections::HashMap::is_empty"
	)]
	pub security_schemes: std::collections::HashMap<String, SecurityScheme>,
	pub skills: Vec<AgentSkill>,
	#[serde(
		rename = "supportsAuthenticatedExtendedCard",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub supports_authenticated_extended_card: Option<bool>,
	pub url: String,
	pub version: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AgentProvider {
	pub organization: String,
	pub url: String,
}

// New types for current schema
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AgentInterface {
	pub transport: String,
	pub url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AgentExtension {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
	pub params: serde_json::Map<String, serde_json::Value>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub required: Option<bool>,
	pub uri: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum SecurityScheme {
	#[serde(rename = "apiKey")]
	ApiKey(ApiKeySecurityScheme),
	#[serde(rename = "http")]
	HttpAuth(HttpAuthSecurityScheme),
	#[serde(rename = "oauth2")]
	OAuth2(Box<OAuth2SecurityScheme>),
	#[serde(rename = "openIdConnect")]
	OpenIdConnect(OpenIdConnectSecurityScheme),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ApiKeySecurityScheme {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(rename = "in")]
	pub in_: ApiKeySecuritySchemeIn,
	pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ApiKeySecuritySchemeIn {
	#[serde(rename = "cookie")]
	Cookie,
	#[serde(rename = "header")]
	Header,
	#[serde(rename = "query")]
	Query,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct HttpAuthSecurityScheme {
	#[serde(
		rename = "bearerFormat",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub bearer_format: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	pub scheme: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct OAuth2SecurityScheme {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	pub flows: OAuthFlows,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct OAuthFlows {
	#[serde(
		rename = "authorizationCode",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub authorization_code: Option<AuthorizationCodeOAuthFlow>,
	#[serde(
		rename = "clientCredentials",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub client_credentials: Option<ClientCredentialsOAuthFlow>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub implicit: Option<ImplicitOAuthFlow>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub password: Option<PasswordOAuthFlow>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AuthorizationCodeOAuthFlow {
	#[serde(rename = "authorizationUrl")]
	pub authorization_url: String,
	#[serde(
		rename = "refreshUrl",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub refresh_url: Option<String>,
	pub scopes: std::collections::HashMap<String, String>,
	#[serde(rename = "tokenUrl")]
	pub token_url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ClientCredentialsOAuthFlow {
	#[serde(
		rename = "refreshUrl",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub refresh_url: Option<String>,
	pub scopes: std::collections::HashMap<String, String>,
	#[serde(rename = "tokenUrl")]
	pub token_url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ImplicitOAuthFlow {
	#[serde(rename = "authorizationUrl")]
	pub authorization_url: String,
	#[serde(
		rename = "refreshUrl",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub refresh_url: Option<String>,
	pub scopes: std::collections::HashMap<String, String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PasswordOAuthFlow {
	#[serde(
		rename = "refreshUrl",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub refresh_url: Option<String>,
	pub scopes: std::collections::HashMap<String, String>,
	#[serde(rename = "tokenUrl")]
	pub token_url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct OpenIdConnectSecurityScheme {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(rename = "openIdConnectUrl")]
	pub open_id_connect_url: String,
	#[serde(rename = "type")]
	pub type_: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AgentSkill {
	// Current fields (new schema)
	pub id: String,
	pub name: String,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub description: String, // Now required in new schema
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub examples: Vec<String>,
	#[serde(rename = "inputModes", default, skip_serializing_if = "Vec::is_empty")]
	pub input_modes: Vec<String>,
	#[serde(rename = "outputModes", default, skip_serializing_if = "Vec::is_empty")]
	pub output_modes: Vec<String>,
	pub tags: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Artifact {
	#[serde(rename = "artifactId")]
	pub artifact_id: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub extensions: Vec<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	pub parts: Vec<Part>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AuthenticationInfo {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub credentials: Option<String>,
	pub schemes: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DataPart {
	pub data: serde_json::Map<String, serde_json::Value>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FileContent {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub bytes: Option<String>,
	#[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
	pub mime_type: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub uri: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FilePart {
	pub file: FileContent,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Id {
	Variant0(i64),
	Variant1(String),
	Variant2,
}
impl From<i64> for Id {
	fn from(value: i64) -> Self {
		Self::Variant0(value)
	}
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct InternalError {
	pub code: i64,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub data: Option<serde_json::Value>,
	pub message: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct InvalidParamsError {
	pub code: i64,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub data: Option<serde_json::Value>,
	pub message: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct InvalidRequestError {
	pub code: i64,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub data: Option<serde_json::Value>,
	pub message: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Message {
	// Legacy fields (deprecated but kept for backward compatibility)
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub content_legacy: Option<Vec<Part>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub role_legacy: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub timestamp_legacy: Option<chrono::NaiveDateTime>,

	// Current fields (new schema)
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub content: Vec<Part>,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub role: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<String>, // Changed from chrono::NaiveDateTime to String in new schema
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MethodNotFoundError {
	pub code: i64,
	pub data: serde_json::Value,
	pub message: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum Part {
	Text(TextPart),
	File(FilePart),
	Data(DataPart),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PushNotificationConfig {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub authentication: Option<AuthenticationInfo>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
	pub url: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PushNotificationNotSupportedError {
	pub code: i64,
	pub data: serde_json::Value,
	pub message: String,
}
#[derive(
	serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum Role {
	#[serde(rename = "user")]
	User,
	#[serde(rename = "agent")]
	Agent,
}
impl Display for Role {
	fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
		match *self {
			Self::User => write!(f, "user"),
			Self::Agent => write!(f, "agent"),
		}
	}
}
impl ::std::str::FromStr for Role {
	type Err = self::error::ConversionError;
	fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
		match value {
			"user" => Ok(Self::User),
			"agent" => Ok(Self::Agent),
			_ => Err("invalid value".into()),
		}
	}
}
impl TryFrom<&str> for Role {
	type Error = self::error::ConversionError;
	fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
		value.parse()
	}
}
impl TryFrom<&String> for Role {
	type Error = self::error::ConversionError;
	fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
		value.parse()
	}
}
impl TryFrom<String> for Role {
	type Error = self::error::ConversionError;
	fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
		value.parse()
	}
}

const_string!(CancelTaskRequestMethod = "tasks/cancel");
pub type CancelTaskRequest = Request<CancelTaskRequestMethod, TaskIdParams>;

const_string!(GetTaskRequestMethod = "tasks/get");
pub type GetTaskRequest = Request<GetTaskRequestMethod, TaskQueryParams>;

const_string!(SendTaskRequestMethod = "tasks/send");
pub type SendTaskRequest = Request<SendTaskRequestMethod, TaskSendParams>;

const_string!(SendSubscribeTaskRequestMethod = "tasks/sendSubscribe");
pub type SendSubscribeTaskRequest = Request<SendSubscribeTaskRequestMethod, TaskSendParams>;

const_string!(TaskResubscribeRequestMethod = "tasks/resubscribe");
pub type TaskResubscribeRequest = Request<TaskResubscribeRequestMethod, TaskQueryParams>;

const_string!(TaskPushNotificationGetRequestMethod = "tasks/pushNotification/get");
pub type TaskPushNotificationGetRequest =
	Request<TaskPushNotificationGetRequestMethod, TaskIdParams>;

const_string!(TaskPushNotificationSetRequestMethod = "tasks/pushNotification/set");
pub type TaskPushNotificationSetRequest =
	Request<TaskPushNotificationSetRequestMethod, TaskPushNotificationConfig>;

// New request types (current version)
const_string!(SendMessageRequestMethod = "message/send");
pub type SendMessageRequest = Request<SendMessageRequestMethod, MessageSendParams>;

const_string!(SendStreamingMessageRequestMethod = "message/stream");
pub type SendStreamingMessageRequest =
	Request<SendStreamingMessageRequestMethod, MessageSendParams>;

const_string!(SetTaskPushNotificationConfigRequestMethod = "tasks/pushNotificationConfig/set");
pub type SetTaskPushNotificationConfigRequest =
	Request<SetTaskPushNotificationConfigRequestMethod, TaskPushNotificationConfig>;

const_string!(GetTaskPushNotificationConfigRequestMethod = "tasks/pushNotificationConfig/get");
pub type GetTaskPushNotificationConfigRequest =
	Request<GetTaskPushNotificationConfigRequestMethod, TaskIdParams>;

const_string!(TaskResubscriptionRequestMethod = "tasks/resubscribe");
pub type TaskResubscriptionRequest = Request<TaskResubscriptionRequestMethod, TaskIdParams>;

const_string!(ListTaskPushNotificationConfigRequestMethod = "tasks/pushNotificationConfig/list");
pub type ListTaskPushNotificationConfigRequest =
	Request<ListTaskPushNotificationConfigRequestMethod, TaskIdParams>;

const_string!(
	DeleteTaskPushNotificationConfigRequestMethod = "tasks/pushNotificationConfig/delete"
);
pub type DeleteTaskPushNotificationConfigRequest =
	Request<DeleteTaskPushNotificationConfigRequestMethod, TaskIdParams>;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[derive(Default)]
pub enum SendTaskStreamingResponseResult {
	Status(TaskStatusUpdateEvent),
	Artifact(TaskArtifactUpdateEvent),
	#[default]
	None,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Task {
	// Legacy fields (deprecated but kept for backward compatibility)
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub artifacts_legacy: Option<Vec<Artifact>>,
	#[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub history_legacy: Option<Vec<Message>>,

	// Current fields (new schema)
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub artifacts: Vec<Artifact>,
	#[serde(rename = "contextId")]
	pub context_id: String,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub history: Vec<Message>,
	pub id: String,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub kind: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
	pub status: TaskStatus,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskArtifactUpdateEvent {
	pub artifact: Artifact,
	pub id: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskIdParams {
	pub id: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskNotCancelableError {
	pub code: i64,
	pub data: serde_json::Value,
	pub message: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskNotFoundError {
	pub code: i64,
	pub data: serde_json::Value,
	pub message: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskPushNotificationConfig {
	pub id: String,
	#[serde(rename = "pushNotificationConfig")]
	pub push_notification_config: PushNotificationConfig,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskQueryParams {
	#[serde(
		rename = "historyLength",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub history_length: Option<i64>,
	pub id: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskSendParams {
	#[serde(
		rename = "historyLength",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub history_length: Option<i64>,
	pub id: String,
	pub message: Message,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
	#[serde(
		rename = "pushNotification",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub push_notification: Option<PushNotificationConfig>,
	#[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MessageSendParams {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub configuration: Option<MessageSendConfiguration>,
	pub message: Message,
	#[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
	pub metadata: serde_json::Map<String, serde_json::Value>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MessageSendConfiguration {
	#[serde(rename = "acceptedOutputModes")]
	pub accepted_output_modes: Vec<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub blocking: Option<bool>,
	#[serde(
		rename = "historyLength",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub history_length: Option<i64>,
	#[serde(
		rename = "pushNotificationConfig",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub push_notification_config: Option<PushNotificationConfig>,
}

#[derive(
	serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum TaskState {
	#[serde(rename = "submitted")]
	Submitted,
	#[serde(rename = "working")]
	Working,
	#[serde(rename = "input-required")]
	InputRequired,
	#[serde(rename = "completed")]
	Completed,
	#[serde(rename = "canceled")]
	Canceled,
	#[serde(rename = "failed")]
	Failed,
	#[serde(rename = "unknown")]
	Unknown,
	// New states from current schema
	#[serde(rename = "rejected")]
	Rejected,
	#[serde(rename = "auth-required")]
	AuthRequired,
}
impl Display for TaskState {
	fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
		match *self {
			Self::Submitted => write!(f, "submitted"),
			Self::Working => write!(f, "working"),
			Self::InputRequired => write!(f, "input-required"),
			Self::Completed => write!(f, "completed"),
			Self::Canceled => write!(f, "canceled"),
			Self::Failed => write!(f, "failed"),
			Self::Unknown => write!(f, "unknown"),
			Self::Rejected => write!(f, "rejected"),
			Self::AuthRequired => write!(f, "auth-required"),
		}
	}
}
impl ::std::str::FromStr for TaskState {
	type Err = self::error::ConversionError;
	fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
		match value {
			"submitted" => Ok(Self::Submitted),
			"working" => Ok(Self::Working),
			"input-required" => Ok(Self::InputRequired),
			"completed" => Ok(Self::Completed),
			"canceled" => Ok(Self::Canceled),
			"failed" => Ok(Self::Failed),
			"unknown" => Ok(Self::Unknown),
			"rejected" => Ok(Self::Rejected),
			"auth-required" => Ok(Self::AuthRequired),
			_ => Err("invalid value".into()),
		}
	}
}
impl TryFrom<&str> for TaskState {
	type Error = self::error::ConversionError;
	fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
		value.parse()
	}
}
impl TryFrom<&String> for TaskState {
	type Error = self::error::ConversionError;
	fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
		value.parse()
	}
}
impl TryFrom<String> for TaskState {
	type Error = self::error::ConversionError;
	fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
		value.parse()
	}
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskStatus {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub message: Option<Message>,
	pub state: TaskState,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskStatusUpdateEvent {
	#[serde(rename = "final", default)]
	pub final_: bool,
	pub id: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
	pub status: TaskStatus,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TextPart {
	pub text: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UnsupportedOperationError {
	pub code: i64,
	pub data: serde_json::Value,
	pub message: String,
}

pub mod error {
	use std::fmt::{Debug, Display, Formatter};
	pub struct ConversionError(::std::borrow::Cow<'static, str>);
	impl ::std::error::Error for ConversionError {}
	impl Display for ConversionError {
		fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::std::fmt::Error> {
			Display::fmt(&self.0, f)
		}
	}
	impl Debug for ConversionError {
		fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::std::fmt::Error> {
			Debug::fmt(&self.0, f)
		}
	}
	impl From<&'static str> for ConversionError {
		fn from(value: &'static str) -> Self {
			Self(value.into())
		}
	}
	impl From<String> for ConversionError {
		fn from(value: String) -> Self {
			Self(value.into())
		}
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn test_serde() {
		let js = serde_json::json! {
		{
			"jsonrpc": "2.0",
			"id": "d1306567eb364c7ba9e7a7b922dba672",
			"result": {
				"id": "8b34914c735a464986e1d5ce5b6ec478",
				"status": {
					"state": "completed",
					"message": {
						"role": "agent",
						"parts": [
							{
								"type": "text",
								"text": "Hello!"
							}
						]
					},
					"timestamp": "2025-04-10T15:07:15.833777"
				},
				"final": false
			}
		}
		};
		let _: crate::JsonRpcMessage = serde_json::from_value(js).unwrap();
	}
}

// New response types (current version)
pub type SendMessageResponse = JsonRpcResponse<SendMessageSuccessResponse>;
pub type SendStreamingMessageResponse = JsonRpcResponse<SendStreamingMessageSuccessResponse>;
pub type GetTaskResponse = JsonRpcResponse<GetTaskSuccessResponse>;
pub type CancelTaskResponse = JsonRpcResponse<CancelTaskSuccessResponse>;
pub type SetTaskPushNotificationConfigResponse =
	JsonRpcResponse<SetTaskPushNotificationConfigSuccessResponse>;
pub type GetTaskPushNotificationConfigResponse =
	JsonRpcResponse<GetTaskPushNotificationConfigSuccessResponse>;
pub type TaskResubscriptionResponse = JsonRpcResponse<TaskResubscriptionSuccessResponse>;
pub type ListTaskPushNotificationConfigResponse =
	JsonRpcResponse<ListTaskPushNotificationConfigSuccessResponse>;
pub type DeleteTaskPushNotificationConfigResponse =
	JsonRpcResponse<DeleteTaskPushNotificationConfigSuccessResponse>;

// Success response types
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SendMessageSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: SendMessageSuccessResponseResult,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SendStreamingMessageSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: SendStreamingMessageSuccessResponseResult,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GetTaskSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: Task,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CancelTaskSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: Task,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SetTaskPushNotificationConfigSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: TaskPushNotificationConfig,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GetTaskPushNotificationConfigSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: TaskPushNotificationConfig,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskResubscriptionSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: (),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ListTaskPushNotificationConfigSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: Vec<TaskPushNotificationConfig>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DeleteTaskPushNotificationConfigSuccessResponse {
	pub id: NumberOrString,
	pub jsonrpc: String,
	pub result: (),
}

// Result types for success responses
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendMessageSuccessResponseResult {
	Task(Box<Task>),
	Message(Message),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendStreamingMessageSuccessResponseResult {
	Task(Task),
	Message(Message),
	TaskStatusUpdateEvent(TaskStatusUpdateEvent),
	TaskArtifactUpdateEvent(TaskArtifactUpdateEvent),
}
