use gloo_net::http::Request;
use patternfly_yew::prelude::{Flex, FlexItem, Spinner};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, html_nested, use_effect_with, use_state_eq, AttrValue, Html, Properties};
use yewdux::use_selector;
use common::status::ServerStatus;
use crate::app::AppState;
use crate::components::ServerStatusCard;
use crate::pages::MyPage;

#[derive(Properties, PartialEq)]
pub struct GamePageProps {
    pub game: AttrValue,
}

#[function_component(GamePage)]
pub fn game_page(props: &GamePageProps) -> Html {
    let logged_in = *use_selector(|s: &AppState| s.user_data.is_some());
    let loading = use_state_eq(|| false);
    let servers = use_state_eq(Vec::<ServerStatus>::new);
    {
        let servers = servers.clone();
        let loading = loading.clone();
        let game = props.game.clone();
        use_effect_with(logged_in, move |_| {
            if logged_in && !*loading {
                loading.set(true);
                spawn_local(async move {
                    let resp = Request::get("/api/servers/status")
                        .query([("game", game)])
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
        <MyPage title={props.game.clone()}>
            {content}
        </MyPage>
    }
}