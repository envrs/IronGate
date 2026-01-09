use irongate_buf_reader::{AsyncBufReadExt, AsyncBufReader};
use std::io::Cursor;
use tokio::io::AsyncReadExt;

#[tokio::test]
async fn test_peek_and_read() {
    let data = b"Hello, world!";
    let cursor = Cursor::new(data);
    let mut reader = AsyncBufReader::new(cursor);

    // Peek 5 bytes
    let peeked = reader.peek(5).await.unwrap();
    assert_eq!(peeked, b"Hello");

    // Buffer should contain more or equal bytes
    assert!(reader.buffer().len() >= 5);

    // Read 5 bytes
    let mut buf = [0u8; 5];
    reader.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"Hello");

    // Consume is handled by read_exact (which uses poll_read)
    // But let's test manual consume with peek

    let peeked = reader.peek(2).await.unwrap();
    assert_eq!(peeked, b", ");

    reader.consume(2);

    let mut remaining = String::new();
    reader.read_to_string(&mut remaining).await.unwrap();
    assert_eq!(remaining, "world!");
}

#[tokio::test]
async fn test_large_peek() {
    // Test peeking more than default chunk size
    let data = vec![0u8; 20000];
    let cursor = Cursor::new(data.clone());
    let mut reader = AsyncBufReader::with_chunk_size(1024, cursor);

    // Peek 10000 bytes (larger than chunk size)
    let peeked = reader.peek(10000).await.unwrap();
    assert_eq!(peeked.len(), 10000);
    assert_eq!(peeked, &data[..10000]);
}

#[tokio::test]
async fn test_passthrough() {
    let data = b"passthrough";
    let cursor = Cursor::new(data);
    let mut reader = AsyncBufReader::new(cursor);

    // Enable passthrough via trait if available, but AsyncBufReader has specific method
    // accessing underlying trait method via AsyncBufPassthrough
    use irongate_buf_reader::AsyncBufPassthrough;
    reader.passthrough(true);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf, data);
}
