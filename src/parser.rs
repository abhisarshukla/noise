use tracing::{debug, instrument};

#[instrument(level = "debug")]
pub fn parse_components(pipeline: &str) -> Vec<String> {
    let mut components = Vec::new();
    let mut current = String::new();
    let mut bracket_level = 0;

    for ch in pipeline.chars() {
        match ch {
            '[' => {
                bracket_level += 1;
                current.push(ch);
            }
            ']' => {
                bracket_level -= 1;
                current.push(ch);
            }
            ',' if bracket_level == 0 => {
                if !current.trim().is_empty() {
                    components.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        components.push(current.trim().to_string());
    }

    debug!("Parsed {} components from pipeline", components.len());
    for (i, comp) in components.iter().enumerate() {
        debug!("Component {}: {}", i, comp);
    }

    components
}
