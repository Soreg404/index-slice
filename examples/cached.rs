use index_slice::IndexSlice;

fn main() {
    struct Parser {
        buffer: Vec<u8>,
        lines: Vec<IndexSlice>
    }
    impl Parser {
        fn push_line(&mut self, line_bytes: &[u8]) {
            let line_idx = IndexSlice::new(0, line_bytes.len())
                .offset_forward(self.buffer.len());

            self.buffer.extend_from_slice(line_bytes);
            self.lines.push(line_idx);
        }
    }

    let mut p = Parser {
        buffer: Vec::new(),
        lines: Vec::new()
    };

    p.push_line(b"hello world!");
    p.push_line(b"Lorem ipsum");
    p.push_line(b"dolor sit amet");
    p.push_line(b"consectetur adipiscing elit");

    println!("buffer contents: {:?}", str::from_utf8(&p.buffer));

    println!("lines:");
    let mut line_n = 1usize;
    for line_idx in p.lines {
        println!(
            "{line_n} {:?}",
            str::from_utf8(line_idx.as_slice_of(&p.buffer))
        );
        line_n += 1;
    }
}
