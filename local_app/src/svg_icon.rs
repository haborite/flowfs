use dioxus::prelude::*;

pub fn general_file() -> Element {
    rsx! {
        svg {
            "stroke-width": "1.5",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke": "currentColor",
            "viewBox": "0 0 24 24",
            "fill": "none",
            class: "w-4 h-4",
            path {
                "stroke-linejoin": "round",
                "d": "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z",
                "stroke-linecap": "round"
            }
        }
    }
}

pub fn image_file() -> Element {
    rsx! {
        svg {
            "stroke": "currentColor",
            "fill": "none",
            "stroke-width": "1.5",
            "viewBox": "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            class: "w-4 h-4",
            path {
                "d": "M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z",
                "stroke-linecap": "round",
                "stroke-linejoin": "round"
            }
        }
    }
}

pub fn document() -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 24 24",
            "stroke": "currentColor",
            "fill": "none",
            "stroke-width": "1.5",
            "xmlns": "http://www.w3.org/2000/svg",
            class: "w-4 h-4",
            path {
                "stroke-linecap": "round",
                "d": "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z",
                "stroke-linejoin": "round"
            }
        }
    }
}

pub fn directory() -> Element {
    rsx! {
        svg {
            "fill": "none",
            "viewBox": "0 0 24 24",
            "stroke": "currentColor",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke-width": "1.5",
            class: "w-4 h-4",
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                "d": "M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
            }
        }
    }
}

pub fn dotsmenu() -> Element {
    rsx! {
        svg {
            "aria-hidden": "true",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "currentColor",
            "viewBox": "0 0 16 3",
            class: "w-5 h-5",
            path { "d": "M2 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Zm6.041 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM14 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Z" }
        }
    }
}

pub fn trash() -> Element {
    rsx! {
        svg {
            width: "24px",
            height: "24px",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#5f6368",            
            "viewBox": "0 0 24 24",
            path { "fill": "none", "d": "M0 0h24v24H0V0z" }
            path { "d": "M16 9v10H8V9h8m-1.5-6h-5l-1 1H5v2h14V4h-3.5l-1-1zM18 7H6v12c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7z" }
        }
    }
}

pub fn reply() -> Element {
    rsx! {
        svg {
            width: "24px",
            height: "24px",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#5f6368",
            "viewBox": "0 -960 960 960",
            path { "d": "M760-200v-160q0-50-35-85t-85-35H273l144 144-57 56-240-240 240-240 57 56-144 144h367q83 0 141.5 58.5T840-360v160h-80Z" }
        }
    }
}

pub fn download() -> Element {
    rsx! {
        svg {
            width: "24px",
            height: "24px",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#5f6368",
            "viewBox": "0 -960 960 960",
            path { "d": "M480-320 280-520l56-58 104 104v-326h80v326l104-104 56 58-200 200ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" }
        }
    }
}