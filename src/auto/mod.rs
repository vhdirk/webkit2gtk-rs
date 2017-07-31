// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

#[cfg(feature = "v2_2")]
mod authentication_request;
#[cfg(feature = "v2_2")]
pub use self::authentication_request::AuthenticationRequest;
#[cfg(feature = "v2_2")]
pub use self::authentication_request::AuthenticationRequestExt;

mod back_forward_list;
pub use self::back_forward_list::BackForwardList;
pub use self::back_forward_list::BackForwardListExt;

mod back_forward_list_item;
pub use self::back_forward_list_item::BackForwardListItem;
pub use self::back_forward_list_item::BackForwardListItemExt;

#[cfg(feature = "v2_8")]
mod color_chooser_request;
#[cfg(feature = "v2_8")]
pub use self::color_chooser_request::ColorChooserRequest;
#[cfg(feature = "v2_8")]
pub use self::color_chooser_request::ColorChooserRequestExt;

mod context_menu;
pub use self::context_menu::ContextMenu;
pub use self::context_menu::ContextMenuExt;

mod context_menu_item;
pub use self::context_menu_item::ContextMenuItem;
pub use self::context_menu_item::ContextMenuItemExt;

mod cookie_manager;
pub use self::cookie_manager::CookieManager;
pub use self::cookie_manager::CookieManagerExt;

mod download;
pub use self::download::Download;
pub use self::download::DownloadExt;

#[cfg(feature = "v2_10")]
mod editor_state;
#[cfg(feature = "v2_10")]
pub use self::editor_state::EditorState;
#[cfg(feature = "v2_10")]
pub use self::editor_state::EditorStateExt;

mod favicon_database;
pub use self::favicon_database::FaviconDatabase;
pub use self::favicon_database::FaviconDatabaseExt;

mod file_chooser_request;
pub use self::file_chooser_request::FileChooserRequest;
pub use self::file_chooser_request::FileChooserRequestExt;

mod find_controller;
pub use self::find_controller::FindController;
pub use self::find_controller::FindControllerExt;

mod form_submission_request;
pub use self::form_submission_request::FormSubmissionRequest;
pub use self::form_submission_request::FormSubmissionRequestExt;

mod geolocation_permission_request;
pub use self::geolocation_permission_request::GeolocationPermissionRequest;

mod hit_test_result;
pub use self::hit_test_result::HitTestResult;
pub use self::hit_test_result::HitTestResultExt;

#[cfg(feature = "v2_10")]
mod install_missing_media_plugins_permission_request;
#[cfg(feature = "v2_10")]
pub use self::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequest;
#[cfg(feature = "v2_10")]
pub use self::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequestExt;

mod navigation_policy_decision;
pub use self::navigation_policy_decision::NavigationPolicyDecision;
pub use self::navigation_policy_decision::NavigationPolicyDecisionExt;

#[cfg(feature = "v2_8")]
mod notification;
#[cfg(feature = "v2_8")]
pub use self::notification::Notification;
#[cfg(feature = "v2_8")]
pub use self::notification::NotificationExt;

mod notification_permission_request;
pub use self::notification_permission_request::NotificationPermissionRequest;

mod permission_request;
pub use self::permission_request::PermissionRequest;
pub use self::permission_request::PermissionRequestExt;

mod plugin;
pub use self::plugin::Plugin;
pub use self::plugin::PluginExt;

mod policy_decision;
pub use self::policy_decision::PolicyDecision;
pub use self::policy_decision::PolicyDecisionExt;

mod print_operation;
pub use self::print_operation::PrintOperation;
pub use self::print_operation::PrintOperationExt;

mod response_policy_decision;
pub use self::response_policy_decision::ResponsePolicyDecision;
pub use self::response_policy_decision::ResponsePolicyDecisionExt;

mod security_manager;
pub use self::security_manager::SecurityManager;
pub use self::security_manager::SecurityManagerExt;

mod settings;
pub use self::settings::Settings;
pub use self::settings::SettingsExt;

mod uri_request;
pub use self::uri_request::URIRequest;
pub use self::uri_request::URIRequestExt;

mod uri_response;
pub use self::uri_response::URIResponse;
pub use self::uri_response::URIResponseExt;

mod uri_scheme_request;
pub use self::uri_scheme_request::URISchemeRequest;
pub use self::uri_scheme_request::URISchemeRequestExt;

#[cfg(feature = "v2_6")]
mod user_content_manager;
#[cfg(feature = "v2_6")]
pub use self::user_content_manager::UserContentManager;
#[cfg(feature = "v2_6")]
pub use self::user_content_manager::UserContentManagerExt;

mod user_media_permission_request;
pub use self::user_media_permission_request::UserMediaPermissionRequest;
pub use self::user_media_permission_request::UserMediaPermissionRequestExt;

mod web_context;
pub use self::web_context::WebContext;
pub use self::web_context::WebContextExt;

mod web_inspector;
pub use self::web_inspector::WebInspector;
pub use self::web_inspector::WebInspectorExt;

mod web_resource;
pub use self::web_resource::WebResource;
pub use self::web_resource::WebResourceExt;

mod web_view;
pub use self::web_view::WebView;
pub use self::web_view::WebViewExt;

#[cfg(feature = "v2_10")]
mod website_data_manager;
#[cfg(feature = "v2_10")]
pub use self::website_data_manager::WebsiteDataManager;
#[cfg(feature = "v2_10")]
pub use self::website_data_manager::WebsiteDataManagerExt;

mod window_properties;
pub use self::window_properties::WindowProperties;
pub use self::window_properties::WindowPropertiesExt;

mod javascript_result;
pub use self::javascript_result::JavascriptResult;

#[cfg(feature = "v2_6")]
mod navigation_action;
#[cfg(feature = "v2_6")]
pub use self::navigation_action::NavigationAction;

#[cfg(feature = "v2_6")]
mod user_script;
#[cfg(feature = "v2_6")]
pub use self::user_script::UserScript;

#[cfg(feature = "v2_6")]
mod user_style_sheet;
#[cfg(feature = "v2_6")]
pub use self::user_style_sheet::UserStyleSheet;

mod enums;
#[cfg(feature = "v2_2")]
pub use self::enums::AuthenticationScheme;
pub use self::enums::CacheModel;
pub use self::enums::ContextMenuAction;
pub use self::enums::CookieAcceptPolicy;
pub use self::enums::CookiePersistentStorage;
#[cfg(feature = "v2_2")]
pub use self::enums::CredentialPersistence;
pub use self::enums::DownloadError;
pub use self::enums::FaviconDatabaseError;
pub use self::enums::InsecureContentEvent;
pub use self::enums::JavascriptError;
pub use self::enums::LoadEvent;
pub use self::enums::NavigationType;
pub use self::enums::NetworkError;
pub use self::enums::PluginError;
pub use self::enums::PolicyDecisionType;
pub use self::enums::PolicyError;
pub use self::enums::PrintError;
pub use self::enums::PrintOperationResponse;
#[cfg(feature = "v2_4")]
pub use self::enums::ProcessModel;
pub use self::enums::SaveMode;
pub use self::enums::ScriptDialogType;
pub use self::enums::SnapshotError;
pub use self::enums::SnapshotRegion;
pub use self::enums::TLSErrorsPolicy;
#[cfg(feature = "v2_6")]
pub use self::enums::UserContentInjectedFrames;
#[cfg(feature = "v2_6")]
pub use self::enums::UserScriptInjectionTime;
#[cfg(feature = "v2_6")]
pub use self::enums::UserStyleLevel;

mod flags;
#[cfg(feature = "v2_10")]
pub use self::flags::EditorTypingAttributes;
#[cfg(feature = "v2_10")]
pub use self::flags::EDITOR_TYPING_ATTRIBUTE_NONE;
#[cfg(feature = "v2_10")]
pub use self::flags::EDITOR_TYPING_ATTRIBUTE_BOLD;
#[cfg(feature = "v2_10")]
pub use self::flags::EDITOR_TYPING_ATTRIBUTE_ITALIC;
#[cfg(feature = "v2_10")]
pub use self::flags::EDITOR_TYPING_ATTRIBUTE_UNDERLINE;
#[cfg(feature = "v2_10")]
pub use self::flags::EDITOR_TYPING_ATTRIBUTE_STRIKETHROUGH;
pub use self::flags::FindOptions;
pub use self::flags::FIND_OPTIONS_NONE;
pub use self::flags::FIND_OPTIONS_CASE_INSENSITIVE;
pub use self::flags::FIND_OPTIONS_AT_WORD_STARTS;
pub use self::flags::FIND_OPTIONS_TREAT_MEDIAL_CAPITAL_AS_WORD_START;
pub use self::flags::FIND_OPTIONS_BACKWARDS;
pub use self::flags::FIND_OPTIONS_WRAP_AROUND;
pub use self::flags::HitTestResultContext;
pub use self::flags::HIT_TEST_RESULT_CONTEXT_DOCUMENT;
pub use self::flags::HIT_TEST_RESULT_CONTEXT_LINK;
pub use self::flags::HIT_TEST_RESULT_CONTEXT_IMAGE;
pub use self::flags::HIT_TEST_RESULT_CONTEXT_MEDIA;
pub use self::flags::HIT_TEST_RESULT_CONTEXT_EDITABLE;
pub use self::flags::HIT_TEST_RESULT_CONTEXT_SCROLLBAR;
pub use self::flags::HIT_TEST_RESULT_CONTEXT_SELECTION;
pub use self::flags::SnapshotOptions;
pub use self::flags::SNAPSHOT_OPTIONS_NONE;
pub use self::flags::SNAPSHOT_OPTIONS_INCLUDE_SELECTION_HIGHLIGHTING;
pub use self::flags::SNAPSHOT_OPTIONS_TRANSPARENT_BACKGROUND;

#[doc(hidden)]
pub mod traits {
    #[cfg(feature = "v2_2")]
    pub use super::AuthenticationRequestExt;
    pub use super::BackForwardListExt;
    pub use super::BackForwardListItemExt;
    #[cfg(feature = "v2_8")]
    pub use super::ColorChooserRequestExt;
    pub use super::ContextMenuExt;
    pub use super::ContextMenuItemExt;
    pub use super::CookieManagerExt;
    pub use super::DownloadExt;
    #[cfg(feature = "v2_10")]
    pub use super::EditorStateExt;
    pub use super::FaviconDatabaseExt;
    pub use super::FileChooserRequestExt;
    pub use super::FindControllerExt;
    pub use super::FormSubmissionRequestExt;
    pub use super::HitTestResultExt;
    #[cfg(feature = "v2_10")]
    pub use super::InstallMissingMediaPluginsPermissionRequestExt;
    pub use super::NavigationPolicyDecisionExt;
    #[cfg(feature = "v2_8")]
    pub use super::NotificationExt;
    pub use super::PermissionRequestExt;
    pub use super::PluginExt;
    pub use super::PolicyDecisionExt;
    pub use super::PrintOperationExt;
    pub use super::ResponsePolicyDecisionExt;
    pub use super::SecurityManagerExt;
    pub use super::SettingsExt;
    pub use super::URIRequestExt;
    pub use super::URIResponseExt;
    pub use super::URISchemeRequestExt;
    #[cfg(feature = "v2_6")]
    pub use super::UserContentManagerExt;
    pub use super::UserMediaPermissionRequestExt;
    pub use super::WebContextExt;
    pub use super::WebInspectorExt;
    pub use super::WebResourceExt;
    pub use super::WebViewExt;
    #[cfg(feature = "v2_10")]
    pub use super::WebsiteDataManagerExt;
    pub use super::WindowPropertiesExt;
}
