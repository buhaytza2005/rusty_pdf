use imagesize::{blob_size, ImageSize};
use lopdf::Document;
use std::{fs, io::Cursor};

use rusty_pdf::{Font, PDFSigningDocument, Rectangle};

// #[allow(non_upper_case_globals)]
// const mm2pt: f64 = 2.834;

fn main() {
    let doc_mem = fs::read("examples/pdf_example.pdf").unwrap_or(vec![]);

    let doc = Document::load_mem(&doc_mem).unwrap_or_default();

    let image_mem = fs::read("examples/signature_example.png").unwrap_or(vec![]);

    let dimensions = blob_size(&image_mem).unwrap_or(ImageSize {
        width: 0,
        height: 0,
    });

    let scaled_vec = Rectangle::scale_image_on_width(
        150.0,
        200.0,
        500.0,
        (dimensions.width as f64, dimensions.height as f64),
    );

    let scaled_2 = Rectangle::scale_image_on_width(
        400.0,
        0.0,
        0.0,
        (dimensions.width as f64, dimensions.height as f64),
    );

    let file = Cursor::new(image_mem);
    let mut test_doc = PDFSigningDocument::new(doc);
    let object_id = test_doc.add_object_from_scaled_vec(scaled_vec);
    let object_id_2 = test_doc.add_object_from_scaled_vec(scaled_2);
    let page_id = *test_doc
        .get_document_ref()
        .get_pages()
        .get(&1)
        .unwrap_or(&(0, 0));

    test_doc
        .add_signature_to_form(file.clone(), "signature_1", page_id, object_id)
        .unwrap();

    test_doc
        .add_signature_to_form(file, "signature_1", page_id, object_id_2)
        .unwrap();

    test_doc
        .add_text_to_doc(
            "Hello from abstracted function",
            (0.0, 250.0),
            Font::Courier,
            27.0,
            page_id,
        )
        .unwrap();

    test_doc
        .add_text_to_doc("Hello again", (0.0, 400.0), Font::Courier, 10.0, page_id)
        .unwrap();

    test_doc.finished().save("new_pdf_with_data.pdf").unwrap();
}