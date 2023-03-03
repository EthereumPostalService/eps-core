use std::env;

use genpdf::{Alignment, Size, PaperSize};

pub fn gen_pdf(id: String, msg: String) -> anyhow::Result<()> {
    // Load a font from the file system
    let p = env::current_dir();
    println!("{:?}", p);
    let font_family = genpdf::fonts::from_files("./assets/fonts", "monospace", None)
        .expect("Failed to load font family");
    // Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);
    doc.set_paper_size(PaperSize::Letter);
    // Change the default settings
    doc.set_title("Demo document");
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    // Add one or more elements
    let mut logo = genpdf::elements::Image::from_path("./assets/imgs/eps-logo.jpg")
        .expect("failed to load logo");
    logo.set_alignment(Alignment::Center);
    doc.push(logo);
    doc.push(genpdf::elements::Break::new(3)); 
    doc.push(genpdf::elements::Paragraph::new(msg));
    // Render the document and write it to a file
    doc.render_to_file(format!("out/{}.pdf", id))
        .expect("Failed to write PDF file");
    Ok(())
}

#[cfg(test)]
mod test {
    use lipsum::lipsum;
    use crate::pdf::*;
    #[tokio::test]
    async fn test_create_pdf() {
        gen_pdf("test".to_string(), "This is the message".to_string()).unwrap();
    }
    #[tokio::test]
    async fn test_create_long_pdf() {
        let msg = lipsum(5000);
        gen_pdf("testlong".to_string(), msg).unwrap();
    }
}
