use chrono::Local;
use chrono_tz::TZ_VARIANTS;
use evet::date::{EventDate, TimezoneData};
use leptos::ev::{Event, MouseEvent};
use leptos::prelude::*;
use leptos::tachys::reactive_graph::bind::GetValue;
use leptos::web_sys::{HtmlCollection, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use wasm_bindgen::JsCast;

fn main() {
    leptos::mount::mount_to_body(Home)
}

#[component]
fn TimezoneSelect() -> impl IntoView {
    let (selected_timezones, _set_selected_timezones) = signal(Vec::new());
    type GroupedTimezones = std::collections::HashMap<&'static str, Vec<&'static str>>;
    let mut grouped_timezones: GroupedTimezones = std::collections::HashMap::new();
    for tz in TZ_VARIANTS.iter() {
        let name = tz.name();
        if name.starts_with("Etc/") {
            continue;
        }
        if let Some((group, zone)) = name.split_once('/') {
            grouped_timezones.entry(group).or_default().push(zone);
        }
    }

    view! {
        <select id="timezone" multiple>
            {grouped_timezones.iter().map(|(group, zones)| {
                view! {
                    <optgroup label={*group}>
                        {zones.iter().map(|zone| {
                            let display_name = zone.replace('/', " - ").replace('_', " ");
                            let value = format!("{}/{}", group, zone); // Corrected value
                            view! {
                                <option value={value} selected={selected_timezones.get().contains(zone)}>
                                    {display_name}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </optgroup>
                }
            }).collect::<Vec<_>>()}
        </select>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (output, set_output) = signal(String::new());
    let timezoneLabel = "Timezones (hold Ctrl/Cmd to select multiple)";
    let handle_submit = move |_| {
        let message = get_element_by_id::<HtmlTextAreaElement>("message").value();
        let date = get_element_by_id::<HtmlInputElement>("date").value();
        let time = get_element_by_id::<HtmlInputElement>("time").value();
        let timezones = {
            let options = get_element_by_id::<HtmlSelectElement>("timezone").selected_options();
            let mut timezones = Vec::new();
            for i in 0..options.length() {
                let option = options.item(i).unwrap();
                timezones.push(option.get_value());
            }
            timezones
        };
        let local_timezone = Local::now().offset().to_string();
        let result = EventDate::new(date, Some(local_timezone), timezones)
            .map(|event_date| {
                format!(
                    "---\n{}\n{}---\n",
                    message,
                    event_date
                        .get_dates_by_timezones()
                        .iter()
                        .map(|tz| tz.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .unwrap_or_else(|err| format!("Error: {}", err));
        set_output.set(result);
    };

    view! {
        <main class="event-form">
            <form id="event-form">
                <div class="event-form__input">
                    <label for="message">Message</label>
                    <textarea id="message" placeholder="Enter your message" required></textarea>
                </div>
                <div class="event-form__datetime">
                    <div class="event-form__input">
                        <label for="date">Date</label>
                        <input type="date" id="date" />
                    </div>
                    <div class="event-form__input">
                        <label for="time">Time</label>
                        <input type="time" id="time" />
                    </div>
                </div>
                <div class="event-form__input">
                    <label for="timezone">{timezoneLabel}</label>
                    <TimezoneSelect />
                </div>
                <button on:click=handle_submit>Submit</button>
                <div>
                    <h2>Output</h2>
                    <p inner_html={move || output.get().replace("\n", "<br>")}></p>
                </div>
            </form>
        </main>
    }
}

fn get_element_by_id<T: JsCast>(id: &str) -> T {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}
