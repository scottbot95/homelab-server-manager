use std::rc::Rc;
use gloo_net::Error;
use gloo_net::http::{Request, Response};
use gloo_utils::window;
use patternfly_yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::pages::MyPage;

use yew::prelude::*;
use yewdux::use_selector;
use common::status::{HealthStatus, ServerStatus};
use crate::app::AppState;
use crate::components::ServerStatusCard;

#[function_component(Factorio)]
pub fn factorio() -> Html {
    let logged_in = *use_selector(|s: &AppState| s.user_data.is_some());
    let loading = use_state_eq(|| false);
    let servers = use_state_eq(|| Vec::<ServerStatus>::new());
    {
        let servers = servers.clone();
        let loading = loading.clone();
        use_effect_with(logged_in, move |_| {
            if logged_in && !*loading {
                loading.set(true);
                spawn_local(async move {
                    let resp = Request::get("/api/servers/status")
                        .send()
                        .await;
                    match resp {
                        Ok(resp) => {
                            let servers_resp = resp
                                .json::<Vec<ServerStatus>>()
                                .await
                                .expect("Failed to parse server status response");
                            servers.set(servers_resp);
                        }
                        Err(e) => {
                            log::error!("Error while getting server status: {e}");
                        }
                    }
                    loading.set(false);
                });
            }
        });
    }

    let content = if !logged_in {
        // window().location().assign("/auth/discord").unwrap();
        html! {
            {"Please log in to view this page"}
        }
    } else if *loading {
        html! {
            <Spinner />
        }
    } else {
        html! {
            <Flex>
                {for servers.iter().map(|status| html_nested!{
                    <FlexItem>
                        <ServerStatusCard status={status.clone()} />
                    </FlexItem>
                })}
            </Flex>
        }
    };

    html! {
        <MyPage title="Factorio">
            {content}
        </MyPage>
    }
}
