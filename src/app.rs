use crate::agents::posts::{PostId, PostStore, Request};
use crate::post::Post;
use crate::text_input::TextInput;
use yew::prelude::*;
use yew::services::ConsoleService;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

pub enum Msg {
    CreatePost(String),
    PostStoreMsg(ReadOnly<PostStore>),
}

pub struct App {
    link: ComponentLink<Self>,
    post_ids: Vec<PostId>,
    post_store: Box<dyn Bridge<StoreWrapper<PostStore>>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::PostStoreMsg);
        Self {
            link,
            post_ids: Vec::new(),
            post_store: PostStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreatePost(text) => {
                self.post_store.send(Request::CreatePost(text));
                false
            }
            Msg::PostStoreMsg(state) => {
                // We can see this is logged once before we click any button.
                // The state of the store is sent when we open a bridge.
                ConsoleService::log("Received update");

                let state = state.borrow();
                if state.posts.len() != self.post_ids.len() {
                    self.post_ids = state.posts.keys().copied().collect();
                    self.post_ids.sort_unstable();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <TextInput value="New post" onsubmit=self.link.callback(Msg::CreatePost) />

                <div>
                    { for self.post_ids.iter().map(|&id| html!{ <Post key=id id=id /> }) }
                </div>
            </>
        }
    }
}

