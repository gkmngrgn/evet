use chrono_tz::TZ_VARIANTS;
use evet::date::EventDate;
use js_sys::{Array, Intl, Object, Reflect};
use leptos::prelude::*;
use leptos::tachys::reactive_graph::bind::GetValue;
use leptos::web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, Url, Blob};
use leptos_use::{UseClipboardReturn, use_clipboard};
use wasm_bindgen::{JsCast, JsValue};

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

    // Sort the groups and the timezones within each group
    let mut sorted_grouped_timezones: Vec<_> = grouped_timezones.into_iter().collect();
    sorted_grouped_timezones.sort_by_key(|(group, _)| *group);
    for (_, zones) in &mut sorted_grouped_timezones {
        zones.sort();
    }

    view! {
        <select id="timezone" multiple>
            {sorted_grouped_timezones.iter().map(|(group, zones)| {
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
fn Description(output: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="description" style={move || {
            if output.get().is_empty() {
                "display: block;"
            } else {
                "display: none;"
            }
        }}>
            <p>{ "Create an event with a message and time, and convert that time across multiple timezones." }</p>
            <p>{ "You can:" }</p>
            <ul>
                <li>{ "copy and paste the invitation message." }</li>
                <li>{ "download the invitation message as text or a calendar event file." }</li>
                <li>{ "share the link to this page to invite others." }</li>
            </ul>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (output, set_output) = signal(String::new());
    let UseClipboardReturn { is_supported, text, copied, copy } = use_clipboard();
    let local_timezone = get_client_timezone();
    let handle_submit = move |_| {
        let message = get_element_by_id::<HtmlTextAreaElement>("message").value();
        let datetime = get_element_by_id::<HtmlInputElement>("datetime")
            .value()
            .replace("T", " ");
        let timezones: Vec<String> = {
            let options = get_element_by_id::<HtmlSelectElement>("timezone").selected_options();
            let mut timezones = Vec::new();
            for i in 0..options.length() {
                let option = options.item(i).unwrap();
                timezones.push(option.get_value());
            }
            timezones
        };
        let local_timezone = get_element_by_id::<HtmlInputElement>("local-timezone").value();

        // Debugging information
        console_log(format!("Message: {}", message.clone()));
        console_log(format!("Datetime: {}", datetime.clone()));
        console_log(format!("Timezones: {:?}", timezones.clone()));

        let result = match EventDate::new(datetime.to_string(), Some(local_timezone), timezones) {
            Ok(d) => {
                let dates_by_timezones = d.get_dates_by_timezones()
                    .iter()
                    .map(|tz| tz.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                let ics_content = format!(
                    "BEGIN:VCALENDAR\nVERSION:2.0\nBEGIN:VEVENT\nSUMMARY:{}\nDTSTART:{}\nEND:VEVENT\nEND:VCALENDAR",
                    message,
                    datetime.replace(" ", "T")
                );

                let blob = Blob::new_with_str_sequence(&Array::of1(&JsValue::from(ics_content))).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();

                let download_link = get_element_by_id::<HtmlElement>("download");
                download_link.set_attribute("href", &url).unwrap();
                download_link.set_attribute("download", "event.ics").unwrap();

                format!("---\n{}\n{}\n---\n", message, dates_by_timezones)
            },
            Err(e) => {
                console_log(format!("Error creating EventDate: {}", e));
                e.to_string()
            }
        };
        set_output.set(result);
    };

    let handle_clear = move |_| {
        if leptos::web_sys::window().unwrap().confirm_with_message("Are you sure you want to clear the form?").unwrap() {
            get_element_by_id::<HtmlTextAreaElement>("message").set_value("");
            get_element_by_id::<HtmlInputElement>("datetime").set_value("");
            get_element_by_id::<HtmlSelectElement>("timezone").set_value("");
            set_output.set(String::new());
        }
    };

    view! {
        <>
            <main id="event-form">
                <nav class="menubar">
                    <ul>
                        <li>EVET - Event Inviter</li>
                    </ul>
                    <ul>
                        <li>
                            <a href="#submit" on:click=handle_submit>
                                Submit form
                            </a>
                        </li>
                        <li>
                            <Show
                                when=move || !output.get().is_empty()
                                fallback=move || view! { <a aria_disabled="true">Clear form</a> }
                            >
                                <a href="#clear" on:click=handle_clear>Clear form</a>
                            </Show>
                        </li>
                        <li>
                            <Show
                                when=move || !output.get().is_empty()
                                fallback=move || view! { <a aria_disabled="true">Copy text</a> }
                            >
                                <a href="#copy" on:click={
                                    let copy = copy.clone();
                                    move |_| copy(&output.get())
                                }>Copy text</a>
                            </Show>
                        </li>
                        <li>
                            <Show
                                when=move || !output.get().is_empty()
                                fallback=move || view! { <a aria_disabled="true">Download ICS file</a> }
                            >
                                <a href="#download" id="download">Download ICS file</a>
                            </Show>
                        </li>
                    </ul>
                </nav>

                <div class="event-form">
                    <div class="event-form__left">
                        <div class="event-form__input">
                            <label for="message">Message</label>
                            <textarea id="message" placeholder="Enter your message"></textarea>
                        </div>
                        <div class="event-form__input">
                            <label for="datetime">Date and Time</label>
                            <input type="datetime-local" id="datetime" />
                        </div>
                        <div class="event-form__input">
                            <label for="timezone">Timezones</label>
                            <TimezoneSelect />
                            <p class="small">hold Ctrl/Cmd to select multiple</p>
                        </div>
                        <div class="event-form__input">
                            <label for="local-timezone">Local Timezone</label>
                            <input type="text" id="local-timezone" value={local_timezone} readonly />
                        </div>
                    </div>
                    <div class="event-form__right">
                        <Description output=output />

                        <p class="output" inner_html={move || {
                            output.get().replace("\n", "<br>")
                        }}></p>
                    </div>
                </div>
            </main>
        </>
    }
}

fn get_client_timezone() -> String {
    let options = Intl::DateTimeFormat::new(&Array::new(), &Object::new()).resolved_options();
    let tz = Reflect::get(&options, &JsValue::from("timeZone"))
        .expect("Cannot get timeZone")
        .as_string()
        .expect("timeZone is not a String");
    tz
}

fn get_element_by_id<T: JsCast>(id: &str) -> T {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}

fn console_log(message: String) {
    leptos::web_sys::console::log_1(&message.into());
}
