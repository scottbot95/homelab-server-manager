use crate::app::state::AppState;
use common::user::UserData;
use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::use_selector;

#[function_component(UserActions)]
pub fn user_actions() -> Html {
    let loading = use_state_eq(|| true);
    // dependency on loading so that we will try to load again if this is set back to true
    use_effect_with(loading.clone(), |loading| {
        let loading = loading.clone();
        if !(*loading) {
            return;
        } // only try to load if loading == true
        spawn_local(async move {
            let resp = Request::get("/api/me").send().await;

            let resp = match resp {
                Ok(resp) => resp,
                Err(e) => {
                    log::error!("Failed to get user info: {}", e);
                    return;
                }
            };

            let user_data = resp.json::<Option<UserData>>().await;
            match user_data {
                #[allow(unused_variables)]
                Ok(user_data) => {
                    // user dispatcher directly so we don't re-render for all state changes
                    #[cfg(target_arch = "wasm32")]
                    yewdux::Dispatch::<AppState>::global()
                        .apply(crate::app::state::AppAction::UpdateUser(user_data.into()));
                    loading.set(false);
                }
                Err(e) => {
                    log::error!("Failed to get user info: {}", e);
                }
            }
        });
    });

    let username =
        use_selector(|state: &AppState| Option::as_ref(&state.user_data).map(|u| u.name.clone()));

    if *loading {
        return html! {
            <Spinner />
        };
    }

    let Some(username) = username.as_ref() else {
        return html! {
            <a href="/auth/discord">
                <Button variant={ButtonVariant::Primary}>
                    {"Login"}
                </Button>
            </a>
        };
    };

    html! {
        <Dropdown
            position={Position::Right}
            text={username.to_string()}
            variant={MenuToggleVariant::Plain}
        >
            <MenuLink href={"/logout"}>{"Logout"}</MenuLink>
        </Dropdown>
    }
}
