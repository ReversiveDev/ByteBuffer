use std::io::prelude::*;
use std::net::TcpStream;

struct ByteBuffer {
	data: [u8; 1024],
	offset: usize
}

impl ByteBuffer {

	fn new() -> ByteBuffer {

		return ByteBuffer {
			data: [0; 1024],
			offset: 0
		}

	}

}

trait BufferWriter {
	fn write_byte(&mut self, value: u8);
	fn compose(&self, out: &mut [u8]);
}

impl BufferWriter for ByteBuffer {

	fn write_byte(&mut self, value: u8) {

		self.data[self.offset] = value;
		self.offset += 1;

	}

	fn compose(&self, out: &mut [u8]) {

		for i in 0..self.offset {
			out[i] = self.data[i];
		}

	}

}

fn main() -> std::io::Result<()> {

	let mut b = ByteBuffer::new();
	
	b.write_byte(10);
	b.write_byte(50);

	let mut out = vec![0; b.offset];
	b.compose(&mut out);

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

	stream.write(&out)?;
    // stream.read(&mut [0; 128]);
	Ok(())
}