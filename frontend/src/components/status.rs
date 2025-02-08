use std::rc::Rc;
use std::string::ToString;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use common::status::{HealthStatus, ServerStatus};

#[derive(Properties, PartialEq)]
pub struct HealthIndicatorProps {
    pub health: HealthStatus,
}

#[function_component(HealthIndicator)]
pub fn health_indicator(props: &HealthIndicatorProps) -> Html  {
    html! {
        <Split>
            // TODO Add some indicator icon
            <SplitItem fill=true>{props.health.to_string()}</SplitItem>
        </Split>
    }
}

#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    pub status: Rc<ServerStatus>,
}

#[function_component(ServerStatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let status = &props.status;
    html! {
        <Card>
            <CardTitle>{&status.name}</CardTitle>
            <CardBody>
                <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                    <DescriptionGroup term="Status">
                        <HealthIndicator health={status.health}/>
                    </DescriptionGroup>
                </DescriptionList>
            </CardBody>
        </Card>
    }
}