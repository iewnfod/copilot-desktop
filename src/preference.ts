export interface Preference {
    width: number,
    start_at_login: boolean,
    hide_window_when_not_focus: boolean,
    always_on_top: boolean,
    show_short_cut: string,
    ai_url: string
}

export const DEFAULT_PREFERENCE: Preference = {
    width: 650,
    start_at_login: false,
    hide_window_when_not_focus: false,
    always_on_top: false,
    show_short_cut: "Ctrl+Shift+P",
    ai_url: "https://copilot.microsoft.com/",
}
