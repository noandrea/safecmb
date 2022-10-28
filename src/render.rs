use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, Pdf};

pub fn render_pdf(output_file: &String, keys: &[usize], words: &Vec<String>) {
    let mut document = Pdf::create(output_file).unwrap();
    document.set_title("Text example");
    document
        .render_page(300.0, 400.0, |c| {
            let helvetica = BuiltinFont::Helvetica;
            c.left_text(10.0, 380.0, BuiltinFont::Times_Roman, 10.0, "MNEMONIC")?;
            c.right_text(290.0, 380.0, BuiltinFont::Times_Roman, 10.0, "SafeCMB")?;

            let mut x = 50.0;
            let mut y = 330.0;
            let fs_n = 12.0;
            let fs_s = 8.0;
            let line_color = Color::rgb(173, 216, 230);
            let line_h = 22.0;

            for (i, w) in words.iter().enumerate().take(words.len() / 2) {
                y -= line_h;
                c.left_text(x, y, helvetica, fs_s, &format!("{:02}", i + 1))?;
                c.left_text(x + 22.0, y, helvetica, fs_n, w)?;
                // draw the separation lines
                c.set_stroke_color(line_color)?;
                let ly = y - line_h / 3.0;
                c.line(10.0, ly, 290.0, ly)?;
                c.stroke()?;
            }

            // updated the coordinates
            x = 175.0;
            y = 330.0;

            for (i, w) in words.iter().enumerate().skip(words.len() / 2) {
                y -= line_h;
                c.left_text(x, y, helvetica, fs_s, &format!("{:02}", i + 1))?;
                c.left_text(x + 22.0, y, helvetica, fs_n, w)?;
            }

            let t = keys
                .iter()
                .map(|v| format!("{:02}", v + 1))
                .collect::<Vec<String>>()
                .join("-");

            // draw the separation lines
            c.set_stroke_color(Color::rgb(10, 10, 10))?;
            c.line(0.0, 24.0, 300.0, 25.0)?;
            c.stroke()?;
            c.left_text(10.0, 20.0, BuiltinFont::ZapfDingbats, 12.0, "✂")?;
            c.center_text(150.0, 10.0, BuiltinFont::Courier, 6.0, &t)?;
            Ok(())
        })
        .unwrap();
    document.finish().unwrap();
}
