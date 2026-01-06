mod health_indicator;

use std::time::Duration;
use gloo_utils::window;
use crate::components::status::health_indicator::HealthIndicator;
use common::factorio::FactorioStatus;
use common::status::ServerStatus;
use patternfly_yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::{use_clipboard, UseClipboardHandle};
use common::generic::GenericStatus;

#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    pub status: ServerStatus,
}

#[function_component(ServerStatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let toaster = use_toaster().unwrap();
    let status = &props.status;
    match status {
        ServerStatus::Factorio(status) => factorio_card(status, toaster),
        ServerStatus::Generic(status) => generic_card(status, toaster),
    }
}

fn factorio_card(status: &FactorioStatus, toaster: Toaster) -> Html {
    let copy_pass = copy_to_clipboard("Password", &status.game_password, toaster.clone());
    let copy_url = copy_to_clipboard("URL", &status.url, toaster.clone());
    html! {
        <Card>
            <CardTitle>{&*status.name}</CardTitle>
            <CardBody>
                <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                    <DescriptionGroup term="Status">
                        <HealthIndicator health={status.health}/>
                    </DescriptionGroup>
                    <DescriptionGroup term="URL">
                        {&*status.url}
                        <Button
                            onclick={copy_url}
                            variant={ButtonVariant::Plain}
                            icon={Icon::Copy}
                            aria_label="Copy URL" />
                    </DescriptionGroup>
                    <DescriptionGroup term="Password">
                        {&*status.game_password}
                        <Button
                            onclick={copy_pass}
                            variant={ButtonVariant::Plain}
                            icon={Icon::Copy}
                            aria_label="Copy Password" />
                    </DescriptionGroup>
                    <DescriptionGroup term="Game Time">
                        {&*status.game_time}
                    </DescriptionGroup>
                    <DescriptionGroup term="Game Version">
                        {&*status.game_version}
                    </DescriptionGroup>
                    <DescriptionGroup term="Online Players">
                        <ul>
                            {
                                if status.players_online.is_empty() {
                                    "None :(".into()
                                } else {
                                    status.players_online.iter().map(|name| {
                                        html! {
                                            <li key={&**name}>{&**name}</li>
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </ul>
                    </DescriptionGroup>
                </DescriptionList>
            </CardBody>
        </Card>
    }
}

fn generic_card(status: &GenericStatus, toaster: Toaster) -> Html {
    let copy_pass = copy_to_clipboard("Password", &status.game_password, toaster.clone());
    let copy_url = copy_to_clipboard("URL", &status.url, toaster.clone());
    html!{
        <Card>
            <CardTitle>{&*status.name}</CardTitle>
            <CardBody>
                <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                    <DescriptionGroup term="Status">
                        <HealthIndicator health={status.health}/>
                    </DescriptionGroup>
                    <DescriptionGroup term="Game">
                        {&*status.game_name}
                    </DescriptionGroup>
                    <DescriptionGroup term="URL">
                        {&*status.url}
                        <Button
                            onclick={copy_url}
                            variant={ButtonVariant::Plain}
                            icon={Icon::Copy}
                            aria_label="Copy URL" />
                    </DescriptionGroup>
                    <DescriptionGroup term="Password">
                        {&*status.game_password}
                        <Button
                            onclick={copy_pass}
                            variant={ButtonVariant::Plain}
                            icon={Icon::Copy}
                            aria_label="Copy Password" />
                    </DescriptionGroup>
                </DescriptionList>
            </CardBody>
        </Card>
    }
}

fn copy_to_clipboard(name: &str, text: &str, toaster: Toaster) -> Callback<MouseEvent> {
    let clipboard = window().navigator().clipboard();
    let text = text.to_owned();

    let resolve_closure = {
        let name = name.to_owned();
        let toaster = toaster.clone();
        Closure::wrap(Box::new(move |_| {
            toaster.toast(Toast {
                title: format!("Copied {} to clipboard", name),
                timeout: Some(Duration::from_secs(3)),
                r#type: AlertType::Info,
                ..Default::default()
            });
        }) as Box<dyn FnMut(JsValue)>)
    };
    let reject_closure = {
        let name = name.to_owned();
        Closure::wrap(Box::new(move |_| {
            toaster.toast(Toast {
                title: format!("Failed to copy {} to clipboard", name),
                timeout: Some(Duration::from_secs(3)),
                r#type: AlertType::Danger,
                ..Default::default()
            });
        }) as Box<dyn FnMut(JsValue)>)
    };

    Callback::from(move |_| {
        let _ = clipboard.write_text(&text)
            .then2(
                &resolve_closure,
                &reject_closure,
            );
    })
}