use patternfly_yew::prelude::*;
use crate::pages::MyPage;

use yew::prelude::*;

#[function_component(Factorio)]
pub fn factorio() -> Html {
    html! {
        <>
            <MyPage title="Factorio">
                <Flex>
                    <FlexItem>
                        <Card>
                            <CardTitle>{"Space Age"}</CardTitle>
                            <CardBody>
                                <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                                    <DescriptionGroup term="Status">{"Unknown"}</DescriptionGroup>
                                </DescriptionList>
                            </CardBody>
                        </Card>
                    </FlexItem>
                </Flex>

            </MyPage>
        </>
    }
}
