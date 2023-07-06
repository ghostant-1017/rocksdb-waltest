use rocksdb::{WriteBatch, WriteBatchIterator};
pub const PREFIX_LEN: usize = 4; // N::ID (u16) + DataID (u16)

fn main() {
    let mut options = rocksdb::Options::default();
    options.set_compression_type(rocksdb::DBCompressionType::Lz4);
    let prefix_extractor = rocksdb::SliceTransform::create_fixed_prefix(PREFIX_LEN);
    options.set_prefix_extractor(prefix_extractor);
    options.increase_parallelism(2);
    options.create_if_missing(true);
    let db = rocksdb::DB::open(&options, "/rocksdbtest/db").unwrap();
    let mut batch = WriteBatch::default();
    for i in 0..100000 {
        let key = format!("test-key-{i}");
        let value = format!("test-value-{i}");
        batch.put(&key, &value);
    }
    if let Err(err) = db.write(batch) {
        println!("err: {}", err);
        let mut batch = WriteBatch::default();
        batch.put("1", "2");
        db.write(batch).unwrap();
        let result = db.get("test-key-1");
        println!("{:?}", result);
    }
}
