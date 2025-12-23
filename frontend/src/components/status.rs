mod health_indicator;

use crate::components::status::health_indicator::HealthIndicator;
use common::factorio::FactorioStatus;
use common::status::ServerStatus;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use common::generic::GenericStatus;

#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    pub status: ServerStatus,
}

#[function_component(ServerStatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let status = &props.status;
    match status {
        ServerStatus::Factorio(status) => factorio_card(status),
        ServerStatus::Generic(status) => generic_card(status),
    }
}

fn factorio_card(status: &FactorioStatus) -> Html {
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
                    </DescriptionGroup>
                    <DescriptionGroup term="Password">
                        {&*status.game_password}
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

fn generic_card(status: &GenericStatus) -> Html {
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
                    </DescriptionGroup>
                    <DescriptionGroup term="Password">
                        {&*status.game_password}
                    </DescriptionGroup>
                </DescriptionList>
            </CardBody>
        </Card>
    }
}
