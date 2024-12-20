use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(Home)
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main class="event-form">
            <div class="event-form__input">
                <label for="message">Message</label>
                <input type="text" id="message" />
            </div>
            <input type="text" placeholder="Enter your message" />
            <input type="date" />
            <input type="time" />
            <select>
                <option value="America/New_York">America/New_York</option>
                <option value="America/Los_Angeles">America/Los_Angeles</option>
                <option value="America/Chicago">America/Chicago</option>
                <option value="America/Denver">America/Denver</option>
                <option value="America/Phoenix">America/Phoenix</option>
                <option value="America/Anchorage">America/Anchorage</option>
                <option value="Pacific/Honolulu">Pacific/Honolulu</option>
            </select>
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
