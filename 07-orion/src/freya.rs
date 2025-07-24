fn app() -> Element {

    render!(
        rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            fill: "red",
            label {
                text: "Hello, Freya!",
                x: 50,
                y: 50,
                font: "Arial",
                size: 12,
                fill: "black",
                anchor: "middle"
            }
        }
        rect {
            x: 100,
            y: 100,
            width: 100,
            height: 100,
            fill: "blue",
            label {
                text: "Hello, Freya!",
                x: 150,
                y: 150,
                font: "Arial",
                size: 12,
                fill: "black",
                anchor: "middle"
            }
        }
    )
}