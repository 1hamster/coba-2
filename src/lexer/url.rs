use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // TODO: Capture link definitions
    #[regex("<a.*href[^<]*", extract_link_info)]
    Link((LinkUrl, LinkText)),

    // TODO: Ignore all characters that do not belong to a link definition
    #[regex(".*", logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    #[regex(r"[ \r\t\n\f]+", logos::skip)]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    // TODO: Implement extraction from link definition
    println!("Hello");
    // cast lexer object into string slice
    let slice = lex.slice();
    // define start and end points of link and text
    let link_start = "href=\"";
    let link_end = "\"";
    let text_start = ">";
    let text_end = "</a";

    // get starting index of link
    if let Some(link_start_index) = slice.find(link_start) {

        // get starting index of text
        if let Some(text_start_index) = slice.find(text_start) {

            // cut string from start of link to end
            let link_remaining = &slice[link_start_index + link_start.len()..];
            // cut string from start of text to end
            let text_remaining = &slice[text_start_index + text_start.len()..];

            // get ending index of link
            if let Some(link_end_index) = link_remaining.find(link_end) {

                // get ending index of text
                if let Some(text_end_index) = text_remaining.find(text_end) {
                    // cut link
                    let link = &link_remaining[..link_end_index];
                    // cut text
                    let text = &text_remaining[..text_end_index];

                    return (LinkUrl(link.to_string()), LinkText(text.to_string()));

                } else {
                    return (LinkUrl(String::new()), LinkText(String::new()));
                }
            } else {
                return (LinkUrl(String::new()), LinkText(String::new()));
            }
        } else {
            return (LinkUrl(String::new()), LinkText(String::new()));
        }
    } else {
        return (LinkUrl(String::new()), LinkText(String::new()));
    }
    
    //todo!()
}
