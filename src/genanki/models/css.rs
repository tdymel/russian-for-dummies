pub const CSS: &str = r#"
    body {
        margin: 0;
        padding: 0;
    }

    .rfd-container {
        font-size: 2rem;
        background: black;
        color: white;
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }

    .rfd-header {
        display: flex;
        justify-content: center;
        align-items: center;
        background: #333333;
        padding: 1rem;
    }

    .rfd-body {
        font-size: 1.5rem;
        color: white;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .rfd-body table {
        width: 100%;
        border: 1px solid #333333;
        border-radius: 0.25rem;
        border-collapse: separate;
        border-spacing: 0;
        font-size: 1.5rem;
        color: white;
        text-align: left;
    }

    .rfd-body td {
        padding: 1rem;
    }

    .rfd-body td:first-child, .rfd-body td:nth-child(2) {
        width: 1%;
        border-right: 1px solid #333333;
    }

    .rfd-body td:nth-child(2), .rfd-body td:nth-child(3) {
        width: auto;
    }

    .rfd-body tr:nth-child(even) {
        background-color: #333333;
    }

    .rfd-body tr:nth-child(even) td:first-child, .rfd-body tr:nth-child(even) td:nth-child(2) {
        border-right: 1px solid black;
    }

    .rfd-stressed {
        color: red !important;
    }

    .rfd-root {
        font-size: 2rem;
        align-self: center;
    }

    .rfd-example {
        font-size: 1.5rem;
        align-self: center;
    }
"#;
