use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::agent::{set_token, Auth};
use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::types::{LoginInfo, LoginInfoWrapper, UserInfo, UserInfoWrapper};

/// Login page
pub struct Login {
    auth: Auth,
    error: Option<Error>,
    request: LoginInfo,
    response: Callback<Result<UserInfoWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    /// Callback when user is logged in successfully
    #[props(required)]
    pub callback: Callback<UserInfo>,
}

pub enum Msg {
    Request,
    Response(Result<UserInfoWrapper, Error>),
    Ignore,
    UpdateEmail(String),
    UpdatePassword(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Login {
            auth: Auth::new(),
            error: None,
            props,
            request: LoginInfo::default(),
            response: link.send_back(Msg::Response),
            router_agent: RouteAgent::bridge(link.send_back(|_| Msg::Ignore)),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let request = LoginInfoWrapper {
                    user: self.request.clone(),
                };
                self.task = Some(self.auth.login(request, self.response.clone()));
            }
            Msg::Response(Ok(user_info)) => {
                // Set global token after logged in
                set_token(Some(user_info.user.token.clone()));
                self.props.callback.emit(user_info.user);
                self.error = None;
                self.task = None;
                // Route to home page after logged in
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateEmail(email) => {
                self.request.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = password;
            }
            Msg::Ignore => {}
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="auth-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-6 offset-md-3 col-xs-12">
                            <h1 class="text-xs-center">{ "Sign In" }</h1>
                            <p class="text-xs-center">
                                <RouterLink text="Need an account?" link="#/register"/>
                            </p>
                            <ListErrors error=&self.error />
                            <form onsubmit=|ev| { ev.prevent_default(); /* Prevent event propagation */ Msg::Request }>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            value=&self.request.email
                                            oninput=|ev| Msg::UpdateEmail(ev.value)
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            value=&self.request.password
                                            oninput=|ev| Msg::UpdatePassword(ev.value)
                                            />
                                    </fieldset>
                                    <button
                                        class="btn btn-lg btn-primary pull-xs-right"
                                        type="submit"
                                        disabled=false>
                                        { "Sign in" }
                                    </button>
                                </fieldset>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
