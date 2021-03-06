use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("notify_send", op_notify_send);
}

#[derive(Serialize)]
struct NotifyResponse<T> {
  err: Option<String>,
  ok: Option<T>,
}

#[derive(Deserialize)]
enum Icon {
  #[serde(rename = "app")]
  App(String),
  #[serde(rename = "path")]
  Path(String),
  #[serde(rename = "name")]
  Name(String),
}

#[derive(Deserialize)]
struct SendNotificationParams {
  title: String,
  message: String,
  icon: Option<Icon>,
  sound: Option<String>,
}

#[derive(Serialize)]
struct SendNotificationResult {}

fn op_notify_send(
  _interface: &mut dyn Interface,
  zero_copy: &mut [ZeroCopyBuf],
) -> Op {
  let mut response: NotifyResponse<SendNotificationResult> = NotifyResponse {
    err: None,
    ok: None,
  };

  let zero_copy = zero_copy.to_vec();

  let data = &zero_copy[0];

  let params: SendNotificationParams = serde_json::from_slice(data).unwrap();

  let mut notification = Notification::new();

  // Basic notification, title & message
  notification.summary(&params.title).body(&params.message);

  // Add an icon
  if let Some(icon_value) = &params.icon {
    notification.icon(match icon_value {
      // App Name
      Icon::App(app_name) => {
        // Mac needs to pretend to be another app
        if let Err(error) = set_app_identifier(app_name) {
          response.err = Some(error);
        }
        app_name
      }
      // Path to icon
      Icon::Path(file_path) => file_path,
      // Icon theme name
      Icon::Name(icon_name) => icon_name,
    });
  }

  // Add a sound
  if let Some(sound_name) = &params.sound {
    notification.sound_name(sound_name);
  }

  // TODO: When adding .wait_for_action support, convert this to a future (and return async)
  // See: https://github.com/PandawanFr/deno_notify/blob/a0ebd0f0eb9ba7c9237f165e99f420692dd7d283/src/lib.rs#L81
  match notification.show() {
    Ok(_) => {
      response.ok = Some(SendNotificationResult {});
    }
    Err(error) => {
      response.err = Some(error.to_string());
    }
  };
  
  let result: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
  
  Op::Sync(result)
}

#[cfg(not(target_os = "macos"))]
fn set_app_identifier(_app_name: &String) -> Result<(), String> {
  Ok(())
}

#[cfg(target_os = "macos")]
fn set_app_identifier(app_name: &String) -> Result<(), String> {
  use notify_rust::{get_bundle_identifier_or_default, set_application};

  let app_id = get_bundle_identifier_or_default(app_name);
  if let Err(err) = set_application(&app_id).map_err(|f| format!("{}", f)) {
    Err(err)
  } else {
    Ok(())
  }
}
