use std::io;

pub trait ReadInput {
    fn read_input(&mut self) -> io::Result<String>;
}

pub struct Reader<R> {
    reader: R,
}

impl<R> ReadInput for Reader<R>
where
    R: io::BufRead,
{
    fn read_input(&mut self) -> io::Result<String> {
        let mut input = String::new();
        self.reader.read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}

impl<R> Reader<R> {
    pub const fn new(reader: R) -> Self {
        Self { reader }
    }
}
