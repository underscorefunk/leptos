use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}

async fn fetch_cats(count: u32) -> Result<Vec<String>, ()> {
    if count > 0 {
        log!("fetching cats");
        let res = reqwasm::http::Request::get(&format!(
            "https://api.thecatapi.com/v1/images/search?limit={}",
            count
        ))
        .send()
        .await
        .map_err(|_| ())?
        .json::<Vec<Cat>>()
        .await
        .map_err(|_| ())?
        .into_iter()
        .map(|cat| cat.url)
        .collect::<Vec<_>>();
        log!("got cats {res:?}");
        Ok(res)
    } else {
        Ok(vec![])
    }
}

pub fn fetch_example(cx: Scope) -> web_sys::Element {
    let (cat_count, set_cat_count) = cx.create_signal::<u32>(3);
    let cats = cx.create_ref(cx.create_resource(cat_count.clone(), |count| fetch_cats(*count)));

    view! {
        <div>
            <label>
                "How many cats would you like?"
                <input type="number"
                    value={move || cat_count.get().to_string()}
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                        log!("set_cat_count {val}");
                        set_cat_count(|n| *n = val);
                    }
                />
            </label>
            <div>
                //<Suspense fallback={"Loading (Suspense Fallback)...".to_string()}>
                    {move || match &*cats.read() {
                        ResourceState::Idle => view! { <p>"(no data)"</p> },
                        ResourceState::Pending { .. } => view! { <p>"Loading..."</p> },
                        ResourceState::Ready { data: Err(_) } => view! { <pre>"Error"</pre> },
                        ResourceState::Ready { data: Ok(cats) } => view! {
                            <div>{
                                cats.iter()
                                    .map(|src| {
                                        view! {
                                            <img src={src}/>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }</div>
                        }
                    }}
                //</Suspense>
            </div>
        </div>
    }
}