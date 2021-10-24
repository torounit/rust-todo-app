use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use Msg::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub content: String,
}

enum Msg {
    GetTodos,
    ReceiveResponse(Result<Vec<Todo>, anyhow::Error>),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    todos: Vec<Todo>,
}

impl Model {
    fn view_todo(&self, (id, todo): (usize, &Todo)) -> Html {
        return html! {
            <li>
                {id}{ ":" }{&todo.content}
            </li>
        };
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            todos: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ReceiveResponse(response) => {
                let todos = response.unwrap();
                self.todos = todos;
                self.fetch_task = None;
                true
            }
            GetTodos => {
                let request = Request::get("http://localhost:8081")
                    .body(Nothing)
                    .expect("Could not build that request");

                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<Todo>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        ReceiveResponse(data)
                    },
                );
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <div>
                <ul>
                    { for self.todos.iter().enumerate().map(|e| self.view_todo(e)) }
                </ul>
            </div>
        };
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            self.update(GetTodos);
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
