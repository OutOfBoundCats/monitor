pub struct HttpWriter;

impl std::io::Write for HttpWriter {
    #[tracing::instrument(skip(self, buf))]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();

        //println!("from http wrtiter{:?}", buf);

        let _s = match std::str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        //http post

        //tracing::info!("buffer in string is {}", &s);
        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
