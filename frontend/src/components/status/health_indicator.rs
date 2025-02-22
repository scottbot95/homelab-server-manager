use common::status::HealthStatus;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HealthIndicatorProps {
    pub health: HealthStatus,
}

#[function_component(HealthIndicator)]
pub fn health_indicator(props: &HealthIndicatorProps) -> Html {
    let color = match props.health {
        HealthStatus::Running => "lime",
        HealthStatus::Starting => "yellow",
        HealthStatus::Offline => "red",
        HealthStatus::Unknown => "red",
    };
    let style = format!(
        "margin-right: 5px; width: 15px; height: 15px; border-radius: 50%; background-color: {};",
        color
    );
    html! {
        <Split>
            <SplitItem>
                <div {style} />
            </SplitItem>
            <SplitItem fill=true>{props.health.to_string()}</SplitItem>
        </Split>
    }
}
