fn outline(bencher: &mut bencher::Bencher) {
    let font_data = std::fs::read("benches/font.ttf").expect("see readme");
    bencher.iter(|| {
        let font = ttf_parser::Font::from_data(&font_data, 0).unwrap();
        for id in 0..font.number_of_glyphs() {
            let glyph = match font.glyph(id) {
                Ok(v) => v,
                Err(_) => continue,
            };

            glyph.outline(&mut Builder(0));
        }
    })
}

struct Builder(usize);

impl ttf_parser::glyf::OutlineBuilder for Builder {
    #[inline]
    fn move_to(&mut self, _: f32, _: f32) {
        self.0 += 1;
    }

    #[inline]
    fn line_to(&mut self, _: f32, _: f32) {
        self.0 += 1;
    }

    #[inline]
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {
        self.0 += 2;
    }
}

bencher::benchmark_group!(outline_group, outline);
bencher::benchmark_main!(outline_group);
