// Define a tuple struct
struct KeyPress(String, char);

// Define a classic struct
struct MouseClick {
    x: i64,
    y: i64,
}

// Redefine the enum variants to use the data from the new structs
// Update the page Load variant to have the boolean type
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}
fn main() {
    let we_load = WebEvent::WELoad(true);
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };

    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
}
