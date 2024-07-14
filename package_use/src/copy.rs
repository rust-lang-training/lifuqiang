
pub mod copy {
    use std::io::{self, Read, Write};

    pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<usize>
        where R: Read, W: Write
    {
        const BUF_SIZE: usize = 1024 * 64;
        let mut buf = [0; BUF_SIZE];
        let mut write_len: usize = 0;

        println!(">>>>{} kb", BUF_SIZE / 1024);

        loop {
            let len = match reader.read(&mut buf) {
                Ok(0) => return Ok(write_len),
                Ok(len) => len,
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => write_len,
                Err(e) => return Err(e),
            };

            let written = writer.write_all(&buf[0..len])?;
            write_len += len;
        }
    }
}