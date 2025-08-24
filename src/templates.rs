use chrono::{DateTime, Utc};
use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::types::{EndpointInfo, EndpointStatus, StatusStore};

const CSS_STYLES: &str = include_str!("../assets/styles.css");
const JAVASCRIPT: &str = include_str!("../assets/app.js");

pub fn dashboard_page(status_store: &StatusStore) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Tor Endpoint Monitor" }
                style { (PreEscaped(CSS_STYLES)) }
                script { (PreEscaped(JAVASCRIPT)) }
            }
            body {
                div class="container" {
                    header class="header" {
                        h1 class="title" {
                            span class="title-icon" { "🔗" }
                            "TOR ENDPOINT MONITOR"
                        }
                        div class="status-summary" {
                            (status_summary(status_store))
                        }
                    }

                    main class="main-content" {
                        @if status_store.is_empty() {
                            div class="no-endpoints" {
                                p { "No endpoints configured" }
                                p class="help-text" { "Add endpoints to config.toml to start monitoring" }
                            }
                        } @else {
                            div class="table-container" {
                                table class="endpoints-table" {
                                    thead {
                                        tr {
                                            th { "Status" }
                                            th { "Name" }
                                            th { "Address" }
                                            th { "Port" }
                                            th { "Response Time" }
                                            th { "Last Check" }
                                            th { "Details" }
                                        }
                                    }
                                    tbody {
                                        @for endpoint_info in sorted_endpoints(status_store) {
                                            (endpoint_row(endpoint_info))
                                        }
                                    }
                                }
                            }
                        }
                    }

                    footer class="footer" {
                        div class="footer-content" {
                            div class="footer-left" {
                                span class="last-update" { "Last updated: " (format_timestamp(&Utc::now())) }
                                span class="auto-refresh" { "Auto-refresh: 30s" }
                            }
                            div class="footer-right" {
                                span class="made-in" {
                                    "Made in "
                                    svg class="usa-flag" viewBox="0 0 120 63" width="24" height="13" fill="none" xmlns="http://www.w3.org/2000/svg" {
                                        // Red and white stripes
                                        rect y="0" width="120" height="4.846" fill="#BD3D44" {}
                                        rect y="4.846" width="120" height="4.846" fill="#FFF" {}
                                        rect y="9.692" width="120" height="4.846" fill="#BD3D44" {}
                                        rect y="14.538" width="120" height="4.846" fill="#FFF" {}
                                        rect y="19.384" width="120" height="4.846" fill="#BD3D44" {}
                                        rect y="24.23" width="120" height="4.846" fill="#FFF" {}
                                        rect y="29.076" width="120" height="4.846" fill="#BD3D44" {}
                                        rect y="33.922" width="120" height="4.846" fill="#FFF" {}
                                        rect y="38.768" width="120" height="4.846" fill="#BD3D44" {}
                                        rect y="43.614" width="120" height="4.846" fill="#FFF" {}
                                        rect y="48.46" width="120" height="4.846" fill="#BD3D44" {}
                                        rect y="53.306" width="120" height="4.846" fill="#FFF" {}
                                        rect y="58.152" width="120" height="4.846" fill="#BD3D44" {}
                                        // Blue field with stars
                                        rect width="48" height="28" fill="#192F5D" {}
                                        // Stars
                                        g fill="#FFF" font-family="Arial" font-size="4" font-weight="bold" letter-spacing="0" {
                                            text y="8" x="4" { "★" }
                                            text y="8" x="12" { "★" }
                                            text y="8" x="20" { "★" }
                                            text y="8" x="28" { "★" }
                                            text y="8" x="36" { "★" }
                                            text y="14" x="8" { "★" }
                                            text y="14" x="16" { "★" }
                                            text y="14" x="24" { "★" }
                                            text y="14" x="32" { "★" }
                                            text y="14" x="40" { "★" }
                                            text y="20" x="4" { "★" }
                                            text y="20" x="12" { "★" }
                                            text y="20" x="20" { "★" }
                                            text y="20" x="28" { "★" }
                                            text y="20" x="36" { "★" }
                                            text y="26" x="8" { "★" }
                                            text y="26" x="16" { "★" }
                                            text y="26" x="24" { "★" }
                                            text y="26" x="32" { "★" }
                                            text y="26" x="40" { "★" }
                                        }
                                    }
                                    " by "
                                    a href="https://github.com/w3irdrobot" target="_blank" rel="noopener noreferrer" class="author-link" { "w3irdrobot" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn sorted_endpoints(status_store: &StatusStore) -> Vec<&EndpointInfo> {
    let mut endpoints: Vec<&EndpointInfo> = status_store.values().collect();

    // Sort by status priority: Online -> Checking -> Unknown -> Offline
    // Within same status, sort by name for consistency
    endpoints.sort_by(|a, b| {
        let status_priority = |status: &EndpointStatus| -> u8 {
            match status {
                EndpointStatus::Online { .. } => 0,
                EndpointStatus::Checking => 1,
                EndpointStatus::Unknown => 2,
                EndpointStatus::Offline { .. } => 3,
            }
        };

        let a_priority = status_priority(&a.status);
        let b_priority = status_priority(&b.status);

        // First sort by status priority
        match a_priority.cmp(&b_priority) {
            std::cmp::Ordering::Equal => {
                // If same status, sort by name
                a.endpoint.name.cmp(&b.endpoint.name)
            }
            other => other,
        }
    });

    endpoints
}

fn status_summary(status_store: &StatusStore) -> Markup {
    let mut online = 0;
    let mut offline = 0;
    let mut checking = 0;
    let mut unknown = 0;

    for endpoint_info in status_store.values() {
        match &endpoint_info.status {
            EndpointStatus::Online { .. } => online += 1,
            EndpointStatus::Offline { .. } => offline += 1,
            EndpointStatus::Checking => checking += 1,
            EndpointStatus::Unknown => unknown += 1,
        }
    }

    html! {
        div class="summary-stats" {
            div class="stat online" {
                span class="stat-number" { (online) }
                span class="stat-label" { "Online" }
            }
            div class="stat offline" {
                span class="stat-number" { (offline) }
                span class="stat-label" { "Offline" }
            }
            div class="stat checking" {
                span class="stat-number" { (checking) }
                span class="stat-label" { "Checking" }
            }
            div class="stat unknown" {
                span class="stat-number" { (unknown) }
                span class="stat-label" { "Unknown" }
            }
        }
    }
}

fn endpoint_row(endpoint_info: &EndpointInfo) -> Markup {
    let status = &endpoint_info.status;
    let endpoint = &endpoint_info.endpoint;

    html! {
        tr class=(status.css_class()) {
            td class="status-cell" {
                span class="status-indicator" {
                    span class="status-emoji" { (status.status_emoji()) }
                    span class="status-text" { (status.status_text()) }
                }
            }
            td class="name-cell" { (endpoint.name) }
            td class="address-cell" {
                code class="address-value clickable-address" data-address=(endpoint.address) title="Click to copy address" { (endpoint.address) }
            }
            td class="port-cell" { (endpoint.port) }
            td class="response-cell" {
                @match status {
                    EndpointStatus::Online { response_time_ms } => {
                        span class="response-time" { (response_time_ms) "ms" }
                    }
                    _ => {
                        span class="no-data" { "—" }
                    }
                }
            }
            td class="time-cell" {
                @if let Some(last_check) = &endpoint_info.last_check {
                    span class="timestamp" { (format_timestamp(last_check)) }
                } @else {
                    span class="no-data" { "Never" }
                }
            }
            td class="details-cell" {
                @match status {
                    EndpointStatus::Offline { error } => {
                        span class="error-message" { (error) }
                    }
                    EndpointStatus::Checking => {
                        span class="checking-message" { "Connecting..." }
                    }
                    _ => {
                        span class="no-data" { "—" }
                    }
                }
            }
        }
    }
}

fn format_timestamp(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
