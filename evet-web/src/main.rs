use chrono_tz::TZ_VARIANTS;
use leptos::prelude::*;

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
    let (count, set_count) = signal(0);
    let timezoneLabel = "Timezones (hold Ctrl/Cmd to select multiple)";

    view! {
        <main class="event-form">
            <div class="event-form__input">
                <label for="message">Message</label>
                <textarea id="message" placeholder="Enter your message"></textarea>
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
            <button>Submit</button>
            <button on:click=move |_| *set_count.write() += 1>
                {move || if count.get() == 0 {
                    "Click me!".to_string()
                } else {
                    count.get().to_string()
                }}
            </button>
        </main>
    }
}
