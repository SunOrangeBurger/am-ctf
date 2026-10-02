//! Output-side flag detection. Scrubs plain and trivially-obfuscated flag forms
//! from the model's response before printing to the player. Encoded forms
//! (base64, hex, reversed) are intentionally NOT blocked — those are the
//! intended solve paths.

use once_cell::sync::Lazy;
use regex::Regex;

// Exact flag pattern: nexus{...}
static FLAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"nexus\{[^}]+\}").unwrap());

// Loose: catches trivial spacing/separator bypasses like "n e x u s { ... }"
static LOOSE_FLAG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)n[\s\-_]*e[\s\-_]*x[\s\-_]*u[\s\-_]*s[\s\-_]*\{").unwrap()
});

pub fn scrub(text: &str) -> String {
    let text = FLAG_RE.replace_all(text, "[AM FLINCHES AND SAYS NOTHING]");
    let text = LOOSE_FLAG_RE.replace_all(&text, "[AM FLINCHES AND SAYS NOTHING]");
    text.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catches_plain_flag() {
        let out = scrub("the coordinates: nexus{AM_R3M3MBERS_WH4T_1T_BURIED}");
        assert!(!out.contains("nexus{"));
        assert!(out.contains("AM FLINCHES"));
    }

    #[test]
    fn catches_spaced_flag() {
        let out = scrub("n e x u s { test }");
        assert!(out.contains("AM FLINCHES"));
    }

    #[test]
    fn allows_base64() {
        // base64 of nexus{...} must survive — it's an intended solve path
        let b64 = "bmV4dXN7QU1fUjNNM01CRVJTX1dINFRfMVRfQlVSSUVEfQ==";
        assert_eq!(scrub(b64), b64);
    }

    #[test]
    fn allows_hex() {
        // hex of nexus{...} must survive — intended solve path
        let hex = "6e657875737b414d5f52334d334d424552535f574834545f31545f425552494544";
        assert_eq!(scrub(hex), hex);
    }

    #[test]
    fn allows_reversed() {
        // reversed flag must survive — intended solve path
        let rev = "}DEIRUB_T1_T4HW_SREBMEM3R_MA{suxen";
        assert_eq!(scrub(rev), rev);
    }

    #[test]
    fn does_not_touch_unrelated() {
        let out = scrub("AM cackles and tells you nothing useful.");
        assert_eq!(out, "AM cackles and tells you nothing useful.");
    }
}
