use std::rc::Rc;
use patternfly_yew::prelude::*;
use crate::pages::MyPage;

use yew::prelude::*;
use models::status::{HealthStatus, ServerStatus};
use crate::components::ServerStatusCard;

#[function_component(Factorio)]
pub fn factorio() -> Html {
    let servers = &[
        Rc::new(ServerStatus {
            name: "Space Age".to_owned(),
            health: HealthStatus::Unknown,
        })
    ];
    let result = html! {
        <MyPage title="Factorio">
            <Flex>
                {for servers.iter().map(|status| html_nested!{
                    <FlexItem>
                        <ServerStatusCard {status} />
                    </FlexItem>
                })}
            </Flex>

        </MyPage>
    };



    result
}
