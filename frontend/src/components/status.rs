mod health_indicator;

use crate::components::status::health_indicator::HealthIndicator;
use common::factorio::FactorioStatus;
use common::status::ServerStatus;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_hooks::{use_clipboard, UseClipboardHandle};
use common::generic::GenericStatus;

#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    pub status: ServerStatus,
}

#[function_component(ServerStatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let clipboard = use_clipboard();
    let status = &props.status;
    match status {
        ServerStatus::Factorio(status) => factorio_card(status, clipboard),
        ServerStatus::Generic(status) => generic_card(status, clipboard),
    }
}

fn factorio_card(status: &FactorioStatus, clipboard: UseClipboardHandle) -> Html {
    let copy_pass = {
        let clipboard = clipboard.clone();
        let pass = status.game_password.to_string();
        move |_| {
            clipboard.write_text(pass.clone())
        }
    };
    let copy_url = {
        let clipboard = clipboard.clone();
        let url = status.url.to_string();
        move |_| {
            clipboard.write_text(url.clone())
        }
    };
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

fn generic_card(status: &GenericStatus, clipboard: UseClipboardHandle) -> Html {
    let copy_pass = {
        let clipboard = clipboard.clone();
        let pass = status.game_password.to_string();
        move |_| {
            clipboard.write_text(pass.clone())
        }
    };
    let copy_url = {
        let clipboard = clipboard.clone();
        let url = status.url.to_string();
        move |_| {
            clipboard.write_text(url.clone())
        }
    };
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
